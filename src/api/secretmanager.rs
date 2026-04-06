//! Secret Manager API client.
//!
//! Provides methods for managing secrets and their metadata.
//!
//! # Examples
//!
//! ```no_run
//! use gcp_lite::GcpHttpClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = GcpHttpClient::from_adc().await?;
//! let secrets = client.secret_manager();
//!
//! // List secrets
//! let response = secrets.list_secrets("my-project", None, None, None).await?;
//! for secret in response.secrets {
//!     println!("Secret: {}", secret.name);
//! }
//! # Ok(())
//! # }
//! ```

use crate::{
    GcpHttpClient, Result,
    ops::secretmanager::SecretmanagerOps,
    types::secretmanager::{Empty, ListSecretsResponse, Secret},
};

/// Client for the Secret Manager API
pub struct SecretManagerClient<'a> {
    ops: SecretmanagerOps<'a>,
}

impl<'a> SecretManagerClient<'a> {
    /// Create a new Secret Manager API client
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: SecretmanagerOps::new(client),
        }
    }

    // ── Secrets ──────────────────────────────────────────────────────────────

    /// Creates a new Secret containing no SecretVersions.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID
    /// * `secret_id` - The ID to assign to the secret (not the full resource name)
    /// * `secret` - The secret metadata to create
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::{GcpHttpClient, types::secretmanager::Secret};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let secrets = client.secret_manager();
    ///
    /// let secret = Secret {
    ///     labels: [("env".to_string(), "prod".to_string())].into(),
    ///     ..Default::default()
    /// };
    ///
    /// let created = secrets.create_secret("my-project", "my-secret", &secret).await?;
    /// println!("Created: {}", created.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_secret(
        &self,
        project: &str,
        secret_id: &str,
        secret: &Secret,
    ) -> Result<Secret> {
        let parent = format!("projects/{}", project);
        self.ops.create_secret(&parent, secret_id, secret).await
    }

    /// Gets metadata for a given Secret.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID
    /// * `secret` - The secret ID (not the full resource name)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gcp_lite::GcpHttpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let secrets = client.secret_manager();
    ///
    /// let secret = secrets.get_secret("my-project", "my-secret").await?;
    /// println!("Secret: {:?}", secret.labels);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_secret(&self, project: &str, secret: &str) -> Result<Secret> {
        let name = format!("projects/{}/secrets/{}", project, secret);
        self.ops.get_secret(&name).await
    }

    /// Lists Secrets.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID
    /// * `filter` - Optional filter expression
    /// * `page_size` - Optional page size
    /// * `page_token` - Optional page token for pagination
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gcp_lite::GcpHttpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let secrets = client.secret_manager();
    ///
    /// let response = secrets.list_secrets("my-project", None, Some("10"), None).await?;
    /// for secret in response.secrets {
    ///     println!("Secret: {}", secret.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_secrets(
        &self,
        project: &str,
        filter: Option<&str>,
        page_size: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<ListSecretsResponse> {
        let parent = format!("projects/{}", project);
        self.ops
            .list_secrets(
                &parent,
                filter.unwrap_or(""),
                page_size.unwrap_or(""),
                page_token.unwrap_or(""),
            )
            .await
    }

    /// Updates metadata of an existing Secret.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID
    /// * `secret` - The secret ID (not the full resource name)
    /// * `update_mask` - Field mask specifying which fields to update
    /// * `secret_data` - The secret metadata with updated fields
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::{GcpHttpClient, types::secretmanager::Secret};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let secrets = client.secret_manager();
    ///
    /// let updated = Secret {
    ///     labels: [("env".to_string(), "staging".to_string())].into(),
    ///     ..Default::default()
    /// };
    ///
    /// let result = secrets
    ///     .patch_secret("my-project", "my-secret", "labels", &updated)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn patch_secret(
        &self,
        project: &str,
        secret: &str,
        update_mask: &str,
        secret_data: &Secret,
    ) -> Result<Secret> {
        let name = format!("projects/{}/secrets/{}", project, secret);
        self.ops.patch_secret(&name, update_mask, secret_data).await
    }

    /// Deletes a Secret.
    ///
    /// # Arguments
    ///
    /// * `project` - The GCP project ID
    /// * `secret` - The secret ID (not the full resource name)
    /// * `etag` - Optional etag for optimistic concurrency control
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gcp_lite::GcpHttpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GcpHttpClient::from_adc().await?;
    /// let secrets = client.secret_manager();
    ///
    /// secrets.delete_secret("my-project", "my-secret", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_secret(
        &self,
        project: &str,
        secret: &str,
        etag: Option<&str>,
    ) -> Result<Empty> {
        let name = format!("projects/{}/secrets/{}", project, secret);
        self.ops.delete_secret(&name, etag.unwrap_or("")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use crate::test_support::SecretmanagerMockHelpers;
    use crate::types::secretmanager::{Automatic, Replication};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_create_secret() {
        let mut mock = MockClient::new();
        mock.expect_create_secret("projects/my-project", "test-secret")
            .returning_json(serde_json::json!({
                "name": "projects/12345/secrets/test-secret",
                "labels": {"env": "test"},
                "replication": {"automatic": {}}
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let replication = Replication {
            automatic: Some(Automatic::default()),
            ..Default::default()
        };
        let mut labels = HashMap::new();
        labels.insert("env".to_string(), "test".to_string());

        let secret = Secret {
            labels,
            replication: Some(replication),
            ..Default::default()
        };

        let result = secrets
            .create_secret("my-project", "test-secret", &secret)
            .await;
        assert!(result.is_ok());
        let created = result.unwrap();
        assert!(created.name.contains("test-secret"));
        assert_eq!(created.labels.get("env").map(|s| s.as_str()), Some("test"));
    }

    #[tokio::test]
    async fn test_get_secret() {
        let mut mock = MockClient::new();
        mock.expect_get_secret("projects/my-project/secrets/test-secret")
            .returning_json(serde_json::json!({
                "name": "projects/12345/secrets/test-secret",
                "labels": {"env": "prod"}
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let result = secrets.get_secret("my-project", "test-secret").await;
        assert!(result.is_ok());
        let secret = result.unwrap();
        assert!(secret.name.contains("test-secret"));
        assert_eq!(secret.labels.get("env").map(|s| s.as_str()), Some("prod"));
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let mut mock = MockClient::new();
        mock.expect_list_secrets("projects/my-project", "", "", "")
            .returning_json(serde_json::json!({
                "secrets": [
                    {"name": "projects/12345/secrets/secret1"},
                    {"name": "projects/12345/secrets/secret2"}
                ],
                "totalSize": 2
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let result = secrets.list_secrets("my-project", None, None, None).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.secrets.len(), 2);
        assert!(response.secrets[0].name.contains("secret1"));
    }

    #[tokio::test]
    async fn test_patch_secret() {
        let mut mock = MockClient::new();
        mock.expect_patch_secret("projects/my-project/secrets/test-secret", "labels")
            .returning_json(serde_json::json!({
                "name": "projects/12345/secrets/test-secret",
                "labels": {"env": "staging"}
            }))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let mut labels = HashMap::new();
        labels.insert("env".to_string(), "staging".to_string());
        let update = Secret {
            labels,
            ..Default::default()
        };

        let result = secrets
            .patch_secret("my-project", "test-secret", "labels", &update)
            .await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(
            updated.labels.get("env").map(|s| s.as_str()),
            Some("staging")
        );
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let mut mock = MockClient::new();
        mock.expect_delete_secret("projects/my-project/secrets/test-secret", "")
            .returning_json(serde_json::json!({}))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let result = secrets
            .delete_secret("my-project", "test-secret", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_secret_with_etag() {
        let mut mock = MockClient::new();
        mock.expect_delete_secret("projects/my-project/secrets/test-secret", "abc123")
            .returning_json(serde_json::json!({}))
            .times(1);

        let client = GcpHttpClient::from_mock(mock);
        let secrets = client.secret_manager();

        let result = secrets
            .delete_secret("my-project", "test-secret", Some("abc123"))
            .await;
        assert!(result.is_ok());
    }
}
