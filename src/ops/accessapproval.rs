//! Operation contracts for the Access Approval API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/accessapproval.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::accessapproval::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Access Approval API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::accessapproval::AccessapprovalClient`] instead.
pub struct AccessapprovalOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> AccessapprovalOps<'a> {
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
        "https://accessapproval.googleapis.com"
    }

    /// Gets the Access Approval settings associated with a project, folder, or organization.
    ///
    /// **GCP API**: `GET v1/{+name}`
    /// **Reference**: <https://cloud.google.com/assured-workloads/access-approval/docs/projects/getAccessApprovalSettings>
    ///
    /// # Path Parameters
    /// - `name` — The name of the AccessApprovalSettings to retrieve. Format: "{projects|folders|organizations}/{id}/accessApprovalSetting *(required)*
    ///
    /// # Response
    /// [`AccessApprovalSettings`]
    #[allow(dead_code)]
    pub(crate) async fn get_access_approval_settings_project(
        &self,
        name: &str,
    ) -> Result<AccessApprovalSettings> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_access_approval_settings_project response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the Access Approval settings associated with a project, folder, or organization.
    ///
    /// **GCP API**: `GET v1/{+name}`
    /// **Reference**: <https://cloud.google.com/assured-workloads/access-approval/docs/folders/getAccessApprovalSettings>
    ///
    /// # Path Parameters
    /// - `name` — The name of the AccessApprovalSettings to retrieve. Format: "{projects|folders|organizations}/{id}/accessApprovalSetting *(required)*
    ///
    /// # Response
    /// [`AccessApprovalSettings`]
    #[allow(dead_code)]
    pub(crate) async fn get_access_approval_settings_folder(
        &self,
        name: &str,
    ) -> Result<AccessApprovalSettings> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_access_approval_settings_folder response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the Access Approval settings associated with a project, folder, or organization.
    ///
    /// **GCP API**: `GET v1/{+name}`
    /// **Reference**: <https://cloud.google.com/assured-workloads/access-approval/docs/organizations/getAccessApprovalSettings>
    ///
    /// # Path Parameters
    /// - `name` — The name of the AccessApprovalSettings to retrieve. Format: "{projects|folders|organizations}/{id}/accessApprovalSetting *(required)*
    ///
    /// # Response
    /// [`AccessApprovalSettings`]
    #[allow(dead_code)]
    pub(crate) async fn get_access_approval_settings_org(
        &self,
        name: &str,
    ) -> Result<AccessApprovalSettings> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_access_approval_settings_org response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_access_approval_settings_project() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(AccessApprovalSettings::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AccessapprovalOps::new(&client);

        let result = ops.get_access_approval_settings_project("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_approval_settings_folder() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(AccessApprovalSettings::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AccessapprovalOps::new(&client);

        let result = ops.get_access_approval_settings_folder("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_access_approval_settings_org() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(AccessApprovalSettings::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = AccessapprovalOps::new(&client);

        let result = ops.get_access_approval_settings_org("test-name").await;
        assert!(result.is_ok());
    }
}
