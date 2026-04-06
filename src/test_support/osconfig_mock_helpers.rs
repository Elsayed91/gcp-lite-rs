//! MockClient helpers for OS Config API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with OS Config API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait OsconfigMockHelpers {
    /// Helper to expect `list_patch_deployments`: Get a page of OS Config patch deployments.
    fn expect_list_patch_deployments(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_inventories`: List inventory data for all VM instances in the
    /// specified zone.
    fn expect_list_inventories(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        view: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl OsconfigMockHelpers for MockClient {
    /// Helper to expect `list_patch_deployments`: Get a page of OS Config patch deployments.
    fn expect_list_patch_deployments(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/patchDeployments");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `list_inventories`: List inventory data for all VM instances in the
    /// specified zone.
    fn expect_list_inventories(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        view: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/inventories");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !view.is_empty() {
            __qp.push(format!("view={}", view));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }
}
