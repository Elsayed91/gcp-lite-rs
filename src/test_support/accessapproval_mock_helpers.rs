//! MockClient helpers for Access Approval API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Access Approval API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AccessapprovalMockHelpers {
    /// Helper to expect `get_access_approval_settings_project`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_project(&mut self, name: &str)
    -> ExpectationBuilder<'_>;

    /// Helper to expect `get_access_approval_settings_folder`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_folder(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_access_approval_settings_org`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_org(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AccessapprovalMockHelpers for MockClient {
    /// Helper to expect `get_access_approval_settings_project`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_project(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `get_access_approval_settings_folder`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_folder(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `get_access_approval_settings_org`: Gets the Access Approval settings
    /// associated with a project, folder, or organization.
    fn expect_get_access_approval_settings_org(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }
}
