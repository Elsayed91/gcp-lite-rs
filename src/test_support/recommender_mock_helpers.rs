//! MockClient helpers for Recommender API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Recommender API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait RecommenderMockHelpers {
    /// Helper to expect `list_recommendations`: Lists recommendations for the specified Cloud
    /// Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    fn expect_list_recommendations(
        &mut self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl RecommenderMockHelpers for MockClient {
    /// Helper to expect `list_recommendations`: Lists recommendations for the specified Cloud
    /// Resource. Requires the recommender.*.list IAM permission for the specified recommender.
    fn expect_list_recommendations(
        &mut self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/recommendations");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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
}
