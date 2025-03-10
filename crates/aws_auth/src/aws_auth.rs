mod provider;

use std::cell::OnceCell;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Error, Result};
use aws_config::Region;
use aws_credential_types::provider::future::ProvideCredentials as FutureProvider;
use aws_credential_types::provider::ProvideCredentials;
use aws_credential_types::Credentials;
use aws_http_client::AwsHttpClient;
pub(crate) use aws_sdk_sso::Client as SsoClient;
pub(crate) use aws_sdk_ssooidc::Client as SsoOidcClient;
use aws_sdk_ssooidc::Config as OidcConfig;
use aws_sdk_sso::Config as SsoConfig;
use gpui::http_client::HttpClient;
use gpui::{App, AppContext, Global, ReadGlobal};
use provider::sso_provider::{ClientRegistration, SsoToken};
use std::collections::HashMap;
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
    region: Option<Region>,
    http_client: Arc<dyn HttpClient>,
    ssooidc_client: OnceCell<SsoOidcClient>,
    sso_client: OnceCell<SsoClient>,
}

impl GlobalAwsAuthProvider {
    fn new(handle: tokio::runtime::Handle, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            handle,
            region: None,
            http_client: http_client.clone(),
            ssooidc_client: OnceCell::new(),
            sso_client: OnceCell::new(),
        }
    }

    fn ssooidc_client(&self) -> &SsoOidcClient {
        self.ssooidc_client.get_or_init(|| {
            let coerced_client = AwsHttpClient::new(self.http_client.clone(), self.handle.clone());

            let ssooidc_client = SsoOidcClient::from_conf(
                OidcConfig::builder()
                    .http_client(coerced_client.clone())
                    .build(),
            );

            ssooidc_client
        })
    }

    fn sso_client(&self) -> &SsoClient {
        self.sso_client.get_or_init(|| {
            let coerced_client = AwsHttpClient::new(self.http_client.clone(), self.handle.clone());

            let sso_client = SsoClient::from_conf(
                SsoConfig::builder()
                    .http_client(coerced_client.clone())
                    .build(),
            );

            sso_client
        })
    }
}

impl Global for GlobalAwsAuthProvider {}

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

pub struct CredentialCache {
    credentials: HashMap<String, Credentials>,
    registrations: HashMap<String, ClientRegistration>,
}

impl CredentialCache {
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
            registrations: HashMap::new(),
        }
    }

    /// Stores a new credential in the cache.
    pub fn store_credential(&mut self, connection_id: String, credential: Credentials) {
        self.credentials.insert(connection_id, credential);
    }

    /// Retrieves a credential from the cache if it is still valid.
    pub fn get_credential(&self, connection_id: &str) -> Option<Credentials> {
        if let Some(credential) = self.credentials.get(connection_id) {
            if let Some(expiry) = credential.expiry() {
                if expiry > SystemTime::now() {
                    return Some(credential.clone());
                }
            } else {
                return Some(credential.clone()); // No expiry set, consider it valid
            }
        }
        None
    }

    /// Removes a credential from the cache.
    pub fn remove_credential(&mut self, connection_id: &str) -> Option<()> {
        self.credentials.remove(connection_id).map(|_| ())
    }

    /// Stores a new client registration in the cache.
    pub fn store_registration(&mut self, connection_id: String, registration: ClientRegistration) {
        self.registrations.insert(connection_id, registration);
    }

    /// Retrieves a client registration from the cache.
    pub fn get_registration(&self, connection_id: &str) -> Option<ClientRegistration> {
        self.registrations.get(connection_id).cloned()
    }

    /// Removes a client registration from the cache.
    pub fn remove_registration(&mut self, connection_id: &str) -> Option<()> {
        self.registrations.remove(connection_id).map(|_| ())
    }

    /// Invalidates all cached credentials and registrations.
    pub fn invalidate_all(&mut self) {
        self.credentials.clear();
        self.registrations.clear();
    }
}
