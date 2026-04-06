//! MockClient helpers for Cloud Billing API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud Billing helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudbillingMockHelpers {
    /// Helper to expect `get_billing_info`: Gets the billing information for a project. The current
    /// authenticated user must have the `resourcemanager.projects.get` permission for the project,
    /// which can be granted by assigning the [Project
    /// Viewer](https://cloud.google.com/iam/docs/understanding-roles#predefined_roles) role.
    fn expect_get_billing_info(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_billing_info`: Sets or updates the billing account associated with
    /// a project. You specify the new billing account by setting the `billing_account_name` in the
    /// `ProjectBillingInfo` resource to the resource name of a billing account. Associating a
    /// project with an open billing account enables billing on the project and allows charges for
    /// resource usage. If the project already had a billing account, this method changes the
    /// billing account used for resource usage charges. *Note:* Incurred charges that have not yet
    /// been reported in the transaction history of the Google Cloud Console might be billed to the
    /// new billing account, even if the charge occurred before the new billing account was assigned
    /// to the project. The current authenticated user must have ownership privileges for both the
    /// [project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo ) and the
    /// [billing account](https://cloud.google.com/billing/docs/how-to/billing-access). You can
    /// disable billing on the project by setting the `billing_account_name` field to empty. This
    /// action disassociates the current billing account from the project. Any billable activity of
    /// your in-use services will stop, and your application could stop functioning as expected. Any
    /// unbilled charges to date will be billed to the previously associated account. The current
    /// authenticated user must be either an owner of the project or an owner of the billing account
    /// for the project. Note that associating a project with a *closed* billing account will have
    /// much the same effect as disabling billing on the project: any paid resources used by the
    /// project will be shut down. Thus, unless you wish to disable billing, you should always call
    /// this method with the name of an *open* billing account.
    fn expect_update_billing_info(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudbillingMockHelpers for MockClient {
    /// Helper to expect `get_billing_info`: Gets the billing information for a project. The current
    /// authenticated user must have the `resourcemanager.projects.get` permission for the project,
    /// which can be granted by assigning the [Project
    /// Viewer](https://cloud.google.com/iam/docs/understanding-roles#predefined_roles) role.
    fn expect_get_billing_info(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/billingInfo");
        self.expect_get(&path)
    }

    /// Helper to expect `update_billing_info`: Sets or updates the billing account associated with
    /// a project. You specify the new billing account by setting the `billing_account_name` in the
    /// `ProjectBillingInfo` resource to the resource name of a billing account. Associating a
    /// project with an open billing account enables billing on the project and allows charges for
    /// resource usage. If the project already had a billing account, this method changes the
    /// billing account used for resource usage charges. *Note:* Incurred charges that have not yet
    /// been reported in the transaction history of the Google Cloud Console might be billed to the
    /// new billing account, even if the charge occurred before the new billing account was assigned
    /// to the project. The current authenticated user must have ownership privileges for both the
    /// [project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo ) and the
    /// [billing account](https://cloud.google.com/billing/docs/how-to/billing-access). You can
    /// disable billing on the project by setting the `billing_account_name` field to empty. This
    /// action disassociates the current billing account from the project. Any billable activity of
    /// your in-use services will stop, and your application could stop functioning as expected. Any
    /// unbilled charges to date will be billed to the previously associated account. The current
    /// authenticated user must be either an owner of the project or an owner of the billing account
    /// for the project. Note that associating a project with a *closed* billing account will have
    /// much the same effect as disabling billing on the project: any paid resources used by the
    /// project will be shut down. Thus, unless you wish to disable billing, you should always call
    /// this method with the name of an *open* billing account.
    fn expect_update_billing_info(
        &mut self,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}/billingInfo");
        self.expect_put(&path)
    }
}
