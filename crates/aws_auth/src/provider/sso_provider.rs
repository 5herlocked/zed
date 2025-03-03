use anyhow::{anyhow, Context, Error, Result};
use aws_credential_types::provider::future::ProvideToken as TokenProviderFuture;
use aws_credential_types::Token;
// use aws_sdk_sso::Client as SsoClient;
use aws_config::Region;
use gpui::{App, AppContext, Context as GpuiContext, Global, ReadGlobal, Task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tokio::sync::OnceCell;

use crate::{AuthError, SsoOidcClient};
// use uuid::Uuid;

/// Token representing an SSO access token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SsoToken {
    pub access_token: String,
    pub expires_at: OffsetDateTime,
    pub refresh_token: Option<String>,
    pub identity: Option<String>,
}

impl SsoToken {
    pub fn is_expired(&self) -> bool {
        // Add a small buffer (5 minutes) to ensure we don't use tokens that are about to expire
        let buffer = Duration::seconds(5 * 60);
        OffsetDateTime::now_utc() + buffer >= self.expires_at
    }
}

/// Client registration for SSO OAuth flow
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientRegistration {
    pub client_id: String,
    pub client_secret: String,
    pub expires_at: OffsetDateTime,
    pub flow: String, // Indicates the auth flow type ("device_code", "auth_code", etc.)
}

impl ClientRegistration {
    pub fn is_expired(&self) -> bool {
        // Add a small buffer (5 minutes) to ensure we don't use tokens that are about to expire
        let buffer = Duration::seconds(5 * 60);
        OffsetDateTime::now_utc() + buffer >= self.expires_at
    }
}

/// Token store for managing SSO tokens and client registrations
#[derive(Default)]
pub struct TokenStore {
    tokens: HashMap<String, SsoToken>,
    registrations: HashMap<String, ClientRegistration>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            registrations: HashMap::new(),
        }
    }

    // Token methods
    pub fn store_token(&mut self, connection_id: &str, token: SsoToken) {
        self.tokens.insert(connection_id.to_string(), token);
    }

    pub fn get_token(&self, connection_id: &str) -> Option<SsoToken> {
        self.tokens.get(connection_id).cloned()
    }

    pub fn remove_token(&mut self, connection_id: &str) {
        self.tokens.remove(connection_id);
    }

    // Registration methods
    pub fn store_registration(&mut self, key: &str, registration: ClientRegistration) {
        self.registrations.insert(key.to_string(), registration);
    }

    pub fn get_registration(&self, key: &str) -> Option<ClientRegistration> {
        self.registrations.get(key).cloned()
    }

    pub fn remove_registration(&mut self, key: &str) {
        self.registrations.remove(key);
    }

    pub fn invalidate_all(&mut self) {
        self.tokens.clear();
        self.registrations.clear();
    }
}

/// SSO Access Token Provider - handles token creation and management
pub struct SsoAccessTokenProvider {
    identifier: String,
    start_url: String,
    region: String,
    scopes: Vec<String>,
    token_store: Arc<Mutex<TokenStore>>,
}

impl SsoAccessTokenProvider {
    pub fn new(
        identifier: String,
        start_url: String,
        region: String,
        scopes: Vec<String>,
        token_store: Arc<Mutex<TokenStore>>,
    ) -> Self {
        Self {
            identifier,
            start_url,
            region,
            scopes,
            token_store,
        }
    }

    /// Get the token cache key
    fn token_cache_key(&self) -> String {
        self.identifier.clone()
    }

    /// Get the registration cache key
    fn registration_cache_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.start_url,
            self.region,
            self.scopes.join(",")
        )
    }

    /// Invalidate the current token
    pub async fn invalidate(&self, reason: &str) -> Result<(), AuthError> {
        println!(
            "SsoAccessTokenProvider: Invalidating token and registration: {}",
            reason
        );

        let mut token_store = self.token_store.lock().unwrap();
        token_store.remove_token(&self.token_cache_key());
        token_store.remove_registration(&self.registration_cache_key());

        Ok(())
    }

    /// Get cached token if available
    pub async fn get_token(&self) -> Option<SsoToken> {
        let token_store = self.token_store.lock().unwrap();
        let token = token_store.get_token(&self.token_cache_key());

        // Return token if it exists and is not expired
        if let Some(token) = token {
            if !token.is_expired() {
                return Some(token);
            }
        }

        None
    }

    /// Register a client with AWS SSO OIDC
    async fn register_client(
        &self,
        client: SsoOidcClient,
    ) -> Result<ClientRegistration, AuthError> {
        let client_name = format!("Zed IDE");
        let client_type = "public";

        let response = client
            .register_client()
            .client_name(client_name)
            .client_type(client_type)
            .set_scopes(Some(self.scopes.clone()))
            .send()
            .await
            .map_err(|e| AuthError::AwsSdkError(e.to_string()))?;

        Ok(ClientRegistration {
            client_id: response
                .client_id()
                .expect("Invalid response from AWS SDK")
                .to_string(),
            client_secret: response
                .client_secret()
                .expect("Invalid response from AWS SDK")
                .to_string(),
            expires_at: OffsetDateTime::from_unix_timestamp(response.client_secret_expires_at)
                .unwrap(),
            flow: "device_code".to_string(),
        })
    }

    async fn authorize(
        &self,
        registration: &ClientRegistration,
        client: SsoOidcClient,
    ) -> Result<SsoToken, AuthError> {
        // Start device authorization
        let device_auth = client
            .start_device_authorization()
            .client_id(&registration.client_id)
            .client_secret(&registration.client_secret)
            .start_url(&self.start_url)
            .send()
            .await
            .map_err(|e| AuthError::AwsSdkError(e.to_string()))?;

        let verification_uri = device_auth.verification_uri_complete().unwrap();

        // TOOD: Figure out how to get GPUI to report this
        println!("Please open: {}", verification_uri);
        println!("User code: {}", device_auth.user_code().unwrap());

        // TODO: Implement proper browser opening for authentication

        // Poll for token completion
        let device_code = device_auth.device_code().unwrap().to_string();
        let interval = device_auth.interval();
        let expiry_time =
            OffsetDateTime::now_utc() + Duration::seconds(device_auth.expires_in() as i64);

        // Poll until we get a token or timeout
        while OffsetDateTime::now_utc() < expiry_time {
            tokio::time::sleep(std::time::Duration::from_secs(interval as u64)).await;

            match client
                .create_token()
                .client_id(&registration.client_id)
                .client_secret(&registration.client_secret)
                .grant_type("urn:ietf:params:oauth:grant-type:device_code")
                .device_code(&device_code)
                .send()
                .await
            {
                Ok(token_result) => {
                    // Create our token structure
                    let expires_in = token_result.expires_in() as u64;
                    let token = SsoToken {
                        access_token: token_result.access_token().unwrap().to_string(),
                        expires_at: Instant::now() + Duration::from_secs(expires_in),
                        refresh_token: token_result.refresh_token().map(|s| s.to_string()),
                        identity: Some(self.token_cache_key()),
                    };

                    return Ok(token);
                }
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.contains("slow_down") {
                        // If we're told to slow down, wait a bit longer
                        tokio::time::sleep(Duration::from_secs(interval as u64 * 2)).await;
                    } else if error_str.contains("authorization_pending") {
                        // This is expected while waiting for user to authenticate
                        continue;
                    } else {
                        // Any other error is a failure
                        return Err(AuthError::AwsSdkError(error_str));
                    }
                }
            }
        }

        // If we've reached this point, we timed out
        Err(AuthError::Timeout)
    }
}
