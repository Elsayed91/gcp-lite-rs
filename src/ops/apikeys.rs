//! Operation contracts for the API Keys API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/apikeys.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::apikeys::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the API Keys API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::apikeys::ApikeysClient`] instead.
pub struct ApikeysOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> ApikeysOps<'a> {
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
        "https://apikeys.googleapis.com"
    }

    /// Lists the API keys owned by a project. The key string of the API key isn't included in
    /// the response. NOTE: Key is a global resource; hence the only supported value for
    /// location is `global`.
    ///
    /// **GCP API**: `GET v2/{+parent}/keys`
    ///
    /// # Path Parameters
    /// - `parent` — Required. Lists all API keys associated with this project. *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. Specifies the maximum number of results to be returned at a time.
    /// - `pageToken` — Optional. Requests a specific page of results.
    /// - `showDeleted` — Optional. Indicate that keys deleted in the past 30 days should also be returned.
    ///
    /// # Response
    /// [`V2ListKeysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_keys(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        show_deleted: &str,
    ) -> Result<V2ListKeysResponse> {
        let url = format!("{}/v2/{}/keys", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("showDeleted", show_deleted),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the metadata for an API key. The key string of the API key isn't included in the
    /// response. NOTE: Key is a global resource; hence the only supported value for location is
    /// `global`.
    ///
    /// **GCP API**: `GET v2/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the API key to get. *(required)*
    ///
    /// # Response
    /// [`V2Key`]
    #[allow(dead_code)]
    pub(crate) async fn get_key(&self, name: &str) -> Result<V2Key> {
        let url = format!("{}/v2/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-parent/keys?pageSize=test-pageSize&pageToken=test-pageToken&showDeleted=test-showDeleted")
            .returning_json(serde_json::to_value(V2ListKeysResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ApikeysOps::new(&client);

        let result = ops
            .list_keys(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-showDeleted",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-name")
            .returning_json(serde_json::to_value(V2Key::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ApikeysOps::new(&client);

        let result = ops.get_key("test-name").await;
        assert!(result.is_ok());
    }
}
