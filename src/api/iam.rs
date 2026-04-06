//! IAM API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::iam::IamOps`. This layer adds:
//! - Ergonomic method signatures (project/email instead of raw resource names)

use crate::{
    GcpHttpClient, Result,
    ops::iam::IamOps,
    types::iam::{
        CreateServiceAccountKeyRequest, CreateServiceAccountRequest,
        ListServiceAccountKeysResponse, ListServiceAccountsResponse, ServiceAccount,
        ServiceAccountKey,
    },
};

/// Client for the IAM API
pub struct IamClient<'a> {
    ops: IamOps<'a>,
}

impl<'a> IamClient<'a> {
    /// Create a new IAM API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: IamOps::new(client),
        }
    }

    // ── Service Accounts ─────────────────────────────────────────────

    /// Create a service account in a project.
    pub async fn create_service_account(
        &self,
        project: &str,
        account_id: &str,
        display_name: &str,
        description: &str,
    ) -> Result<ServiceAccount> {
        let name = format!("projects/{}", project);
        let body = CreateServiceAccountRequest {
            account_id: account_id.to_string(),
            service_account: Some(ServiceAccount {
                display_name: Some(display_name.to_string()),
                description: Some(description.to_string()),
                ..Default::default()
            }),
        };
        self.ops.create_service_account(&name, &body).await
    }

    /// Get a service account by project and email.
    pub async fn get_service_account(&self, project: &str, email: &str) -> Result<ServiceAccount> {
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        self.ops.get_service_account(&name).await
    }

    /// List all service accounts in a project.
    pub async fn list_service_accounts(
        &self,
        project: &str,
    ) -> Result<ListServiceAccountsResponse> {
        let name = format!("projects/{}", project);
        self.ops.list_service_accounts(&name).await
    }

    /// Delete a service account by project and email.
    pub async fn delete_service_account(&self, project: &str, email: &str) -> Result<()> {
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        self.ops.delete_service_account(&name).await?;
        Ok(())
    }

    // ── Service Account Keys ─────────────────────────────────────────

    /// Create a key for a service account.
    pub async fn create_service_account_key(
        &self,
        project: &str,
        email: &str,
        body: &CreateServiceAccountKeyRequest,
    ) -> Result<ServiceAccountKey> {
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        self.ops.create_service_account_key(&name, body).await
    }

    /// List all keys for a service account.
    pub async fn list_service_account_keys(
        &self,
        project: &str,
        email: &str,
    ) -> Result<ListServiceAccountKeysResponse> {
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        self.ops.list_service_account_keys(&name).await
    }

    /// Delete a service account key.
    pub async fn delete_service_account_key(
        &self,
        project: &str,
        email: &str,
        key_id: &str,
    ) -> Result<()> {
        let name = format!(
            "projects/{}/serviceAccounts/{}/keys/{}",
            project, email, key_id
        );
        self.ops.delete_service_account_key(&name).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_create_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/serviceAccounts")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/serviceAccounts/test-sa@test-project.iam.gserviceaccount.com",
                "email": "test-sa@test-project.iam.gserviceaccount.com",
                "displayName": "Test SA",
                "description": "A test service account",
                "projectId": "test-project",
                "uniqueId": "123456789"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam
            .create_service_account(
                "test-project",
                "test-sa",
                "Test SA",
                "A test service account",
            )
            .await;

        assert!(result.is_ok());
        let sa = result.unwrap();
        assert_eq!(
            sa.email,
            Some("test-sa@test-project.iam.gserviceaccount.com".to_string())
        );
        assert_eq!(sa.display_name, Some("Test SA".to_string()));
    }

    #[tokio::test]
    async fn test_get_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/v1/projects/my-project/serviceAccounts/sa@my-project.iam.gserviceaccount.com",
        )
        .returning_json(serde_json::json!({
            "name": "projects/my-project/serviceAccounts/sa@my-project.iam.gserviceaccount.com",
            "email": "sa@my-project.iam.gserviceaccount.com",
            "displayName": "My SA",
            "projectId": "my-project",
            "uniqueId": "987654321"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam
            .get_service_account("my-project", "sa@my-project.iam.gserviceaccount.com")
            .await;

        assert!(result.is_ok());
        let sa = result.unwrap();
        assert_eq!(sa.display_name, Some("My SA".to_string()));
    }

    #[tokio::test]
    async fn test_list_service_accounts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/serviceAccounts")
            .returning_json(serde_json::json!({
                "accounts": [
                    {
                        "name": "projects/test-project/serviceAccounts/sa1@test-project.iam.gserviceaccount.com",
                        "email": "sa1@test-project.iam.gserviceaccount.com",
                        "displayName": "SA One"
                    },
                    {
                        "name": "projects/test-project/serviceAccounts/sa2@test-project.iam.gserviceaccount.com",
                        "email": "sa2@test-project.iam.gserviceaccount.com",
                        "displayName": "SA Two"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam.list_service_accounts("test-project").await;

        assert!(result.is_ok());
        let list = result.unwrap();
        assert_eq!(list.accounts.len(), 2);
        assert_eq!(list.accounts[0].display_name, Some("SA One".to_string()));
    }

    #[tokio::test]
    async fn test_delete_service_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete(
            "/v1/projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com",
        )
        .returning_json(serde_json::json!({}))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam
            .delete_service_account("test-project", "sa@test-project.iam.gserviceaccount.com")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_service_account_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys/abc123",
                "keyAlgorithm": "KEY_ALG_RSA_2048",
                "keyType": "USER_MANAGED",
                "privateKeyData": "base64encodedkey=="
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let body = crate::types::iam::CreateServiceAccountKeyRequest::default();
        let result = iam
            .create_service_account_key(
                "test-project",
                "sa@test-project.iam.gserviceaccount.com",
                &body,
            )
            .await;

        assert!(result.is_ok());
        let key = result.unwrap();
        assert_eq!(key.key_algorithm, Some("KEY_ALG_RSA_2048".to_string()));
        assert_eq!(key.private_key_data, Some("base64encodedkey==".to_string()));
    }

    #[tokio::test]
    async fn test_list_service_account_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys")
            .returning_json(serde_json::json!({
                "keys": [
                    {
                        "name": "projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys/key1",
                        "keyType": "USER_MANAGED"
                    },
                    {
                        "name": "projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys/key2",
                        "keyType": "SYSTEM_MANAGED"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam
            .list_service_account_keys("test-project", "sa@test-project.iam.gserviceaccount.com")
            .await;

        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.keys.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_service_account_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/projects/test-project/serviceAccounts/sa@test-project.iam.gserviceaccount.com/keys/abc123")
            .returning_json(serde_json::json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        let result = iam
            .delete_service_account_key(
                "test-project",
                "sa@test-project.iam.gserviceaccount.com",
                "abc123",
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_account_lifecycle() {
        let mut mock = crate::MockClient::new();

        // Create
        mock.expect_post("/v1/projects/test-project/serviceAccounts")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/serviceAccounts/lifecycle-sa@test-project.iam.gserviceaccount.com",
                "email": "lifecycle-sa@test-project.iam.gserviceaccount.com",
                "displayName": "Lifecycle SA",
                "description": "Testing lifecycle"
            }))
            .times(1);

        // Get
        mock.expect_get("/v1/projects/test-project/serviceAccounts/lifecycle-sa@test-project.iam.gserviceaccount.com")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/serviceAccounts/lifecycle-sa@test-project.iam.gserviceaccount.com",
                "email": "lifecycle-sa@test-project.iam.gserviceaccount.com",
                "displayName": "Lifecycle SA"
            }))
            .times(1);

        // Delete
        mock.expect_delete("/v1/projects/test-project/serviceAccounts/lifecycle-sa@test-project.iam.gserviceaccount.com")
            .returning_json(serde_json::json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let iam = client.iam();

        // Create
        let sa = iam
            .create_service_account(
                "test-project",
                "lifecycle-sa",
                "Lifecycle SA",
                "Testing lifecycle",
            )
            .await
            .unwrap();
        let email = sa.email.unwrap();

        // Get
        let fetched = iam
            .get_service_account("test-project", &email)
            .await
            .unwrap();
        assert_eq!(fetched.display_name, Some("Lifecycle SA".to_string()));

        // Delete
        iam.delete_service_account("test-project", &email)
            .await
            .unwrap();
    }
}
