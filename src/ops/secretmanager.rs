//! Operation contracts for the Secret Manager API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/secretmanager.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::secretmanager::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Secret Manager API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::secretmanager::SecretmanagerClient`] instead.
pub struct SecretmanagerOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> SecretmanagerOps<'a> {
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
        "https://secretmanager.googleapis.com"
    }

    /// Creates a new Secret containing no SecretVersions.
    ///
    /// **GCP API**: `POST v1/{+parent}/secrets`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the project to associate with the Secret, in the format `projects/*` or `projects/*/locat *(required)*
    ///
    /// # Query Parameters
    /// - `secretId` — Required. This must be unique within the project. A secret ID is a string with a maximum length of 255 characters and ca
    ///
    /// # Request Body
    /// [`Secret`]
    ///
    /// # Response
    /// [`Secret`]
    #[allow(dead_code)]
    pub(crate) async fn create_secret(
        &self,
        parent: &str,
        secret_id: &str,
        body: &Secret,
    ) -> Result<Secret> {
        let url = format!("{}/v1/{}/secrets", self.base_url(), parent,);
        let url = crate::append_query_params(url, &[("secretId", secret_id)]);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets metadata for a given Secret.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the Secret, in the format `projects/*/secrets/*` or `projects/*/locations/*/secrets/*`. *(required)*
    ///
    /// # Response
    /// [`Secret`]
    #[allow(dead_code)]
    pub(crate) async fn get_secret(&self, name: &str) -> Result<Secret> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Lists Secrets.
    ///
    /// **GCP API**: `GET v1/{+parent}/secrets`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource name of the project associated with the Secrets, in the format `projects/*` or `projects/*/locati *(required)*
    ///
    /// # Query Parameters
    /// - `filter` — Optional. Filter string, adhering to the rules in [List-operation filtering](https://cloud.google.com/secret-manager/doc
    /// - `pageSize` — Optional. The maximum number of results to be returned in a single page. If set to 0, the server decides the number of r
    /// - `pageToken` — Optional. Pagination token, returned earlier via ListSecretsResponse.next_page_token.
    ///
    /// # Response
    /// [`ListSecretsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_secrets(
        &self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListSecretsResponse> {
        let url = format!("{}/v1/{}/secrets", self.base_url(), parent,);
        let url = crate::append_query_params(
            url,
            &[
                ("filter", filter),
                ("pageSize", page_size),
                ("pageToken", page_token),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_secrets response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Updates metadata of an existing Secret.
    ///
    /// **GCP API**: `PATCH v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Output only. The resource name of the Secret in the format `projects/*/secrets/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `updateMask` — Required. Specifies the fields to be updated.
    ///
    /// # Request Body
    /// [`Secret`]
    ///
    /// # Response
    /// [`Secret`]
    #[allow(dead_code)]
    pub(crate) async fn patch_secret(
        &self,
        name: &str,
        update_mask: &str,
        body: &Secret,
    ) -> Result<Secret> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let url = crate::append_query_params(url, &[("updateMask", update_mask)]);
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a Secret.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The resource name of the Secret to delete in the format `projects/*/secrets/*`. *(required)*
    ///
    /// # Query Parameters
    /// - `etag` — Optional. Etag of the Secret. The request succeeds if it matches the etag of the currently stored secret object. If the
    ///
    /// # Response
    /// [`Empty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_secret(&self, name: &str, etag: &str) -> Result<Empty> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let url = crate::append_query_params(url, &[("etag", etag)]);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-parent/secrets?secretId=test-secretId")
            .returning_json(serde_json::to_value(Secret::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SecretmanagerOps::new(&client);

        let body = Secret::fixture();
        let result = ops
            .create_secret("test-parent", "test-secretId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(Secret::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SecretmanagerOps::new(&client);

        let result = ops.get_secret("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/secrets?filter=test-filter&pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListSecretsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SecretmanagerOps::new(&client);

        let result = ops
            .list_secrets(
                "test-parent",
                "test-filter",
                "test-pageSize",
                "test-pageToken",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/v1/test-name?updateMask=test-updateMask")
            .returning_json(serde_json::to_value(Secret::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SecretmanagerOps::new(&client);

        let body = Secret::fixture();
        let result = ops
            .patch_secret("test-name", "test-updateMask", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name?etag=test-etag")
            .returning_json(serde_json::to_value(Empty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = SecretmanagerOps::new(&client);

        let result = ops.delete_secret("test-name", "test-etag").await;
        assert!(result.is_ok());
    }
}
