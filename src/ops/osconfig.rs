//! Operation contracts for the OS Config API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/osconfig.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::osconfig::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the OS Config API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::osconfig::OsconfigClient`] instead.
pub struct OsconfigOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> OsconfigOps<'a> {
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
        "https://osconfig.googleapis.com"
    }

    /// Get a page of OS Config patch deployments.
    ///
    /// **GCP API**: `GET v1/{+parent}/patchDeployments`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the parent in the form `projects/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. The maximum number of patch deployments to return. Default is 100.
    /// - `pageToken` — Optional. A pagination token returned from a previous call to ListPatchDeployments that indicates where this listing sho
    ///
    /// # Response
    /// [`ListPatchDeploymentsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_patch_deployments(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListPatchDeploymentsResponse> {
        let url = format!("{}/v1/{}/patchDeployments", self.base_url(), parent,);
        let url =
            crate::append_query_params(url, &[("pageSize", page_size), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_patch_deployments response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// List inventory data for all VM instances in the specified zone.
    ///
    /// **GCP API**: `GET v1/{+parent}/inventories`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The parent resource name. Format: `projects/{project}/locations/{location}/instances/-` For `{project}`, eithe *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — If provided, this field specifies the criteria that must be met by a `Inventory` API resource to be included in the resp
    /// - `pageSize` — The maximum number of results to return.
    /// - `pageToken` — A pagination token returned from a previous call to `ListInventories` that indicates where this listing should continue
    /// - `view` — Inventory view indicating what information should be included in the inventory resource. If unspecified, the default vie
    ///
    /// # Response
    /// [`ListInventoriesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_inventories(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        view: &str,
        filter: &str,
    ) -> Result<ListInventoriesResponse> {
        let url = format!("{}/v1/{}/inventories", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("view", view),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_inventories response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_patch_deployments() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/v1/test-parent/patchDeployments?pageSize=test-pageSize&pageToken=test-pageToken",
        )
        .returning_json(serde_json::to_value(ListPatchDeploymentsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = OsconfigOps::new(&client);

        let result = ops
            .list_patch_deployments("test-parent", "test-pageSize", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_inventories() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/inventories?pageSize=test-pageSize&pageToken=test-pageToken&view=test-view&filter=test-filter")
            .returning_json(serde_json::to_value(ListInventoriesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = OsconfigOps::new(&client);

        let result = ops
            .list_inventories(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-view",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }
}
