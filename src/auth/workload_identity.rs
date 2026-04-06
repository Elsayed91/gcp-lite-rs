//! Workload Identity Federation credential implementation.

use async_trait::async_trait;
use std::sync::Arc;

use crate::auth::external_account::{
    ExternalAccountConfig, ExternalAccountError, extract_sa_email,
};
use crate::auth::oidc_providers::{
    AwsWebIdentityProvider, AzureWorkloadIdentityProvider, GitHubActionsProvider, OidcError,
    OidcTokenProvider, auto_detect_provider,
};
use crate::token::{TokenError, TokenProvider};
use std::path::Path;

/// Errors specific to Workload Identity Federation.
#[derive(Debug, thiserror::Error)]
pub enum WifError {
    /// OIDC provider error.
    #[error("OIDC error: {0}")]
    Oidc(#[from] OidcError),

    /// Missing environment variable.
    #[error("Environment variable missing: {0}")]
    MissingEnvVar(String),

    /// Invalid audience format.
    #[error("Invalid audience format: {0}")]
    InvalidAudience(String),
}

/// Workload Identity Federation credential.
///
/// Authenticates using OIDC tokens from GitHub Actions, AWS, or Azure,
/// exchanging them for GCP access tokens via Workload Identity Federation.
pub struct WorkloadIdentityCredential {
    oidc_provider: Box<dyn OidcTokenProvider>,
    /// Workload Identity Pool audience URL.
    pub(crate) audience: String,
    /// Service account email to impersonate.
    pub(crate) service_account_email: String,
    http_client: Arc<reqwest::Client>,
    sts_endpoint: Option<String>,
    impersonation_endpoint: Option<String>,
}

impl WorkloadIdentityCredential {
    /// Create a new Workload Identity credential.
    ///
    /// # Arguments
    ///
    /// * `oidc_provider` - Provider for fetching OIDC tokens
    /// * `audience` - Workload Identity Pool audience URL
    /// * `service_account_email` - Service account to impersonate
    pub fn new(
        oidc_provider: Box<dyn OidcTokenProvider>,
        audience: String,
        service_account_email: String,
    ) -> Self {
        Self {
            oidc_provider,
            audience,
            service_account_email,
            http_client: Arc::new(reqwest::Client::new()),
            sts_endpoint: None,
            impersonation_endpoint: None,
        }
    }

    /// Exchange OIDC token for STS token.
    ///
    /// # Arguments
    ///
    /// * `oidc_token` - OIDC token from provider
    ///
    /// # Errors
    ///
    /// Returns an error if the STS exchange fails.
    pub async fn exchange_sts_token(&self, oidc_token: &str) -> Result<String, TokenError> {
        let endpoint = self
            .sts_endpoint
            .as_deref()
            .unwrap_or("https://sts.googleapis.com/v1/token");

        let body = serde_json::json!({
            "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
            "audience": self.audience,
            "scope": "https://www.googleapis.com/auth/cloud-platform",
            "requested_token_type": "urn:ietf:params:oauth:token-type:access_token",
            "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
            "subject_token": oidc_token,
        });

        let response = self
            .http_client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| TokenError::RefreshFailed {
                message: format!("STS token exchange failed: {}", e),
            })?;

        let json: serde_json::Value =
            response
                .json()
                .await
                .map_err(|e| TokenError::InvalidCredentials {
                    message: format!("Failed to parse STS response: {}", e),
                })?;

        json["access_token"]
            .as_str()
            .ok_or(TokenError::InvalidCredentials {
                message: "No access_token in STS response".into(),
            })
            .map(String::from)
    }

    /// Impersonate service account with STS token.
    ///
    /// # Arguments
    ///
    /// * `sts_token` - STS token from exchange
    /// * `scopes` - OAuth scopes to request
    ///
    /// # Errors
    ///
    /// Returns an error if impersonation fails.
    pub async fn impersonate_service_account(
        &self,
        sts_token: &str,
        scopes: &[&str],
    ) -> Result<String, TokenError> {
        let base_url = self
            .impersonation_endpoint
            .as_deref()
            .unwrap_or("https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/");

        let url = format!(
            "{}{}:generateAccessToken",
            base_url, self.service_account_email
        );

        let body = serde_json::json!({
            "scope": scopes,
            "lifetime": "3600s",
        });

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(sts_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| TokenError::RefreshFailed {
                message: format!("Service account impersonation failed: {}", e),
            })?;

        let json: serde_json::Value =
            response
                .json()
                .await
                .map_err(|e| TokenError::InvalidCredentials {
                    message: format!("Failed to parse impersonation response: {}", e),
                })?;

        json["accessToken"]
            .as_str()
            .ok_or(TokenError::InvalidCredentials {
                message: "No accessToken in impersonation response".into(),
            })
            .map(String::from)
    }

    /// Get OIDC token from provider (exposed for advanced use).
    ///
    /// # Errors
    ///
    /// Returns an error if OIDC token fetch fails.
    pub async fn get_oidc_token(&self) -> Result<String, OidcError> {
        self.oidc_provider.get_token().await
    }

    /// Create from environment variables.
    ///
    /// Detects OIDC provider automatically and builds credential.
    ///
    /// # Environment Variables
    ///
    /// - `GCP_WORKLOAD_IDENTITY_AUDIENCE` or `GCP_PROJECT_NUMBER` + `GCP_WORKLOAD_IDENTITY_POOL_ID` + `GCP_WORKLOAD_IDENTITY_PROVIDER_ID`
    /// - `GCP_SERVICE_ACCOUNT_EMAIL`
    /// - Provider-specific vars (see individual providers)
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or provider detection fails.
    pub fn from_environment() -> Result<Self, WifError> {
        let audience = std::env::var("GCP_WORKLOAD_IDENTITY_AUDIENCE")
            .or_else(|_| build_audience_from_parts())?;

        let service_account_email = std::env::var("GCP_SERVICE_ACCOUNT_EMAIL")
            .map_err(|_| WifError::MissingEnvVar("GCP_SERVICE_ACCOUNT_EMAIL".into()))?;

        let provider = auto_detect_provider()?;

        Ok(Self::new(provider, audience, service_account_email))
    }

    /// Create from external account JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - External account JSON configuration
    ///
    /// # Errors
    ///
    /// Returns an error if JSON is invalid or provider cannot be created.
    pub fn from_json(json: &str) -> Result<Self, ExternalAccountError> {
        let config: ExternalAccountConfig = serde_json::from_str(json)?;

        if config.credential_type != "external_account" {
            return Err(ExternalAccountError::UnsupportedType(
                config.credential_type,
            ));
        }

        // Determine provider from credential_source
        let provider: Box<dyn OidcTokenProvider> =
            if let Some(env_id) = &config.credential_source.environment_id {
                match env_id.as_str() {
                    "github" => Box::new(GitHubActionsProvider::new()?),
                    "aws1" => Box::new(AwsWebIdentityProvider::from_env()?),
                    "azure" => Box::new(AzureWorkloadIdentityProvider::new()?),
                    _ => return Err(ExternalAccountError::UnsupportedEnvironment(env_id.clone())),
                }
            } else if let Some(_file_path) = &config.credential_source.file {
                // TODO: Implement FileTokenProvider if needed
                return Err(ExternalAccountError::MissingCredentialSource);
            } else {
                return Err(ExternalAccountError::MissingCredentialSource);
            };

        // Extract service account email from impersonation URL
        let impersonation_url = config
            .service_account_impersonation_url
            .ok_or(ExternalAccountError::MissingImpersonationUrl)?;
        let sa_email = extract_sa_email(&impersonation_url)?;

        Ok(Self::new(provider, config.audience, sa_email))
    }

    /// Create from external account JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to external account JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if file cannot be read or JSON is invalid.
    pub fn from_file(path: &Path) -> Result<Self, ExternalAccountError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| ExternalAccountError::InvalidJson(serde_json::Error::io(e)))?;
        Self::from_json(&json)
    }
}

fn build_audience_from_parts() -> Result<String, WifError> {
    let project = std::env::var("GCP_PROJECT_NUMBER").map_err(|_| {
        WifError::MissingEnvVar("GCP_PROJECT_NUMBER or GCP_WORKLOAD_IDENTITY_AUDIENCE".into())
    })?;
    let pool = std::env::var("GCP_WORKLOAD_IDENTITY_POOL_ID")
        .map_err(|_| WifError::MissingEnvVar("GCP_WORKLOAD_IDENTITY_POOL_ID".into()))?;
    let provider = std::env::var("GCP_WORKLOAD_IDENTITY_PROVIDER_ID")
        .map_err(|_| WifError::MissingEnvVar("GCP_WORKLOAD_IDENTITY_PROVIDER_ID".into()))?;

    Ok(format!(
        "//iam.googleapis.com/projects/{}/locations/global/workloadIdentityPools/{}/providers/{}",
        project, pool, provider
    ))
}

#[async_trait]
impl TokenProvider for WorkloadIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> Result<String, TokenError> {
        // Step 1: Get OIDC token
        let oidc_token =
            self.oidc_provider
                .get_token()
                .await
                .map_err(|e| TokenError::RefreshFailed {
                    message: format!("OIDC token fetch failed: {}", e),
                })?;

        // Step 2: Exchange for STS token
        let sts_token = self.exchange_sts_token(&oidc_token).await?;

        // Step 3: Impersonate service account
        let sa_token = self.impersonate_service_account(&sts_token, scopes).await?;

        Ok(sa_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serial_test::serial;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct MockOidcProvider(String);

    #[async_trait]
    impl OidcTokenProvider for MockOidcProvider {
        async fn get_token(&self) -> Result<String, OidcError> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn test_workload_identity_credential_new() {
        let provider = Box::new(MockOidcProvider("mock-oidc-token".into()));
        let cred = WorkloadIdentityCredential::new(
            provider,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider".into(),
            "sa@project.iam.gserviceaccount.com".into(),
        );

        assert_eq!(
            cred.audience,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider"
        );
        assert_eq!(
            cred.service_account_email,
            "sa@project.iam.gserviceaccount.com"
        );
    }

    #[tokio::test]
    async fn test_exchange_sts_token_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/token"))
            .and(body_json(serde_json::json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
                "audience": "//iam.googleapis.com/test-audience",
                "scope": "https://www.googleapis.com/auth/cloud-platform",
                "requested_token_type": "urn:ietf:params:oauth:token-type:access_token",
                "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
                "subject_token": "mock-oidc-token",
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "mock-sts-token",
                "expires_in": 3600,
            })))
            .mount(&mock_server)
            .await;

        let provider = Box::new(MockOidcProvider("mock-oidc-token".into()));
        let mut cred = WorkloadIdentityCredential::new(
            provider,
            "//iam.googleapis.com/test-audience".into(),
            "sa@project.iam.gserviceaccount.com".into(),
        );
        cred.sts_endpoint = Some(format!("{}/v1/token", mock_server.uri()));

        let sts_token = cred.exchange_sts_token("mock-oidc-token").await.unwrap();
        assert_eq!(sts_token, "mock-sts-token");
    }

    #[tokio::test]
    async fn test_impersonate_service_account_success() {
        use wiremock::matchers::{bearer_token, path};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/v1/projects/-/serviceAccounts/sa@project.iam.gserviceaccount.com:generateAccessToken",
            ))
            .and(bearer_token("mock-sts-token"))
            .and(body_json(serde_json::json!({
                "scope": ["https://www.googleapis.com/auth/cloud-platform"],
                "lifetime": "3600s",
            })))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "accessToken": "mock-sa-token",
                })),
            )
            .mount(&mock_server)
            .await;

        let provider = Box::new(MockOidcProvider("mock-oidc-token".into()));
        let mut cred = WorkloadIdentityCredential::new(
            provider,
            "//iam.googleapis.com/test-audience".into(),
            "sa@project.iam.gserviceaccount.com".into(),
        );
        cred.impersonation_endpoint = Some(format!(
            "{}/v1/projects/-/serviceAccounts/",
            mock_server.uri()
        ));

        let sa_token = cred
            .impersonate_service_account(
                "mock-sts-token",
                &["https://www.googleapis.com/auth/cloud-platform"],
            )
            .await
            .unwrap();

        assert_eq!(sa_token, "mock-sa-token");
    }

    #[tokio::test]
    async fn test_get_token_full_flow() {
        use wiremock::matchers::path_regex;

        let mock_server = MockServer::start().await;

        // Mock STS endpoint
        Mock::given(method("POST"))
            .and(path("/v1/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "mock-sts-token",
                "expires_in": 3600,
            })))
            .mount(&mock_server)
            .await;

        // Mock impersonation endpoint
        Mock::given(method("POST"))
            .and(path_regex(
                r"/v1/projects/-/serviceAccounts/.*:generateAccessToken",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "accessToken": "final-access-token",
            })))
            .mount(&mock_server)
            .await;

        let provider = Box::new(MockOidcProvider("mock-oidc-token".into()));
        let mut cred = WorkloadIdentityCredential::new(
            provider,
            "//iam.googleapis.com/test-audience".into(),
            "sa@project.iam.gserviceaccount.com".into(),
        );
        cred.sts_endpoint = Some(format!("{}/v1/token", mock_server.uri()));
        cred.impersonation_endpoint = Some(format!(
            "{}/v1/projects/-/serviceAccounts/",
            mock_server.uri()
        ));

        let token = cred
            .get_token(&["https://www.googleapis.com/auth/cloud-platform"])
            .await
            .unwrap();

        assert_eq!(token, "final-access-token");
    }

    #[test]
    #[serial]
    fn test_from_environment_with_full_audience() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var(
                "GCP_WORKLOAD_IDENTITY_AUDIENCE",
                "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider",
            );
            std::env::set_var(
                "GCP_SERVICE_ACCOUNT_EMAIL",
                "sa@project.iam.gserviceaccount.com",
            );
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "token");
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_URL", "http://localhost");
        }

        let cred = WorkloadIdentityCredential::from_environment().unwrap();
        assert_eq!(
            cred.audience,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider"
        );
        assert_eq!(
            cred.service_account_email,
            "sa@project.iam.gserviceaccount.com"
        );

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("GCP_WORKLOAD_IDENTITY_AUDIENCE");
            std::env::remove_var("GCP_SERVICE_ACCOUNT_EMAIL");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
        }
    }

    #[test]
    #[serial]
    fn test_from_environment_build_audience_from_parts() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("GCP_WORKLOAD_IDENTITY_AUDIENCE");
            std::env::set_var("GCP_PROJECT_NUMBER", "123");
            std::env::set_var("GCP_WORKLOAD_IDENTITY_POOL_ID", "my-pool");
            std::env::set_var("GCP_WORKLOAD_IDENTITY_PROVIDER_ID", "github-provider");
            std::env::set_var(
                "GCP_SERVICE_ACCOUNT_EMAIL",
                "sa@project.iam.gserviceaccount.com",
            );
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "token");
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_URL", "http://localhost");
        }

        let cred = WorkloadIdentityCredential::from_environment().unwrap();
        assert_eq!(
            cred.audience,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/my-pool/providers/github-provider"
        );

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("GCP_PROJECT_NUMBER");
            std::env::remove_var("GCP_WORKLOAD_IDENTITY_POOL_ID");
            std::env::remove_var("GCP_WORKLOAD_IDENTITY_PROVIDER_ID");
            std::env::remove_var("GCP_SERVICE_ACCOUNT_EMAIL");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
        }
    }

    #[test]
    #[serial]
    fn test_from_json_github() {
        let json = r#"{
            "type": "external_account",
            "audience": "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider",
            "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
            "token_url": "https://sts.googleapis.com/v1/token",
            "credential_source": {
                "environment_id": "github"
            },
            "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/sa@project.iam.gserviceaccount.com:generateAccessToken"
        }"#;

        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "token");
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_URL", "http://localhost");
        }

        let cred = WorkloadIdentityCredential::from_json(json).unwrap();
        assert_eq!(
            cred.audience,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider"
        );
        assert_eq!(
            cred.service_account_email,
            "sa@project.iam.gserviceaccount.com"
        );

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
        }
    }
}
