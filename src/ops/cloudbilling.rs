//! Operation contracts for the Cloud Billing API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cloudbilling.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cloudbilling::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Cloud Billing API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cloudbilling::CloudbillingClient`] instead.
pub struct CloudbillingOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> CloudbillingOps<'a> {
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
        "https://cloudbilling.googleapis.com"
    }

    /// Gets the billing information for a project. The current authenticated user must have the
    /// `resourcemanager.projects.get` permission for the project, which can be granted by
    /// assigning the [Project Viewer](https://cloud.google.com/iam/docs/understanding-
    /// roles#predefined_roles) role.
    ///
    /// **GCP API**: `GET v1/{+name}/billingInfo`
    /// **Reference**: <https://cloud.google.com/billing/docs/apis/projects/getBillingInfo>
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the project for which billing information is retrieved. For example, `projects/tokyo-rain *(required)*
    ///
    /// # Response
    /// [`ProjectBillingInfo`]
    #[allow(dead_code)]
    pub(crate) async fn get_billing_info(&self, name: &str) -> Result<ProjectBillingInfo> {
        let url = format!("{}/v1/{}/billingInfo", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_billing_info response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Sets or updates the billing account associated with a project. You specify the new
    /// billing account by setting the `billing_account_name` in the `ProjectBillingInfo`
    /// resource to the resource name of a billing account. Associating a project with an open
    /// billing account enables billing on the project and allows charges for resource usage. If
    /// the project already had a billing account, this method changes the billing account used
    /// for resource usage charges. *Note:* Incurred charges that have not yet been reported in
    /// the transaction history of the Google Cloud Console might be billed to the new billing
    /// account, even if the charge occurred before the new billing account was assigned to the
    /// project. The current authenticated user must have ownership privileges for both the
    /// [project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo ) and the
    /// [billing account](https://cloud.google.com/billing/docs/how-to/billing-access). You can
    /// disable billing on the project by setting the `billing_account_name` field to empty.
    /// This action disassociates the current billing account from the project. Any billable
    /// activity of your in-use services will stop, and your application could stop functioning
    /// as expected. Any unbilled charges to date will be billed to the previously associated
    /// account. The current authenticated user must be either an owner of the project or an
    /// owner of the billing account for the project. Note that associating a project with a
    /// *closed* billing account will have much the same effect as disabling billing on the
    /// project: any paid resources used by the project will be shut down. Thus, unless you wish
    /// to disable billing, you should always call this method with the name of an *open*
    /// billing account.
    ///
    /// **GCP API**: `PUT v1/{+name}/billingInfo`
    /// **Reference**: <https://cloud.google.com/billing/docs/apis/projects/updateBillingInfo>
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the project associated with the billing information that you want to update. For example, *(required)*
    ///
    /// # Request Body
    /// [`ProjectBillingInfo`]
    ///
    /// # Response
    /// [`ProjectBillingInfo`]
    #[allow(dead_code)]
    pub(crate) async fn update_billing_info(
        &self,
        name: &str,
        body: &ProjectBillingInfo,
    ) -> Result<ProjectBillingInfo> {
        let url = format!("{}/v1/{}/billingInfo", self.base_url(), name,);
        let response = self.client.put(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse update_billing_info response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_billing_info() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name/billingInfo")
            .returning_json(serde_json::to_value(ProjectBillingInfo::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudbillingOps::new(&client);

        let result = ops.get_billing_info("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_billing_info() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/v1/test-name/billingInfo")
            .returning_json(serde_json::to_value(ProjectBillingInfo::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = CloudbillingOps::new(&client);

        let body = ProjectBillingInfo::fixture();
        let result = ops.update_billing_info("test-name", &body).await;
        assert!(result.is_ok());
    }
}
