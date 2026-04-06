//! Azure Workload Identity OIDC provider implementation.

use super::{OidcError, OidcTokenProvider};
use async_trait::async_trait;

/// OIDC provider for Azure Workload Identity.
///
/// Fetches OIDC tokens from Azure metadata service using environment variables:
/// - `IDENTITY_ENDPOINT` - Metadata endpoint URL
/// - `IDENTITY_HEADER` - Identity header value
pub struct AzureWorkloadIdentityProvider {
    endpoint: String,
    header_value: String,
}

impl AzureWorkloadIdentityProvider {
    /// Create a new Azure Workload Identity provider from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing.
    pub fn new() -> Result<Self, OidcError> {
        let endpoint =
            std::env::var("IDENTITY_ENDPOINT").map_err(|_| OidcError::MissingEnvVar {
                var: "IDENTITY_ENDPOINT".into(),
            })?;

        let header_value =
            std::env::var("IDENTITY_HEADER").map_err(|_| OidcError::MissingEnvVar {
                var: "IDENTITY_HEADER".into(),
            })?;

        Ok(Self {
            endpoint,
            header_value,
        })
    }
}

#[async_trait]
impl OidcTokenProvider for AzureWorkloadIdentityProvider {
    async fn get_token(&self) -> Result<String, OidcError> {
        let client = reqwest::Client::new();
        let response = client
            .get(&self.endpoint)
            .header("X-IDENTITY-HEADER", &self.header_value)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        json["access_token"]
            .as_str()
            .ok_or(OidcError::InvalidResponse)
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    #[serial]
    async fn test_azure_provider_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/metadata/identity/oauth2/token"))
            .and(header("X-IDENTITY-HEADER", "test-identity-header"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "mock-azure-oidc-token"
            })))
            .mount(&mock_server)
            .await;

        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var(
                "IDENTITY_ENDPOINT",
                format!("{}/metadata/identity/oauth2/token", mock_server.uri()),
            );
            std::env::set_var("IDENTITY_HEADER", "test-identity-header");
        }

        let provider = AzureWorkloadIdentityProvider::new().unwrap();
        let token = provider.get_token().await.unwrap();

        assert_eq!(token, "mock-azure-oidc-token");

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("IDENTITY_ENDPOINT");
            std::env::remove_var("IDENTITY_HEADER");
        }
    }

    #[test]
    #[serial]
    fn test_azure_provider_missing_env() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("IDENTITY_ENDPOINT");
            std::env::remove_var("IDENTITY_HEADER");
        }

        let result = AzureWorkloadIdentityProvider::new();
        assert!(result.is_err());
    }
}
