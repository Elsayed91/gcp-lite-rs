//! MockClient helpers for Cloud KMS API API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Cloud KMS API helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CloudkmsMockHelpers {
    /// Helper to expect `list_locations`: Lists information about the supported locations for this
    /// service. This method can be called in two ways: * **List all public locations:** Use the
    /// path `GET /v1/locations`. * **List project-visible locations:** Use the path `GET
    /// /v1/projects/{project_id}/locations`. This may include public locations as well as private
    /// or other locations specifically visible to the project.
    fn expect_list_locations(
        &mut self,
        name: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_key_rings`: Lists KeyRings.
    fn expect_list_key_rings(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_key_ring`: Returns metadata for a given KeyRing.
    fn expect_get_key_ring(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_crypto_keys`: Lists CryptoKeys.
    fn expect_list_crypto_keys(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_crypto_key`: Returns metadata for a given CryptoKey, as well as its
    /// primary CryptoKeyVersion.
    fn expect_get_crypto_key(&mut self, name: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_key_ring_iam_policy`: Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy set.
    fn expect_get_key_ring_iam_policy(&mut self, resource: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_crypto_key_iam_policy`: Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy set.
    fn expect_get_crypto_key_iam_policy(&mut self, resource: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_crypto_key_iam_policy`: Sets the access control policy on the
    /// specified resource. Replaces any existing policy. Can return `NOT_FOUND`,
    /// `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    fn expect_set_crypto_key_iam_policy(&mut self, resource: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_crypto_key`: Update a CryptoKey.
    fn expect_update_crypto_key(&mut self, name: &str, update_mask: &str)
    -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CloudkmsMockHelpers for MockClient {
    /// Helper to expect `list_locations`: Lists information about the supported locations for this
    /// service. This method can be called in two ways: * **List all public locations:** Use the
    /// path `GET /v1/locations`. * **List project-visible locations:** Use the path `GET
    /// /v1/projects/{project_id}/locations`. This may include public locations as well as private
    /// or other locations specifically visible to the project.
    fn expect_list_locations(
        &mut self,
        name: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{name}/locations");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `list_key_rings`: Lists KeyRings.
    fn expect_list_key_rings(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/keyRings");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_key_ring`: Returns metadata for a given KeyRing.
    fn expect_get_key_ring(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `list_crypto_keys`: Lists CryptoKeys.
    fn expect_list_crypto_keys(
        &mut self,
        parent: &str,
        page_size: &str,
        page_token: &str,
        filter: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!("/v1/{parent}/cryptoKeys");
        let mut __qp: Vec<String> = Vec::new();
        if !page_size.is_empty() {
            __qp.push(format!("pageSize={}", page_size));
        }
        if !page_token.is_empty() {
            __qp.push(format!("pageToken={}", page_token));
        }
        if !filter.is_empty() {
            __qp.push(format!("filter={}", filter));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `get_crypto_key`: Returns metadata for a given CryptoKey, as well as its
    /// primary CryptoKeyVersion.
    fn expect_get_crypto_key(&mut self, name: &str) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{name}");
        self.expect_get(&path)
    }

    /// Helper to expect `get_key_ring_iam_policy`: Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy set.
    fn expect_get_key_ring_iam_policy(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{resource}:getIamPolicy");
        self.expect_get(&path)
    }

    /// Helper to expect `get_crypto_key_iam_policy`: Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy set.
    fn expect_get_crypto_key_iam_policy(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{resource}:getIamPolicy");
        self.expect_get(&path)
    }

    /// Helper to expect `set_crypto_key_iam_policy`: Sets the access control policy on the
    /// specified resource. Replaces any existing policy. Can return `NOT_FOUND`,
    /// `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    fn expect_set_crypto_key_iam_policy(
        &mut self,
        resource: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/v1/{resource}:setIamPolicy");
        self.expect_post(&path)
    }

    /// Helper to expect `update_crypto_key`: Update a CryptoKey.
    fn expect_update_crypto_key(
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
}
