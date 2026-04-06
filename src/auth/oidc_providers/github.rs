//! GitHub Actions OIDC provider implementation.

use super::{OidcError, OidcTokenProvider};
use async_trait::async_trait;
use urlencoding::encode;

/// OIDC provider for GitHub Actions.
///
/// Fetches OIDC tokens from GitHub Actions runtime using environment variables:
/// - `ACTIONS_ID_TOKEN_REQUEST_TOKEN` - Request token
/// - `ACTIONS_ID_TOKEN_REQUEST_URL` - Request URL
/// - `ACTIONS_ID_TOKEN_AUDIENCE` - Optional audience (defaults to none)
pub struct GitHubActionsProvider {
    request_token: String,
    request_url: String,
    audience: Option<String>,
}

impl GitHubActionsProvider {
    /// Create a new GitHub Actions provider from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing.
    pub fn new() -> Result<Self, OidcError> {
        let request_token = std::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN").map_err(|_| {
            OidcError::MissingEnvVar {
                var: "ACTIONS_ID_TOKEN_REQUEST_TOKEN".into(),
            }
        })?;

        let request_url = std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").map_err(|_| {
            OidcError::MissingEnvVar {
                var: "ACTIONS_ID_TOKEN_REQUEST_URL".into(),
            }
        })?;

        let audience = std::env::var("ACTIONS_ID_TOKEN_AUDIENCE").ok();

        Ok(Self {
            request_token,
            request_url,
            audience,
        })
    }
}

#[async_trait]
impl OidcTokenProvider for GitHubActionsProvider {
    async fn get_token(&self) -> Result<String, OidcError> {
        let url = if let Some(aud) = &self.audience {
            format!("{}?audience={}", self.request_url, encode(aud))
        } else {
            self.request_url.clone()
        };

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .bearer_auth(&self.request_token)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        json["value"]
            .as_str()
            .ok_or(OidcError::InvalidResponse)
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    #[serial]
    async fn test_github_provider_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/token"))
            .and(query_param("audience", "https://iam.googleapis.com"))
            .and(bearer_token("test-request-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": "mock-oidc-token"
            })))
            .mount(&mock_server)
            .await;

        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "test-request-token");
            std::env::set_var(
                "ACTIONS_ID_TOKEN_REQUEST_URL",
                format!("{}/token", mock_server.uri()),
            );
            std::env::set_var("ACTIONS_ID_TOKEN_AUDIENCE", "https://iam.googleapis.com");
        }

        let provider = GitHubActionsProvider::new().unwrap();
        let token = provider.get_token().await.unwrap();

        assert_eq!(token, "mock-oidc-token");

        // SAFETY: Test cleanup
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
            std::env::remove_var("ACTIONS_ID_TOKEN_AUDIENCE");
        }
    }

    #[test]
    #[serial]
    fn test_github_provider_missing_env() {
        // SAFETY: Test is single-threaded for env vars
        unsafe {
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
            std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
        }

        let result = GitHubActionsProvider::new();
        assert!(result.is_err());
    }
}
