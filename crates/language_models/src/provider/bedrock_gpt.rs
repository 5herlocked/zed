use std::sync::{Arc, LazyLock};

use anyhow::{Result, anyhow};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, ProviderConfigurationView, RateLimiter, env_var,
};
use open_ai::responses::{
    Request as ResponseRequest, StreamEvent as ResponsesStreamEvent, stream_response,
};
use settings::{OpenAiAvailableModel as AvailableModel, Settings, SettingsStore};
use ui::prelude::*;
use util::ResultExt;

pub use open_ai::completion::{OpenAiResponseEventMapper, into_open_ai_response};

use crate::AllLanguageModelSettings;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("bedrock-gpt");
const PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Amazon Bedrock GPT");

const API_KEY_ENV_VAR_NAME: &str = "ZED_BEDROCK_GPT_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

/// IAM access key environment variable for fallback token exchange.
static IAM_ACCESS_KEY_VAR: LazyLock<EnvVar> = env_var!("ZED_BEDROCK_GPT_ACCESS_KEY_ID");
/// IAM secret key environment variable for fallback token exchange.
static IAM_SECRET_KEY_VAR: LazyLock<EnvVar> = env_var!("ZED_BEDROCK_GPT_SECRET_ACCESS_KEY");
/// IAM session token environment variable (optional, for temporary credentials).
static IAM_SESSION_TOKEN_VAR: LazyLock<EnvVar> = env_var!("ZED_BEDROCK_GPT_SESSION_TOKEN");

const DEFAULT_REGION: &str = "us-east-1";

/// Models available via Bedrock's bedrock-mantle endpoint.
#[derive(Clone, Debug)]
pub struct BedrockGptModel {
    pub id: String,
    pub display_name: String,
    pub max_tokens: u64,
    pub max_output_tokens: Option<u64>,
    pub supports_images: bool,
}

impl BedrockGptModel {
    fn builtin_models() -> Vec<Self> {
        vec![
            Self {
                id: "gpt-5.4".into(),
                display_name: "GPT-5.4".into(),
                max_tokens: 1_050_000,
                max_output_tokens: Some(128_000),
                supports_images: true,
            },
            Self {
                id: "gpt-5.4-mini".into(),
                display_name: "GPT-5.4 Mini".into(),
                max_tokens: 400_000,
                max_output_tokens: Some(128_000),
                supports_images: true,
            },
            Self {
                id: "gpt-5.5".into(),
                display_name: "GPT-5.5".into(),
                max_tokens: 1_050_000,
                max_output_tokens: Some(128_000),
                supports_images: true,
            },
            Self {
                id: "gpt-5.5-pro".into(),
                display_name: "GPT-5.5 Pro".into(),
                max_tokens: 1_050_000,
                max_output_tokens: Some(128_000),
                supports_images: true,
            },
        ]
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct BedrockGptSettings {
    pub api_url: Option<String>,
    pub region: Option<String>,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct State {
    api_key_state: ApiKeyState,
    /// Cached bearer token obtained via IAM credential exchange.
    iam_derived_token: Option<String>,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key() || self.iam_derived_token.is_some()
    }

    /// Returns the resolved bearer token: direct API key takes priority,
    /// then IAM-exchanged token.
    fn bearer_token(&self, api_url: &str) -> Option<String> {
        self.api_key_state
            .key(api_url)
            .or_else(|| self.iam_derived_token.clone())
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.iam_derived_token = None;
        let credentials_provider = self.credentials_provider.clone();
        let api_url = BedrockGptLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        // Stage 1: Try loading direct API key (env var or keychain).
        let credentials_provider = self.credentials_provider.clone();
        let api_url = BedrockGptLanguageModelProvider::api_url(cx);

        cx.spawn(async move |this, cx| {
            // Try to load stored/env API key first.
            let load_result = this.update(cx, |this, cx| {
                this.api_key_state.load_if_needed(
                    api_url.clone(),
                    |this| &mut this.api_key_state,
                    credentials_provider.clone(),
                    cx,
                )
            })?;
            let _ = load_result.await;

            let has_direct_key = this.read_with(cx, |this, _| this.api_key_state.has_key());
            if has_direct_key {
                return Ok(());
            }

            // Stage 2: Attempt IAM credential exchange.
            let access_key = IAM_ACCESS_KEY_VAR.value.clone();
            let secret_key = IAM_SECRET_KEY_VAR.value.clone();

            if let (Some(access_key), Some(secret_key)) = (access_key, secret_key) {
                let session_token = IAM_SESSION_TOKEN_VAR.value.clone();
                let region = this.read_with(cx, |_, cx| {
                    BedrockGptLanguageModelProvider::region(cx).to_string()
                });

                match exchange_iam_for_bearer_token(
                    &access_key,
                    &secret_key,
                    session_token.as_deref(),
                    &region,
                )
                .await
                {
                    Ok(token) => {
                        this.update(cx, |this, cx| {
                            this.iam_derived_token = Some(token);
                            cx.notify();
                        })?;
                        return Ok(());
                    }
                    Err(e) => {
                        log::error!("Bedrock GPT IAM token exchange failed: {e}");
                        return Err(AuthenticateError::Other(anyhow!(
                            "IAM token exchange failed: {e}"
                        )));
                    }
                }
            }

            Err(AuthenticateError::NoCredentials)
        })
    }
}

/// Exchange IAM credentials for a short-lived bearer token via the Bedrock
/// token generator endpoint. Returns the bearer token string on success.
///
/// This is the integration point for "Recipe 1" token generation.
/// Currently returns an error indicating the exchange is not yet configured;
/// replace the body with the actual token-generation call when available.
async fn exchange_iam_for_bearer_token(
    _access_key_id: &str,
    _secret_access_key: &str,
    _session_token: Option<&str>,
    _region: &str,
) -> Result<String> {
    // TODO: Implement actual IAM-to-bearer token exchange via Recipe 1's
    // token generator. The implementation should:
    // 1. Sign a request using the IAM credentials (SigV4).
    // 2. Call the Bedrock token endpoint for the given region.
    // 3. Return the short-lived bearer token from the response.
    Err(anyhow!(
        "IAM credential exchange is not yet configured. \
        Set {} with a Bedrock API bearer token, or implement the token generator.",
        API_KEY_ENV_VAR_NAME
    ))
}

pub struct BedrockGptLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

impl BedrockGptLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                iam_derived_token: None,
                credentials_provider,
            }
        });

        Self { http_client, state }
    }

    fn settings(cx: &App) -> &BedrockGptSettings {
        &crate::AllLanguageModelSettings::get_global(cx).bedrock_gpt
    }

    fn region(cx: &App) -> &str {
        Self::settings(cx)
            .region
            .as_deref()
            .unwrap_or(DEFAULT_REGION)
    }

    fn api_url(cx: &App) -> SharedString {
        let settings = Self::settings(cx);
        if let Some(url) = settings.api_url.as_deref().filter(|u| !u.is_empty()) {
            SharedString::new(url)
        } else {
            let region = settings.region.as_deref().unwrap_or(DEFAULT_REGION);
            SharedString::from(format!("https://bedrock-mantle.{region}.api.aws/v1"))
        }
    }

    fn create_language_model(&self, model: BedrockGptModel) -> Arc<dyn LanguageModel> {
        Arc::new(BedrockGptLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for BedrockGptLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for BedrockGptLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiBedrock)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let models = BedrockGptModel::builtin_models();
        models
            .into_iter()
            .next()
            .map(|m| self.create_language_model(m))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let models = BedrockGptModel::builtin_models();
        models
            .into_iter()
            .find(|m| m.id == "gpt-5.4-mini")
            .map(|m| self.create_language_model(m))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::new();

        for m in BedrockGptModel::builtin_models() {
            models.insert(m.id.clone(), m);
        }

        for m in &Self::settings(cx).available_models {
            models.insert(
                m.name.clone(),
                BedrockGptModel {
                    id: m.name.clone(),
                    display_name: m.display_name.clone().unwrap_or_else(|| m.name.clone()),
                    max_tokens: m.max_tokens,
                    max_output_tokens: m.max_output_tokens,
                    supports_images: true,
                },
            );
        }

        models
            .into_values()
            .map(|m| self.create_language_model(m))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }

    fn configuration_view_v2(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> ProviderConfigurationView {
        let state = self.state.clone();
        ProviderConfigurationView::Inline(
            cx.new(|cx| {
                crate::ApiKeyEditor::new(
                    state,
                    "https://console.aws.amazon.com/bedrock",
                    "Bearer token...",
                    |state, _cx| crate::api_key_status(&state.api_key_state),
                    |state, key, cx| state.update(cx, |state, cx| state.set_api_key(Some(key), cx)),
                    |state, cx| state.update(cx, |state, cx| state.set_api_key(None, cx)),
                    window,
                    cx,
                )
            })
            .into(),
        )
    }
}

pub struct BedrockGptLanguageModel {
    id: LanguageModelId,
    model: BedrockGptModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl BedrockGptLanguageModel {
    fn stream_response(
        &self,
        request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = BedrockGptLanguageModelProvider::api_url(cx);
            let extra_headers = BedrockGptLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.bearer_token(&api_url), api_url, extra_headers)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let response = stream_response(
                http_client.as_ref(),
                PROVIDER_NAME.0.as_str(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            )
            .await
            .map_err(|e| match &e {
                open_ai::RequestError::HttpResponseError {
                    status_code,
                    body,
                    ..
                } => {
                    if status_code.as_u16() == 401 || status_code.as_u16() == 403 {
                        LanguageModelCompletionError::PermissionError {
                            provider: PROVIDER_NAME,
                            message: format!(
                                "Authentication failed (HTTP {}). Verify your Bedrock GPT API key or IAM credentials have the required permissions. Details: {}",
                                status_code, body
                            ),
                        }
                    } else {
                        LanguageModelCompletionError::HttpResponseError {
                            provider: PROVIDER_NAME,
                            status_code: *status_code,
                            message: body.clone(),
                        }
                    }
                }
                open_ai::RequestError::Other(err) => {
                    LanguageModelCompletionError::HttpSend {
                        provider: PROVIDER_NAME,
                        error: anyhow::anyhow!("{err}"),
                    }
                }
            })?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for BedrockGptLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        format!("bedrock-gpt/{}", self.model.id)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_open_ai_response(
            request,
            &self.model.id,
            false, // supports_parallel_tool_calls
            true,  // supports_prompt_cache_key
            self.max_output_tokens(),
            None,  // default_reasoning_effort
            false, // supports_none_reasoning_effort
        );
        let completions = self.stream_response(request, cx);
        async move {
            let mapper = OpenAiResponseEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| cx.notify()).detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            state,
            load_credentials_task,
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        v_flex().child(if is_authenticated {
            Label::new("Bedrock GPT API key configured.").into_any_element()
        } else {
            Label::new(format!(
                "Set the {} environment variable or configure a key in settings.",
                API_KEY_ENV_VAR_NAME
            ))
            .into_any_element()
        })
    }
}
