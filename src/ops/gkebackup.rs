//! Operation contracts for the Backup for GKE API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/gkebackup.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::gkebackup::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Backup for GKE API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::gkebackup::GkebackupClient`] instead.
pub struct GkebackupOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> GkebackupOps<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://gkebackup.googleapis.com"
    }

    /// Retrieve the details of a single BackupPlan.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. Fully qualified BackupPlan name. Format: `projects/*/locations/*/backupPlans/*` *(required)*
    ///
    /// # Response
    /// [`BackupPlan`]
    #[allow(dead_code)]
    pub(crate) async fn get_backup_plan(&self, name: &str) -> Result<BackupPlan> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_backup_plan response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists BackupPlans in a given location.
    ///
    /// **GCP API**: `GET v1/{+parent}/backupPlans`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The location that contains the BackupPlans to list. Format: `projects/*/locations/*` *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. Field match expression used to filter the results.
    /// - `orderBy` — Optional. Field by which to sort the results.
    /// - `pageSize` — Optional. The target number of results to return in a single response. If not specified, a default value will be chosen
    /// - `pageToken` — Optional. The value of next_page_token received from a previous `ListBackupPlans` call. Provide this to retrieve the sub
    ///
    /// # Response
    /// [`ListBackupPlansResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_backup_plans(&self, parent: &str) -> Result<ListBackupPlansResponse> {
        let url = format!("{}/v1/{}/backupPlans", self.base_url(), parent,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_backup_plans response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new BackupPlan in a given location.
    ///
    /// **GCP API**: `POST v1/{+parent}/backupPlans`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The location within which to create the BackupPlan. Format: `projects/*/locations/*` *(required)*
    ///
    /// # Query Parameters
    /// - `backupPlanId` — Required. The client-provided short name for the BackupPlan resource. This name must: - be between 1 and 63 characters l
    ///
    /// # Request Body
    /// [`BackupPlan`]
    ///
    /// # Response
    /// [`GkeBackupLro`]
    #[allow(dead_code)]
    pub(crate) async fn create_backup_plan(
        &self,
        parent: &str,
        backup_plan_id: &str,
        body: &BackupPlan,
    ) -> Result<GkeBackupLro> {
        let url = format!("{}/v1/{}/backupPlans", self.base_url(), parent,);
        let url = crate::append_query_params(url, &[("backupPlanId", backup_plan_id)]);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_backup_plan response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes an existing BackupPlan.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. Fully qualified BackupPlan name. Format: `projects/*/locations/*/backupPlans/*` *(required)*
    ///
    /// # Query Parameters
    /// - `etag` — Optional. If provided, this value must match the current value of the target BackupPlan's etag field or the request is r
    ///
    /// # Response
    /// [`GkeBackupLro`]
    #[allow(dead_code)]
    pub(crate) async fn delete_backup_plan(&self, name: &str) -> Result<GkeBackupLro> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_backup_plan response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_backup_plan() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(BackupPlan::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = GkebackupOps::new(&client);

        let result = ops.get_backup_plan("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_backup_plans() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/backupPlans")
            .returning_json(serde_json::to_value(ListBackupPlansResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = GkebackupOps::new(&client);

        let result = ops.list_backup_plans("test-parent").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_backup_plan() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-parent/backupPlans?backupPlanId=test-backupPlanId")
            .returning_json(serde_json::to_value(GkeBackupLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = GkebackupOps::new(&client);

        let body = BackupPlan::fixture();
        let result = ops
            .create_backup_plan("test-parent", "test-backupPlanId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_backup_plan() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(GkeBackupLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = GkebackupOps::new(&client);

        let result = ops.delete_backup_plan("test-name").await;
        assert!(result.is_ok());
    }
}
