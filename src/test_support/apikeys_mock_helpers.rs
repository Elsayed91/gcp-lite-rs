//! MockClient helpers for API Keys API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with API Keys API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ApikeysMockHelpers {
    /// Helper to expect `list_keys`: Lists the API keys owned by a project. The key string of the
    /// API key isn't included in the response. NOTE: Key is a global resource; hence the only
    /// supported value for location is `global`.
    fn expect_list_keys(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        show_deleted: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_key`: Gets the metadata for an API key. The key string of the API key
    /// isn't included in the response. NOTE: Key is a global resource; hence the only supported
    /// value for location is `global`.
    fn expect_get_key(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ApikeysMockHelpers for MockClient {
    /// Helper to expect `list_keys`: Lists the API keys owned by a project. The key string of the
    /// API key isn't included in the response. NOTE: Key is a global resource; hence the only
    /// supported value for location is `global`.
    fn expect_list_keys(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        show_deleted: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v2/{parent}/keys");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !show_deleted.is_empty() {
            __qp.push(format!("showDeleted={}", show_deleted));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_key`: Gets the metadata for an API key. The key string of the API key
    /// isn't included in the response. NOTE: Key is a global resource; hence the only supported
    /// value for location is `global`.
    fn expect_get_key(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v2/{name}");
        self.expect_get(&path)
    }
}
