//! MockClient helpers for Service Usage API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Service Usage helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ServiceUsageMockHelpers {
    /// Helper to expect `get_service`: Returns the service configuration and enabled state for a
    /// given service.
    fn expect_get_service(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `enable_service`: Enable a service so that it can be used with a project.
    fn expect_enable_service(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `disable_service`: Disable a service so that it can no longer be used with
    /// a project. This prevents unintended usage that may cause unexpected billing charges or
    /// security leaks. It is not valid to call the disable method on a service that is not
    /// currently enabled. Callers will receive a `FAILED_PRECONDITION` status if the target service
    /// is not currently enabled.
    fn expect_disable_service(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_services`: List all services available to the specified project, and
    /// the current state of those services with respect to the project. The list includes all
    /// public services, all services for which the calling user has the
    /// `servicemanagement.services.bind` permission, and all services that have already been
    /// enabled on the project. The list can be filtered to only include services in a specific
    /// state, for example to only include services enabled on the project. WARNING: If you need to
    /// query enabled services frequently or across an organization, you should use [Cloud Asset
    /// Inventory API](https://cloud.google.com/asset-inventory/docs/apis), which provides higher
    /// throughput and richer filtering capability.
    fn expect_list_services(
        &mut self,
        parent: &str,
        page_token: &str,
        filter: &str,
        page_size: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `batch_enable_services`: Enable multiple services on a project. The
    /// operation is atomic: if enabling any service fails, then the entire batch fails, and no
    /// state changes occur. To enable a single service, use the `EnableService` method instead.
    fn expect_batch_enable_services(&mut self, parent: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ServiceUsageMockHelpers for MockClient {
    /// Helper to expect `get_service`: Returns the service configuration and enabled state for a
    /// given service.
    fn expect_get_service(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `enable_service`: Enable a service so that it can be used with a project.
    fn expect_enable_service(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}:enable");
        self.expect_post(&path)
    }

    /// Helper to expect `disable_service`: Disable a service so that it can no longer be used with
    /// a project. This prevents unintended usage that may cause unexpected billing charges or
    /// security leaks. It is not valid to call the disable method on a service that is not
    /// currently enabled. Callers will receive a `FAILED_PRECONDITION` status if the target service
    /// is not currently enabled.
    fn expect_disable_service(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}:disable");
        self.expect_post(&path)
    }

    /// Helper to expect `list_services`: List all services available to the specified project, and
    /// the current state of those services with respect to the project. The list includes all
    /// public services, all services for which the calling user has the
    /// `servicemanagement.services.bind` permission, and all services that have already been
    /// enabled on the project. The list can be filtered to only include services in a specific
    /// state, for example to only include services enabled on the project. WARNING: If you need to
    /// query enabled services frequently or across an organization, you should use [Cloud Asset
    /// Inventory API](https://cloud.google.com/asset-inventory/docs/apis), which provides higher
    /// throughput and richer filtering capability.
    fn expect_list_services(
        &mut self,
        parent: &str,
        page_token: &str,
        filter: &str,
        page_size: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/services");
        let mut __qp: Vec<String> = Vec::new();
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `batch_enable_services`: Enable multiple services on a project. The
    /// operation is atomic: if enabling any service fails, then the entire batch fails, and no
    /// state changes occur. To enable a single service, use the `EnableService` method instead.
    fn expect_batch_enable_services(
        &mut self,
        parent: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/services:batchEnable");
        self.expect_post(&path)
    }
}
