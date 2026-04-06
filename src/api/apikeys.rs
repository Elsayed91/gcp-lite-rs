//! API Keys API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::apikeys::ApikeysOps`. This layer adds ergonomic method
//! signatures (project/key_id instead of raw resource names) and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 1.12 (iam_api_keys_active_services): inspect restrictions.apiTargets
//!   - CIS 1.13 (iam_api_keys_restricted_hosts): inspect restrictions.browserKeyRestrictions / serverKeyRestrictions / androidKeyRestrictions / iosKeyRestrictions
//!   - CIS 1.14 (iam_api_keys_restricted_apis): inspect restrictions.apiTargets
//!   - CIS 1.15 (iam_api_keys_rotation): inspect createTime (key age)

use crate::{GcpHttpClient, Result, ops::apikeys::ApikeysOps, types::apikeys::V2Key};

/// Client for the API Keys API.
pub struct ApikeysClient<'a> {
    ops: ApikeysOps<'a>,
}

impl<'a> ApikeysClient<'a> {
    /// Create a new API Keys API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: ApikeysOps::new(client),
        }
    }

    // ── Keys ──────────────────────────────────────────────────────────

    /// List all API keys for a project (auto-paginated).
    ///
    /// API keys are global resources; this always uses location `global`.
    pub async fn list_keys(&self, project: &str) -> Result<Vec<V2Key>> {
        let parent = format!("projects/{}/locations/global", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self.ops.list_keys(&parent, "200", &page_token, "").await?;
            all.extend(resp.keys);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get the metadata for a specific API key by project and key ID.
    ///
    /// The key string is NOT included in the response.
    pub async fn get_key(&self, project: &str, key_id: &str) -> Result<V2Key> {
        let name = format!("projects/{}/locations/global/keys/{}", project, key_id);
        self.ops.get_key(&name).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/locations/global/keys?pageSize=200")
            .returning_json(json!({
                "keys": [
                    {
                        "name": "projects/my-project/locations/global/keys/key-1",
                        "uid": "uid-1",
                        "displayName": "Key One",
                        "createTime": "2025-01-01T00:00:00Z",
                        "restrictions": {
                            "apiTargets": [
                                {"service": "translate.googleapis.com"}
                            ]
                        }
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let apikeys = client.apikeys();
        let result = apikeys.list_keys("my-project").await;
        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert!(keys[0].name.contains("key-1"));
        assert_eq!(keys[0].display_name.as_deref(), Some("Key One"));
        let restrictions = keys[0].restrictions.as_ref().unwrap();
        assert_eq!(restrictions.api_targets.len(), 1);
        assert_eq!(
            restrictions.api_targets[0].service.as_deref(),
            Some("translate.googleapis.com")
        );
    }

    #[tokio::test]
    async fn test_get_key() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v2/projects/my-project/locations/global/keys/my-key-id")
            .returning_json(json!({
                "name": "projects/my-project/locations/global/keys/my-key-id",
                "uid": "uid-123",
                "displayName": "My Key",
                "createTime": "2024-06-01T00:00:00Z",
                "restrictions": {
                    "browserKeyRestrictions": {
                        "allowedReferrers": ["*.example.com/*"]
                    },
                    "apiTargets": [
                        {"service": "maps.googleapis.com"}
                    ]
                }
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let apikeys = client.apikeys();
        let result = apikeys.get_key("my-project", "my-key-id").await;
        assert!(result.is_ok());
        let key = result.unwrap();
        assert!(key.name.contains("my-key-id"));
        assert_eq!(key.uid.as_deref(), Some("uid-123"));
        assert_eq!(key.create_time.as_deref(), Some("2024-06-01T00:00:00Z"));
        let restrictions = key.restrictions.as_ref().unwrap();
        let browser = restrictions.browser_key_restrictions.as_ref().unwrap();
        assert_eq!(browser.allowed_referrers, vec!["*.example.com/*"]);
        assert_eq!(restrictions.api_targets.len(), 1);
        assert_eq!(
            restrictions.api_targets[0].service.as_deref(),
            Some("maps.googleapis.com")
        );
    }

    #[tokio::test]
    async fn test_list_keys_paginated() {
        let mut mock = crate::MockClient::new();
        // Second page first — more specific (starts with longer prefix) must come before first page
        // to avoid the StartsWith matcher matching both pages against the first expectation.
        mock.expect_get(
            "/v2/projects/my-project/locations/global/keys?pageSize=200&pageToken=token123",
        )
        .returning_json(json!({
            "keys": [{"name": "projects/my-project/locations/global/keys/key-2"}]
        }))
        .times(1);
        // First page
        mock.expect_get("/v2/projects/my-project/locations/global/keys?pageSize=200")
            .returning_json(json!({
                "keys": [{"name": "projects/my-project/locations/global/keys/key-1"}],
                "nextPageToken": "token123"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let apikeys = client.apikeys();
        let result = apikeys.list_keys("my-project").await;
        assert!(result.is_ok());
        let keys = result.unwrap();
        assert_eq!(keys.len(), 2);
    }
}
