//! MockClient helpers for Cloud DLP API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud DLP API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait DlpMockHelpers {
    /// Helper to expect `list_discovery_configs`: Lists discovery configurations.
    fn expect_list_discovery_configs(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_project_data_profiles`: Lists project data profiles for an
    /// organization.
    fn expect_list_project_data_profiles(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl DlpMockHelpers for MockClient {
    /// Helper to expect `list_discovery_configs`: Lists discovery configurations.
    fn expect_list_discovery_configs(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v2/{parent}/discoveryConfigs");
        let mut __qp: Vec<String> = Vec::new();
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

    /// Helper to expect `list_project_data_profiles`: Lists project data profiles for an
    /// organization.
    fn expect_list_project_data_profiles(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        order_by: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v2/{parent}/projectDataProfiles");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !order_by.is_empty() {
            __qp.push(format!("orderBy={}", order_by));
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
