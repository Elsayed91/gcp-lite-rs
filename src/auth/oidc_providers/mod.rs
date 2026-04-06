//! OIDC token provider abstractions for Workload Identity Federation.
//!
//! This module provides the `OidcTokenProvider` trait and implementations for
//! various cloud providers (GitHub Actions, AWS, Azure).

pub mod aws;
pub mod azure;
pub mod github;

use async_trait::async_trait;

pub use aws::AwsWebIdentityProvider;
pub use azure::AzureWorkloadIdentityProvider;
pub use github::GitHubActionsProvider;

/// Errors that can occur when fetching OIDC tokens.
#[derive(Debug, thiserror::Error)]
pub enum OidcError {
    /// No OIDC provider detected in environment.
    #[error("No OIDC provider detected in environment")]
    NoProviderFound,

    /// Failed to fetch OIDC token from provider.
    #[error("Failed to fetch OIDC token: {0}")]
    FetchFailed(#[from] reqwest::Error),

    /// Invalid OIDC token response.
    #[error("Invalid OIDC token response")]
    InvalidResponse,

    /// Token file not found.
    #[error("Token file not found: {path}")]
    FileNotFound {
        /// Path to the missing file.
        path: String,
    },

    /// Environment variable missing.
    #[error("Environment variable missing: {var}")]
    MissingEnvVar {
        /// Name of the missing variable.
        var: String,
    },

    /// IO error when reading token file.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Trait for OIDC token providers.
///
/// Implementations fetch OIDC tokens from various sources (GitHub Actions, AWS, Azure).
#[async_trait]
pub trait OidcTokenProvider: Send + Sync {
    /// Get an OIDC token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token cannot be fetched.
    async fn get_token(&self) -> Result<String, OidcError>;
}

/// Auto-detect OIDC provider from environment variables.
///
/// Checks for GitHub Actions, AWS, then Azure environment variables in order.
///
/// # Errors
///
/// Returns `NoProviderFound` if no provider is detected.
pub fn auto_detect_provider() -> Result<Box<dyn OidcTokenProvider>, OidcError> {
    // Check GitHub Actions
    if std::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN").is_ok()
        && std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").is_ok()
    {
        return Ok(Box::new(GitHubActionsProvider::new()?));
    }

    // Check AWS
    if std::env::var("AWS_WEB_IDENTITY_TOKEN_FILE").is_ok() {
        return Ok(Box::new(AwsWebIdentityProvider::from_env()?));
    }

    // Check Azure
    if std::env::var("IDENTITY_ENDPOINT").is_ok() && std::env::var("IDENTITY_HEADER").is_ok() {
        return Ok(Box::new(AzureWorkloadIdentityProvider::new()?));
    }

    Err(OidcError::NoProviderFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_oidc_error_display() {
        let err = OidcError::NoProviderFound;
        assert_eq!(err.to_string(), "No OIDC provider detected in environment");

        let err = OidcError::InvalidResponse;
        assert_eq!(err.to_string(), "Invalid OIDC token response");

        let err = OidcError::FileNotFound {
            path: "/tmp/token".into(),
        };
        assert_eq!(err.to_string(), "Token file not found: /tmp/token");

        let err = OidcError::MissingEnvVar {
            var: "MY_VAR".into(),
        };
        assert_eq!(err.to_string(), "Environment variable missing: MY_VAR");
    }

    #[test]
    #[serial]
    fn test_auto_detect_github() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "token");
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_URL", "http://localhost");
        }

        let result = auto_detect_provider();
        assert!(result.is_ok());

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
        }
    }

    #[test]
    #[serial]
    fn test_auto_detect_aws() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::set_var("AWS_WEB_IDENTITY_TOKEN_FILE", "/tmp/token");
        }

        let result = auto_detect_provider();
        assert!(result.is_ok());

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("AWS_WEB_IDENTITY_TOKEN_FILE");
        }
    }

    #[test]
    #[serial]
    fn test_auto_detect_azure() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("AWS_WEB_IDENTITY_TOKEN_FILE");
            std::env::set_var("IDENTITY_ENDPOINT", "http://localhost");
            std::env::set_var("IDENTITY_HEADER", "value");
        }

        let result = auto_detect_provider();
        assert!(result.is_ok());

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("IDENTITY_ENDPOINT");
            std::env::remove_var("IDENTITY_HEADER");
        }
    }

    #[test]
    #[serial]
    fn test_auto_detect_none() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("AWS_WEB_IDENTITY_TOKEN_FILE");
            std::env::remove_var("IDENTITY_ENDPOINT");
        }

        let result = auto_detect_provider();
        assert!(result.is_err());
        match result {
            Err(OidcError::NoProviderFound) => {}
            _ => panic!("Expected NoProviderFound error"),
        }
    }
}
