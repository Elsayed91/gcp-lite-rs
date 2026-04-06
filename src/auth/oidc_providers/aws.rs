//! AWS Web Identity OIDC provider implementation.

use super::{OidcError, OidcTokenProvider};
use async_trait::async_trait;

/// OIDC provider for AWS Web Identity.
///
/// Reads OIDC tokens from the file specified by `AWS_WEB_IDENTITY_TOKEN_FILE`.
pub struct AwsWebIdentityProvider {
    /// Path to the token file.
    pub(crate) token_file_path: String,
}

impl AwsWebIdentityProvider {
    /// Create a new AWS Web Identity provider with explicit token file path.
    pub fn new(token_file_path: String) -> Self {
        Self { token_file_path }
    }

    /// Create from `AWS_WEB_IDENTITY_TOKEN_FILE` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variable is not set.
    pub fn from_env() -> Result<Self, OidcError> {
        let token_file_path =
            std::env::var("AWS_WEB_IDENTITY_TOKEN_FILE").map_err(|_| OidcError::MissingEnvVar {
                var: "AWS_WEB_IDENTITY_TOKEN_FILE".into(),
            })?;

        Ok(Self::new(token_file_path))
    }
}

#[async_trait]
impl OidcTokenProvider for AwsWebIdentityProvider {
    async fn get_token(&self) -> Result<String, OidcError> {
        std::fs::read_to_string(&self.token_file_path).map_err(|_| OidcError::FileNotFound {
            path: self.token_file_path.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_aws_provider_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"mock-aws-oidc-token").unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let provider = AwsWebIdentityProvider::new(path);
        let token = provider.get_token().await.unwrap();

        assert_eq!(token, "mock-aws-oidc-token");
    }

    #[tokio::test]
    async fn test_aws_provider_file_not_found() {
        let provider = AwsWebIdentityProvider::new("/nonexistent/token".into());
        let result = provider.get_token().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            OidcError::FileNotFound { .. }
        ));
    }

    #[test]
    #[serial]
    fn test_aws_provider_from_env() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var("AWS_WEB_IDENTITY_TOKEN_FILE", "/tmp/token");
        }

        let provider = AwsWebIdentityProvider::from_env().unwrap();
        assert_eq!(provider.token_file_path, "/tmp/token");

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("AWS_WEB_IDENTITY_TOKEN_FILE");
        }
    }

    #[test]
    #[serial]
    fn test_aws_provider_from_env_missing() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("AWS_WEB_IDENTITY_TOKEN_FILE");
        }

        let result = AwsWebIdentityProvider::from_env();
        assert!(result.is_err());
    }
}
