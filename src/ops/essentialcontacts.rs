//! Operation contracts for the Essential Contacts API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/essentialcontacts.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::essentialcontacts::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Essential Contacts API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::essentialcontacts::EssentialcontactsClient`] instead.
pub struct EssentialcontactsOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> EssentialcontactsOps<'a> {
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
        "https://essentialcontacts.googleapis.com"
    }

    /// Lists the contacts that have been set on a resource.
    ///
    /// **GCP API**: `GET v1/{+parent}/contacts`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The parent resource name. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id *(required)*
    ///
    /// # Query Parameters
    /// - `pageSize` — Optional. The maximum number of results to return from this request. Non-positive values are ignored. The presence of `n
    /// - `pageToken` — Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be t
    ///
    /// # Response
    /// [`ListContactsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_contacts(
        &self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> Result<ListContactsResponse> {
        let url = format!("{}/v1/{}/contacts", self.base_url(), parent,);
        let url =
            crate::append_query_params(url, &[("pageSize", page_size), ("pageToken", page_token)]);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_contacts response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets a single contact.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the contact to retrieve. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{f *(required)*
    ///
    /// # Response
    /// [`EssentialContact`]
    #[allow(dead_code)]
    pub(crate) async fn get_contact(&self, name: &str) -> Result<EssentialContact> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_contact response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Adds a new contact for a resource.
    ///
    /// **GCP API**: `POST v1/{+parent}/contacts`
    ///
    /// # Path Parameters
    /// - `parent` — Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or project *(required)*
    ///
    /// # Request Body
    /// [`EssentialContact`]
    ///
    /// # Response
    /// [`EssentialContact`]
    #[allow(dead_code)]
    pub(crate) async fn create_contact(
        &self,
        parent: &str,
        body: &EssentialContact,
    ) -> Result<EssentialContact> {
        let url = format!("{}/v1/{}/contacts", self.base_url(), parent,);
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_contact response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a contact.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — Required. The name of the contact to delete. Format: organizations/{organization_id}/contacts/{contact_id}, folders/{fol *(required)*
    ///
    /// # Response
    /// [`EssentialContactsEmpty`]
    #[allow(dead_code)]
    pub(crate) async fn delete_contact(&self, name: &str) -> Result<EssentialContactsEmpty> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_contact response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_contacts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/contacts?pageSize=test-pageSize&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(ListContactsResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = EssentialcontactsOps::new(&client);

        let result = ops
            .list_contacts("test-parent", "test-pageSize", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_contact() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(EssentialContact::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = EssentialcontactsOps::new(&client);

        let result = ops.get_contact("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_contact() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/v1/test-parent/contacts")
            .returning_json(serde_json::to_value(EssentialContact::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = EssentialcontactsOps::new(&client);

        let body = EssentialContact::fixture();
        let result = ops.create_contact("test-parent", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_contact() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(EssentialContactsEmpty::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = EssentialcontactsOps::new(&client);

        let result = ops.delete_contact("test-name").await;
        assert!(result.is_ok());
    }
}
