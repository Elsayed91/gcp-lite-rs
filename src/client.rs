//! Core HTTP client for GCP API access.

use crate::rate_limit::{RateLimitConfig, RateLimitStats, RateLimiter};
use crate::{retry::RetryConfig, token::TokenProvider};
use std::path::Path;
use std::sync::Arc;

/// HTTP client for GCP API operations.
///
/// Provides automatic retry, backoff, and token injection.
pub struct GcpHttpClient {
    http: reqwest::Client,
    token_provider: Arc<dyn TokenProvider>,
    retry_config: RetryConfig,
    /// Optional explicit quota project override.
    quota_project_override: Option<String>,
    rate_limiter: RateLimiter,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) base_url: Option<String>,
    #[cfg(feature = "test-support")]
    mock: Option<Arc<crate::mock_client::MockClient>>,
}

impl GcpHttpClient {
    /// Create a new client builder.
    pub fn builder() -> GcpHttpClientBuilder {
        GcpHttpClientBuilder::default()
    }

    /// Get the effective quota project ID.
    ///
    /// Returns in priority order:
    /// 1. Explicit override set via builder
    /// 2. Quota project from the token provider
    ///
    /// Note: Environment variable `GOOGLE_CLOUD_QUOTA_PROJECT` is checked
    /// at request time, not here.
    pub fn quota_project_id(&self) -> Option<&str> {
        // Priority 1: Explicit override
        if let Some(ref project) = self.quota_project_override {
            return Some(project.as_str());
        }

        // Priority 2: From token provider
        self.token_provider.quota_project_id()
    }

    /// Get a snapshot of current rate limiting state.
    ///
    /// Returns stats for each configured API and the default fallback.
    pub fn rate_limit_stats(&self) -> Vec<RateLimitStats> {
        self.rate_limiter.stats()
    }

    /// Create a client using Application Default Credentials (ADC).
    ///
    /// This is the recommended way to create a client in most environments.
    /// ADC automatically resolves credentials from:
    ///
    /// 1. `GOOGLE_APPLICATION_CREDENTIALS` environment variable
    /// 2. Well-known path `~/.config/gcloud/application_default_credentials.json`
    /// 3. GCP metadata server (when running on GCP infrastructure)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::GcpHttpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let response = client.get("https://example.googleapis.com/api").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if no valid credentials can be found or if a credential
    /// file exists but is invalid.
    pub async fn from_adc() -> Result<Self, crate::auth::AdcError> {
        let credential = crate::auth::AdcCredential::new().await?;

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| crate::auth::AdcError::HttpClientCreation { source: e })?;

        Ok(Self {
            http,
            token_provider: Arc::new(credential),
            retry_config: RetryConfig::default(),
            quota_project_override: None,
            rate_limiter: RateLimiter::new(RateLimitConfig::default().0),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(feature = "test-support")]
            mock: None,
        })
    }

    /// Create a client from a service account JSON key file.
    ///
    /// This is useful when you have a specific service account key file
    /// and don't want to rely on ADC resolution.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the service account JSON key file
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::GcpHttpClient;
    /// use std::path::Path;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_service_account_file(
    ///     Path::new("/path/to/service-account.json")
    /// )?;
    /// let response = client.get("https://example.googleapis.com/api").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or contains invalid credentials.
    pub fn from_service_account_file(
        path: &Path,
    ) -> Result<Self, crate::auth::ServiceAccountError> {
        let credential = crate::auth::ServiceAccountCredential::from_file(path)?;

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| crate::auth::ServiceAccountError::HttpClientCreation { source: e })?;

        Ok(Self {
            http,
            token_provider: Arc::new(credential),
            retry_config: RetryConfig::default(),
            quota_project_override: None,
            rate_limiter: RateLimiter::new(RateLimitConfig::default().0),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(feature = "test-support")]
            mock: None,
        })
    }

    /// Create a client from a mock for testing
    #[cfg(feature = "test-support")]
    pub fn from_mock(mock: crate::mock_client::MockClient) -> Self {
        Self {
            http: reqwest::Client::new(),
            token_provider: Arc::new(crate::token::StaticTokenProvider::new("mock-token")),
            retry_config: RetryConfig::default(),
            quota_project_override: None,
            rate_limiter: RateLimiter::new(RateLimitConfig::default().0),
            base_url: None, // Use default API base URLs for mocks
            mock: Some(Arc::new(mock)),
        }
    }

    /// Perform GET request with retry.
    pub async fn get(&self, url: &str) -> crate::Result<bytes::Bytes> {
        self.request_with_retry(reqwest::Method::GET, url, None)
            .await
    }

    /// Perform POST request with retry.
    pub async fn post(
        &self,
        url: &str,
        body: &impl serde::Serialize,
    ) -> crate::Result<bytes::Bytes> {
        let bytes = serde_json::to_vec(body).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to serialize request body: {e}"),
            body: None,
        })?;
        self.request_with_retry(reqwest::Method::POST, url, Some(bytes))
            .await
    }

    /// Perform DELETE request with retry.
    pub async fn delete(&self, url: &str) -> crate::Result<bytes::Bytes> {
        self.request_with_retry(reqwest::Method::DELETE, url, None)
            .await
    }

    /// Perform PUT request with retry.
    pub async fn put(
        &self,
        url: &str,
        body: &impl serde::Serialize,
    ) -> crate::Result<bytes::Bytes> {
        let bytes = serde_json::to_vec(body).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to serialize request body: {e}"),
            body: None,
        })?;
        self.request_with_retry(reqwest::Method::PUT, url, Some(bytes))
            .await
    }

    /// Perform PATCH request with retry.
    pub async fn patch(
        &self,
        url: &str,
        body: &impl serde::Serialize,
    ) -> crate::Result<bytes::Bytes> {
        let bytes = serde_json::to_vec(body).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to serialize request body: {e}"),
            body: None,
        })?;
        self.request_with_retry(reqwest::Method::PATCH, url, Some(bytes))
            .await
    }

    // === Generated API Accessors (do not edit) ===

    /// Access the Access Approval API API
    pub fn access_approval(&self) -> crate::api::AccessApprovalClient<'_> {
        crate::api::AccessApprovalClient::new(self)
    }

    /// Access the API Keys API API
    pub fn apikeys(&self) -> crate::api::ApikeysClient<'_> {
        crate::api::ApikeysClient::new(self)
    }

    /// Access the App Engine Admin API API
    pub fn appengine(&self) -> crate::api::AppEngineClient<'_> {
        crate::api::AppEngineClient::new(self)
    }

    /// Access the BigQuery API
    pub fn bigquery(&self) -> crate::api::BigqueryClient<'_> {
        crate::api::BigqueryClient::new(self)
    }

    /// Access the Cloud Asset API
    pub fn cloud_asset(&self) -> crate::api::CloudAssetClient<'_> {
        crate::api::CloudAssetClient::new(self)
    }

    /// Access the Cloud Billing API
    pub fn billing(&self) -> crate::api::BillingClient<'_> {
        crate::api::BillingClient::new(self)
    }

    /// Access the Cloud KMS API API
    pub fn kms(&self) -> crate::api::KmsClient<'_> {
        crate::api::KmsClient::new(self)
    }

    /// Access the Cloud Resource Manager API
    pub fn projects(&self) -> crate::api::ProjectsClient<'_> {
        crate::api::ProjectsClient::new(self)
    }

    /// Access the Cloud Scheduler API
    pub fn scheduler(&self) -> crate::api::SchedulerClient<'_> {
        crate::api::SchedulerClient::new(self)
    }

    /// Access the Compute Engine API
    pub fn compute(&self) -> crate::api::ComputeClient<'_> {
        crate::api::ComputeClient::new(self)
    }

    /// Access the Kubernetes Engine API
    pub fn container(&self) -> crate::api::ContainerClient<'_> {
        crate::api::ContainerClient::new(self)
    }

    /// Access the Cloud DLP API API
    pub fn dlp(&self) -> crate::api::DlpClient<'_> {
        crate::api::DlpClient::new(self)
    }

    /// Access the Cloud DNS API API
    pub fn dns(&self) -> crate::api::DnsClient<'_> {
        crate::api::DnsClient::new(self)
    }

    /// Access the Essential Contacts API API
    pub fn essential_contacts(&self) -> crate::api::EssentialContactsClient<'_> {
        crate::api::EssentialContactsClient::new(self)
    }

    /// Access the Backup for GKE API
    pub fn gkebackup(&self) -> crate::api::GkeBackupClient<'_> {
        crate::api::GkeBackupClient::new(self)
    }

    /// Access the IAM API
    pub fn iam(&self) -> crate::api::IamClient<'_> {
        crate::api::IamClient::new(self)
    }

    /// Access the Cloud Logging API API
    pub fn logging(&self) -> crate::api::LoggingClient<'_> {
        crate::api::LoggingClient::new(self)
    }

    /// Access the Cloud Monitoring API API
    pub fn monitoring(&self) -> crate::api::MonitoringClient<'_> {
        crate::api::MonitoringClient::new(self)
    }

    /// Access the OS Config API API
    pub fn osconfig(&self) -> crate::api::OsConfigClient<'_> {
        crate::api::OsConfigClient::new(self)
    }

    /// Access the Recommender API API
    pub fn recommender(&self) -> crate::api::RecommenderClient<'_> {
        crate::api::RecommenderClient::new(self)
    }

    /// Access the Secret Manager API
    pub fn secret_manager(&self) -> crate::api::SecretManagerClient<'_> {
        crate::api::SecretManagerClient::new(self)
    }

    /// Access the Service Usage API
    pub fn service_usage(&self) -> crate::api::ServiceUsageClient<'_> {
        crate::api::ServiceUsageClient::new(self)
    }

    /// Access the Cloud SQL Admin API API
    pub fn sqladmin(&self) -> crate::api::SqladminClient<'_> {
        crate::api::SqladminClient::new(self)
    }

    /// Access the Cloud Storage JSON API API
    pub fn storage(&self) -> crate::api::StorageClient<'_> {
        crate::api::StorageClient::new(self)
    }
    // === End Generated API Accessors ===

    /// Resolve the effective quota project ID.
    ///
    /// Priority order:
    /// 1. Explicit override set via builder
    /// 2. `GOOGLE_CLOUD_QUOTA_PROJECT` environment variable
    /// 3. Quota project from the token provider
    fn resolve_quota_project(&self) -> Option<String> {
        // Priority 1: Explicit override
        if let Some(ref project) = self.quota_project_override {
            return Some(project.clone());
        }

        // Priority 2: Environment variable
        if let Ok(project) = std::env::var("GOOGLE_CLOUD_QUOTA_PROJECT")
            && !project.is_empty()
        {
            return Some(project);
        }

        // Priority 3: From token provider
        self.token_provider
            .quota_project_id()
            .map(|s| s.to_string())
    }

    /// Internal: Perform request with retry logic.
    async fn request_with_retry(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<Vec<u8>>,
    ) -> crate::Result<bytes::Bytes> {
        // Check for mock first
        #[cfg(feature = "test-support")]
        if let Some(ref mock) = self.mock {
            let value = body
                .as_deref()
                .map(serde_json::from_slice::<serde_json::Value>)
                .transpose()
                .map_err(|e| crate::GcpError::InvalidResponse {
                    message: format!("Failed to deserialize request body for mock: {e}"),
                    body: None,
                })?;
            let result = mock.execute(method.as_str(), url, value.as_ref()).await?;
            return Ok(bytes::Bytes::from(result));
        }

        let _permit = self.rate_limiter.acquire(url).await;

        // Resolve quota project once (priority: explicit > env var > provider)
        let quota_project = self.resolve_quota_project();

        let mut attempt = 0;
        let mut backoff = self.retry_config.initial_backoff;

        loop {
            // Get fresh token
            let token = self
                .token_provider
                .get_token(&["https://www.googleapis.com/auth/cloud-platform"])
                .await
                .map_err(|e| crate::GcpError::InvalidResponse {
                    message: format!("Failed to get token: {}", e),
                    body: None,
                })?;

            // Build request
            let mut request = self.http.request(method.clone(), url);
            request = request.header("Authorization", format!("Bearer {}", token));

            // Add quota project header if available
            if let Some(ref project) = quota_project {
                request = request.header("x-goog-user-project", project.as_str());
            }

            if let Some(ref body) = body {
                request = request
                    .header("Content-Type", "application/json")
                    .body(body.clone());
            }

            // Execute request
            let response = request.send().await.map_err(crate::GcpError::from)?;

            // Classify response
            match self.classify_response(response, url, method.as_str()).await {
                Ok(data) => return Ok(data),

                Err(crate::GcpError::Auth { .. })
                    if attempt == 0 && self.retry_config.retry_on_401 =>
                {
                    // Token expired, invalidate and retry once
                    self.token_provider.on_token_rejected();
                    attempt += 1;
                    continue;
                }

                Err(e) if e.is_retryable() && attempt < self.retry_config.max_retries => {
                    // Retryable error, backoff and retry
                    let delay = self.retry_config.compute_backoff(backoff, e.retry_after());
                    tokio::time::sleep(delay).await;

                    backoff = std::time::Duration::from_secs_f64(
                        backoff.as_secs_f64() * self.retry_config.backoff_multiplier,
                    );
                    attempt += 1;
                    continue;
                }

                Err(e) => return Err(e),
            }
        }
    }

    /// Classify HTTP response into appropriate GcpError.
    async fn classify_response(
        &self,
        response: reqwest::Response,
        resource: &str,
        method: &str,
    ) -> crate::Result<bytes::Bytes> {
        let status = response.status();
        let _status_code = status.as_u16();

        match status {
            reqwest::StatusCode::OK
            | reqwest::StatusCode::CREATED
            | reqwest::StatusCode::NO_CONTENT => {
                let body = response.bytes().await.map_err(crate::GcpError::from)?;
                Ok(body)
            }

            reqwest::StatusCode::UNAUTHORIZED => {
                let body = response.text().await.unwrap_or_default();
                Err(crate::GcpError::Auth {
                    message: Self::extract_error_message(&body),
                })
            }

            reqwest::StatusCode::FORBIDDEN => {
                let body = response.text().await.unwrap_or_default();
                let message = Self::extract_error_message(&body);

                // Check if it's API not enabled
                if message.contains("API") && message.contains("not enabled") {
                    let api = Self::extract_api_name(&message);
                    Err(crate::GcpError::ApiNotEnabled { api, message })
                } else {
                    Err(crate::GcpError::PermissionDenied {
                        message,
                        resource: resource.to_string(),
                        method: method.to_string(),
                    })
                }
            }

            reqwest::StatusCode::NOT_FOUND => Err(crate::GcpError::NotFound {
                resource: resource.to_string(),
                method: method.to_string(),
            }),

            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok());

                let body = response.text().await.unwrap_or_default();
                Err(crate::GcpError::RateLimited {
                    retry_after,
                    message: Self::extract_error_message(&body),
                    resource: resource.to_string(),
                })
            }

            reqwest::StatusCode::BAD_REQUEST => {
                let body = response.text().await.unwrap_or_default();
                let message = Self::extract_error_message(&body);
                let field = Self::extract_field_name(&body);

                Err(crate::GcpError::InvalidArgument { message, field })
            }

            s if s.is_server_error() => {
                let body = response.text().await.unwrap_or_default();
                let message = Self::extract_error_message(&body);

                let retryable = matches!(
                    s,
                    reqwest::StatusCode::SERVICE_UNAVAILABLE
                        | reqwest::StatusCode::INTERNAL_SERVER_ERROR
                );

                Err(crate::GcpError::ServerError {
                    status: s.as_u16(),
                    message,
                    resource: resource.to_string(),
                    retryable,
                })
            }

            _ => {
                let body = response.text().await.ok();
                Err(crate::GcpError::InvalidResponse {
                    message: format!("Unexpected status: {}", status),
                    body,
                })
            }
        }
    }

    fn extract_error_message(body: &str) -> String {
        // Parse GCP's JSON error response
        // { "error": { "message": "...", "code": 403, ... } }
        serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|json| {
                json.get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| body.to_string())
    }

    fn extract_api_name(message: &str) -> String {
        // Simple extraction: look for "*.googleapis.com"
        message
            .split_whitespace()
            .find(|s| s.contains("googleapis.com"))
            .unwrap_or("unknown")
            .to_string()
    }

    fn extract_field_name(body: &str) -> Option<String> {
        serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|json| {
                json.get("error")
                    .and_then(|e| e.get("field"))
                    .and_then(|f| f.as_str())
                    .map(|s| s.to_string())
            })
    }
}

/// Errors that can occur when building a GcpHttpClient.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Token provider was not set.
    #[error("TokenProvider is required")]
    MissingTokenProvider,

    /// Failed to create HTTP client.
    #[error("Failed to create HTTP client: {source}")]
    HttpClientCreation {
        /// The underlying reqwest error.
        #[source]
        source: reqwest::Error,
    },
}

/// Builder for GcpHttpClient.
#[derive(Default)]
pub struct GcpHttpClientBuilder {
    token_provider: Option<Arc<dyn TokenProvider>>,
    http_client: Option<reqwest::Client>,
    retry_config: Option<RetryConfig>,
    rate_limit_config: Option<RateLimitConfig>,
    quota_project: Option<String>,
    #[cfg(any(test, feature = "test-support"))]
    base_url: Option<String>,
}

impl GcpHttpClientBuilder {
    /// Set the token provider.
    ///
    /// # Arguments
    ///
    /// * `provider` - Implementation of TokenProvider trait
    pub fn token_provider<T: TokenProvider + 'static>(mut self, provider: T) -> Self {
        self.token_provider = Some(Arc::new(provider));
        self
    }

    /// Set custom retry configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    /// Use a pre-built `reqwest::Client` instead of creating one internally.
    ///
    /// This enables connection pooling when multiple `GcpHttpClient` instances
    /// share the same underlying HTTP client. The `reqwest::Client` uses `Arc`
    /// internally, so cloning is cheap.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use gcp_lite::GcpHttpClient;
    /// use gcp_lite::token::StaticTokenProvider;
    /// use std::time::Duration;
    ///
    /// let shared_http = reqwest::Client::builder()
    ///     .pool_max_idle_per_host(10)
    ///     .timeout(Duration::from_secs(60))
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = GcpHttpClient::builder()
    ///     .http_client(shared_http)
    ///     .token_provider(StaticTokenProvider::new("token"))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Set custom rate limiting configuration.
    ///
    /// By default, all clients use `RateLimitConfig::default()` which provides
    /// conservative per-API concurrency limits based on GCP quotas.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use gcp_lite::{GcpHttpClient, RateLimitConfig};
    /// use gcp_lite::token::StaticTokenProvider;
    ///
    /// let client = GcpHttpClient::builder()
    ///     .token_provider(StaticTokenProvider::new("token"))
    ///     .rate_limit(
    ///         RateLimitConfig::default()
    ///             .with_api_limit("cloudasset.googleapis.com", 5)
    ///     )
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit_config = Some(config);
        self
    }

    /// Set an explicit quota project for billing.
    ///
    /// When set, this overrides any quota project from the token provider
    /// or environment variables. The `x-goog-user-project` header will be
    /// added to all requests.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID to use for quota/billing
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use gcp_lite::GcpHttpClient;
    /// use gcp_lite::token::StaticTokenProvider;
    ///
    /// let client = GcpHttpClient::builder()
    ///     .token_provider(StaticTokenProvider::new("token"))
    ///     .quota_project("my-billing-project")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn quota_project(mut self, project: impl Into<String>) -> Self {
        self.quota_project = Some(project.into());
        self
    }

    /// Set base URL for testing.
    ///
    /// # Arguments
    ///
    /// * `url` - Base URL for API requests (only available in tests)
    #[cfg(any(test, feature = "test-support"))]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Build the client.
    ///
    /// # Errors
    ///
    /// Returns an error if token_provider was not set or if HTTP client creation fails.
    pub fn build(self) -> Result<GcpHttpClient, BuilderError> {
        let token_provider = self
            .token_provider
            .ok_or(BuilderError::MissingTokenProvider)?;

        let retry_config = self.retry_config.unwrap_or_default();
        let rate_limiter = RateLimiter::new(self.rate_limit_config.unwrap_or_default().0);

        let http = match self.http_client {
            Some(client) => client,
            None => reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .map_err(|e| BuilderError::HttpClientCreation { source: e })?,
        };

        Ok(GcpHttpClient {
            http,
            token_provider,
            retry_config,
            quota_project_override: self.quota_project,
            rate_limiter,
            #[cfg(any(test, feature = "test-support"))]
            base_url: self.base_url,
            #[cfg(feature = "test-support")]
            mock: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::StaticTokenProvider;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn builder_creates_client() {
        let provider = StaticTokenProvider::new("token");
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .build()
            .unwrap();

        assert!(Arc::strong_count(&client.token_provider) >= 1);
    }

    // Test RSA private key (DO NOT USE IN PRODUCTION - this is for testing only)
    const TEST_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEAv/ZgLU6ZAC6tVLSjrzeVeigCbmFVXFqeohH0wsveMsgUnktr
KoQUhclteEs7iGLwKPQdWyOMIaQFC320l1wqvJKg7XbWtYyC856yBtHBisXzjUIP
vppA7Ie6N0uKtqZ1HLXKbqEd5bNPEU61LJJgLXOdXRbb+9EhLusQpWQb3cPLI/za
qmSa6UYwEv8GCtIgNqTjKycHCi0MqKpjNZ6wFzvPruLqnkOhtA2LVsklp+9jYxca
yv9UBS5xVQ01WHHSR/J//G2v0yCUzmitdDJTQyOd4zlPkHpm6T69m7NaE1fTVSiP
sVfO3Hn7VfzAgbQkZH40Q+OSlTubBQcZ8JyJWQIDAQABAoIBAAJOMBqt3GO2lnbT
YjmJvPDAt8IXHFUVsoeG7diuuvOtraUMCf9dTY4gx0DgAxjwz+pnVM3s0p3vJW9d
T7SsqEe8/r6eBCyd1s8cYLjOaUO50Q0T1h1nfAWgiKw+1Zg6zg0YTX7VeQbdR9hm
SKyTx8tr8sp01T4EOuDgdQH40+aD0ivfbFdIyim82IGh7HHvJyMVxTsMHG1fRo/d
kYpT3g2jOpEYCe0EnmAp2bB1kLLonlW+Xp3OOYShLXUtwXf8q/fOMcYbm3BV2OvC
zhTaKmvStEpMhLHihsNJIf0uQZypY6lNu9IDhj2dacKjRNEpZn+ulwfdcx46VEAy
gnt9IKECgYEA/Y4fSVQ4TsGJvH4vjb0+3o4uckWEPTffUwJG6r4PYSZGA8qQpr1E
oyxXdZt0atcsXs1Qd1E2t2FruumS18CTQvJE0+q06kqa6djYH7svOzfjxC5F4Np4
EkMYEVecJ5qG1bU77BqJwY4rTRQ96e5PmgjTPV9hfxvNX51JTm06gfECgYEAwdA3
pabK+6x2hjxyNLldPdcctFZqA16OeL5VWk5/7L2kLFUYVjyz129vAtkJKnB+GAJs
uZcNwhBbqSiK0vv5WoQOIzrSzbzLZ7STjymMzZ6FOkdElbD6H811idKpI87fQD0O
Eo4L5wslqIVxR8ktoRJgRvI6sFm1ajSsarlrlekCgYAC7cJUwYFI/5lMsRRxia8R
OQk2TrFBV8Tfm5YgHgPldmC2qH9VPbhuPhPgiuQkW8nqamq0hh6graJl7U7B6TqK
OmwrGnnufuAdNWEBtNLN105tNK+f8kYSx+2ePanTF0jZbRd9Ga1fq/m6ETLJ4fPP
bqyp99ETe8m6ggGXw1E6sQKBgG950rf90pylWtrk44992qqaEtGLLpjXhzzdxPwX
UK8beNVi8IeRjKNqXcCWkxYM9AndQyoQPwKTJBWM0yR9d7PfZr5OtDdP0vLIQ2NB
s9IEzn5xxXoP/B3UsDlgqJaHA5PQSkrT1vbCS5u9fSWcChmuFyBXbPhH8PewakdM
dRwZAoGAexRCCrNBek6bxaCVI8JqRRIGYPAwr9sUjKwn8Tdhutx8lvcKOk6AHG1I
uQAVf8HQ3eHRgsCSodf1XeoLWX+0Nxt/KJ1KotVlchFlCLuSzpNkR7WbLC7QfCkJ
RLK9OKOIcBVVctVsUtrWLjTEHyKVhYwIW98X+LAal+i55n75SHU=
-----END RSA PRIVATE KEY-----";

    fn create_test_service_account_json() -> String {
        format!(
            r#"{{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "key123",
            "private_key": {:?},
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }}"#,
            TEST_PRIVATE_KEY
        )
    }

    #[test]
    fn from_service_account_file_creates_client() {
        let temp_dir = TempDir::new().unwrap();
        let key_path = temp_dir.path().join("service_account.json");
        fs::write(&key_path, create_test_service_account_json()).unwrap();

        let client = GcpHttpClient::from_service_account_file(&key_path).unwrap();

        assert!(Arc::strong_count(&client.token_provider) >= 1);
    }

    #[test]
    fn from_service_account_file_not_found() {
        let path = Path::new("/nonexistent/file.json");
        let result = GcpHttpClient::from_service_account_file(path);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::auth::ServiceAccountError::FileReadError { .. })
        ));
    }

    #[test]
    fn from_service_account_file_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let key_path = temp_dir.path().join("invalid.json");
        fs::write(&key_path, "not valid json").unwrap();

        let result = GcpHttpClient::from_service_account_file(&key_path);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::auth::ServiceAccountError::InvalidJson { .. })
        ));
    }

    #[tokio::test]
    #[serial]
    async fn from_adc_with_env_var() {
        let temp_dir = TempDir::new().unwrap();
        let cred_path = temp_dir.path().join("adc_creds.json");
        fs::write(&cred_path, create_test_service_account_json()).unwrap();

        // Set the environment variable
        let prev_value = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();
        // SAFETY: This is a test and we restore the value after
        unsafe {
            std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", &cred_path);
        }

        let result = GcpHttpClient::from_adc().await;

        // Restore the environment variable
        // SAFETY: This is a test and we're restoring the original value
        unsafe {
            if let Some(prev) = prev_value {
                std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", prev);
            } else {
                std::env::remove_var("GOOGLE_APPLICATION_CREDENTIALS");
            }
        }

        assert!(result.is_ok());
        let client = result.unwrap();
        assert!(Arc::strong_count(&client.token_provider) >= 1);
    }

    #[tokio::test]
    #[serial]
    async fn from_adc_with_invalid_env_var() {
        // Set the environment variable to a non-existent file
        let prev_value = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();
        let prev_gcloud = std::env::var("GOOGLE_AUTH_USE_GCLOUD").ok();
        // SAFETY: This is a test and we restore the value after
        unsafe {
            std::env::set_var(
                "GOOGLE_APPLICATION_CREDENTIALS",
                "/nonexistent/path/to/creds.json",
            );
            std::env::remove_var("GOOGLE_AUTH_USE_GCLOUD");
        }

        let result = GcpHttpClient::from_adc().await;

        // Restore the environment variable
        // SAFETY: This is a test and we're restoring the original value
        unsafe {
            if let Some(prev) = prev_value {
                std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", prev);
            } else {
                std::env::remove_var("GOOGLE_APPLICATION_CREDENTIALS");
            }
            if let Some(prev) = prev_gcloud {
                std::env::set_var("GOOGLE_AUTH_USE_GCLOUD", prev);
            }
        }

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(crate::auth::AdcError::FileReadError { .. })
        ));
    }

    #[test]
    fn builder_default_has_rate_limiter() {
        let provider = StaticTokenProvider::new("token");
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .build()
            .unwrap();

        // Should have stats with default limits
        let stats = client.rate_limit_stats();
        let default = stats.iter().find(|s| s.api == "default").unwrap();
        assert_eq!(default.limit, 20);
    }

    #[test]
    fn builder_with_custom_rate_limit() {
        let provider = StaticTokenProvider::new("token");
        let config = crate::RateLimitConfig::default().with_default_limit(50);
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .rate_limit(config)
            .build()
            .unwrap();

        let stats = client.rate_limit_stats();
        let default = stats.iter().find(|s| s.api == "default").unwrap();
        assert_eq!(default.limit, 50);
    }

    #[test]
    fn builder_with_disabled_rate_limit() {
        let provider = StaticTokenProvider::new("token");
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .rate_limit(crate::RateLimitConfig::disabled())
            .build()
            .unwrap();

        let stats = client.rate_limit_stats();
        let default = stats.iter().find(|s| s.api == "default").unwrap();
        // usize::MAX gets capped to tokio's Semaphore::MAX_PERMITS
        assert_eq!(default.limit, tokio::sync::Semaphore::MAX_PERMITS);
    }

    #[test]
    fn builder_with_quota_project() {
        let provider = StaticTokenProvider::new("token");
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .quota_project("explicit-quota-project")
            .build()
            .unwrap();

        assert_eq!(client.quota_project_id(), Some("explicit-quota-project"));
    }

    #[test]
    fn builder_without_quota_project_uses_provider() {
        // StaticTokenProvider returns None for quota_project_id
        let provider = StaticTokenProvider::new("token");
        let client = GcpHttpClient::builder()
            .token_provider(provider)
            .build()
            .unwrap();

        assert!(client.quota_project_id().is_none());
    }
}
