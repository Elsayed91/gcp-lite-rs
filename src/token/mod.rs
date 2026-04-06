//! Token provider abstractions for GCP authentication.

use async_trait::async_trait;

mod static_provider;

pub use static_provider::StaticTokenProvider;

/// Provides access tokens for GCP API authentication.
///
/// Implementations handle credential resolution, token caching, and refresh.
#[async_trait]
pub trait TokenProvider: Send + Sync {
    /// Get a valid access token for the specified scopes.
    ///
    /// This is called on every HTTP request. Implementations should handle
    /// caching internally.
    ///
    /// # Arguments
    ///
    /// * `scopes` - OAuth2 scopes required for the request
    ///
    /// # Returns
    ///
    /// A valid bearer token string
    async fn get_token(&self, scopes: &[&str]) -> Result<String, TokenError>;

    /// Called when a token is rejected by GCP (401 response).
    ///
    /// Allows the provider to invalidate cached tokens.
    fn on_token_rejected(&self) {
        // Default: no-op
    }

    /// Get the quota project ID for billing purposes.
    ///
    /// Returns the project ID that should be used for quota and billing.
    /// When set, the `x-goog-user-project` header will be added to requests.
    ///
    /// Default implementation returns `None`, meaning no quota project header
    /// will be added.
    fn quota_project_id(&self) -> Option<&str> {
        None
    }
}

/// Errors that can occur during token acquisition
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    /// No credentials found in any source
    #[error("No credentials found")]
    NoCredentialsFound,

    /// Failed to read credential file
    #[error("Failed to read credentials file: {path}")]
    CredentialFileError {
        /// Path to the credential file
        path: std::path::PathBuf,
        /// Underlying IO error
        #[source]
        source: std::io::Error,
    },

    /// Invalid credential format
    #[error("Invalid credentials: {message}")]
    InvalidCredentials {
        /// Error description
        message: String,
    },

    /// Token refresh failed
    #[error("Token refresh failed: {message}")]
    RefreshFailed {
        /// Error description
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProvider {
        quota_project: Option<String>,
    }

    #[async_trait]
    impl TokenProvider for TestProvider {
        async fn get_token(&self, _scopes: &[&str]) -> Result<String, TokenError> {
            Ok("test-token".to_string())
        }

        fn quota_project_id(&self) -> Option<&str> {
            self.quota_project.as_deref()
        }
    }

    #[test]
    fn test_quota_project_id_default_is_none() {
        // Default implementation should return None
        let provider = crate::token::StaticTokenProvider::new("token");
        assert!(provider.quota_project_id().is_none());
    }

    #[test]
    fn test_quota_project_id_custom_impl() {
        let provider = TestProvider {
            quota_project: Some("my-project".to_string()),
        };
        assert_eq!(provider.quota_project_id(), Some("my-project"));
    }
}
