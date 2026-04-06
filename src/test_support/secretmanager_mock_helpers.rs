//! MockClient helpers for Secret Manager API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Secret Manager helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SecretmanagerMockHelpers {
    /// Helper to expect `create_secret`: Creates a new Secret containing no SecretVersions.
    fn expect_create_secret(&mut self, parent: &str, secret_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_secret`: Gets metadata for a given Secret.
    fn expect_get_secret(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_secrets`: Lists Secrets.
    fn expect_list_secrets(
        &mut self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_secret`: Updates metadata of an existing Secret.
    fn expect_patch_secret(&mut self, name: &str, update_mask: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_secret`: Deletes a Secret.
    fn expect_delete_secret(&mut self, name: &str, etag: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SecretmanagerMockHelpers for MockClient {
    /// Helper to expect `create_secret`: Creates a new Secret containing no SecretVersions.
    fn expect_create_secret(
        &mut self,
        parent: &str,
        secret_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/secrets");
        let mut __qp: Vec<String> = Vec::new();
        if !secret_id.is_empty() {
            __qp.push(format!("secretId={}", secret_id));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_post(&path)
    }

    /// Helper to expect `get_secret`: Gets metadata for a given Secret.
    fn expect_get_secret(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_secrets`: Lists Secrets.
    fn expect_list_secrets(
        &mut self,
        parent: &str,
        filter: &str,
        page_size: &str,
        page_token: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/secrets");
        let mut __qp: Vec<String> = Vec::new();
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
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

    /// Helper to expect `patch_secret`: Updates metadata of an existing Secret.
    fn expect_patch_secret(
        &mut self,
        name: &str,
        update_mask: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{name}");
        let mut __qp: Vec<String> = Vec::new();
        if !update_mask.is_empty() {
            __qp.push(format!("updateMask={}", update_mask));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_secret`: Deletes a Secret.
    fn expect_delete_secret(
        &mut self,
        name: &str,
        etag: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{name}");
        let mut __qp: Vec<String> = Vec::new();
        if !etag.is_empty() {
            __qp.push(format!("etag={}", etag));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_delete(&path)
    }
}
