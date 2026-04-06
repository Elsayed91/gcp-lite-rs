//! OS Config API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::osconfig::OsconfigOps`. This layer adds ergonomic method
//! signatures and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 4.12 (compute_os_patches): verify OS patching is configured and VMs
//!     are reporting inventory (OS Config agent is installed and running).

use crate::{
    GcpHttpClient, Result,
    ops::osconfig::OsconfigOps,
    types::osconfig::{Inventory, PatchDeployment},
};

/// Client for the OS Config API.
pub struct OsConfigClient<'a> {
    ops: OsconfigOps<'a>,
}

impl<'a> OsConfigClient<'a> {
    /// Create a new OS Config client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: OsconfigOps::new(client),
        }
    }

    // ── Patch Deployments ─────────────────────────────────────────────────

    /// List all OS Config patch deployments for a project (auto-paginated).
    ///
    /// CIS 4.12: at least one ACTIVE patch deployment should exist to ensure
    /// VMs receive regular OS patches.
    pub async fn list_patch_deployments(&self, project: &str) -> Result<Vec<PatchDeployment>> {
        let parent = format!("projects/{}", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_patch_deployments(&parent, "100", &page_token)
                .await?;
            all.extend(resp.patch_deployments);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    // ── Inventory ─────────────────────────────────────────────────────────

    /// List OS inventory for all VM instances across all zones in a project (auto-paginated).
    ///
    /// Uses the `-` wildcard for both location and instance to aggregate across the
    /// entire project. Pass `view = "FULL"` to include full package details; `""` or
    /// `"BASIC"` returns only OS metadata.
    ///
    /// CIS 4.12: VMs with recent `updateTime` indicate OS Config agent is active.
    pub async fn list_inventories(&self, project: &str, view: &str) -> Result<Vec<Inventory>> {
        // Use '-' wildcard for location and instance to aggregate across the whole project
        let parent = format!("projects/{}/locations/-/instances/-", project);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_inventories(&parent, "100", &page_token, view, "")
                .await?;
            all.extend(resp.inventories);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// List OS inventory for all VM instances in a specific zone (auto-paginated).
    ///
    /// Use this for zone-scoped checks; prefer `list_inventories` for project-wide coverage.
    pub async fn list_inventories_in_zone(
        &self,
        project: &str,
        zone: &str,
        view: &str,
    ) -> Result<Vec<Inventory>> {
        let parent = format!("projects/{}/locations/{}/instances/-", project, zone);
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_inventories(&parent, "100", &page_token, view, "")
                .await?;
            all.extend(resp.inventories);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_patch_deployments() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/v1/projects/my-project/patchDeployments?pageSize=100")
            .returning_json(json!({
                "patchDeployments": [
                    {
                        "name": "projects/my-project/patchDeployments/weekly-patches",
                        "description": "Weekly OS patching",
                        "state": "ACTIVE",
                        "createTime": "2024-01-01T00:00:00Z",
                        "lastExecuteTime": "2025-02-01T00:00:00Z"
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let os = client.osconfig();
        let result = os.list_patch_deployments("my-project").await;
        assert!(result.is_ok());
        let deployments = result.unwrap();
        assert_eq!(deployments.len(), 1);
        assert_eq!(
            deployments[0].name,
            "projects/my-project/patchDeployments/weekly-patches"
        );
        assert_eq!(deployments[0].state.as_deref(), Some("ACTIVE"));
        assert_eq!(
            deployments[0].last_execute_time.as_deref(),
            Some("2025-02-01T00:00:00Z")
        );
    }

    #[tokio::test]
    async fn test_list_patch_deployments_paginated() {
        let mut mock = crate::MockClient::new();
        // Second page first — more specific URL before the less specific first-page prefix
        mock.expect_get(
            "/v1/projects/my-project/patchDeployments?pageSize=100&pageToken=tok123",
        )
        .returning_json(json!({
            "patchDeployments": [{"name": "projects/my-project/patchDeployments/pd-2", "state": "ACTIVE"}]
        }))
        .times(1);
        // First page
        mock.expect_get("/v1/projects/my-project/patchDeployments?pageSize=100")
            .returning_json(json!({
                "patchDeployments": [{"name": "projects/my-project/patchDeployments/pd-1", "state": "ACTIVE"}],
                "nextPageToken": "tok123"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let os = client.osconfig();
        let result = os.list_patch_deployments("my-project").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_list_inventories() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/v1/projects/my-project/locations/-/instances/-/inventories?pageSize=100&view=BASIC",
        )
        .returning_json(json!({
            "inventories": [
                {
                    "name": "projects/my-project/locations/us-central1-a/instances/vm-1/inventory",
                    "updateTime": "2025-02-10T12:00:00Z",
                    "osInfo": {
                        "hostname": "vm-1",
                        "longName": "Debian GNU/Linux 11",
                        "shortName": "debian",
                        "version": "11",
                        "architecture": "x86_64",
                        "kernelVersion": "5.10.0"
                    }
                }
            ]
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let os = client.osconfig();
        let result = os.list_inventories("my-project", "BASIC").await;
        assert!(result.is_ok());
        let inventories = result.unwrap();
        assert_eq!(inventories.len(), 1);
        assert!(inventories[0].name.contains("vm-1"));
        assert_eq!(
            inventories[0].update_time.as_deref(),
            Some("2025-02-10T12:00:00Z")
        );
        let os_info = inventories[0].os_info.as_ref().unwrap();
        assert_eq!(os_info.hostname.as_deref(), Some("vm-1"));
        assert_eq!(os_info.short_name.as_deref(), Some("debian"));
    }

    #[tokio::test]
    async fn test_list_inventories_in_zone() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/v1/projects/my-project/locations/us-central1-a/instances/-/inventories?pageSize=100&view=BASIC",
        )
        .returning_json(json!({
            "inventories": [
                {
                    "name": "projects/my-project/locations/us-central1-a/instances/vm-1/inventory",
                    "updateTime": "2025-02-10T12:00:00Z"
                }
            ]
        }))
        .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let os = client.osconfig();
        let result = os
            .list_inventories_in_zone("my-project", "us-central1-a", "BASIC")
            .await;
        assert!(result.is_ok());
        let inventories = result.unwrap();
        assert_eq!(inventories.len(), 1);
        assert!(inventories[0].name.contains("us-central1-a"));
    }
}
