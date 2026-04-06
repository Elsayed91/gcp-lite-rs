//! Types for the Cloud Billing API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/cloudbilling/v1/rest`

use serde::{Deserialize, Serialize};

/// Encapsulation of billing information for a Google Cloud Console project. A project has at
/// most one associated billing account at a time (but a billing account can be assigned to
/// multiple projects).
///
/// **GCP API**: `cloudbilling.v1.ProjectBillingInfo`
/// **Reference**: <https://cloud.google.com/billing/docs/apis/ProjectBillingInfo>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBillingInfo {
    /// The resource name of the billing account associated with the project, if any. For
    /// example, `billingAccounts/012345-567890-ABCDEF`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,

    /// Output only. True if the project is associated with an open billing account, to which
    /// usage on the project is charged. False if the project is associated with a closed
    /// billing account, or no billing account at all, and therefore cannot use paid services.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_enabled: Option<bool>,

    /// Output only. The resource name for the `ProjectBillingInfo`; has the form
    /// `projects/{project_id}/billingInfo`. For example, the resource name for the billing
    /// information for project `tokyo-rain-123` would be `projects/tokyo-rain-123/billingInfo`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// Output only. The ID of the project that this `ProjectBillingInfo` represents, such as
    /// `tokyo-rain-123`. This is a convenience field so that you don't need to parse the `name`
    /// field to obtain a project ID.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ProjectBillingInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            billing_account_name: Some("test-billing_account_name".into()),
            billing_enabled: Some(false),
            name: "test-project_billing_info".into(),
            project_id: Some("test-project_id".into()),
        }
    }
}
