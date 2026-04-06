//! Service Usage API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::service_usage::ServiceUsageOps`. This layer adds:
//! - Ergonomic method signatures (project/service instead of raw resource names)
//! - Blocking variants that poll operations to completion

use crate::{
    GcpHttpClient, Result,
    operation::{PollConfig, ServiceUsageOperation},
    ops::service_usage::ServiceUsageOps,
    types::service_usage::{
        BatchEnableServicesRequest, DisableServiceRequest, EnableServiceRequest,
        ListServicesResponse, ServiceState, ServiceStateEnum, ServiceUsageLro,
    },
};

/// Client for the Service Usage API
pub struct ServiceUsageClient<'a> {
    ops: ServiceUsageOps<'a>,
}

impl<'a> ServiceUsageClient<'a> {
    /// Create a new Service Usage API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: ServiceUsageOps::new(client),
        }
    }

    // ── Query ────────────────────────────────────────────────────────

    /// Get a service's state for a project.
    pub async fn get_service(&self, project: &str, service: &str) -> Result<ServiceState> {
        let name = format!("projects/{}/services/{}", project, service);
        self.ops.get_service(&name).await
    }

    /// Check if a service is enabled for a project.
    pub async fn is_service_enabled(&self, project: &str, service: &str) -> Result<bool> {
        let svc = self.get_service(project, service).await?;
        Ok(svc.state == Some(ServiceStateEnum::Enabled))
    }

    /// List all services available to a project.
    pub async fn list_services(&self, project: &str) -> Result<ListServicesResponse> {
        let parent = format!("projects/{}", project);
        self.ops.list_services(&parent, "", "", "").await
    }

    /// List services with a filter (e.g. `"state:ENABLED"`).
    pub async fn list_services_with_filter(
        &self,
        project: &str,
        filter: &str,
    ) -> Result<ListServicesResponse> {
        let parent = format!("projects/{}", project);
        self.ops.list_services(&parent, "", filter, "").await
    }

    /// Stream all services for a project, automatically handling pagination.
    pub fn list_services_stream(
        &self,
        project: &str,
    ) -> impl futures::Stream<Item = Result<ServiceState>> + '_ {
        let parent = format!("projects/{}", project);
        async_stream::try_stream! {
            let mut page_token: Option<String> = None;
            loop {
                let response = self.ops
                    .list_services(&parent, page_token.as_deref().unwrap_or(""), "", "")
                    .await?;
                for item in response.services { yield item; }
                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    // ── Enable ───────────────────────────────────────────────────────

    /// Enable a service (blocks until complete).
    pub async fn enable_service(&self, project: &str, service: &str) -> Result<()> {
        let op = self.enable_service_start(project, service).await?;
        op.wait().await
    }

    /// Enable a service (returns operation for manual polling).
    pub async fn enable_service_start(
        &self,
        project: &str,
        service: &str,
    ) -> Result<ServiceUsageOperation<'a>> {
        let name = format!("projects/{}/services/{}", project, service);
        let lro = self
            .ops
            .enable_service(&name, &EnableServiceRequest {})
            .await?;
        self.lro_operation(lro)
    }

    // ── Disable ──────────────────────────────────────────────────────

    /// Disable a service (blocks until complete).
    pub async fn disable_service(&self, project: &str, service: &str) -> Result<()> {
        let op = self.disable_service_start(project, service).await?;
        op.wait().await
    }

    /// Disable a service (returns operation for manual polling).
    pub async fn disable_service_start(
        &self,
        project: &str,
        service: &str,
    ) -> Result<ServiceUsageOperation<'a>> {
        let name = format!("projects/{}/services/{}", project, service);
        let lro = self
            .ops
            .disable_service(&name, &DisableServiceRequest::default())
            .await?;
        self.lro_operation(lro)
    }

    /// Disable a service with custom options (blocks until complete).
    pub async fn disable_service_with_request(
        &self,
        project: &str,
        service: &str,
        request: &DisableServiceRequest,
    ) -> Result<()> {
        let op = self
            .disable_service_with_request_start(project, service, request)
            .await?;
        op.wait().await
    }

    /// Disable a service with custom options (returns operation for manual polling).
    pub async fn disable_service_with_request_start(
        &self,
        project: &str,
        service: &str,
        request: &DisableServiceRequest,
    ) -> Result<ServiceUsageOperation<'a>> {
        let name = format!("projects/{}/services/{}", project, service);
        let lro = self.ops.disable_service(&name, request).await?;
        self.lro_operation(lro)
    }

    // ── Batch ────────────────────────────────────────────────────────

    /// Batch-enable multiple services (blocks until complete).
    pub async fn batch_enable_services(
        &self,
        project: &str,
        service_ids: Vec<String>,
    ) -> Result<()> {
        let op = self
            .batch_enable_services_start(project, service_ids)
            .await?;
        op.wait().await
    }

    /// Batch-enable multiple services (returns operation for manual polling).
    pub async fn batch_enable_services_start(
        &self,
        project: &str,
        service_ids: Vec<String>,
    ) -> Result<ServiceUsageOperation<'a>> {
        let parent = format!("projects/{}", project);
        let body = BatchEnableServicesRequest { service_ids };
        let lro = self.ops.batch_enable_services(&parent, &body).await?;
        self.lro_operation(lro)
    }

    // ── Helpers ──────────────────────────────────────────────────────

    fn lro_operation(&self, lro: ServiceUsageLro) -> Result<ServiceUsageOperation<'a>> {
        // If the LRO is already done (e.g., enabling an already-enabled service),
        // check for errors and short-circuit — the operation name may be a
        // placeholder (like "DONE_OPERATION") that GCP rejects on poll.
        if lro.done
            && let Some(error) = &lro.error
        {
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
            return Err(crate::GcpError::OperationFailed {
                operation: lro.name,
                message,
                code,
            });
        }
        let config = PollConfig::service_usage_operation();
        Ok(ServiceUsageOperation::new(
            self.ops.client,
            lro.name,
            config.initial_interval(),
            config.timeout(),
            lro.done,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_is_service_enabled_returns_true_when_enabled() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/services/compute.googleapis.com")
            .returning_json(serde_json::json!({
                "name": "projects/123/services/compute.googleapis.com",
                "state": "ENABLED"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let service_usage = client.service_usage();

        let result = service_usage
            .is_service_enabled("test-project", "compute.googleapis.com")
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_service_enabled_returns_false_when_disabled() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/services/storage.googleapis.com")
            .returning_json(serde_json::json!({
                "name": "projects/123/services/storage.googleapis.com",
                "state": "DISABLED"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let service_usage = client.service_usage();

        let result = service_usage
            .is_service_enabled("test-project", "storage.googleapis.com")
            .await;

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_enable_service_blocks_until_complete() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/my-project/services/compute.googleapis.com:enable")
            .returning_json(serde_json::json!({
                "name": "operations/acf.123456",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v1/operations/acf.123456")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "name": "operations/acf.123456",
                    "done": false
                }),
                serde_json::json!({
                    "name": "operations/acf.123456",
                    "done": true
                }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .enable_service("my-project", "compute.googleapis.com")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_service_start_returns_operation() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-proj/services/storage.googleapis.com:enable")
            .returning_json(serde_json::json!({
                "name": "operations/acf.789",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v1/operations/acf.789")
            .returning_json(serde_json::json!({
                "name": "operations/acf.789",
                "done": true
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);

        let op = client
            .service_usage()
            .enable_service_start("test-proj", "storage.googleapis.com")
            .await;

        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    #[tokio::test]
    async fn test_list_services_stream_paginates() {
        use futures::StreamExt;

        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/my-project/services?pageToken=page2")
            .returning_json(serde_json::json!({
                "services": [
                    {"name": "projects/123/services/bigquery.googleapis.com", "state": "ENABLED"}
                ]
            }));

        mock.expect_get("/v1/projects/my-project/services")
            .returning_json(serde_json::json!({
                "services": [
                    {"name": "projects/123/services/compute.googleapis.com", "state": "ENABLED"},
                    {"name": "projects/123/services/storage.googleapis.com", "state": "DISABLED"}
                ],
                "nextPageToken": "page2"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let su = client.service_usage();
        let stream = su.list_services_stream("my-project");
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(svc)) = stream.next().await {
            names.push(svc.name.clone());
        }
        assert_eq!(names.len(), 3);
        assert!(names[0].contains("compute"));
        assert!(names[2].contains("bigquery"));
    }

    #[tokio::test]
    async fn test_disable_service_blocks_until_complete() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/my-project/services/storage.googleapis.com:disable")
            .returning_json(serde_json::json!({
                "name": "operations/acf.disable123",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v1/operations/acf.disable123")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "name": "operations/acf.disable123",
                    "done": false
                }),
                serde_json::json!({
                    "name": "operations/acf.disable123",
                    "done": true
                }),
            ])
            .times(2);

        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .disable_service("my-project", "storage.googleapis.com")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disable_service_start_returns_operation() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-proj/services/compute.googleapis.com:disable")
            .returning_json(serde_json::json!({
                "name": "operations/acf.disable456",
                "done": false
            }))
            .times(1);

        mock.expect_get("/v1/operations/acf.disable456")
            .returning_json(serde_json::json!({
                "name": "operations/acf.disable456",
                "done": true
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);

        let op = client
            .service_usage()
            .disable_service_start("test-proj", "compute.googleapis.com")
            .await;

        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_enable_already_enabled_service_skips_polling() {
        let mut mock = crate::MockClient::new();

        // GCP returns done:true with placeholder name for already-enabled service
        mock.expect_post("/v1/projects/my-project/services/compute.googleapis.com:enable")
            .returning_json(serde_json::json!({
                "name": "DONE_OPERATION",
                "done": true
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .enable_service("my-project", "compute.googleapis.com")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_service_start_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/my-project/services/storage.googleapis.com:enable")
            .returning_json(serde_json::json!({
                "name": "DONE_OPERATION",
                "done": true
            }))
            .times(1);

        // NO expect_get — short-circuit must prevent polling
        let client = crate::GcpHttpClient::from_mock(mock);

        let op = client
            .service_usage()
            .enable_service_start("my-project", "storage.googleapis.com")
            .await;
        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    #[tokio::test]
    async fn test_initially_done_lro_with_error_returns_error() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/my-project/services/bad.googleapis.com:enable")
            .returning_json(serde_json::json!({
                "name": "DONE_OPERATION",
                "done": true,
                "error": {
                    "code": 403,
                    "message": "Permission denied"
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .enable_service("my-project", "bad.googleapis.com")
            .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::GcpError::OperationFailed { .. }),
            "Expected OperationFailed, got: {:?}",
            err
        );
    }

    #[tokio::test]
    async fn test_disable_already_disabled_service_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/my-project/services/storage.googleapis.com:disable")
            .returning_json(serde_json::json!({
                "name": "DONE_OPERATION",
                "done": true
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let result = client
            .service_usage()
            .disable_service("my-project", "storage.googleapis.com")
            .await;

        assert!(result.is_ok());
    }
}
