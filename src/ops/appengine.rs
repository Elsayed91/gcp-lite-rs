//! Operation contracts for the App Engine Admin API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/appengine.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::appengine::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the App Engine Admin API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::appengine::AppengineClient`] instead.
pub struct AppengineOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> AppengineOps<'a> {
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
        "https://appengine.googleapis.com"
    }

    /// Gets information about an application.
    ///
    /// **GCP API**: `GET v1/apps/{appsId}`
    ///
    /// # Path Parameters
    /// - `appsId` — Part of `name`. Required. Name of the Application resource to get. Example: apps/myapp. *(required)*
    ///
    /// # Query Parameters
    /// - `includeExtraData` — Options to include extra data
    ///
    /// # Response
    /// [`Application`]
    #[allow(dead_code)]
    pub(crate) async fn get_app(&self, apps_id: &str) -> Result<Application> {
        let url = format!("{}/v1/apps/{}", self.base_url(), encode(apps_id),);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_app response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists all the services in the application.
    ///
    /// **GCP API**: `GET v1/apps/{appsId}/services`
    ///
    /// # Path Parameters
    /// - `appsId` — Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Maximum results to return per page.
    /// - `pageToken` — Continuation token for fetching the next page of results.
    ///
    /// # Response
    /// [`ListServicesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_services(
        &self,
        apps_id: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListServicesResponse> {
        let url = format!("{}/v1/apps/{}/services", self.base_url(), encode(apps_id),);
        let url =
            crate::append_query_params(url, &[("pageSize", page_size), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_services response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the current configuration of the specified service.
    ///
    /// **GCP API**: `GET v1/apps/{appsId}/services/{servicesId}`
    ///
    /// # Path Parameters
    /// - `appsId` — Part of `name`. Required. Name of the resource requested. Example: apps/myapp/services/default. *(required)*
    /// - `servicesId` — Part of `name`. See documentation of `appsId`. *(required)*
    ///
    /// # Response
    /// [`Service`]
    #[allow(dead_code)]
    pub(crate) async fn get_service(&self, apps_id: &str, services_id: &str) -> Result<Service> {
        let url = format!(
            "{}/v1/apps/{}/services/{}",
            self.base_url(),
            encode(apps_id),
            encode(services_id),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_service response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_app() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/apps/test-appsId")
            .returning_json(serde_json::to_value(Application::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AppengineOps::new(&client);

        let result = ops.get_app("test-appsId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_services() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/v1/apps/test-appsId/services?pageSize=test-pageSize&pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(ListServicesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AppengineOps::new(&client);

        let result = ops
            .list_services("test-appsId", "test-pageSize", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_service() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/apps/test-appsId/services/test-servicesId")
            .returning_json(serde_json::to_value(Service::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AppengineOps::new(&client);

        let result = ops.get_service("test-appsId", "test-servicesId").await;
        assert!(result.is_ok());
    }
}
