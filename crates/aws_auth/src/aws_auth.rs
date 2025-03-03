mod provider;

use std::cell::OnceCell;
use std::sync::Arc;

use anyhow::{anyhow, Context, Error, Result};
use aws_config::Region;
use aws_credential_types::provider::future::ProvideCredentials as FutureProvider;
use aws_credential_types::provider::ProvideCredentials;
use aws_credential_types::Credentials;
use aws_credential_types::Token;
use aws_http_client::AwsHttpClient;
pub(crate) use aws_sdk_ssooidc::Client as SsoOidcClient;
use aws_sdk_ssooidc::Config;
use gpui::http_client::HttpClient;
use gpui::{App, AppContext, Global, ReadGlobal};
use thiserror::Error;

pub fn init(cx: &mut App, handle: tokio::runtime::Handle, http_client: Arc<dyn HttpClient>) {
    cx.set_global(GlobalAwsAuthProvider::new(handle, http_client));
}

/// Types of AWS connection authentication methods
#[derive(Debug, Clone)]
pub enum AwsConnectionType {
    /// SSO-based authentication
    Sso {
        start_url: String,
        sso_region: String,
        scopes: Vec<String>,
    },
    /// IAM credential-based authentication
    Iam { profile_name: String },
    /// Environment variables-based authentication
    Environment,
    /// EC2 instance metadata-based authentication
    InstanceMetadata,
}

/// Possible connection states
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Valid,
    Invalid,
    Authenticating,
    Unauthenticated,
}

struct GlobalAwsAuthProvider {
    handle: tokio::runtime::Handle,
    sso_oidc_client: OnceCell<SsoOidcClient>,
}

impl GlobalAwsAuthProvider {
    fn new(handle: tokio::runtime::Handle, http_client: Arc<dyn HttpClient>) -> Self {
        let coerced_client = AwsHttpClient::new(http_client.clone(), handle.clone());

        let ssooidc_client = SsoOidcClient::from_conf(
            Config::builder()
                .http_client(coerced_client.clone())
                .build(),
        );

        Self {
            handle,
            sso_oidc_client: OnceCell::from(ssooidc_client),
        }
    }
}

impl Global for GlobalAwsAuthProvider {}

#[derive(Debug)]
pub struct AwsAuthProvider {}

impl ProvideCredentials for AwsAuthProvider {
    /// Provides Credentials
    fn provide_credentials<'a>(&'a self) -> FutureProvider<'a>
    where
        Self: 'a,
    {
        todo!()
    }

    fn fallback_on_interrupt(&self) -> Option<Credentials> {
        let creds = Credentials::for_tests();
        Some(creds)
    }
}

/// Errors that can occur during authentication operations
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("Token expired")]
    TokenExpired,

    #[error("No active connection")]
    NoActiveConnection,

    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),

    #[error("AWS SDK error: {0}")]
    AwsSdkError(String),

    #[error("Authorization pending")]
    AuthorizationPending,

    #[error("Slow down")]
    SlowDown,

    #[error("User cancelled")]
    UserCancelled,

    #[error("Timeout")]
    Timeout,
}
