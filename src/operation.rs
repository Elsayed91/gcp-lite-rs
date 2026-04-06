//! Long-running operation polling for GCP APIs.

use crate::{GcpError, GcpHttpClient, Result};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Status of an operation poll
#[derive(Debug)]
enum PollStatus {
    /// Operation is still in progress
    InProgress,
    /// Operation completed successfully
    Done,
    /// Operation failed with error
    Failed {
        message: String,
        code: Option<String>,
    },
}

/// Trait for polling GCP operations with different response formats
trait OperationPoller {
    /// Build the polling URL for this operation
    fn build_poll_url(&self, client: &GcpHttpClient) -> String;

    /// Parse the response and determine operation status
    fn parse_status(&self, response: &[u8]) -> Result<PollStatus>;

    /// Get operation identifier for error reporting
    fn operation_id(&self) -> &str;
}

/// Generic polling function for any operation type
async fn poll_operation<P: OperationPoller>(
    poller: &P,
    client: &GcpHttpClient,
    initial_interval: Duration,
    timeout: Duration,
) -> Result<()> {
    let mut interval = initial_interval;
    let deadline = Instant::now() + timeout;
    let max_interval = Duration::from_secs(5);
    let backoff_multiplier = 1.5;

    let url = poller.build_poll_url(client);

    loop {
        // Poll operation status first (always attempt at least one poll)
        let response = client.get(&url).await?;
        let status = poller.parse_status(&response)?;

        match status {
            PollStatus::Done => return Ok(()),
            PollStatus::Failed { message, code } => {
                return Err(GcpError::OperationFailed {
                    operation: poller.operation_id().to_string(),
                    message,
                    code,
                });
            }
            PollStatus::InProgress => {
                // Check timeout after polling (ensures at least one poll attempt)
                if Instant::now() >= deadline {
                    return Err(GcpError::OperationTimeout {
                        operation: poller.operation_id().to_string(),
                        timeout,
                    });
                }

                // Wait before next poll
                tokio::time::sleep(interval).await;

                // Adaptive backoff
                interval = Duration::from_secs_f64(
                    (interval.as_secs_f64() * backoff_multiplier).min(max_interval.as_secs_f64()),
                );
            }
        }
    }
}

/// A long-running Compute Engine operation.
///
/// Polls via the operation's `selfLink` URL until `status == "DONE"`.
pub struct Operation<'a> {
    client: &'a GcpHttpClient,
    operation_url: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> Operation<'a> {
    /// Create a new operation.
    pub fn new(
        client: &'a GcpHttpClient,
        operation_url: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            operation_url,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl<'a> OperationPoller for Operation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.operation_url.clone()
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        let status: ComputeOperationStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.status == "DONE" {
            if let Some(error) = status.error {
                let message = error
                    .errors
                    .first()
                    .and_then(|e| e.message.clone())
                    .unwrap_or_else(|| "Unknown error".to_string());
                let code = error.errors.first().and_then(|e| e.code.clone());

                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_url
    }
}

/// A long-running Service Usage operation.
///
/// Polls via `serviceusage.googleapis.com/v1/{name}` until `done == true`.
pub struct ServiceUsageOperation<'a> {
    client: &'a GcpHttpClient,
    operation_name: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> ServiceUsageOperation<'a> {
    /// Create a new Service Usage operation.
    pub fn new(
        client: &'a GcpHttpClient,
        operation_name: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            operation_name,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return format!("{}{}", base.trim_end_matches('/'), path);
            }
        }

        format!("https://serviceusage.googleapis.com{}", path)
    }
}

impl<'a> OperationPoller for ServiceUsageOperation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.build_url(&format!("/v1/{}", self.operation_name))
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        let status: LroStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.done {
            if let Some(error) = status.error {
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = error.get("code").and_then(|v| {
                    v.as_str()
                        .map(String::from)
                        .or_else(|| v.as_i64().map(|n| n.to_string()))
                });
                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_name
    }
}

/// A long-running Cloud Resource Manager operation.
///
/// Polls via `cloudresourcemanager.googleapis.com/v3/{name}` until `done == true`.
pub struct ResourceManagerOperation<'a> {
    client: &'a GcpHttpClient,
    operation_name: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> ResourceManagerOperation<'a> {
    /// Create a new Resource Manager operation.
    pub fn new(
        client: &'a GcpHttpClient,
        operation_name: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            operation_name,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return format!("{}{}", base.trim_end_matches('/'), path);
            }
        }

        format!("https://cloudresourcemanager.googleapis.com{}", path)
    }
}

impl<'a> OperationPoller for ResourceManagerOperation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.build_url(&format!("/v3/{}", self.operation_name))
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        let status: LroStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.done {
            if let Some(error) = status.error {
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = error.get("code").and_then(|v| {
                    v.as_str()
                        .map(String::from)
                        .or_else(|| v.as_i64().map(|n| n.to_string()))
                });
                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_name
    }
}

/// A long-running GKE Container operation.
///
/// Polls via the operation's `selfLink` URL until `status == "DONE"`.
/// GKE operations use status-string polling (like Compute) with selfLink-based URLs.
pub struct ContainerOperation<'a> {
    client: &'a GcpHttpClient,
    operation_url: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> ContainerOperation<'a> {
    /// Create a new Container operation.
    pub fn new(
        client: &'a GcpHttpClient,
        operation_url: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            operation_url,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl<'a> OperationPoller for ContainerOperation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.operation_url.clone()
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        let status: ContainerOperationStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.status == "DONE" {
            if let Some(error) = status.error {
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = error
                    .get("code")
                    .and_then(|v| v.as_i64().map(|n| n.to_string()));
                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_url
    }
}

/// A long-running GKE Backup operation.
///
/// Polls via `gkebackup.googleapis.com/v1/{name}` until `done == true`.
pub struct GkeBackupOperation<'a> {
    client: &'a GcpHttpClient,
    operation_name: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> GkeBackupOperation<'a> {
    /// Create a new GKE Backup operation.
    pub fn new(
        client: &'a GcpHttpClient,
        operation_name: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            operation_name,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return format!("{}{}", base.trim_end_matches('/'), path);
            }
        }

        format!("https://gkebackup.googleapis.com{}", path)
    }
}

impl<'a> OperationPoller for GkeBackupOperation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.build_url(&format!("/v1/{}", self.operation_name))
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        let status: LroStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.done {
            if let Some(error) = status.error {
                let message = error
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = error.get("code").and_then(|v| {
                    v.as_str()
                        .map(String::from)
                        .or_else(|| v.as_i64().map(|n| n.to_string()))
                });
                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_name
    }
}

/// A long-running Cloud SQL operation.
///
/// Polls via `sqladmin.googleapis.com/v1/projects/{project}/operations/{name}`
/// until `status == "DONE"`.
pub struct SqlOperation<'a> {
    client: &'a GcpHttpClient,
    project: String,
    operation_name: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}

impl<'a> SqlOperation<'a> {
    /// Create a new Cloud SQL operation.
    pub fn new(
        client: &'a GcpHttpClient,
        project: String,
        operation_name: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self {
            client,
            project,
            operation_name,
            initial_interval,
            timeout,
            initially_done,
        }
    }

    /// Wait for the operation to complete.
    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    /// Override the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return format!("{}{}", base.trim_end_matches('/'), path);
            }
        }

        format!("https://sqladmin.googleapis.com{}", path)
    }
}

impl<'a> OperationPoller for SqlOperation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.build_url(&format!(
            "/v1/projects/{}/operations/{}",
            self.project, self.operation_name
        ))
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        // Cloud SQL uses status == "DONE" (same format as Compute)
        let status: ComputeOperationStatus =
            serde_json::from_slice(response).map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.status == "DONE" {
            if let Some(error) = status.error {
                let message = error
                    .errors
                    .first()
                    .and_then(|e| e.message.clone())
                    .unwrap_or_else(|| "Unknown error".to_string());
                let code = error.errors.first().and_then(|e| e.code.clone());

                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_name
    }
}

// ── Internal types ──────────────────────────────────────────────────

/// Compute Engine operation poll response.
#[derive(Debug, Deserialize, Serialize)]
struct ComputeOperationStatus {
    name: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ComputeOperationError>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComputeOperationError {
    errors: Vec<ComputeOperationErrorDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComputeOperationErrorDetail {
    code: Option<String>,
    message: Option<String>,
}

/// GKE Container operation poll response (status-string based, like Compute).
#[derive(Debug, Deserialize)]
struct ContainerOperationStatus {
    #[allow(dead_code)]
    name: String,
    status: String,
    error: Option<serde_json::Value>,
}

/// Google LRO-style poll response (used by Service Usage, GKE Backup, etc.).
#[derive(Debug, Deserialize)]
struct LroStatus {
    #[serde(default)]
    done: bool,
    error: Option<serde_json::Value>,
}

// ── Poll configs ────────────────────────────────────────────────────

/// Per-operation-type polling configuration.
pub struct PollConfig {
    initial_interval: Duration,
    timeout: Duration,
}

impl PollConfig {
    /// Configuration for disk operations (5 minutes).
    pub fn disk_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(1),
            timeout: Duration::from_secs(300),
        }
    }

    /// Configuration for service usage operations (2 minutes).
    pub fn service_usage_operation() -> Self {
        Self {
            initial_interval: Duration::from_millis(500),
            timeout: Duration::from_secs(120),
        }
    }

    /// Configuration for project operations (10 minutes — project ops are slow).
    pub fn project_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(2),
            timeout: Duration::from_secs(600),
        }
    }

    /// Configuration for GKE container operations (10 minutes — cluster ops can be slow).
    pub fn container_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(2),
            timeout: Duration::from_secs(600),
        }
    }

    /// Configuration for GKE Backup operations (10 minutes — plan creation can be slow).
    pub fn gke_backup_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(2),
            timeout: Duration::from_secs(600),
        }
    }

    /// Configuration for Cloud SQL operations (15 minutes — instance creation is slow).
    pub fn sql_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(2),
            timeout: Duration::from_secs(900),
        }
    }

    /// Configuration for network operations (3 minutes — address/router ops).
    pub fn network_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(1),
            timeout: Duration::from_secs(180),
        }
    }

    /// Configuration for instance operations (5 minutes — machine type changes, etc.).
    pub fn instance_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(1),
            timeout: Duration::from_secs(300),
        }
    }

    /// Get initial polling interval.
    pub fn initial_interval(&self) -> Duration {
        self.initial_interval
    }

    /// Get timeout.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}
