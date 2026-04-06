//! MockClient helpers for Cloud DNS API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud DNS API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait DnsMockHelpers {
    /// Helper to expect `get_managed_zone`: Fetches the representation of an existing ManagedZone.
    fn expect_get_managed_zone(
        &mut self,
        project: &str,
        managed_zone: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_managed_zones`: Enumerates ManagedZones that have been created but
    /// not yet deleted.
    fn expect_list_managed_zones(
        &mut self,
        project: &str,
        max_results: &str,
        page_token: &str,
        dns_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_managed_zone`: Applies a partial update to an existing ManagedZone.
    fn expect_patch_managed_zone(
        &mut self,
        project: &str,
        managed_zone: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_dns_policies`: Enumerates all policies associated with a project.
    fn expect_list_dns_policies(
        &mut self,
        project: &str,
        max_results: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_dns_policy`: Fetches the representation of an existing policy.
    fn expect_get_dns_policy(&mut self, project: &str, policy: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_dns_policy`: Creates a new policy.
    fn expect_create_dns_policy(&mut self, project: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_dns_policy`: Applies a partial update to an existing policy.
    fn expect_patch_dns_policy(&mut self, project: &str, policy: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_dns_policy`: Deletes a previously created policy. Fails if the
    /// policy is still being referenced by a network.
    fn expect_delete_dns_policy(&mut self, project: &str, policy: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl DnsMockHelpers for MockClient {
    /// Helper to expect `get_managed_zone`: Fetches the representation of an existing ManagedZone.
    fn expect_get_managed_zone(
        &mut self,
        project: &str,
        managed_zone: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/managedZones/{managed_zone}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_managed_zones`: Enumerates ManagedZones that have been created but
    /// not yet deleted.
    fn expect_list_managed_zones(
        &mut self,
        project: &str,
        max_results: &str,
        page_token: &str,
        dns_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/dns/v1/projects/{project}/managedZones");
        let mut __qp: Vec<String> = Vec::new();
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !dns_name.is_empty() {
            __qp.push(format!("dnsName={}", dns_name));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `patch_managed_zone`: Applies a partial update to an existing ManagedZone.
    fn expect_patch_managed_zone(
        &mut self,
        project: &str,
        managed_zone: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/managedZones/{managed_zone}");
        self.expect_patch(&path)
    }

    /// Helper to expect `list_dns_policies`: Enumerates all policies associated with a project.
    fn expect_list_dns_policies(
        &mut self,
        project: &str,
        max_results: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/dns/v1/projects/{project}/policies");
        let mut __qp: Vec<String> = Vec::new();
        if !max_results.is_empty() {
            __qp.push(format!("maxResults={}", max_results));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_dns_policy`: Fetches the representation of an existing policy.
    fn expect_get_dns_policy(
        &mut self,
        project: &str,
        policy: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/policies/{policy}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_dns_policy`: Creates a new policy.
    fn expect_create_dns_policy(
        &mut self,
        project: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/policies");
        self.expect_post(&path)
    }

    /// Helper to expect `patch_dns_policy`: Applies a partial update to an existing policy.
    fn expect_patch_dns_policy(
        &mut self,
        project: &str,
        policy: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/policies/{policy}");
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_dns_policy`: Deletes a previously created policy. Fails if the
    /// policy is still being referenced by a network.
    fn expect_delete_dns_policy(
        &mut self,
        project: &str,
        policy: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/dns/v1/projects/{project}/policies/{policy}");
        self.expect_delete(&path)
    }
}
