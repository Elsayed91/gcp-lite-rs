//! Cloud Billing API client.
//!
//! Provides methods to query and manage billing accounts for GCP projects.

use crate::types::cloudbilling::*;
use crate::{GcpHttpClient, Result, ops::cloudbilling::CloudbillingOps};

/// Cloud Billing API client.
pub struct BillingClient<'a> {
    ops: CloudbillingOps<'a>,
}

impl<'a> BillingClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: CloudbillingOps::new(client),
        }
    }

    // ── Project Billing Info ─────────────────────────────────────────────────

    /// Gets the billing information for a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID (e.g., "my-project").
    pub async fn get_billing_info(&self, project: &str) -> Result<ProjectBillingInfo> {
        let name = format!("projects/{}", project);
        self.ops.get_billing_info(&name).await
    }

    /// Sets or updates the billing account associated with a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID.
    /// * `billing_account` - The billing account ID (e.g. "012345-567890-ABCDEF") or None to disable billing.
    pub async fn update_billing_info(
        &self,
        project: &str,
        billing_account: Option<&str>,
    ) -> Result<ProjectBillingInfo> {
        let name = format!("projects/{}", project);
        // "billingAccounts/{id}" or empty string to disable
        let billing_account_name = billing_account
            .map(|id| format!("billingAccounts/{}", id))
            .unwrap_or_default();

        let body = ProjectBillingInfo {
            name: format!("{}/billingInfo", name),
            billing_account_name: Some(billing_account_name),
            ..Default::default()
        };

        self.ops.update_billing_info(&name, &body).await
    }
}
