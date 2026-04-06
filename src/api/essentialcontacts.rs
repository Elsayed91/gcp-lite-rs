//! Essential Contacts API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::essentialcontacts::EssentialcontactsOps`. This layer adds
//! ergonomic method signatures and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 1.16 (iam_essential_contacts): list/create/delete notification contacts

use crate::{
    GcpHttpClient, Result, ops::essentialcontacts::EssentialcontactsOps,
    types::essentialcontacts::EssentialContact,
};

/// Client for the Essential Contacts API.
pub struct EssentialContactsClient<'a> {
    ops: EssentialcontactsOps<'a>,
}

impl<'a> EssentialContactsClient<'a> {
    /// Create a new Essential Contacts API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: EssentialcontactsOps::new(client),
        }
    }

    // ── Contacts ───────────────────────────────────────────────────────

    /// List all essential contacts for a project (auto-paginated).
    pub async fn list_contacts(&self, project: &str) -> Result<Vec<EssentialContact>> {
        let parent = format!("projects/{}", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_contacts(&parent, "100", &page_token).await?;
            all.extend(resp.contacts);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a single contact by its full resource name.
    ///
    /// The `contact_name` is the full resource name returned by the API,
    /// e.g., `projects/{project}/contacts/{id}`.
    pub async fn get_contact(&self, contact_name: &str) -> Result<EssentialContact> {
        self.ops.get_contact(contact_name).await
    }

    /// Create a new essential contact for a project.
    pub async fn create_contact(
        &self,
        project: &str,
        contact: &EssentialContact,
    ) -> Result<EssentialContact> {
        let parent = format!("projects/{}", project);
        self.ops.create_contact(&parent, contact).await
    }

    /// Delete an essential contact by its full resource name.
    ///
    /// The `contact_name` is the full resource name returned by the API,
    /// e.g., `projects/{project}/contacts/{id}`.
    pub async fn delete_contact(&self, contact_name: &str) -> Result<()> {
        let _ = self.ops.delete_contact(contact_name).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::essentialcontacts::EssentialContact;

    #[tokio::test]
    async fn test_list_contacts() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/test-project/contacts?pageSize=100")
            .returning_json(serde_json::json!({
                "contacts": [{
                    "name": "projects/test-project/contacts/1",
                    "email": "admin@example.com",
                    "notificationCategorySubscriptions": ["SECURITY"],
                    "languageTag": "en"
                }]
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ec = EssentialContactsClient::new(&client);

        let result = ec.list_contacts("test-project").await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].email, "admin@example.com");
    }

    #[tokio::test]
    async fn test_get_contact() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/test-project/contacts/123")
            .returning_json(serde_json::to_value(EssentialContact::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ec = EssentialContactsClient::new(&client);

        let result = ec.get_contact("projects/test-project/contacts/123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_contact() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/v1/projects/test-project/contacts")
            .returning_json(serde_json::json!({
                "name": "projects/test-project/contacts/456",
                "email": "security@example.com",
                "notificationCategorySubscriptions": ["SECURITY"],
                "languageTag": "en"
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ec = EssentialContactsClient::new(&client);

        let contact = EssentialContact {
            email: "security@example.com".to_string(),
            notification_category_subscriptions: vec!["SECURITY".to_string()],
            language_tag: Some("en".to_string()),
            ..Default::default()
        };
        let result = ec.create_contact("test-project", &contact).await.unwrap();
        assert_eq!(result.email, "security@example.com");
        assert_eq!(
            result.name,
            Some("projects/test-project/contacts/456".to_string())
        );
    }

    #[tokio::test]
    async fn test_delete_contact() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/v1/projects/test-project/contacts/789")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ec = EssentialContactsClient::new(&client);

        let result = ec
            .delete_contact("projects/test-project/contacts/789")
            .await;
        assert!(result.is_ok());
    }
}
