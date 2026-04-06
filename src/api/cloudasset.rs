//! Cloud Asset API client.
//!
//! Provides asset inventory, search, and IAM policy operations for Google Cloud resources.
//! This module wraps generated ops with ergonomic methods and pagination support.
//!
//! # Pagination
//!
//! `list_assets`, `search_all_resources`, and `search_all_iam_policies` return paginated results.
//! This client provides three patterns for each operation:
//!
//! 1. **Single page** - `list_assets()` / `search_all_resources()` / `search_all_iam_policies()` - Returns one page
//! 2. **Stream** - `*_stream()` - Async stream of items
//! 3. **Collect all** - `*_all()` - Collects all pages into Vec

use crate::{
    GcpHttpClient, Result,
    ops::cloudasset::CloudassetOps,
    types::cloudasset::{
        Asset, IamPolicySearchResult, ListAssetsResponse, ResourceSearchResult,
        SearchAllIamPoliciesResponse, SearchAllResourcesResponse,
    },
};

/// Options for listing assets.
#[derive(Debug, Clone, Default)]
pub struct ListAssetsOptions<'a> {
    /// Maximum number of assets to return per page (1-1000, default 100).
    pub page_size: Option<u32>,
    /// Content type to include (e.g., "RESOURCE", "IAM_POLICY").
    pub content_type: Option<&'a str>,
    /// Filter by asset types (e.g., "compute.googleapis.com/Disk").
    pub asset_types: Option<&'a [&'a str]>,
    /// Snapshot timestamp (RFC 3339 format).
    pub read_time: Option<&'a str>,
}

/// Options for searching IAM policies.
#[derive(Debug, Clone, Default)]
pub struct SearchIamPoliciesOptions<'a> {
    /// The search query (see Cloud Asset docs for IAM policy query syntax).
    pub query: Option<&'a str>,
    /// Filter by asset types.
    pub asset_types: Option<&'a [&'a str]>,
    /// Maximum results per page (1-500, default 500).
    pub page_size: Option<u32>,
    /// Sort order (e.g., "resource desc").
    pub order_by: Option<&'a str>,
}

/// Options for searching resources.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions<'a> {
    /// The search query (see Cloud Asset docs for query syntax).
    pub query: Option<&'a str>,
    /// Filter by asset types.
    pub asset_types: Option<&'a [&'a str]>,
    /// Maximum results per page (1-500, default 500).
    pub page_size: Option<u32>,
    /// Sort order (e.g., "createTime desc").
    pub order_by: Option<&'a str>,
    /// Fields to return (comma-separated).
    pub read_mask: Option<&'a str>,
}

/// Client for the Cloud Asset API.
pub struct CloudAssetClient<'a> {
    ops: CloudassetOps<'a>,
}

impl<'a> CloudAssetClient<'a> {
    /// Create a new Cloud Asset API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: CloudassetOps::new(client),
        }
    }

    // ── Assets ─────────────────────────────────────────────────────────────────

    /// Lists assets for a parent scope (project, folder, or organization).
    ///
    /// Returns a single page of results. Use `list_assets_stream` or
    /// `list_assets_all` for automatic pagination.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Optional filters and pagination settings
    /// * `page_token` - Token from previous response for pagination
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = gcp_lite::GcpHttpClient::from_adc().await?;
    /// use gcp_lite::api::cloudasset::ListAssetsOptions;
    ///
    /// let options = ListAssetsOptions {
    ///     page_size: Some(100),
    ///     content_type: Some("RESOURCE"),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.cloud_asset()
    ///     .list_assets("projects/my-project", &options, None)
    ///     .await?;
    ///
    /// for asset in response.assets {
    ///     println!("Asset: {:?}", asset.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_assets(
        &self,
        parent: &str,
        options: &ListAssetsOptions<'_>,
        page_token: Option<&str>,
    ) -> Result<ListAssetsResponse> {
        self.list_assets_raw(
            parent,
            options.page_size,
            page_token.unwrap_or(""),
            options.content_type.unwrap_or(""),
            options.asset_types.unwrap_or(&[]),
            options.read_time.unwrap_or(""),
        )
        .await
    }

    /// Returns an async stream of assets, automatically handling pagination.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Optional filters and pagination settings
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = gcp_lite::GcpHttpClient::from_adc().await?;
    /// use futures::StreamExt;
    /// use std::pin::pin;
    /// use gcp_lite::api::cloudasset::ListAssetsOptions;
    ///
    /// let options = ListAssetsOptions::default();
    /// let cloud_asset = client.cloud_asset();
    /// let mut stream = pin!(cloud_asset.list_assets_stream("projects/my-project", options));
    ///
    /// while let Some(result) = stream.next().await {
    ///     let asset = result?;
    ///     println!("Asset: {:?}", asset.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_assets_stream(
        &self,
        parent: &str,
        options: ListAssetsOptions<'_>,
    ) -> impl futures::Stream<Item = Result<Asset>> + '_ {
        let parent = parent.to_string();
        let page_size = options.page_size;
        let content_type = options.content_type.unwrap_or("").to_string();
        let asset_types: Vec<String> = options
            .asset_types
            .map(|types| types.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let read_time = options.read_time.unwrap_or("").to_string();

        async_stream::try_stream! {
            let mut page_token: Option<String> = None;

            loop {
                let at_refs: Vec<&str> = asset_types.iter().map(|s| s.as_str()).collect();
                let response = self
                    .list_assets_raw(
                        &parent,
                        page_size,
                        page_token.as_deref().unwrap_or(""),
                        &content_type,
                        &at_refs,
                        &read_time,
                    )
                    .await?;

                for asset in response.assets {
                    yield asset;
                }

                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Collects all assets across all pages into a Vec.
    ///
    /// # Warning
    ///
    /// This can return a very large number of results. Consider using
    /// `list_assets_stream` for memory-efficient processing, or set
    /// appropriate filters in `options`.
    ///
    /// # Arguments
    ///
    /// * `parent` - Parent scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Optional filters and pagination settings
    pub async fn list_assets_all(
        &self,
        parent: &str,
        options: &ListAssetsOptions<'_>,
    ) -> Result<Vec<Asset>> {
        use futures::StreamExt;

        let stream = self.list_assets_stream(parent, options.clone());
        futures::pin_mut!(stream);

        let mut assets = Vec::new();
        while let Some(result) = stream.next().await {
            assets.push(result?);
        }
        Ok(assets)
    }

    // ── Search ─────────────────────────────────────────────────────────────────

    /// Searches all resources within a scope.
    ///
    /// Returns a single page of results. Use `search_all_resources_stream` or
    /// `search_all_resources_all` for automatic pagination.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    /// * `page_token` - Token from previous response for pagination
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = gcp_lite::GcpHttpClient::from_adc().await?;
    /// use gcp_lite::api::cloudasset::SearchOptions;
    ///
    /// let options = SearchOptions {
    ///     query: Some("name:my-instance"),
    ///     page_size: Some(100),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.cloud_asset()
    ///     .search_all_resources("projects/my-project", &options, None)
    ///     .await?;
    ///
    /// for result in response.results {
    ///     println!("Found: {} ({})", result.name.as_deref().unwrap_or(""), result.asset_type.as_deref().unwrap_or(""));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_all_resources(
        &self,
        scope: &str,
        options: &SearchOptions<'_>,
        page_token: Option<&str>,
    ) -> Result<SearchAllResourcesResponse> {
        let page_size_str = options
            .page_size
            .map(|ps| ps.to_string())
            .unwrap_or_default();
        self.ops
            .search_all_resources(
                scope,
                options.query.unwrap_or(""),
                options.asset_types.unwrap_or(&[]),
                &page_size_str,
                page_token.unwrap_or(""),
                options.order_by.unwrap_or(""),
                options.read_mask.unwrap_or(""),
            )
            .await
    }

    /// Returns an async stream of search results, automatically handling pagination.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = gcp_lite::GcpHttpClient::from_adc().await?;
    /// use futures::StreamExt;
    /// use std::pin::pin;
    /// use gcp_lite::api::cloudasset::SearchOptions;
    ///
    /// let options = SearchOptions {
    ///     query: Some("location:us-central1"),
    ///     ..Default::default()
    /// };
    ///
    /// let cloud_asset = client.cloud_asset();
    /// let mut stream = pin!(cloud_asset.search_all_resources_stream("projects/my-project", options));
    ///
    /// while let Some(result) = stream.next().await {
    ///     let resource = result?;
    ///     println!("Resource: {:?}", resource.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_all_resources_stream(
        &self,
        scope: &str,
        options: SearchOptions<'_>,
    ) -> impl futures::Stream<Item = Result<ResourceSearchResult>> + '_ {
        let scope = scope.to_string();
        let query = options.query.unwrap_or("").to_string();
        let page_size = options.page_size;
        let asset_types: Vec<String> = options
            .asset_types
            .map(|types| types.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let order_by = options.order_by.unwrap_or("").to_string();
        let read_mask = options.read_mask.unwrap_or("").to_string();

        let page_size_str = page_size.map(|ps| ps.to_string()).unwrap_or_default();

        async_stream::try_stream! {
            let mut page_token: Option<String> = None;

            loop {
                let at_refs: Vec<&str> = asset_types.iter().map(|s| s.as_str()).collect();
                let response = self
                    .ops
                    .search_all_resources(
                        &scope,
                        &query,
                        &at_refs,
                        &page_size_str,
                        page_token.as_deref().unwrap_or(""),
                        &order_by,
                        &read_mask,
                    )
                    .await?;

                for result in response.results {
                    yield result;
                }

                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Collects all search results across all pages into a Vec.
    ///
    /// # Warning
    ///
    /// This can return a very large number of results. Consider using
    /// `search_all_resources_stream` for memory-efficient processing, or
    /// use filters in `options` to limit results.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    pub async fn search_all_resources_all(
        &self,
        scope: &str,
        options: &SearchOptions<'_>,
    ) -> Result<Vec<ResourceSearchResult>> {
        use futures::StreamExt;

        let stream = self.search_all_resources_stream(scope, options.clone());
        futures::pin_mut!(stream);

        let mut results = Vec::new();
        while let Some(result) = stream.next().await {
            results.push(result?);
        }
        Ok(results)
    }

    // ── IAM Policy Search ─────────────────────────────────────────────────────

    /// Searches all IAM policies within a scope.
    ///
    /// Returns a single page of results. Use `search_all_iam_policies_stream` or
    /// `search_all_iam_policies_all` for automatic pagination.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    /// * `page_token` - Token from previous response for pagination
    pub async fn search_all_iam_policies(
        &self,
        scope: &str,
        options: &SearchIamPoliciesOptions<'_>,
        page_token: Option<&str>,
    ) -> Result<SearchAllIamPoliciesResponse> {
        let page_size_str = options
            .page_size
            .map(|ps| ps.to_string())
            .unwrap_or_default();
        self.ops
            .search_all_iam_policies(
                scope,
                options.query.unwrap_or(""),
                options.asset_types.unwrap_or(&[]),
                &page_size_str,
                page_token.unwrap_or(""),
                options.order_by.unwrap_or(""),
            )
            .await
    }

    /// Returns an async stream of IAM policy search results, automatically handling pagination.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    pub fn search_all_iam_policies_stream(
        &self,
        scope: &str,
        options: SearchIamPoliciesOptions<'_>,
    ) -> impl futures::Stream<Item = Result<IamPolicySearchResult>> + '_ {
        let scope = scope.to_string();
        let query = options.query.unwrap_or("").to_string();
        let page_size = options.page_size;
        let asset_types: Vec<String> = options
            .asset_types
            .map(|types| types.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let order_by = options.order_by.unwrap_or("").to_string();

        let page_size_str = page_size.map(|ps| ps.to_string()).unwrap_or_default();

        async_stream::try_stream! {
            let mut page_token: Option<String> = None;

            loop {
                let at_refs: Vec<&str> = asset_types.iter().map(|s| s.as_str()).collect();
                let response = self
                    .ops
                    .search_all_iam_policies(
                        &scope,
                        &query,
                        &at_refs,
                        &page_size_str,
                        page_token.as_deref().unwrap_or(""),
                        &order_by,
                    )
                    .await?;

                for result in response.results {
                    yield result;
                }

                match response.next_page_token {
                    Some(token) if !token.is_empty() => page_token = Some(token),
                    _ => break,
                }
            }
        }
    }

    /// Collects all IAM policy search results across all pages into a Vec.
    ///
    /// # Warning
    ///
    /// This can return a very large number of results. Consider using
    /// `search_all_iam_policies_stream` for memory-efficient processing, or
    /// use filters in `options` to limit results.
    ///
    /// # Arguments
    ///
    /// * `scope` - Search scope: "projects/PROJECT", "folders/FOLDER", or "organizations/ORG"
    /// * `options` - Search query and filters
    pub async fn search_all_iam_policies_all(
        &self,
        scope: &str,
        options: &SearchIamPoliciesOptions<'_>,
    ) -> Result<Vec<IamPolicySearchResult>> {
        use futures::StreamExt;

        let stream = self.search_all_iam_policies_stream(scope, options.clone());
        futures::pin_mut!(stream);

        let mut results = Vec::new();
        while let Some(result) = stream.next().await {
            results.push(result?);
        }
        Ok(results)
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.ops.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://cloudasset.googleapis.com"
    }

    /// Make a list_assets request with proper repeated `assetTypes` query params.
    ///
    /// The Cloud Asset `list_assets` endpoint requires `assetTypes` as repeated
    /// query parameters (`assetTypes=A&assetTypes=B`), not comma-separated.
    /// This bypasses the generated ops layer (which only supports single-value
    /// params) to construct the URL correctly.
    async fn list_assets_raw(
        &self,
        parent: &str,
        page_size: Option<u32>,
        page_token: &str,
        content_type: &str,
        asset_types: &[&str],
        read_time: &str,
    ) -> Result<ListAssetsResponse> {
        let url = format!("{}/v1/{}/assets", self.base_url(), parent);

        let mut params: Vec<String> = Vec::new();
        if let Some(ps) = page_size {
            params.push(format!("pageSize={}", ps));
        }
        if !page_token.is_empty() {
            params.push(format!("pageToken={}", urlencoding::encode(page_token)));
        }
        if !content_type.is_empty() {
            params.push(format!("contentType={}", urlencoding::encode(content_type)));
        }
        for t in asset_types {
            params.push(format!("assetTypes={}", urlencoding::encode(t)));
        }
        if !read_time.is_empty() {
            params.push(format!("readTime={}", urlencoding::encode(read_time)));
        }

        let url = if params.is_empty() {
            url
        } else {
            format!("{}?{}", url, params.join("&"))
        };

        let response = self.ops.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_assets response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use serde_json::json;

    #[tokio::test]
    async fn test_list_assets_single_page() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project/assets?pageSize=10&contentType=RESOURCE")
            .returning_json(json!({
                "assets": [
                    {
                        "name": "//compute.googleapis.com/projects/test-project/zones/us-central1-a/instances/vm-1",
                        "assetType": "compute.googleapis.com/Instance"
                    },
                    {
                        "name": "//storage.googleapis.com/projects/_/buckets/my-bucket",
                        "assetType": "storage.googleapis.com/Bucket"
                    }
                ],
                "readTime": "2024-01-15T10:00:00Z"
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = ListAssetsOptions {
            page_size: Some(10),
            content_type: Some("RESOURCE"),
            ..Default::default()
        };

        let response = asset_client
            .list_assets("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.assets.len(), 2);
        assert!(
            response.assets[0]
                .name
                .as_ref()
                .unwrap()
                .contains("instances/vm-1")
        );
        assert_eq!(
            response.assets[0].asset_type.as_deref(),
            Some("compute.googleapis.com/Instance")
        );
    }

    #[tokio::test]
    async fn test_list_assets_with_asset_types_sends_repeated_params() {
        let mut mock = MockClient::new();

        // The URL must have repeated assetTypes params, NOT comma-joined
        mock.expect_get("/v1/projects/test-project/assets?contentType=RESOURCE&assetTypes=compute.googleapis.com%2FDisk&assetTypes=compute.googleapis.com%2FInstance")
            .returning_json(json!({
                "assets": [
                    {"name": "//disk-1", "assetType": "compute.googleapis.com/Disk"},
                    {"name": "//vm-1", "assetType": "compute.googleapis.com/Instance"}
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let asset_types = [
            "compute.googleapis.com/Disk",
            "compute.googleapis.com/Instance",
        ];
        let options = ListAssetsOptions {
            content_type: Some("RESOURCE"),
            asset_types: Some(&asset_types),
            ..Default::default()
        };

        let response = asset_client
            .list_assets("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.assets.len(), 2);
    }

    #[tokio::test]
    async fn test_list_assets_stream_with_asset_types_sends_repeated_params() {
        use futures::StreamExt;

        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project/assets?contentType=RESOURCE&assetTypes=compute.googleapis.com%2FDisk&assetTypes=storage.googleapis.com%2FBucket")
            .returning_json(json!({
                "assets": [
                    {"name": "//disk-1", "assetType": "compute.googleapis.com/Disk"},
                    {"name": "//bucket-1", "assetType": "storage.googleapis.com/Bucket"}
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let asset_types = [
            "compute.googleapis.com/Disk",
            "storage.googleapis.com/Bucket",
        ];
        let options = ListAssetsOptions {
            content_type: Some("RESOURCE"),
            asset_types: Some(&asset_types),
            ..Default::default()
        };

        let stream = asset_client.list_assets_stream("projects/test-project", options);
        futures::pin_mut!(stream);

        let mut names = Vec::new();
        while let Some(Ok(asset)) = stream.next().await {
            names.push(asset.name.clone());
        }
        assert_eq!(names.len(), 2);
    }

    #[tokio::test]
    async fn test_list_assets_pagination() {
        let mut mock = MockClient::new();

        // Page 1
        mock.expect_get("/v1/projects/test-project/assets?pageSize=1&contentType=RESOURCE")
            .returning_json(json!({
                "assets": [{"name": "//asset-1", "assetType": "type/A"}],
                "nextPageToken": "token-page-2"
            }));

        // Page 2
        mock.expect_get("/v1/projects/test-project/assets?pageSize=1&pageToken=token-page-2&contentType=RESOURCE")
            .returning_json(json!({
                "assets": [{"name": "//asset-2", "assetType": "type/B"}]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = ListAssetsOptions {
            page_size: Some(1),
            content_type: Some("RESOURCE"),
            ..Default::default()
        };

        // Get page 1
        let page1 = asset_client
            .list_assets("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(page1.assets.len(), 1);
        assert_eq!(page1.next_page_token.as_deref(), Some("token-page-2"));

        // Get page 2
        let page2 = asset_client
            .list_assets(
                "projects/test-project",
                &options,
                page1.next_page_token.as_deref(),
            )
            .await
            .unwrap();

        assert_eq!(page2.assets.len(), 1);
        assert!(page2.next_page_token.is_none());
    }

    #[tokio::test]
    async fn test_list_assets_all() {
        let mut mock = MockClient::new();

        // Register more specific path (with pageToken) FIRST due to StartsWith matching
        // Page 2
        mock.expect_get("/v1/projects/test-project/assets?pageToken=tok2")
            .returning_json(json!({
                "assets": [{"name": "//asset-3", "assetType": "type/C"}]
            }));

        // Page 1 (less specific, registered second)
        mock.expect_get("/v1/projects/test-project/assets")
            .returning_json(json!({
                "assets": [
                    {"name": "//asset-1", "assetType": "type/A"},
                    {"name": "//asset-2", "assetType": "type/B"}
                ],
                "nextPageToken": "tok2"
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = ListAssetsOptions::default();
        let all_assets = asset_client
            .list_assets_all("projects/test-project", &options)
            .await
            .unwrap();

        assert_eq!(all_assets.len(), 3);
        assert_eq!(all_assets[0].name.as_deref(), Some("//asset-1"));
        assert_eq!(all_assets[2].name.as_deref(), Some("//asset-3"));
    }

    #[tokio::test]
    async fn test_search_all_resources() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project:searchAllResources?query=name%3Amy-vm&pageSize=50")
            .returning_json(json!({
                "results": [
                    {
                        "name": "//compute.googleapis.com/projects/test-project/zones/us-central1-a/instances/my-vm",
                        "assetType": "compute.googleapis.com/Instance",
                        "project": "projects/123456",
                        "location": "us-central1-a",
                        "displayName": "my-vm"
                    }
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = SearchOptions {
            query: Some("name:my-vm"),
            page_size: Some(50),
            ..Default::default()
        };

        let response = asset_client
            .search_all_resources("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.results.len(), 1);
        assert!(response.results[0].name.as_ref().unwrap().contains("my-vm"));
        assert_eq!(response.results[0].display_name.as_deref(), Some("my-vm"));
    }

    #[tokio::test]
    async fn test_search_all_resources_with_asset_types() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project:searchAllResources?assetTypes=compute.googleapis.com%2FInstance&assetTypes=storage.googleapis.com%2FBucket")
            .returning_json(json!({
                "results": [
                    {"name": "//vm-1", "assetType": "compute.googleapis.com/Instance"},
                    {"name": "//bucket-1", "assetType": "storage.googleapis.com/Bucket"}
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let asset_types = [
            "compute.googleapis.com/Instance",
            "storage.googleapis.com/Bucket",
        ];
        let options = SearchOptions {
            asset_types: Some(&asset_types),
            ..Default::default()
        };

        let response = asset_client
            .search_all_resources("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.results.len(), 2);
    }

    #[tokio::test]
    async fn test_search_all_resources_all() {
        let mut mock = MockClient::new();

        // Register more specific path (with pageToken) FIRST due to StartsWith matching
        // Page 2
        mock.expect_get("/v1/projects/test-project:searchAllResources?pageToken=search-tok-2")
            .returning_json(json!({
                "results": [{"name": "//res-3", "assetType": "type/C"}]
            }));

        // Page 1 (less specific, registered second)
        mock.expect_get("/v1/projects/test-project:searchAllResources")
            .returning_json(json!({
                "results": [
                    {"name": "//res-1", "assetType": "type/A"},
                    {"name": "//res-2", "assetType": "type/B"}
                ],
                "nextPageToken": "search-tok-2"
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = SearchOptions::default();
        let all_results = asset_client
            .search_all_resources_all("projects/test-project", &options)
            .await
            .unwrap();

        assert_eq!(all_results.len(), 3);
        assert_eq!(all_results[0].name.as_deref(), Some("//res-1"));
        assert_eq!(all_results[2].name.as_deref(), Some("//res-3"));
    }

    #[tokio::test]
    async fn test_search_with_order_by() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project:searchAllResources?orderBy=createTime%20desc")
            .returning_json(json!({
                "results": [
                    {"name": "//newest", "createTime": "2024-01-15T10:00:00Z"},
                    {"name": "//older", "createTime": "2024-01-14T10:00:00Z"}
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = SearchOptions {
            order_by: Some("createTime desc"),
            ..Default::default()
        };

        let response = asset_client
            .search_all_resources("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].name.as_deref(), Some("//newest"));
    }

    #[tokio::test]
    async fn test_list_assets_with_iam_policy() {
        let mut mock = MockClient::new();

        mock.expect_get("/v1/projects/test-project/assets?pageSize=10&contentType=IAM_POLICY")
            .returning_json(json!({
                "assets": [
                    {
                        "name": "//storage.googleapis.com/projects/_/buckets/my-bucket",
                        "assetType": "storage.googleapis.com/Bucket",
                        "iamPolicy": {
                            "version": 1,
                            "bindings": [
                                {
                                    "role": "roles/storage.admin",
                                    "members": ["user:admin@example.com"]
                                },
                                {
                                    "role": "roles/storage.objectViewer",
                                    "members": [
                                        "serviceAccount:sa@project.iam.gserviceaccount.com",
                                        "group:viewers@example.com"
                                    ]
                                }
                            ],
                            "etag": "BwXyz123="
                        }
                    }
                ]
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = ListAssetsOptions {
            page_size: Some(10),
            content_type: Some("IAM_POLICY"),
            ..Default::default()
        };

        let response = asset_client
            .list_assets("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.assets.len(), 1);
        let asset = &response.assets[0];
        assert!(asset.iam_policy.is_some());

        let policy = asset.iam_policy.as_ref().unwrap();
        assert_eq!(policy.version, Some(1));
        assert_eq!(policy.etag.as_deref(), Some("BwXyz123="));
        assert_eq!(policy.bindings.len(), 2);
        assert_eq!(
            policy.bindings[0].role.as_deref(),
            Some("roles/storage.admin")
        );
        assert_eq!(policy.bindings[0].members.len(), 1);
        assert_eq!(policy.bindings[1].members.len(), 2);
    }

    #[tokio::test]
    async fn test_search_all_iam_policies() {
        let mut mock = MockClient::new();

        mock.expect_get(
            "/v1/projects/test-project:searchAllIamPolicies?query=policy%3Aroles%2Feditor&pageSize=50",
        )
        .returning_json(json!({
            "results": [
                {
                    "resource": "//cloudresourcemanager.googleapis.com/projects/test-project",
                    "assetType": "cloudresourcemanager.googleapis.com/Project",
                    "project": "projects/123456",
                    "policy": {
                        "bindings": [
                            {
                                "role": "roles/editor",
                                "members": ["user:dev@example.com"]
                            }
                        ]
                    }
                }
            ]
        }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = SearchIamPoliciesOptions {
            query: Some("policy:roles/editor"),
            page_size: Some(50),
            ..Default::default()
        };

        let response = asset_client
            .search_all_iam_policies("projects/test-project", &options, None)
            .await
            .unwrap();

        assert_eq!(response.results.len(), 1);
        let result = &response.results[0];
        assert_eq!(
            result.asset_type.as_deref(),
            Some("cloudresourcemanager.googleapis.com/Project")
        );
        assert!(result.policy.is_some());
        let policy = result.policy.as_ref().unwrap();
        assert_eq!(policy.bindings.len(), 1);
        assert_eq!(policy.bindings[0].role.as_deref(), Some("roles/editor"));
    }

    #[tokio::test]
    async fn test_search_all_iam_policies_all() {
        let mut mock = MockClient::new();

        // Page 2 (more specific, registered first)
        mock.expect_get("/v1/projects/test-project:searchAllIamPolicies?pageToken=iam-tok-2")
            .returning_json(json!({
                "results": [
                    {
                        "resource": "//storage.googleapis.com/projects/_/buckets/bucket-2",
                        "assetType": "storage.googleapis.com/Bucket"
                    }
                ]
            }));

        // Page 1 (less specific, registered second)
        mock.expect_get("/v1/projects/test-project:searchAllIamPolicies")
            .returning_json(json!({
                "results": [
                    {
                        "resource": "//storage.googleapis.com/projects/_/buckets/bucket-1",
                        "assetType": "storage.googleapis.com/Bucket"
                    }
                ],
                "nextPageToken": "iam-tok-2"
            }));

        let client = GcpHttpClient::from_mock(mock);
        let asset_client = client.cloud_asset();

        let options = SearchIamPoliciesOptions::default();
        let all_results = asset_client
            .search_all_iam_policies_all("projects/test-project", &options)
            .await
            .unwrap();

        assert_eq!(all_results.len(), 2);
        assert!(
            all_results[0]
                .resource
                .as_ref()
                .unwrap()
                .contains("bucket-1")
        );
        assert!(
            all_results[1]
                .resource
                .as_ref()
                .unwrap()
                .contains("bucket-2")
        );
    }
}
