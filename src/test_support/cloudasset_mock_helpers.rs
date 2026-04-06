//! MockClient helpers for Cloud Asset API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Asset helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudassetMockHelpers {
    /// Helper to expect `search_all_resources`: Searches all Google Cloud resources within the
    /// specified scope, such as a project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllResources` permission on the desired scope, otherwise the
    /// request will be rejected.
    #[allow(clippy::too_many_arguments)]
    fn expect_search_all_resources(
        &mut self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
        read_mask: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `search_all_iam_policies`: Searches all IAM policies within the specified
    /// scope, such as a project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllIamPolicies` permission on the desired scope, otherwise the
    /// request will be rejected.
    fn expect_search_all_iam_policies(
        &mut self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudassetMockHelpers for MockClient {
    /// Helper to expect `search_all_resources`: Searches all Google Cloud resources within the
    /// specified scope, such as a project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllResources` permission on the desired scope, otherwise the
    /// request will be rejected.
    #[allow(clippy::too_many_arguments)]
    fn expect_search_all_resources(
        &mut self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
        read_mask: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{scope}:searchAllResources");
        let mut __qp: Vec<String> = Vec::new();
        if !query.is_empty() {
            __qp.push(format!("query={}", query));
        }
        for __v in asset_types {
            __qp.push(format!("assetTypes={}", __v));
        }
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !order_by.is_empty() {
            __qp.push(format!("orderBy={}", order_by));
        }
        if !read_mask.is_empty() {
            __qp.push(format!("readMask={}", read_mask));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `search_all_iam_policies`: Searches all IAM policies within the specified
    /// scope, such as a project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllIamPolicies` permission on the desired scope, otherwise the
    /// request will be rejected.
    fn expect_search_all_iam_policies(
        &mut self,
        scope: &str,
        query: &str,
        asset_types: &[&str],
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{scope}:searchAllIamPolicies");
        let mut __qp: Vec<String> = Vec::new();
        if !query.is_empty() {
            __qp.push(format!("query={}", query));
        }
        for __v in asset_types {
            __qp.push(format!("assetTypes={}", __v));
        }
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !order_by.is_empty() {
            __qp.push(format!("orderBy={}", order_by));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }
}
