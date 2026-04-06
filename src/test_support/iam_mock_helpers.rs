//! MockClient helpers for IAM API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with IAM helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait IamMockHelpers {
    /// Helper to expect `create_service_account`: Creates a ServiceAccount.
    fn expect_create_service_account(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_service_account`: Gets a ServiceAccount.
    fn expect_get_service_account(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_service_accounts`: Lists every ServiceAccount that belongs to a
    /// specific project.
    fn expect_list_service_accounts(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_service_account`: Deletes a ServiceAccount. **Warning:** After you
    /// delete a service account, you might not be able to undelete it. If you know that you need to
    /// re-enable the service account in the future, use DisableServiceAccount instead. If you
    /// delete a service account, IAM permanently removes the service account 30 days later. Google
    /// Cloud cannot recover the service account after it is permanently removed, even if you file a
    /// support request. To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use DisableServiceAccount to disable the service account, then
    /// wait at least 24 hours and watch for unintended consequences. If there are no unintended
    /// consequences, you can delete the service account.
    fn expect_delete_service_account(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_service_account_key`: Creates a ServiceAccountKey.
    fn expect_create_service_account_key(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_service_account_keys`: Lists every ServiceAccountKey for a service
    /// account.
    fn expect_list_service_account_keys(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_service_account_key`: Deletes a ServiceAccountKey. Deleting a
    /// service account key does not revoke short-lived credentials that have been issued based on
    /// the service account key.
    fn expect_delete_service_account_key(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl IamMockHelpers for MockClient {
    /// Helper to expect `create_service_account`: Creates a ServiceAccount.
    fn expect_create_service_account(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/serviceAccounts");
        self.expect_post(&path)
    }

    /// Helper to expect `get_service_account`: Gets a ServiceAccount.
    fn expect_get_service_account(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_service_accounts`: Lists every ServiceAccount that belongs to a
    /// specific project.
    fn expect_list_service_accounts(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/serviceAccounts");
        self.expect_get(&path)
    }

    /// Helper to expect `delete_service_account`: Deletes a ServiceAccount. **Warning:** After you
    /// delete a service account, you might not be able to undelete it. If you know that you need to
    /// re-enable the service account in the future, use DisableServiceAccount instead. If you
    /// delete a service account, IAM permanently removes the service account 30 days later. Google
    /// Cloud cannot recover the service account after it is permanently removed, even if you file a
    /// support request. To help avoid unplanned outages, we recommend that you disable the service
    /// account before you delete it. Use DisableServiceAccount to disable the service account, then
    /// wait at least 24 hours and watch for unintended consequences. If there are no unintended
    /// consequences, you can delete the service account.
    fn expect_delete_service_account(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }

    /// Helper to expect `create_service_account_key`: Creates a ServiceAccountKey.
    fn expect_create_service_account_key(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/keys");
        self.expect_post(&path)
    }

    /// Helper to expect `list_service_account_keys`: Lists every ServiceAccountKey for a service
    /// account.
    fn expect_list_service_account_keys(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/keys");
        self.expect_get(&path)
    }

    /// Helper to expect `delete_service_account_key`: Deletes a ServiceAccountKey. Deleting a
    /// service account key does not revoke short-lived credentials that have been issued based on
    /// the service account key.
    fn expect_delete_service_account_key(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }
}
