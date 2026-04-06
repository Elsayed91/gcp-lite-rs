//! Operation contracts for the IAM API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/iam.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::iam::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the IAM API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::iam::IamClient`] instead.
pub struct IamOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> IamOps<'a> {
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
        "https://iam.googleapis.com"
    }

    /// Creates a ServiceAccount.
    ///
    /// **GCP API**: `POST v1/{+name}/serviceAccounts`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the project associated with the service accounts, such as `projects/my-project-123`. *(required)*
    ///
    /// # Request Body
    /// [`CreateServiceAccountRequest`]
    ///
    /// # Response
    /// [`ServiceAccount`]
    #[allow(dead_code)]
    pub(crate) async fn create_service_account(
        &self,
        name: &str,
        body: &CreateServiceAccountRequest,
    ) -> Result<ServiceAccount> {
        let url = format!("{}/v1/{}/serviceAccounts", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_service_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a ServiceAccount.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAc *(required)*
    ///
    /// # Response
    /// [`ServiceAccount`]
    #[allow(dead_code)]
    pub(crate) async fn get_service_account(&self, name: &str) -> Result<ServiceAccount> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_service_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists every ServiceAccount that belongs to a specific project.
    ///
    /// **GCP API**: `GET v1/{+name}/serviceAccounts`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the project associated with the service accounts, such as `projects/my-project-123`. *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional limit on the number of service accounts to include in the response. Further accounts can subsequently be obtain
    /// - `pageToken` — Optional pagination token returned in an earlier ListServiceAccountsResponse.next_page_token.
    ///
    /// # Response
    /// [`ListServiceAccountsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_service_accounts(
        &self,
        name: &str,
    ) -> Result<ListServiceAccountsResponse> {
        let url = format!("{}/v1/{}/serviceAccounts", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_service_accounts response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a ServiceAccount. **Warning:** After you delete a service account, you might not
    /// be able to undelete it. If you know that you need to re-enable the service account in
    /// the future, use DisableServiceAccount instead. If you delete a service account, IAM
    /// permanently removes the service account 30 days later. Google Cloud cannot recover the
    /// service account after it is permanently removed, even if you file a support request. To
    /// help avoid unplanned outages, we recommend that you disable the service account before
    /// you delete it. Use DisableServiceAccount to disable the service account, then wait at
    /// least 24 hours and watch for unintended consequences. If there are no unintended
    /// consequences, you can delete the service account.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAc *(required)*
    ///
    /// # Response
    /// [`IamEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_service_account(&self, name: &str) -> Result<IamEmpty> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_service_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a ServiceAccountKey.
    ///
    /// **GCP API**: `POST v1/{+name}/keys`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAc *(required)*
    ///
    /// # Request Body
    /// [`CreateServiceAccountKeyRequest`]
    ///
    /// # Response
    /// [`ServiceAccountKey`]
    #[allow(dead_code)]
    pub(crate) async fn create_service_account_key(
        &self,
        name: &str,
        body: &CreateServiceAccountKeyRequest,
    ) -> Result<ServiceAccountKey> {
        let url = format!("{}/v1/{}/keys", self.base_url(), name,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_service_account_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists every ServiceAccountKey for a service account.
    ///
    /// **GCP API**: `GET v1/{+name}/keys`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAc *(required)*
    ///
    /// # Query Parameters
    /// - `keyTypes` — Filters the types of keys the user wants to include in the list response. Duplicate key types are not allowed. If no key
    ///
    /// # Response
    /// [`ListServiceAccountKeysResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_service_account_keys(
        &self,
        name: &str,
    ) -> Result<ListServiceAccountKeysResponse> {
        let url = format!("{}/v1/{}/keys", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_service_account_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a ServiceAccountKey. Deleting a service account key does not revoke short-lived
    /// credentials that have been issued based on the service account key.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the service account key. Use one of the following formats: * `projects/{PROJECT_ID}/servi *(required)*
    ///
    /// # Response
    /// [`IamEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_service_account_key(&self, name: &str) -> Result<IamEmpty> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_service_account_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name/serviceAccounts")
            .returning_json(serde_json::to_value(ServiceAccount::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = CreateServiceAccountRequest::fixture();
        let result = ops.create_service_account("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(ServiceAccount::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.get_service_account("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_service_accounts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name/serviceAccounts")
            .returning_json(serde_json::to_value(ListServiceAccountsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.list_service_accounts("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(IamEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.delete_service_account("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_service_account_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-name/keys")
            .returning_json(serde_json::to_value(ServiceAccountKey::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let body = CreateServiceAccountKeyRequest::fixture();
        let result = ops.create_service_account_key("test-name", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_service_account_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name/keys").returning_json(
            serde_json::to_value(ListServiceAccountKeysResponse::fixture()).unwrap(),
        );

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.list_service_account_keys("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_service_account_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(IamEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = IamOps::new(&client);

        let result = ops.delete_service_account_key("test-name").await;
        assert!(result.is_ok());
    }
}
