//! Metadata server credential implementation for GCP authentication.
//!
//! This module provides `MetadataServerCredential`, which implements the `TokenProvider`
//! trait for authenticating with GCP APIs when running on GCP infrastructure (GCE, Cloud Run,
//! GKE, etc.) using the instance metadata server.
//!
//! # Example
//!
//! ```no_run
//! use gcp_lite::auth::MetadataServerCredential;
//! use gcp_lite::token::TokenProvider;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Use the default service account
//! let cred = MetadataServerCredential::new();
//!
//! // Or specify a service account
//! let cred = MetadataServerCredential::with_service_account("my-sa@my-project.iam.gserviceaccount.com");
//!
//! // Get a token
//! let token = cred.get_token(&["https://www.googleapis.com/auth/cloud-platform"]).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::types::{AccessToken, CachedToken};
use crate::token::{TokenError, TokenProvider};

/// Default expiry buffer in seconds (60 seconds).
/// Tokens will be refreshed this many seconds before actual expiry.
const TOKEN_EXPIRY_BUFFER_SECS: u64 = 60;

/// Base URL for the GCP metadata server.
const METADATA_SERVER_BASE: &str = "http://metadata.google.internal/computeMetadata/v1";

/// Response from the metadata server token endpoint.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    /// The access token.
    access_token: String,
    /// Token lifetime in seconds.
    expires_in: u64,
    /// Token type (usually "Bearer").
    #[allow(dead_code)]
    token_type: String,
}

/// A credential provider that obtains access tokens from the GCP metadata server.
///
/// This credential type:
/// - Works only when running on GCP infrastructure (GCE, Cloud Run, GKE, etc.)
/// - Fetches tokens from the instance metadata server
/// - Caches tokens with a 60-second expiry buffer
/// - Uses the attached service account's scopes (ignores requested scopes)
///
/// # Thread Safety
///
/// `MetadataServerCredential` is `Send + Sync` and can be safely shared across threads.
/// Token caching is handled internally using `tokio::sync::RwLock`.
#[derive(Debug)]
pub struct MetadataServerCredential {
    /// Service account email (or "default" for the default service account).
    service_account: String,
    /// Cached access token.
    cache: CachedToken,
    /// HTTP client for metadata server requests.
    http_client: reqwest::Client,
    /// Base URL for the metadata server (configurable for testing).
    metadata_base_url: String,
}

impl MetadataServerCredential {
    /// Create a new credential using the default service account.
    ///
    /// This uses the service account attached to the GCP instance (VM, Cloud Run service, etc.).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::MetadataServerCredential;
    ///
    /// let cred = MetadataServerCredential::new();
    /// ```
    pub fn new() -> Self {
        Self::with_service_account("default")
    }

    /// Create a new credential using a specific service account.
    ///
    /// # Arguments
    ///
    /// * `service_account` - The service account email or "default"
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::MetadataServerCredential;
    ///
    /// let cred = MetadataServerCredential::with_service_account("my-sa@my-project.iam.gserviceaccount.com");
    /// ```
    pub fn with_service_account(service_account: impl Into<String>) -> Self {
        Self {
            service_account: service_account.into(),
            cache: CachedToken::new(),
            http_client: reqwest::Client::new(),
            metadata_base_url: METADATA_SERVER_BASE.to_string(),
        }
    }

    /// Create a new credential with a custom HTTP client and metadata server URL.
    ///
    /// This is primarily useful for testing with a mock server.
    ///
    /// # Arguments
    ///
    /// * `service_account` - The service account email or "default"
    /// * `http_client` - A custom reqwest::Client
    /// * `metadata_base_url` - Base URL for the metadata server
    pub fn with_custom_client(
        service_account: impl Into<String>,
        http_client: reqwest::Client,
        metadata_base_url: impl Into<String>,
    ) -> Self {
        Self {
            service_account: service_account.into(),
            cache: CachedToken::new(),
            http_client,
            metadata_base_url: metadata_base_url.into(),
        }
    }

    /// Get the service account being used.
    pub fn service_account(&self) -> &str {
        &self.service_account
    }

    /// Build the token endpoint URL.
    fn token_url(&self) -> String {
        format!(
            "{}/instance/service-accounts/{}/token",
            self.metadata_base_url, self.service_account
        )
    }

    /// Fetch a fresh access token from the metadata server.
    async fn fetch_token(&self) -> Result<AccessToken, MetadataServerError> {
        let url = self.token_url();

        let response = self
            .http_client
            .get(&url)
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|e| MetadataServerError::RequestFailed {
                message: format!("HTTP request failed: {}", e),
            })?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|e| MetadataServerError::RequestFailed {
                    message: format!("Failed to read response body: {}", e),
                })?;

        if !status.is_success() {
            return Err(MetadataServerError::RequestFailed {
                message: format!("Metadata server returned {}: {}", status, response_text),
            });
        }

        let token_response: TokenResponse = serde_json::from_str(&response_text).map_err(|e| {
            MetadataServerError::InvalidResponse {
                message: format!("Failed to parse token response: {}", e),
            }
        })?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| MetadataServerError::RequestFailed {
                message: format!("Failed to get current time: {}", e),
            })?
            .as_secs();

        Ok(AccessToken::new(
            token_response.access_token,
            now + token_response.expires_in,
        ))
    }
}

impl Default for MetadataServerCredential {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TokenProvider for MetadataServerCredential {
    async fn get_token(&self, _scopes: &[&str]) -> Result<String, TokenError> {
        // Note: The metadata server ignores scopes - the token uses whatever
        // scopes the attached service account has.

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
}

/// Errors that can occur when using metadata server credentials.
#[derive(Debug, thiserror::Error)]
pub enum MetadataServerError {
    /// HTTP request to metadata server failed.
    #[error("Metadata server request failed: {message}")]
    RequestFailed {
        /// Error description.
        message: String,
    },

    /// Invalid response from metadata server.
    #[error("Invalid metadata server response: {message}")]
    InvalidResponse {
        /// Error description.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_new_default_service_account() {
        let cred = MetadataServerCredential::new();
        assert_eq!(cred.service_account(), "default");
    }

    #[test]
    fn test_with_service_account() {
        let cred = MetadataServerCredential::with_service_account(
            "my-sa@my-project.iam.gserviceaccount.com",
        );
        assert_eq!(
            cred.service_account(),
            "my-sa@my-project.iam.gserviceaccount.com"
        );
    }

    #[test]
    fn test_default_impl() {
        let cred = MetadataServerCredential::default();
        assert_eq!(cred.service_account(), "default");
    }

    #[test]
    fn test_token_url() {
        let cred = MetadataServerCredential::new();
        assert_eq!(
            cred.token_url(),
            "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token"
        );

        let cred = MetadataServerCredential::with_service_account("custom@example.com");
        assert_eq!(
            cred.token_url(),
            "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/custom@example.com/token"
        );
    }

    #[tokio::test]
    async fn test_get_token_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "test-access-token-12345",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        let token = cred.get_token(&["scope1"]).await.unwrap();
        assert_eq!(token, "test-access-token-12345");
    }

    #[tokio::test]
    async fn test_get_token_caching() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "cached-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(1) // Should only be called once due to caching
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        // First call - hits the server
        let token1 = cred.get_token(&["scope1"]).await.unwrap();
        assert_eq!(token1, "cached-token");

        // Second call - uses cache
        let token2 = cred.get_token(&["scope1"]).await.unwrap();
        assert_eq!(token2, "cached-token");
    }

    #[tokio::test]
    async fn test_get_token_custom_service_account() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/instance/service-accounts/custom@project.iam.gserviceaccount.com/token",
            ))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "custom-sa-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "custom@project.iam.gserviceaccount.com",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        let token = cred.get_token(&[]).await.unwrap();
        assert_eq!(token, "custom-sa-token");
    }

    #[tokio::test]
    async fn test_get_token_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        let result = cred.get_token(&[]).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, TokenError::RefreshFailed { .. }));
        assert!(err.to_string().contains("500"));
    }

    #[tokio::test]
    async fn test_get_token_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        let result = cred.get_token(&[]).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, TokenError::RefreshFailed { .. }));
    }

    #[tokio::test]
    async fn test_get_token_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/nonexistent/token"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "nonexistent",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        let result = cred.get_token(&[]).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, TokenError::RefreshFailed { .. }));
        assert!(err.to_string().contains("404"));
    }

    #[tokio::test]
    async fn test_on_token_rejected_clears_cache() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "new-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(2) // Called twice: once before rejection, once after
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        // First call - populates cache
        let _token1 = cred.get_token(&[]).await.unwrap();

        // Simulate token rejection
        cred.on_token_rejected();

        // Give the spawned task time to clear the cache
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Second call - should fetch new token since cache was cleared
        let _token2 = cred.get_token(&[]).await.unwrap();
    }

    #[tokio::test]
    async fn test_token_expiry_buffer() {
        let mock_server = MockServer::start().await;

        // Return a token that expires in 30 seconds (less than the 60-second buffer)
        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "short-lived-token",
                "expires_in": 30,  // Expires soon
                "token_type": "Bearer"
            })))
            .expect(2) // Should be called twice since token expires within buffer
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        // First call - gets token
        let _token1 = cred.get_token(&[]).await.unwrap();

        // Second call - token is within expiry buffer, should fetch new one
        let _token2 = cred.get_token(&[]).await.unwrap();
    }

    #[tokio::test]
    async fn test_scopes_are_ignored() {
        let mock_server = MockServer::start().await;

        // The same endpoint is used regardless of scopes
        Mock::given(method("GET"))
            .and(path("/instance/service-accounts/default/token"))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "same-token",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cred = MetadataServerCredential::with_custom_client(
            "default",
            reqwest::Client::new(),
            mock_server.uri(),
        );

        // Call with different scopes - should get same token from same endpoint
        let token = cred
            .get_token(&[
                "https://www.googleapis.com/auth/cloud-platform",
                "https://www.googleapis.com/auth/compute",
            ])
            .await
            .unwrap();
        assert_eq!(token, "same-token");
    }

    #[test]
    fn test_error_display() {
        let err = MetadataServerError::RequestFailed {
            message: "connection refused".to_string(),
        };
        assert!(err.to_string().contains("connection refused"));
        assert!(err.to_string().contains("request failed"));

        let err = MetadataServerError::InvalidResponse {
            message: "invalid JSON".to_string(),
        };
        assert!(err.to_string().contains("invalid JSON"));
        assert!(err.to_string().contains("Invalid metadata server response"));
    }
}
