//! Operation contracts for the Cloud Asset API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudasset.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudasset::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Cloud Asset API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudasset::CloudassetClient`] instead.
pub struct CloudassetOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> CloudassetOps<'a> {
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
        "https://cloudasset.googleapis.com"
    }

    /// Searches all Google Cloud resources within the specified scope, such as a project,
    /// folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllResources` permission on the desired scope, otherwise the
    /// request will be rejected.
    ///
    /// **GCP API**: `GET v1/{+scope}:searchAllResources`
    /// **Reference**: <https://cloud.google.com/asset-inventory/docs/v1/searchAllResources>
    ///
    /// # Path Parameters
    /// - `scope` — Required. A scope can be a project, a folder, or an organization. The search is limited to the resources within the `sco *(required)*
    ///
    /// # Query Parameters
    /// - `assetTypes` — Optional. A list of asset types that this request searches for. If empty, it will search all the asset types [supported
    /// - `orderBy` — Optional. A comma-separated list of fields specifying the sorting order of the results. The default order is ascending.
    /// - `pageSize` — Optional. The page size for search result pagination. Page size is capped at 500 even if a larger value is given. If set
    /// - `pageToken` — Optional. If present, then retrieve the next batch of results from the preceding call to this method. `page_token` must
    /// - `query` — Optional. The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-re
    /// - `readMask` — Optional. A comma-separated list of fields that you want returned in the results. The following fields are returned by d
    ///
    /// # Response
    /// [`SearchAllResourcesResponse`]
    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub(crate) async fn search_all_resources(
        &self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
        read_mask: &str,
    ) -> Result<SearchAllResourcesResponse> {
        let url = format!("{}/v1/{}:searchAllResources", self.base_url(), scope,);
        let mut __qp: Vec<String> = Vec::new();
        if !query.is_empty() {
            __qp.push(format!("query={}", encode(query)));
        }
        for __v in asset_types {
            __qp.push(format!("assetTypes={}", encode(__v)));
        }
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", encode(page_size)));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", encode(page_token)));
        }
        if !order_by.is_empty() {
            __qp.push(format!("orderBy={}", encode(order_by)));
        }
        if !read_mask.is_empty() {
            __qp.push(format!("readMask={}", encode(read_mask)));
        }
        let url = if __qp.is_empty() {
            url
        } else {
            format!("{}?{}", url, __qp.join("&"))
        };
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse search_all_resources response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Searches all IAM policies within the specified scope, such as a project, folder, or
    /// organization. The caller must be granted the `cloudasset.assets.searchAllIamPolicies`
    /// permission on the desired scope, otherwise the request will be rejected.
    ///
    /// **GCP API**: `GET v1/{+scope}:searchAllIamPolicies`
    /// **Reference**: <https://cloud.google.com/asset-inventory/docs/v1/searchAllIamPolicies>
    ///
    /// # Path Parameters
    /// - `scope` — Required. A scope can be a project, a folder, or an organization. The search is limited to the IAM policies within the ` *(required)*
    ///
    /// # Query Parameters
    /// - `assetTypes` — Optional. A list of asset types that the IAM policies are attached to. If empty, it will search the IAM policies that ar
    /// - `orderBy` — Optional. A comma-separated list of fields specifying the sorting order of the results. The default order is ascending.
    /// - `pageSize` — Optional. The page size for search result pagination. Page size is capped at 500 even if a larger value is given. If set
    /// - `pageToken` — Optional. If present, retrieve the next batch of results from the preceding call to this method. `page_token` must be th
    /// - `query` — Optional. The query statement. See [how to construct a query](https://cloud.google.com/asset-inventory/docs/searching-ia
    ///
    /// # Response
    /// [`SearchAllIamPoliciesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn search_all_iam_policies(
        &self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> Result<SearchAllIamPoliciesResponse> {
        let url = format!("{}/v1/{}:searchAllIamPolicies", self.base_url(), scope,);
        let mut __qp: Vec<String> = Vec::new();
        if !query.is_empty() {
            __qp.push(format!("query={}", encode(query)));
        }
        for __v in asset_types {
            __qp.push(format!("assetTypes={}", encode(__v)));
        }
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", encode(page_size)));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", encode(page_token)));
        }
        if !order_by.is_empty() {
            __qp.push(format!("orderBy={}", encode(order_by)));
        }
        let url = if __qp.is_empty() {
            url
        } else {
            format!("{}?{}", url, __qp.join("&"))
        };
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse search_all_iam_policies response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_all_resources() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-scope:searchAllResources?query=test-query&assetTypes=test-assetTypes&pageSize=test-pageSize&pageToken=test-pageToken&orderBy=test-orderBy&readMask=test-readMask")
            .returning_json(serde_json::to_value(SearchAllResourcesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudassetOps::new(&client);

        let result = ops
            .search_all_resources(
                "test-scope",
                "test-query",
                &["test-assetTypes"],
                "test-pageSize",
                "test-pageToken",
                "test-orderBy",
                "test-readMask",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_all_iam_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-scope:searchAllIamPolicies?query=test-query&assetTypes=test-assetTypes&pageSize=test-pageSize&pageToken=test-pageToken&orderBy=test-orderBy")
            .returning_json(serde_json::to_value(SearchAllIamPoliciesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudassetOps::new(&client);

        let result = ops
            .search_all_iam_policies(
                "test-scope",
                "test-query",
                &["test-assetTypes"],
                "test-pageSize",
                "test-pageToken",
                "test-orderBy",
            )
            .await;
        assert!(result.is_ok());
    }
}
