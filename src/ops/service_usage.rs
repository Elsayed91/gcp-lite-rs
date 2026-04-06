//! Operation contracts for the Service Usage API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/service_usage.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::service_usage::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Service Usage API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::service_usage::ServiceUsageClient`] instead.
pub struct ServiceUsageOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> ServiceUsageOps<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://serviceusage.googleapis.com"
    }

    /// Returns the service configuration and enabled state for a given service.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Name of the consumer and service to get the `ConsumerState` for. An example name would be: `projects/123/services/servic *(required)*
    ///
    /// # Response
    /// [`ServiceState`]
    #[allow(dead_code)]
    pub(crate) async fn get_service(&self, name: &str) -> Result<ServiceState> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Enable a service so that it can be used with a project.
    ///
    /// **GCP API**: `POST v1/{+name}:enable`
    ///
    /// # Path Parameters
    /// - `name` — Name of the consumer and service to enable the service on. The `EnableService` and `DisableService` methods currently on *(required)*
    ///
    /// # Request Body
    /// [`EnableServiceRequest`]
    ///
    /// # Response
    /// [`ServiceUsageLro`]
    #[allow(dead_code)]
    pub(crate) async fn enable_service(
        &self,
        name: &str,
        body: &EnableServiceRequest,
    ) -> Result<ServiceUsageLro> {
        let url = format!("{}/v1/{}:enable", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse enable_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Disable a service so that it can no longer be used with a project. This prevents
    /// unintended usage that may cause unexpected billing charges or security leaks. It is not
    /// valid to call the disable method on a service that is not currently enabled. Callers
    /// will receive a `FAILED_PRECONDITION` status if the target service is not currently
    /// enabled.
    ///
    /// **GCP API**: `POST v1/{+name}:disable`
    ///
    /// # Path Parameters
    /// - `name` — Name of the consumer and service to disable the service on. The enable and disable methods currently only support projec *(required)*
    ///
    /// # Request Body
    /// [`DisableServiceRequest`]
    ///
    /// # Response
    /// [`ServiceUsageLro`]
    #[allow(dead_code)]
    pub(crate) async fn disable_service(
        &self,
        name: &str,
        body: &DisableServiceRequest,
    ) -> Result<ServiceUsageLro> {
        let url = format!("{}/v1/{}:disable", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse disable_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// List all services available to the specified project, and the current state of those
    /// services with respect to the project. The list includes all public services, all
    /// services for which the calling user has the `servicemanagement.services.bind`
    /// permission, and all services that have already been enabled on the project. The list can
    /// be filtered to only include services in a specific state, for example to only include
    /// services enabled on the project. WARNING: If you need to query enabled services
    /// frequently or across an organization, you should use [Cloud Asset Inventory
    /// API](https://cloud.google.com/asset-inventory/docs/apis), which provides higher
    /// throughput and richer filtering capability.
    ///
    /// **GCP API**: `GET v1/{+parent}/services`
    ///
    /// # Path Parameters
    /// - `parent` — Parent to search for services on. An example name would be: `projects/123` where `123` is the project number. *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Only list services that conform to the given filter. The allowed filter strings are `state:ENABLED` and `state:DISABLED`
    /// - `pageSize` — Requested size of the next page of data. Requested page size cannot exceed 200. If not set, the default page size is 50.
    /// - `pageToken` — Token identifying which result to start with, which is returned by a previous list call.
    ///
    /// # Response
    /// [`ListServicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_services(
        &self,
        parent: &str,
        page_token: &str,
        filter: &str,
        page_size: &str,
    ) -> Result<ListServicesResponse> {
        let url = format!("{}/v1/{}/services", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageToken", page_token),
                ("filter", filter),
                ("pageSize", page_size),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Enable multiple services on a project. The operation is atomic: if enabling any service
    /// fails, then the entire batch fails, and no state changes occur. To enable a single
    /// service, use the `EnableService` method instead.
    ///
    /// **GCP API**: `POST v1/{+parent}/services:batchEnable`
    ///
    /// # Path Parameters
    /// - `parent` — Parent to enable services on. An example name would be: `projects/123` where `123` is the project number. The `BatchEnab *(required)*
    ///
    /// # Request Body
    /// [`BatchEnableServicesRequest`]
    ///
    /// # Response
    /// [`ServiceUsageLro`]
    #[allow(dead_code)]
    pub(crate) async fn batch_enable_services(
        &self,
        parent: &str,
        body: &BatchEnableServicesRequest,
    ) -> Result<ServiceUsageLro> {
        let url = format!("{}/v1/{}/services:batchEnable", self.base_url(), parent,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse batch_enable_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(ServiceState::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ServiceUsageOps::new(&client);

        let result = ops.get_service("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enable_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name:enable")
            .returning_json(serde_json::to_value(ServiceUsageLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ServiceUsageOps::new(&client);

        let body = EnableServiceRequest::fixture();
        let result = ops.enable_service("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disable_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name:disable")
            .returning_json(serde_json::to_value(ServiceUsageLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ServiceUsageOps::new(&client);

        let body = DisableServiceRequest::fixture();
        let result = ops.disable_service("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/services?pageToken=test-pageToken&filter=test-filter&pageSize=test-pageSize")
            .returning_json(serde_json::to_value(ListServicesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ServiceUsageOps::new(&client);

        let result = ops
            .list_services(
                "test-parent",
                "test-pageToken",
                "test-filter",
                "test-pageSize",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_enable_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-parent/services:batchEnable")
            .returning_json(serde_json::to_value(ServiceUsageLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ServiceUsageOps::new(&client);

        let body = BatchEnableServicesRequest::fixture();
        let result = ops.batch_enable_services("test-parent", &body).await;
        assert!(result.is_ok());
    }
}
