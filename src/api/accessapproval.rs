//! Access Approval API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::accessapproval::AccessapprovalOps`. This layer adds ergonomic
//! method signatures (project/folder/org IDs instead of raw resource paths).
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 2.15 (logging_access_approval): verify Access Approval is enrolled
//!     for at least one service on the org/project.

use crate::{
    GcpHttpClient, Result, ops::accessapproval::AccessapprovalOps,
    types::accessapproval::AccessApprovalSettings,
};

/// Client for the Access Approval API.
pub struct AccessApprovalClient<'a> {
    ops: AccessapprovalOps<'a>,
}

impl<'a> AccessApprovalClient<'a> {
    /// Create a new Access Approval client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: AccessapprovalOps::new(client),
        }
    }

    // ── Settings ──────────────────────────────────────────────────────────

    /// Get the Access Approval settings for a project.
    ///
    /// Returns the enrolled services, notification emails, and other settings.
    /// If Access Approval has never been configured, returns a default settings
    /// object with an empty `enrolledServices` list.
    pub async fn get_project_settings(&self, project: &str) -> Result<AccessApprovalSettings> {
        let name = format!("projects/{}/accessApprovalSettings", project);
        self.ops.get_access_approval_settings_project(&name).await
    }

    /// Get the Access Approval settings for a folder.
    pub async fn get_folder_settings(&self, folder: &str) -> Result<AccessApprovalSettings> {
        let name = format!("folders/{}/accessApprovalSettings", folder);
        self.ops.get_access_approval_settings_folder(&name).await
    }

    /// Get the Access Approval settings for an organization.
    pub async fn get_org_settings(&self, org_id: &str) -> Result<AccessApprovalSettings> {
        let name = format!("organizations/{}/accessApprovalSettings", org_id);
        self.ops.get_access_approval_settings_org(&name).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_get_project_settings() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/accessApprovalSettings")
            .returning_json(json!({
                "name": "projects/my-project/accessApprovalSettings",
                "notificationEmails": ["security@example.com"],
                "enrolledServices": [
                    {"cloudProduct": "all", "enrollmentLevel": "BLOCK_ALL"}
                ],
                "enrolledAncestor": false
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let aa = client.access_approval();
        let result = aa.get_project_settings("my-project").await;
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.name, "projects/my-project/accessApprovalSettings");
        assert_eq!(settings.notification_emails, vec!["security@example.com"]);
        assert_eq!(settings.enrolled_services.len(), 1);
        assert_eq!(
            settings.enrolled_services[0].cloud_product.as_deref(),
            Some("all")
        );
        assert_eq!(settings.enrolled_ancestor, Some(false));
    }

    #[tokio::test]
    async fn test_get_org_settings() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/organizations/123456789/accessApprovalSettings")
            .returning_json(json!({
                "name": "organizations/123456789/accessApprovalSettings",
                "enrolledServices": [],
                "notificationEmails": []
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let aa = client.access_approval();
        let result = aa.get_org_settings("123456789").await;
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(
            settings.name,
            "organizations/123456789/accessApprovalSettings"
        );
        assert!(settings.enrolled_services.is_empty());
    }

    #[tokio::test]
    async fn test_get_folder_settings() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/folders/987654321/accessApprovalSettings")
            .returning_json(json!({
                "name": "folders/987654321/accessApprovalSettings",
                "enrolledServices": [],
                "notificationEmails": [],
                "enrolledAncestor": true
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let aa = client.access_approval();
        let result = aa.get_folder_settings("987654321").await;
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.name, "folders/987654321/accessApprovalSettings");
        assert_eq!(settings.enrolled_ancestor, Some(true));
    }
}
