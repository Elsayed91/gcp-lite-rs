//! Gcloud CLI authentication support for ADC.
//!
//! This module provides `GcloudCredential`, which uses the `gcloud` CLI tool
//! to obtain access tokens from an active `gcloud auth login` session.
//!
//! Tokens are cached to avoid shelling out on every request. The cache is
//! invalidated when tokens are rejected or after approximately 55 minutes
//! (gcloud tokens typically expire after 1 hour).

use crate::auth::types::{AccessToken, CachedToken};
use crate::token::{TokenError, TokenProvider};
use async_trait::async_trait;

/// Token expiry margin in seconds. We refresh 5 minutes before actual expiry.
const TOKEN_EXPIRY_MARGIN_SECS: u64 = 300;

/// Assumed token lifetime in seconds. Gcloud tokens typically last 1 hour.
const TOKEN_LIFETIME_SECS: u64 = 3600;

/// Errors that can occur when using gcloud credentials.
#[derive(Debug, thiserror::Error)]
pub enum GcloudError {
    /// Gcloud command not found in PATH.
    #[error("gcloud command not found in PATH")]
    NotInstalled,

    /// Gcloud command execution failed.
    #[error("gcloud command failed: {0}")]
    CommandFailed(String),

    /// Gcloud authentication failed.
    #[error("gcloud auth failed: {0}")]
    AuthFailed(String),

    /// Gcloud returned an empty token.
    #[error("gcloud returned empty token")]
    EmptyToken,

    /// Gcloud returned invalid token format.
    #[error("gcloud returned invalid token format")]
    InvalidTokenFormat,
}

/// Credential provider using gcloud CLI.
///
/// Tokens are cached to reduce latency. The cache is automatically
/// refreshed when tokens expire or are rejected.
#[derive(Debug)]
pub struct GcloudCredential {
    quota_project_id: Option<String>,
    cached_token: CachedToken,
}

impl GcloudCredential {
    /// Check if gcloud is installed and accessible.
    #[allow(dead_code)]
    fn check_gcloud_installed() -> Result<(), GcloudError> {
        Self::check_gcloud_installed_impl("gcloud")
    }

    /// Internal implementation for testing.
    fn check_gcloud_installed_impl(command: &str) -> Result<(), GcloudError> {
        which::which(command).map_err(|e| {
            tracing::debug!("gcloud command '{}' not found in PATH: {:?}", command, e);
            GcloudError::NotInstalled
        })?;
        Ok(())
    }

    /// Get access token from gcloud.
    async fn get_access_token() -> Result<String, GcloudError> {
        Self::get_access_token_impl("gcloud").await
    }

    /// Internal implementation for testing.
    async fn get_access_token_impl(command: &str) -> Result<String, GcloudError> {
        Self::get_access_token_impl_with_args(command, &["auth", "print-access-token", "--quiet"])
            .await
    }

    /// Internal implementation with custom args for testing.
    async fn get_access_token_impl_with_args(
        command: &str,
        args: &[&str],
    ) -> Result<String, GcloudError> {
        let output = tokio::process::Command::new(command)
            .args(args)
            .output()
            .await
            .map_err(|e| GcloudError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GcloudError::AuthFailed(stderr.to_string()));
        }

        let token = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if token.is_empty() {
            return Err(GcloudError::EmptyToken);
        }

        Ok(token)
    }

    /// Get quota project from gcloud config.
    #[allow(dead_code)]
    async fn get_quota_project() -> Result<String, GcloudError> {
        Self::get_quota_project_impl("gcloud").await
    }

    /// Internal implementation for testing.
    async fn get_quota_project_impl(command: &str) -> Result<String, GcloudError> {
        Self::get_quota_project_impl_with_args(
            command,
            &["config", "get-value", "project", "--quiet"],
        )
        .await
    }

    /// Internal implementation with custom args for testing.
    async fn get_quota_project_impl_with_args(
        command: &str,
        args: &[&str],
    ) -> Result<String, GcloudError> {
        let output = tokio::process::Command::new(command)
            .args(args)
            .output()
            .await
            .map_err(|e| GcloudError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(GcloudError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let project = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if project.is_empty() {
            return Err(GcloudError::CommandFailed(
                "no project configured".to_string(),
            ));
        }

        Ok(project)
    }

    /// Create a new gcloud credential.
    ///
    /// Verifies gcloud is installed and authenticated, then retrieves
    /// the quota project from gcloud config.
    pub async fn new() -> Result<Self, GcloudError> {
        Self::new_impl("gcloud").await
    }

    /// Internal implementation for testing.
    async fn new_impl(command: &str) -> Result<Self, GcloudError> {
        // Check gcloud is installed
        Self::check_gcloud_installed_impl(command)?;

        // Verify authentication by attempting to get a token
        let token = Self::get_access_token_impl(command).await?;

        // Try to get quota project (non-fatal if not set)
        let quota_project_id = Self::get_quota_project_impl(command).await.ok();

        // Initialize with the token we just fetched (it's valid)
        let cached_token = CachedToken::new();
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            + TOKEN_LIFETIME_SECS;
        cached_token.set(AccessToken::new(token, expires_at)).await;

        Ok(Self {
            quota_project_id,
            cached_token,
        })
    }
}

#[async_trait]
impl TokenProvider for GcloudCredential {
    async fn get_token(&self, _scopes: &[&str]) -> Result<String, TokenError> {
        // Scopes are ignored - gcloud tokens have broad cloud-platform scope

        // Check cache first
        if let Some(token) = self.cached_token.get(TOKEN_EXPIRY_MARGIN_SECS).await {
            return Ok(token);
        }

        // Cache miss or expired - fetch new token
        let token = Self::get_access_token()
            .await
            .map_err(|e| TokenError::RefreshFailed {
                message: e.to_string(),
            })?;

        // Cache the new token
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            + TOKEN_LIFETIME_SECS;
        self.cached_token
            .set(AccessToken::new(token.clone(), expires_at))
            .await;

        Ok(token)
    }

    fn on_token_rejected(&self) {
        // Clear cache so next get_token() fetches fresh
        self.cached_token.clear_sync();
    }

    fn quota_project_id(&self) -> Option<&str> {
        self.quota_project_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcloud_error_display() {
        let err = GcloudError::NotInstalled;
        assert_eq!(err.to_string(), "gcloud command not found in PATH");

        let err = GcloudError::CommandFailed("exec error".to_string());
        assert!(err.to_string().contains("gcloud command failed"));
        assert!(err.to_string().contains("exec error"));

        let err = GcloudError::AuthFailed("not logged in".to_string());
        assert!(err.to_string().contains("gcloud auth failed"));
        assert!(err.to_string().contains("not logged in"));

        let err = GcloudError::EmptyToken;
        assert_eq!(err.to_string(), "gcloud returned empty token");

        let err = GcloudError::InvalidTokenFormat;
        assert_eq!(err.to_string(), "gcloud returned invalid token format");
    }

    #[test]
    fn test_check_gcloud_installed_not_found() {
        // This test assumes gcloud is not at /nonexistent/gcloud
        // We can't easily mock PATH, so we test the error type
        let result = GcloudCredential::check_gcloud_installed_impl("/nonexistent/gcloud");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GcloudError::NotInstalled));
    }

    #[test]
    fn test_check_gcloud_installed_success() {
        // Test with 'ls' which should exist on all systems
        let result = GcloudCredential::check_gcloud_installed_impl("ls");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_token_command_fails() {
        // Mock command that fails
        let result = GcloudCredential::get_access_token_impl("false").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GcloudError::AuthFailed(_)));
    }

    #[tokio::test]
    async fn test_get_access_token_empty_output() {
        // Mock command that succeeds but returns empty (echo with -n and empty string)
        let result = GcloudCredential::get_access_token_impl_with_args("printf", &[""]).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GcloudError::EmptyToken));
    }

    #[tokio::test]
    async fn test_get_access_token_success() {
        // Mock with echo to simulate token
        let result =
            GcloudCredential::get_access_token_impl_with_args("echo", &["test-token"]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-token");
    }

    #[tokio::test]
    async fn test_get_quota_project_success() {
        // Mock with echo to simulate project
        let result =
            GcloudCredential::get_quota_project_impl_with_args("echo", &["my-project"]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my-project");
    }

    #[tokio::test]
    async fn test_get_quota_project_empty() {
        // Mock command that returns empty (unset project)
        let result = GcloudCredential::get_quota_project_impl_with_args("printf", &[""]).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GcloudError::CommandFailed(_)));
    }

    #[tokio::test]
    async fn test_get_quota_project_command_fails() {
        let result = GcloudCredential::get_quota_project_impl("false").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_gcloud_credential_new_not_installed() {
        // This will fail in CI where gcloud isn't installed
        // Test the error path
        let result = GcloudCredential::new_impl("nonexistent-command").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GcloudError::NotInstalled));
    }

    #[tokio::test]
    async fn test_token_provider_get_token() {
        // Create credential with mock quota project
        let cred = GcloudCredential {
            quota_project_id: Some("test-project".to_string()),
            cached_token: CachedToken::new(),
        };

        // We can't easily test actual token retrieval without mocking
        // Test that quota_project_id works
        assert_eq!(cred.quota_project_id(), Some("test-project"));
    }

    #[test]
    fn test_token_provider_quota_project_none() {
        let cred = GcloudCredential {
            quota_project_id: None,
            cached_token: CachedToken::new(),
        };
        assert_eq!(cred.quota_project_id(), None);
    }

    #[tokio::test]
    async fn test_on_token_rejected_clears_cache() {
        use crate::auth::types::AccessToken;

        let cred = GcloudCredential {
            quota_project_id: None,
            cached_token: CachedToken::new(),
        };

        // Pre-populate cache with a test token
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            + 3600;
        cred.cached_token
            .set(AccessToken::new("test-token", expires_at))
            .await;

        // Should not panic and should clear cache
        cred.on_token_rejected();

        // Give spawned task time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Cache should be cleared
        assert!(cred.cached_token.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_token_caching() {
        use crate::auth::types::AccessToken;

        let cred = GcloudCredential {
            quota_project_id: None,
            cached_token: CachedToken::new(),
        };

        // Pre-populate cache with a test token
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            + 3600; // 1 hour from now
        cred.cached_token
            .set(AccessToken::new("cached-token", expires_at))
            .await;

        // get_token should return cached token without shelling out
        let result = cred.get_token(&[]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cached-token");

        // Clear cache via on_token_rejected
        cred.on_token_rejected();

        // Give spawned task time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Cache should be empty now (verified by checking the cache directly)
        assert!(cred.cached_token.get(0).await.is_none());
    }
}
