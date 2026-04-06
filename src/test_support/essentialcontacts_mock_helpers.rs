//! MockClient helpers for Essential Contacts API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Essential Contacts API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait EssentialcontactsMockHelpers {
    /// Helper to expect `list_contacts`: Lists the contacts that have been set on a resource.
    fn expect_list_contacts(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_contact`: Gets a single contact.
    fn expect_get_contact(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_contact`: Adds a new contact for a resource.
    fn expect_create_contact(&mut self, parent: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_contact`: Deletes a contact.
    fn expect_delete_contact(&mut self, name: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl EssentialcontactsMockHelpers for MockClient {
    /// Helper to expect `list_contacts`: Lists the contacts that have been set on a resource.
    fn expect_list_contacts(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/contacts");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_contact`: Gets a single contact.
    fn expect_get_contact(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `create_contact`: Adds a new contact for a resource.
    fn expect_create_contact(
        &mut self,
        parent: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{parent}/contacts");
        self.expect_post(&path)
    }

    /// Helper to expect `delete_contact`: Deletes a contact.
    fn expect_delete_contact(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_delete(&path)
    }
}
