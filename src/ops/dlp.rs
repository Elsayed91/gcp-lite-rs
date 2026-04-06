//! Operation contracts for the Cloud DLP API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/dlp.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::dlp::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud DLP API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::dlp::DlpClient`] instead.
pub struct DlpOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> DlpOps<'a> {
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
        "https://dlp.googleapis.com"
    }

    /// Lists discovery configurations.
    ///
    /// **GCP API**: `GET v2/{+parent}/discoveryConfigs`
    ///
    /// # Path Parameters
    /// - `parent` — Required. Parent resource name. The format of this value is as follows: `projects/{project_id}/locations/{location_id}`  *(required)*
    ///
    /// # Query Parameters
    /// - `orderBy` — Comma-separated list of config fields to order by, followed by `asc` or `desc` postfix. This list is case insensitive. T
    /// - `pageSize` — Size of the page. This value can be limited by a server.
    /// - `pageToken` — Page token to continue retrieval. Comes from the previous call to ListDiscoveryConfigs. `order_by` field must not change
    ///
    /// # Response
    /// [`GooglePrivacyDlpV2ListDiscoveryConfigsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_discovery_configs(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> Result<GooglePrivacyDlpV2ListDiscoveryConfigsResponse> {
        let url = format!("{}/v2/{}/discoveryConfigs", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("orderBy", order_by),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_discovery_configs response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists project data profiles for an organization.
    ///
    /// **GCP API**: `GET v2/{+parent}/projectDataProfiles`
    ///
    /// # Path Parameters
    /// - `parent` — Required. organizations/{org_id}/locations/{loc_id} *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Allows filtering. Supported syntax: * Filter expressions are made up of one or more restrictions. * Restrictions can be
    /// - `orderBy` — Comma-separated list of fields to order by, followed by `asc` or `desc` postfix. This list is case insensitive. The defa
    /// - `pageSize` — Size of the page. This value can be limited by the server. If zero, server returns a page of max size 100.
    /// - `pageToken` — Page token to continue retrieval.
    ///
    /// # Response
    /// [`GooglePrivacyDlpV2ListProjectDataProfilesResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_project_data_profiles(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
        filter: &str,
    ) -> Result<GooglePrivacyDlpV2ListProjectDataProfilesResponse> {
        let url = format!("{}/v2/{}/projectDataProfiles", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("pageSize", page_size),
                ("pageToken", page_token),
                ("orderBy", order_by),
                ("filter", filter),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_project_data_profiles response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_discovery_configs() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-parent/discoveryConfigs?pageSize=test-pageSize&pageToken=test-pageToken&orderBy=test-orderBy")
            .returning_json(serde_json::to_value(GooglePrivacyDlpV2ListDiscoveryConfigsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DlpOps::new(&client);

        let result = ops
            .list_discovery_configs(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-orderBy",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_project_data_profiles() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v2/test-parent/projectDataProfiles?pageSize=test-pageSize&pageToken=test-pageToken&orderBy=test-orderBy&filter=test-filter")
            .returning_json(serde_json::to_value(GooglePrivacyDlpV2ListProjectDataProfilesResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DlpOps::new(&client);

        let result = ops
            .list_project_data_profiles(
                "test-parent",
                "test-pageSize",
                "test-pageToken",
                "test-orderBy",
                "test-filter",
            )
            .await;
        assert!(result.is_ok());
    }
}
