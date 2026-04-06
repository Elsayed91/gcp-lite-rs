//! Authorized user credential implementation for GCP authentication.
//!
//! This module provides `AuthorizedUserCredential`, which implements the `TokenProvider`
//! trait for authenticating with GCP APIs using OAuth2 authorized user credentials.
//!
//! These credentials are typically obtained from `gcloud auth application-default login`
//! or other OAuth2 consent flows where a user authorizes an application to access
//! their GCP resources.
//!
//! # Example
//!
//! ```no_run
//! use gcp_lite::auth::AuthorizedUserCredential;
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // From individual parameters
//! let cred = AuthorizedUserCredential::new(
//!     "client_id.apps.googleusercontent.com",
//!     "client_secret",
//!     "refresh_token"
//! );
//!
//! // Or from JSON string (e.g., from gcloud)
//! let json = std::fs::read_to_string("/path/to/credentials.json")?;
//! let cred = AuthorizedUserCredential::from_json(&json)?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use serde::Deserialize;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::types::{AccessToken, AuthorizedUserCreds, CachedToken};
use crate::token::{TokenError, TokenProvider};

/// Default expiry buffer in seconds (60 seconds).
/// Tokens will be refreshed this many seconds before actual expiry.
const TOKEN_EXPIRY_BUFFER_SECS: u64 = 60;

/// Token endpoint for Google OAuth2.
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

/// Response from the OAuth2 token endpoint.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    /// The access token.
    access_token: String,
    /// Token lifetime in seconds.
    expires_in: u64,
    /// Token type (usually "Bearer").
    #[allow(dead_code)]
    token_type: String,
    /// Optional scope (returned by Google OAuth2).
    #[allow(dead_code)]
    scope: Option<String>,
}

/// A credential provider that uses authorized user credentials to obtain access tokens.
///
/// This credential type:
/// - Uses OAuth2 refresh tokens to obtain access tokens
/// - Exchanges refresh tokens via Google's OAuth2 token endpoint
/// - Caches tokens with a 60-second expiry buffer
///
/// # Thread Safety
///
/// `AuthorizedUserCredential` is `Send + Sync` and can be safely shared across threads.
/// Token caching is handled internally using `tokio::sync::RwLock`.
#[derive(Debug)]
pub struct AuthorizedUserCredential {
    /// OAuth2 client ID.
    client_id: String,
    /// OAuth2 client secret.
    client_secret: String,
    /// OAuth2 refresh token.
    refresh_token: String,
    /// Optional quota project ID for billing.
    quota_project_id: Option<String>,
    /// Cached access token.
    cache: CachedToken,
    /// HTTP client for token exchange.
    http_client: reqwest::Client,
}

impl AuthorizedUserCredential {
    /// Create a new credential from individual parameters.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The OAuth2 client ID
    /// * `client_secret` - The OAuth2 client secret
    /// * `refresh_token` - The OAuth2 refresh token
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::AuthorizedUserCredential;
    ///
    /// let cred = AuthorizedUserCredential::new(
    ///     "123456789.apps.googleusercontent.com",
    ///     "your-client-secret",
    ///     "1//your-refresh-token"
    /// );
    /// ```
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
            quota_project_id: None,
            cache: CachedToken::new(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new credential from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - The authorized user credentials JSON as a string
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid or missing required fields.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::AuthorizedUserCredential;
    ///
    /// let json = r#"{
    ///     "type": "authorized_user",
    ///     "client_id": "123456789.apps.googleusercontent.com",
    ///     "client_secret": "your-client-secret",
    ///     "refresh_token": "1//your-refresh-token"
    /// }"#;
    ///
    /// let cred = AuthorizedUserCredential::from_json(json).unwrap();
    /// ```
    pub fn from_json(json: &str) -> Result<Self, AuthorizedUserError> {
        let creds: AuthorizedUserCreds =
            serde_json::from_str(json).map_err(|e| AuthorizedUserError::InvalidJson {
                message: e.to_string(),
            })?;

        Self::validate_creds(&creds)?;

        Ok(Self {
            client_id: creds.client_id,
            client_secret: creds.client_secret,
            refresh_token: creds.refresh_token,
            quota_project_id: creds.quota_project_id,
            cache: CachedToken::new(),
            http_client: reqwest::Client::new(),
        })
    }

    /// Create a new credential from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the authorized user credentials JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the JSON is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::AuthorizedUserCredential;
    /// use std::path::Path;
    ///
    /// let cred = AuthorizedUserCredential::from_file(
    ///     Path::new("/path/to/credentials.json")
    /// ).unwrap();
    /// ```
    pub fn from_file(path: &Path) -> Result<Self, AuthorizedUserError> {
        let json =
            std::fs::read_to_string(path).map_err(|e| AuthorizedUserError::FileReadError {
                path: path.to_path_buf(),
                source: e,
            })?;

        Self::from_json(&json)
    }

    /// Create a new credential with a custom HTTP client.
    ///
    /// This is useful for testing or when you need to customize the HTTP client
    /// used for token exchange (e.g., proxy settings, timeouts).
    ///
    /// # Arguments
    ///
    /// * `client_id` - The OAuth2 client ID
    /// * `client_secret` - The OAuth2 client secret
    /// * `refresh_token` - The OAuth2 refresh token
    /// * `http_client` - A custom reqwest::Client to use for token exchange
    pub fn with_http_client(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
        http_client: reqwest::Client,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
            quota_project_id: None,
            cache: CachedToken::new(),
            http_client,
        }
    }

    /// Get the client ID.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Validate that the credentials have all required fields.
    fn validate_creds(creds: &AuthorizedUserCreds) -> Result<(), AuthorizedUserError> {
        if creds.cred_type != "authorized_user" {
            return Err(AuthorizedUserError::InvalidCredType {
                expected: "authorized_user".to_string(),
                actual: creds.cred_type.clone(),
            });
        }

        if creds.client_id.is_empty() {
            return Err(AuthorizedUserError::MissingField {
                field: "client_id".to_string(),
            });
        }

        if creds.client_secret.is_empty() {
            return Err(AuthorizedUserError::MissingField {
                field: "client_secret".to_string(),
            });
        }

        if creds.refresh_token.is_empty() {
            return Err(AuthorizedUserError::MissingField {
                field: "refresh_token".to_string(),
            });
        }

        Ok(())
    }

    /// Exchange the refresh token for a new access token.
    async fn fetch_token(&self) -> Result<AccessToken, AuthorizedUserError> {
        let body = format!(
            "client_id={}&client_secret={}&refresh_token={}&grant_type=refresh_token",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.client_secret),
            urlencoding::encode(&self.refresh_token),
        );

        let response = self
            .http_client
            .post(TOKEN_ENDPOINT)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .map_err(|e| AuthorizedUserError::TokenExchangeFailed {
                message: format!("HTTP request failed: {}", e),
            })?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|e| AuthorizedUserError::TokenExchangeFailed {
                    message: format!("Failed to read response body: {}", e),
                })?;

        if !status.is_success() {
            return Err(AuthorizedUserError::TokenExchangeFailed {
                message: format!("Token endpoint returned {}: {}", status, response_text),
            });
        }

        let token_response: TokenResponse = serde_json::from_str(&response_text).map_err(|e| {
            AuthorizedUserError::TokenExchangeFailed {
                message: format!("Failed to parse token response: {}", e),
            }
        })?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthorizedUserError::TokenExchangeFailed {
                message: format!("Failed to get current time: {}", e),
            })?
            .as_secs();

        Ok(AccessToken::new(
            token_response.access_token,
            now + token_response.expires_in,
        ))
    }
}

#[async_trait]
impl TokenProvider for AuthorizedUserCredential {
    async fn get_token(&self, _scopes: &[&str]) -> Result<String, TokenError> {
        // Check cache first
        if let Some(token) = self.cache.get(TOKEN_EXPIRY_BUFFER_SECS).await {
            return Ok(token);
        }

        // Fetch a new token
        let token = self
            .fetch_token()
            .await
            .map_err(|e| TokenError::RefreshFailed {
                message: e.to_string(),
            })?;

        let token_string = token.token.clone();
        self.cache.set(token).await;

        Ok(token_string)
    }

    fn on_token_rejected(&self) {
        // Clear the cache synchronously when a token is rejected
        // Uses try_write() to avoid blocking - if contended, the next
        // get_token() call will refresh anyway
        self.cache.clear_sync();
    }

    fn quota_project_id(&self) -> Option<&str> {
        self.quota_project_id.as_deref()
    }
}

/// Errors that can occur when using authorized user credentials.
#[derive(Debug, thiserror::Error)]
pub enum AuthorizedUserError {
    /// Failed to read the credentials file.
    #[error("Failed to read credentials file at {path}: {source}")]
    FileReadError {
        /// Path to the file that could not be read.
        path: std::path::PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Invalid JSON format.
    #[error("Invalid JSON: {message}")]
    InvalidJson {
        /// Error description.
        message: String,
    },

    /// Invalid credential type (expected "authorized_user").
    #[error("Invalid credential type: expected '{expected}', got '{actual}'")]
    InvalidCredType {
        /// Expected credential type.
        expected: String,
        /// Actual credential type found.
        actual: String,
    },

    /// Missing required field.
    #[error("Missing required field: {field}")]
    MissingField {
        /// Name of the missing field.
        field: String,
    },

    /// Failed to exchange refresh token for access token.
    #[error("Token exchange failed: {message}")]
    TokenExchangeFailed {
        /// Error description.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_creds_json() -> String {
        r#"{
            "type": "authorized_user",
            "client_id": "test-client-id.apps.googleusercontent.com",
            "client_secret": "test-client-secret",
            "refresh_token": "1//test-refresh-token"
        }"#
        .to_string()
    }

    #[test]
    fn test_new() {
        let cred = AuthorizedUserCredential::new("client-id", "client-secret", "refresh-token");

        assert_eq!(cred.client_id(), "client-id");
    }

    #[test]
    fn test_from_json_valid() {
        let json = test_creds_json();
        let cred = AuthorizedUserCredential::from_json(&json).unwrap();

        assert_eq!(
            cred.client_id(),
            "test-client-id.apps.googleusercontent.com"
        );
    }

    #[test]
    fn test_from_json_with_quota_project() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "test-client-id.apps.googleusercontent.com",
            "client_secret": "test-client-secret",
            "refresh_token": "1//test-refresh-token",
            "quota_project_id": "my-quota-project"
        }"#;

        let cred = AuthorizedUserCredential::from_json(json).unwrap();
        assert_eq!(
            cred.client_id(),
            "test-client-id.apps.googleusercontent.com"
        );
    }

    #[test]
    fn test_from_json_invalid_json() {
        let result = AuthorizedUserCredential::from_json("not valid json");
        assert!(matches!(
            result,
            Err(AuthorizedUserError::InvalidJson { .. })
        ));
    }

    #[test]
    fn test_from_json_wrong_type() {
        let json = r#"{
            "type": "service_account",
            "client_id": "123",
            "client_secret": "secret",
            "refresh_token": "token"
        }"#;

        let result = AuthorizedUserCredential::from_json(json);
        assert!(matches!(
            result,
            Err(AuthorizedUserError::InvalidCredType { .. })
        ));
    }

    #[test]
    fn test_from_json_missing_client_id() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "",
            "client_secret": "secret",
            "refresh_token": "token"
        }"#;

        let result = AuthorizedUserCredential::from_json(json);
        assert!(matches!(
            result,
            Err(AuthorizedUserError::MissingField { field }) if field == "client_id"
        ));
    }

    #[test]
    fn test_from_json_missing_client_secret() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "client-id",
            "client_secret": "",
            "refresh_token": "token"
        }"#;

        let result = AuthorizedUserCredential::from_json(json);
        assert!(matches!(
            result,
            Err(AuthorizedUserError::MissingField { field }) if field == "client_secret"
        ));
    }

    #[test]
    fn test_from_json_missing_refresh_token() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "client-id",
            "client_secret": "secret",
            "refresh_token": ""
        }"#;

        let result = AuthorizedUserCredential::from_json(json);
        assert!(matches!(
            result,
            Err(AuthorizedUserError::MissingField { field }) if field == "refresh_token"
        ));
    }

    #[test]
    fn test_from_file_not_found() {
        let result = AuthorizedUserCredential::from_file(Path::new("/nonexistent/file.json"));
        assert!(matches!(
            result,
            Err(AuthorizedUserError::FileReadError { .. })
        ));
    }

    #[test]
    fn test_error_display() {
        let err = AuthorizedUserError::InvalidJson {
            message: "test error".to_string(),
        };
        assert!(err.to_string().contains("Invalid JSON"));

        let err = AuthorizedUserError::InvalidCredType {
            expected: "authorized_user".to_string(),
            actual: "other".to_string(),
        };
        assert!(err.to_string().contains("authorized_user"));
        assert!(err.to_string().contains("other"));

        let err = AuthorizedUserError::MissingField {
            field: "client_id".to_string(),
        };
        assert!(err.to_string().contains("client_id"));

        let err = AuthorizedUserError::TokenExchangeFailed {
            message: "exchange error".to_string(),
        };
        assert!(err.to_string().contains("exchange error"));
    }

    #[tokio::test]
    async fn test_token_caching() {
        // Create a credential - we can't test actual token exchange without a real server,
        // but we can verify the caching mechanism is in place
        let cred = AuthorizedUserCredential::new("client-id", "client-secret", "refresh-token");

        // Set up a cached token manually via the cache
        let token = AccessToken::new("cached-token", u64::MAX);
        cred.cache.set(token).await;

        // Get token should return the cached value
        let result = cred.get_token(&[]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cached-token");
    }

    #[tokio::test]
    async fn test_expired_token_not_returned() {
        let cred = AuthorizedUserCredential::new("client-id", "client-secret", "refresh-token");

        // Set an expired token
        let token = AccessToken::new("expired-token", 0);
        cred.cache.set(token).await;

        // Get token should try to fetch a new one (which will fail since we don't have a real server)
        let result = cred.get_token(&[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_near_expiry_not_returned() {
        let cred = AuthorizedUserCredential::new("client-id", "client-secret", "refresh-token");

        // Set a token that expires in 30 seconds (less than the 60-second buffer)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let token = AccessToken::new("near-expiry-token", now + 30);
        cred.cache.set(token).await;

        // Get token should try to fetch a new one because it's within the expiry buffer
        let result = cred.get_token(&[]).await;
        assert!(result.is_err()); // Will fail since no real server
    }

    #[tokio::test]
    async fn test_on_token_rejected() {
        let cred = AuthorizedUserCredential::new("client-id", "client-secret", "refresh-token");

        // Set up a cached token
        let token = AccessToken::new("valid-token", u64::MAX);
        cred.cache.set(token).await;

        // Verify token is cached
        let cached = cred.cache.get(0).await;
        assert_eq!(cached, Some("valid-token".to_string()));

        // Call on_token_rejected
        cred.on_token_rejected();

        // Give the spawned task time to clear the cache
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Cache should be cleared
        let cached = cred.cache.get(0).await;
        assert!(cached.is_none());
    }

    #[test]
    fn test_with_http_client() {
        let custom_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        let cred = AuthorizedUserCredential::with_http_client(
            "client-id",
            "client-secret",
            "refresh-token",
            custom_client,
        );

        assert_eq!(cred.client_id(), "client-id");
    }

    #[test]
    fn test_quota_project_id_from_json() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "test-client-id.apps.googleusercontent.com",
            "client_secret": "test-client-secret",
            "refresh_token": "1//test-refresh-token",
            "quota_project_id": "my-billing-project"
        }"#;

        let cred = AuthorizedUserCredential::from_json(json).unwrap();
        assert_eq!(cred.quota_project_id(), Some("my-billing-project"));
    }

    #[test]
    fn test_quota_project_id_none_when_missing() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "test-client-id.apps.googleusercontent.com",
            "client_secret": "test-client-secret",
            "refresh_token": "1//test-refresh-token"
        }"#;

        let cred = AuthorizedUserCredential::from_json(json).unwrap();
        assert!(cred.quota_project_id().is_none());
    }
}
