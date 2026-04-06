//! MockClient helpers for Kubernetes Engine API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Kubernetes Engine helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ContainerMockHelpers {
    /// Helper to expect `list_clusters`: Lists all clusters owned by a project in either the
    /// specified zone or all zones.
    fn expect_list_clusters(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_cluster`: Gets the details of a specific cluster.
    fn expect_get_cluster(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_cluster`: Deletes the cluster, including the Kubernetes endpoint
    /// and all worker nodes. Firewalls and routes that were configured during cluster creation are
    /// also deleted. Other Google Compute Engine resources that might be in use by the cluster,
    /// such as load balancer resources, are not deleted if they weren't present when the cluster
    /// was initially created.
    fn expect_delete_cluster(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ContainerMockHelpers for MockClient {
    /// Helper to expect `list_clusters`: Lists all clusters owned by a project in either the
    /// specified zone or all zones.
    fn expect_list_clusters(&mut self, parent: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/clusters");
        self.expect_get(&path)
    }

    /// Helper to expect `get_cluster`: Gets the details of a specific cluster.
    fn expect_get_cluster(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `delete_cluster`: Deletes the cluster, including the Kubernetes endpoint
    /// and all worker nodes. Firewalls and routes that were configured during cluster creation are
    /// also deleted. Other Google Compute Engine resources that might be in use by the cluster,
    /// such as load balancer resources, are not deleted if they weren't present when the cluster
    /// was initially created.
    fn expect_delete_cluster(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }
}
