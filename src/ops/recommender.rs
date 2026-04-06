//! Operation contracts for the Recommender API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/recommender.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::recommender::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Recommender API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::recommender::RecommenderClient`] instead.
pub struct RecommenderOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> RecommenderOps<'a> {
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
        "https://recommender.googleapis.com"
    }

    /// Lists recommendations for the specified Cloud Resource. Requires the recommender.*.list
    /// IAM permission for the specified recommender.
    ///
    /// **GCP API**: `GET v1/{+parent}/recommendations`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The container resource on which to execute the request. Acceptable formats: * `projects/[PROJECT_NUMBER]/locat *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Filter expression to restrict the recommendations returned. Supported filter fields: * `state_info.state` * `recommender
    /// - `pageSize` — Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified,
    /// - `pageToken` — Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be t
    ///
    /// # Response
    /// [`ListRecommendationsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_recommendations(
        &self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListRecommendationsResponse> {
        let url = format!("{}/v1/{}/recommendations", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_recommendations response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_recommendations() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/recommendations?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListRecommendationsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = RecommenderOps::new(&client);

        let result = ops
            .list_recommendations(
                "test-parent",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }
}
