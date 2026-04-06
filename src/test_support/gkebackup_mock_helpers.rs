//! MockClient helpers for Backup for GKE API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Backup for GKE helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait GkebackupMockHelpers {
    /// Helper to expect `get_backup_plan`: Retrieve the details of a single BackupPlan.
    fn expect_get_backup_plan(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_backup_plans`: Lists BackupPlans in a given location.
    fn expect_list_backup_plans(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_backup_plan`: Creates a new BackupPlan in a given location.
    fn expect_create_backup_plan(
        &mut self,
        parent: &str,
        backup_plan_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_backup_plan`: Deletes an existing BackupPlan.
    fn expect_delete_backup_plan(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl GkebackupMockHelpers for MockClient {
    /// Helper to expect `get_backup_plan`: Retrieve the details of a single BackupPlan.
    fn expect_get_backup_plan(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_backup_plans`: Lists BackupPlans in a given location.
    fn expect_list_backup_plans(
        &mut self,
        parent: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/backupPlans");
        self.expect_get(&path)
    }

    /// Helper to expect `create_backup_plan`: Creates a new BackupPlan in a given location.
    fn expect_create_backup_plan(
        &mut self,
        parent: &str,
        backup_plan_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/backupPlans");
        let mut __qp: Vec<String> = Vec::new();
        if !backup_plan_id.is_empty() {
            __qp.push(format!("backupPlanId={}", backup_plan_id));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `delete_backup_plan`: Deletes an existing BackupPlan.
    fn expect_delete_backup_plan(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }
}
