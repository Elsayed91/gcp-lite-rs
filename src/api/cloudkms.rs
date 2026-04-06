//! Cloud KMS API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::cloudkms::CloudkmsOps`. This layer adds ergonomic method
//! signatures (project/location/key_ring/crypto_key instead of raw resource names)
//! and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 1.9 (iam_kms_public_access): get/set IAM policy on CryptoKey
//!   - CIS 1.10 (iam_kms_key_rotation): list keys, check/set rotation schedule

use crate::{
    GcpHttpClient, Result,
    ops::cloudkms::CloudkmsOps,
    types::cloudkms::{CryptoKey, KeyRing, Location, Policy, SetIamPolicyRequest},
};

/// Client for the Cloud KMS API.
pub struct KmsClient<'a> {
    ops: CloudkmsOps<'a>,
}

impl<'a> KmsClient<'a> {
    /// Create a new Cloud KMS API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: CloudkmsOps::new(client),
        }
    }

    // ── Locations ──────────────────────────────────────────────────────

    /// List all KMS locations for a project (auto-paginated).
    pub async fn list_locations(&self, project: &str) -> Result<Vec<Location>> {
        let name = format!("projects/{}", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_locations(&name, "100", &page_token, "")
                .await?;
            all.extend(resp.locations);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    // ── Key Rings ─────────────────────────────────────────────────────

    /// List all key rings in a location (auto-paginated).
    pub async fn list_key_rings(&self, project: &str, location: &str) -> Result<Vec<KeyRing>> {
        let parent = format!("projects/{}/locations/{}", project, location);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_key_rings(&parent, "100", &page_token, "")
                .await?;
            all.extend(resp.key_rings);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a key ring by project, location, and key ring ID.
    pub async fn get_key_ring(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
    ) -> Result<KeyRing> {
        let name = format!(
            "projects/{}/locations/{}/keyRings/{}",
            project, location, key_ring
        );
        self.ops.get_key_ring(&name).await
    }

    // ── Crypto Keys ───────────────────────────────────────────────────

    /// List all crypto keys in a key ring (auto-paginated).
    pub async fn list_crypto_keys(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
    ) -> Result<Vec<CryptoKey>> {
        let parent = format!(
            "projects/{}/locations/{}/keyRings/{}",
            project, location, key_ring
        );
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_crypto_keys(&parent, "100", &page_token, "")
                .await?;
            all.extend(resp.crypto_keys);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a crypto key by its path components.
    pub async fn get_crypto_key(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
        crypto_key: &str,
    ) -> Result<CryptoKey> {
        let name = format!(
            "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}",
            project, location, key_ring, crypto_key
        );
        self.ops.get_crypto_key(&name).await
    }

    // ── IAM Policy ────────────────────────────────────────────────────

    /// Get the IAM policy for a KeyRing.
    ///
    /// CIS 1.11 (iam_kms_separation_of_duties): checks that no principal has
    /// both `cloudkms.admin` and `cloudkms.cryptoKeyEncrypterDecrypter` on the
    /// same key ring.
    pub async fn get_key_ring_iam_policy(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
    ) -> Result<Policy> {
        let resource = format!(
            "projects/{}/locations/{}/keyRings/{}",
            project, location, key_ring
        );
        self.ops.get_key_ring_iam_policy(&resource).await
    }

    /// Get the IAM policy for a CryptoKey.
    pub async fn get_crypto_key_iam_policy(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
        crypto_key: &str,
    ) -> Result<Policy> {
        let resource = format!(
            "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}",
            project, location, key_ring, crypto_key
        );
        self.ops.get_crypto_key_iam_policy(&resource).await
    }

    /// Set the IAM policy for a CryptoKey.
    pub async fn set_crypto_key_iam_policy(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
        crypto_key: &str,
        policy: Policy,
    ) -> Result<Policy> {
        let resource = format!(
            "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}",
            project, location, key_ring, crypto_key
        );
        let body = SetIamPolicyRequest {
            policy,
            update_mask: None,
        };
        self.ops.set_crypto_key_iam_policy(&resource, &body).await
    }

    // ── Rotation Schedule ─────────────────────────────────────────────

    /// Update a CryptoKey's rotation schedule.
    ///
    /// - `rotation_period`: duration string like `"7776000s"` (90 days).
    /// - `next_rotation_time`: RFC 3339 timestamp for next rotation.
    pub async fn update_crypto_key_rotation(
        &self,
        project: &str,
        location: &str,
        key_ring: &str,
        crypto_key: &str,
        rotation_period: &str,
        next_rotation_time: &str,
    ) -> Result<CryptoKey> {
        let name = format!(
            "projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}",
            project, location, key_ring, crypto_key
        );
        let body = CryptoKey {
            rotation_period: Some(rotation_period.to_string()),
            next_rotation_time: Some(next_rotation_time.to_string()),
            ..Default::default()
        };
        self.ops
            .update_crypto_key(&name, "rotationPeriod,nextRotationTime", &body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_locations() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/locations")
            .returning_json(json!({
                "locations": [
                    {"name": "projects/my-project/locations/global", "locationId": "global", "displayName": "Global"},
                    {"name": "projects/my-project/locations/us-central1", "locationId": "us-central1", "displayName": "Iowa, USA"}
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms.list_locations("my-project").await;
        assert!(result.is_ok());
        let locations = result.unwrap();
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0].location_id, "global");
        assert_eq!(locations[1].location_id, "us-central1");
    }

    #[tokio::test]
    async fn test_list_key_rings() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/locations/global/keyRings")
            .returning_json(json!({
                "keyRings": [
                    {
                        "name": "projects/my-project/locations/global/keyRings/my-ring",
                        "createTime": "2026-01-01T00:00:00Z"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms.list_key_rings("my-project", "global").await;
        assert!(result.is_ok());
        let rings = result.unwrap();
        assert_eq!(rings.len(), 1);
        assert!(rings[0].name.contains("my-ring"));
        assert_eq!(
            rings[0].create_time.as_deref(),
            Some("2026-01-01T00:00:00Z")
        );
    }

    #[tokio::test]
    async fn test_get_key_ring() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/locations/global/keyRings/my-ring")
            .returning_json(json!({
                "name": "projects/my-project/locations/global/keyRings/my-ring",
                "createTime": "2026-01-01T00:00:00Z"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms.get_key_ring("my-project", "global", "my-ring").await;
        assert!(result.is_ok());
        let ring = result.unwrap();
        assert!(ring.name.contains("my-ring"));
    }

    #[tokio::test]
    async fn test_list_crypto_keys() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/v1/projects/my-project/locations/global/keyRings/my-ring/cryptoKeys",
        )
        .returning_json(json!({
            "cryptoKeys": [
                {
                    "name": "projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key",
                    "purpose": "ENCRYPT_DECRYPT",
                    "rotationPeriod": "7776000s",
                    "nextRotationTime": "2026-06-01T00:00:00Z"
                }
            ]
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms
            .list_crypto_keys("my-project", "global", "my-ring")
            .await;
        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].purpose.as_deref(), Some("ENCRYPT_DECRYPT"));
        assert_eq!(keys[0].rotation_period.as_deref(), Some("7776000s"));
    }

    #[tokio::test]
    async fn test_get_crypto_key() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/v1/projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key",
        )
        .returning_json(json!({
            "name": "projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key",
            "purpose": "ENCRYPT_DECRYPT",
            "createTime": "2026-01-01T00:00:00Z",
            "primary": {
                "name": "projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1",
                "state": "ENABLED",
                "algorithm": "GOOGLE_SYMMETRIC_ENCRYPTION"
            }
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms
            .get_crypto_key("my-project", "global", "my-ring", "my-key")
            .await;
        assert!(result.is_ok());
        let key = result.unwrap();
        assert!(key.name.contains("my-key"));
        assert_eq!(key.purpose.as_deref(), Some("ENCRYPT_DECRYPT"));
        assert!(key.primary.is_some());
        assert_eq!(
            key.primary.as_ref().unwrap().state.as_deref(),
            Some("ENABLED")
        );
    }

    #[tokio::test]
    async fn test_get_key_ring_iam_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/locations/global/keyRings/my-ring:getIamPolicy")
            .returning_json(json!({
                "version": 1,
                "etag": "ACAB",
                "bindings": [
                    {
                        "role": "roles/cloudkms.admin",
                        "members": ["serviceAccount:admin@my-project.iam.gserviceaccount.com"]
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms
            .get_key_ring_iam_policy("my-project", "global", "my-ring")
            .await;
        assert!(result.is_ok());
        let policy = result.unwrap();
        assert_eq!(policy.etag.as_deref(), Some("ACAB"));
        assert_eq!(policy.bindings.len(), 1);
        assert_eq!(policy.bindings[0].role, "roles/cloudkms.admin");
        assert_eq!(
            policy.bindings[0].members,
            vec!["serviceAccount:admin@my-project.iam.gserviceaccount.com"]
        );
    }

    #[tokio::test]
    async fn test_get_crypto_key_iam_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/v1/projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key:getIamPolicy",
        )
        .returning_json(json!({
            "version": 1,
            "etag": "BwZL0Ct7bgQ=",
            "bindings": [
                {
                    "role": "roles/cloudkms.viewer",
                    "members": ["user:test@example.com"]
                }
            ]
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms
            .get_crypto_key_iam_policy("my-project", "global", "my-ring", "my-key")
            .await;
        assert!(result.is_ok());
        let policy = result.unwrap();
        assert_eq!(policy.etag.as_deref(), Some("BwZL0Ct7bgQ="));
        assert_eq!(policy.bindings.len(), 1);
        assert_eq!(policy.bindings[0].role, "roles/cloudkms.viewer".to_string());
        assert_eq!(
            policy.bindings[0].members,
            vec!["user:test@example.com".to_string()]
        );
    }

    #[tokio::test]
    async fn test_set_crypto_key_iam_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_post(
            "/v1/projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key:setIamPolicy",
        )
        .returning_json(json!({
            "version": 1,
            "etag": "ACAB",
            "bindings": []
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();

        let policy = crate::types::cloudkms::Policy {
            etag: Some("ACAB".to_string()),
            ..Default::default()
        };

        let result = kms
            .set_crypto_key_iam_policy("my-project", "global", "my-ring", "my-key", policy)
            .await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.etag.as_deref(), Some("ACAB"));
    }

    #[tokio::test]
    async fn test_update_crypto_key_rotation() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch(
            "/v1/projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key",
        )
        .returning_json(json!({
            "name": "projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key",
            "purpose": "ENCRYPT_DECRYPT",
            "rotationPeriod": "7776000s",
            "nextRotationTime": "2026-06-01T00:00:00Z"
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let kms = client.kms();
        let result = kms
            .update_crypto_key_rotation(
                "my-project",
                "global",
                "my-ring",
                "my-key",
                "7776000s",
                "2026-06-01T00:00:00Z",
            )
            .await;
        assert!(result.is_ok());
        let key = result.unwrap();
        assert_eq!(key.rotation_period.as_deref(), Some("7776000s"));
        assert_eq!(
            key.next_rotation_time.as_deref(),
            Some("2026-06-01T00:00:00Z")
        );
    }
}
