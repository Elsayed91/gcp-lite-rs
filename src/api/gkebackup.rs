//! Backup for GKE API client.
//!
//! Provides ergonomic wrappers over generated ops for GKE backup plan management.

use crate::operation::{GkeBackupOperation, PollConfig};
use crate::ops::gkebackup::GkebackupOps;
use crate::types::gkebackup::{BackupPlan, GkeBackupLro};
use crate::{GcpHttpClient, Result};

/// Client for the Backup for GKE API.
pub struct GkeBackupClient<'a> {
    ops: GkebackupOps<'a>,
}

impl<'a> GkeBackupClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: GkebackupOps::new(client),
        }
    }

    // ── Backup Plans ──────────────────────────────────────────────────

    /// Get a backup plan by name.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `backup_plan` - The backup plan ID
    pub async fn get_backup_plan(
        &self,
        project: &str,
        location: &str,
        backup_plan: &str,
    ) -> Result<BackupPlan> {
        let name = format!(
            "projects/{}/locations/{}/backupPlans/{}",
            project, location, backup_plan
        );
        self.ops.get_backup_plan(&name).await
    }

    /// List backup plans in a location.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    pub async fn list_backup_plans(
        &self,
        project: &str,
        location: &str,
    ) -> Result<Vec<BackupPlan>> {
        let parent = format!("projects/{}/locations/{}", project, location);
        let response = self.ops.list_backup_plans(&parent).await?;
        Ok(response.backup_plans)
    }

    /// Create a backup plan (blocks until complete).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `backup_plan_id` - The ID for the new backup plan
    /// * `backup_plan` - The backup plan configuration
    pub async fn create_backup_plan(
        &self,
        project: &str,
        location: &str,
        backup_plan_id: &str,
        backup_plan: &BackupPlan,
    ) -> Result<()> {
        let op = self
            .create_backup_plan_start(project, location, backup_plan_id, backup_plan)
            .await?;
        op.wait().await
    }

    /// Create a backup plan (returns operation for manual polling).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `backup_plan_id` - The ID for the new backup plan
    /// * `backup_plan` - The backup plan configuration
    pub async fn create_backup_plan_start(
        &self,
        project: &str,
        location: &str,
        backup_plan_id: &str,
        backup_plan: &BackupPlan,
    ) -> Result<GkeBackupOperation<'a>> {
        let parent = format!("projects/{}/locations/{}", project, location);
        let lro = self
            .ops
            .create_backup_plan(&parent, backup_plan_id, backup_plan)
            .await?;
        self.gke_backup_operation(lro)
    }

    /// Delete a backup plan (blocks until complete).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `backup_plan` - The backup plan ID
    pub async fn delete_backup_plan(
        &self,
        project: &str,
        location: &str,
        backup_plan: &str,
    ) -> Result<()> {
        let op = self
            .delete_backup_plan_start(project, location, backup_plan)
            .await?;
        op.wait().await
    }

    /// Delete a backup plan (returns operation for manual polling).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `backup_plan` - The backup plan ID
    pub async fn delete_backup_plan_start(
        &self,
        project: &str,
        location: &str,
        backup_plan: &str,
    ) -> Result<GkeBackupOperation<'a>> {
        let name = format!(
            "projects/{}/locations/{}/backupPlans/{}",
            project, location, backup_plan
        );
        let lro = self.ops.delete_backup_plan(&name).await?;
        self.gke_backup_operation(lro)
    }

    // ── Helpers ───────────────────────────────────────────────────────

    fn gke_backup_operation(&self, lro: GkeBackupLro) -> Result<GkeBackupOperation<'a>> {
        // If the LRO is already done, check for errors and short-circuit —
        // the operation name may be a placeholder that GCP rejects on poll.
        if lro.done
            && let Some(error) = &lro.error
        {
            let message = error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            let code = error.get("code").and_then(|v| {
                v.as_str()
                    .map(String::from)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });
            return Err(crate::GcpError::OperationFailed {
                operation: lro.name,
                message,
                code,
            });
        }
        let config = PollConfig::gke_backup_operation();
        Ok(GkeBackupOperation::new(
            self.ops.client,
            lro.name,
            config.initial_interval(),
            config.timeout(),
            lro.done,
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_get_backup_plan() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/backupPlans/my-plan")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/backupPlans/my-plan",
                "cluster": "projects/test-project/locations/us-central1/clusters/my-cluster",
                "state": "READY",
                "description": "Test backup plan",
                "backupConfig": {
                    "allNamespaces": true
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let backup = client.gkebackup();

        let plan = backup
            .get_backup_plan("test-project", "us-central1", "my-plan")
            .await
            .unwrap();

        assert!(plan.name.ends_with("my-plan"));
        assert_eq!(
            plan.state,
            Some(crate::types::gkebackup::BackupPlanState::Ready)
        );
        assert_eq!(plan.description, Some("Test backup plan".to_string()));
        assert_eq!(
            plan.backup_config.as_ref().and_then(|c| c.all_namespaces),
            Some(true)
        );
    }

    #[tokio::test]
    async fn test_list_backup_plans() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/backupPlans")
            .returning_json(json!({
                "backupPlans": [
                    {
                        "name": "projects/test-project/locations/us-central1/backupPlans/plan-1",
                        "cluster": "projects/test-project/locations/us-central1/clusters/cluster-1",
                        "state": "READY"
                    },
                    {
                        "name": "projects/test-project/locations/us-central1/backupPlans/plan-2",
                        "cluster": "projects/test-project/locations/us-central1/clusters/cluster-2",
                        "state": "PROVISIONING"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let plans = client
            .gkebackup()
            .list_backup_plans("test-project", "us-central1")
            .await
            .unwrap();

        assert_eq!(plans.len(), 2);
        assert!(plans[0].name.ends_with("plan-1"));
        assert!(plans[1].name.ends_with("plan-2"));
        assert_eq!(
            plans[1].state,
            Some(crate::types::gkebackup::BackupPlanState::Provisioning)
        );
    }

    #[tokio::test]
    async fn test_list_backup_plans_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/backupPlans")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let plans = client
            .gkebackup()
            .list_backup_plans("test-project", "us-central1")
            .await
            .unwrap();
        assert!(plans.is_empty());
    }

    #[tokio::test]
    async fn test_create_backup_plan_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/v1/projects/test-project/locations/us-central1/backupPlans?backupPlanId=my-plan",
        )
        .returning_json(json!({
            "name": "projects/test-project/locations/us-central1/operations/op-123",
            "done": false
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let backup = client.gkebackup();

        let plan = crate::types::gkebackup::BackupPlan {
            cluster: Some(
                "projects/test-project/locations/us-central1/clusters/my-cluster".to_string(),
            ),
            backup_config: Some(crate::types::gkebackup::BackupConfig {
                all_namespaces: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };

        let op = backup
            .create_backup_plan_start("test-project", "us-central1", "my-plan", &plan)
            .await;
        assert!(op.is_ok());
    }

    #[tokio::test]
    async fn test_delete_backup_plan_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/projects/test-project/locations/us-central1/backupPlans/my-plan")
            .returning_json(json!({
                "name": "projects/test-project/locations/us-central1/operations/op-456",
                "done": false
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let op = client
            .gkebackup()
            .delete_backup_plan_start("test-project", "us-central1", "my-plan")
            .await;
        assert!(op.is_ok());
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_create_backup_plan_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/v1/projects/test-project/locations/us-central1/backupPlans?backupPlanId=my-plan",
        )
        .returning_json(json!({
            "name": "DONE_OPERATION",
            "done": true
        }))
        .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);
        let backup = client.gkebackup();

        let plan = crate::types::gkebackup::BackupPlan {
            cluster: Some(
                "projects/test-project/locations/us-central1/clusters/my-cluster".to_string(),
            ),
            ..Default::default()
        };

        let op = backup
            .create_backup_plan_start("test-project", "us-central1", "my-plan", &plan)
            .await;
        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }

    #[tokio::test]
    async fn test_initially_done_lro_with_error_returns_error() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/v1/projects/test-project/locations/us-central1/backupPlans?backupPlanId=my-plan",
        )
        .returning_json(json!({
            "name": "DONE_OPERATION",
            "done": true,
            "error": {
                "code": 409,
                "message": "Backup plan already exists"
            }
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let backup = client.gkebackup();

        let plan = crate::types::gkebackup::BackupPlan {
            cluster: Some(
                "projects/test-project/locations/us-central1/clusters/my-cluster".to_string(),
            ),
            ..Default::default()
        };

        let result = backup
            .create_backup_plan_start("test-project", "us-central1", "my-plan", &plan)
            .await;
        let Err(err) = result else {
            panic!("Expected error from initially-done LRO with error");
        };
        assert!(
            matches!(err, crate::GcpError::OperationFailed { .. }),
            "Expected OperationFailed, got: {:?}",
            err
        );
    }
}
