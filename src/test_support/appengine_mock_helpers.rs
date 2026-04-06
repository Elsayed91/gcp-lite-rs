//! MockClient helpers for App Engine Admin API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with App Engine Admin API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AppengineMockHelpers {
    /// Helper to expect `get_app`: Gets information about an application.
    fn expect_get_app(&mut self, apps_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_services`: Lists all the services in the application.
    fn expect_list_services(
        &mut self,
        apps_id: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_service`: Gets the current configuration of the specified service.
    fn expect_get_service(&mut self, apps_id: &str, services_id: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AppengineMockHelpers for MockClient {
    /// Helper to expect `get_app`: Gets information about an application.
    fn expect_get_app(&mut self, apps_id: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/apps/{apps_id}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_services`: Lists all the services in the application.
    fn expect_list_services(
        &mut self,
        apps_id: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/apps/{apps_id}/services");
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

    /// Helper to expect `get_service`: Gets the current configuration of the specified service.
    fn expect_get_service(
        &mut self,
        apps_id: &str,
        services_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/apps/{apps_id}/services/{services_id}");
        self.expect_get(&path)
    }
}
