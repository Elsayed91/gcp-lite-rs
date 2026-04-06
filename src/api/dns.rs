//! Cloud DNS API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::dns::DnsOps`. This layer adds ergonomic method signatures
//! and auto-pagination.
//!
//! Needed by GCP CIS benchmark checks:
//!   - CIS 3.3 (vpc_dnssec_enabled): patch managed zone DNSSEC config
//!   - CIS 3.4 (vpc_dnssec_no_rsasha1_ksk): inspect DNSSEC key specs
//!   - CIS 3.5 (vpc_dnssec_no_rsasha1_zsk): inspect DNSSEC key specs
//!   - CIS 2.12 (vpc_dns_logging): patch managed zone logging config

use crate::{
    GcpHttpClient, Result,
    ops::dns::DnsOps,
    types::dns::{ManagedZone, ManagedZoneCloudLoggingConfig, ManagedZoneDnsSecConfig, Policy},
};

/// Client for the Cloud DNS API.
pub struct DnsClient<'a> {
    ops: DnsOps<'a>,
}

impl<'a> DnsClient<'a> {
    /// Create a new Cloud DNS API client.
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: DnsOps::new(client),
        }
    }

    // ── Managed Zones ──────────────────────────────────────────────────

    /// Get a managed zone by project and zone name.
    pub async fn get_managed_zone(&self, project: &str, zone: &str) -> Result<ManagedZone> {
        self.ops.get_managed_zone(project, zone).await
    }

    /// List all managed zones for a project (auto-paginated).
    pub async fn list_managed_zones(&self, project: &str) -> Result<Vec<ManagedZone>> {
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_managed_zones(project, "100", &page_token, "")
                .await?;
            all.extend(resp.managed_zones);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Patch the DNSSEC configuration of a managed zone.
    ///
    /// Sends a PATCH with only `dnssecConfig` populated so other zone fields
    /// are left untouched. The DNS API returns a synchronous operation
    /// (typically immediately DONE).
    pub async fn patch_managed_zone_dnssec(
        &self,
        project: &str,
        zone: &str,
        dnssec_config: ManagedZoneDnsSecConfig,
    ) -> Result<()> {
        let body = ManagedZone {
            name: zone.to_string(),
            dnssec_config: Some(dnssec_config),
            ..Default::default()
        };
        let _ = self.ops.patch_managed_zone(project, zone, &body).await?;
        Ok(())
    }

    /// Patch the cloud logging configuration of a managed zone.
    ///
    /// Sends a PATCH with only `cloudLoggingConfig` populated.
    pub async fn patch_managed_zone_logging(
        &self,
        project: &str,
        zone: &str,
        logging_config: ManagedZoneCloudLoggingConfig,
    ) -> Result<()> {
        let body = ManagedZone {
            name: zone.to_string(),
            cloud_logging_config: Some(logging_config),
            ..Default::default()
        };
        let _ = self.ops.patch_managed_zone(project, zone, &body).await?;
        Ok(())
    }

    // ── DNS Policies ───────────────────────────────────────────────────

    /// List all DNS policies for a project (auto-paginated).
    pub async fn list_dns_policies(&self, project: &str) -> Result<Vec<Policy>> {
        let mut all = Vec::new();
        let mut page_token = String::new();
        loop {
            let resp = self
                .ops
                .list_dns_policies(project, "100", &page_token)
                .await?;
            all.extend(resp.policies);
            match resp.next_page_token {
                Some(tok) if !tok.is_empty() => page_token = tok,
                _ => break,
            }
        }
        Ok(all)
    }

    /// Get a DNS policy by project and policy name.
    pub async fn get_dns_policy(&self, project: &str, policy: &str) -> Result<Policy> {
        self.ops.get_dns_policy(project, policy).await
    }

    /// Create a new DNS policy.
    pub async fn create_dns_policy(&self, project: &str, policy: &Policy) -> Result<Policy> {
        self.ops.create_dns_policy(project, policy).await
    }

    /// Patch an existing DNS policy (e.g. to enable/disable logging).
    pub async fn patch_dns_policy(
        &self,
        project: &str,
        policy_name: &str,
        patch: &Policy,
    ) -> Result<Policy> {
        let resp = self
            .ops
            .patch_dns_policy(project, policy_name, patch)
            .await?;
        Ok(resp.policy.unwrap_or_default())
    }

    /// Delete a DNS policy by project and policy name.
    pub async fn delete_dns_policy(&self, project: &str, policy: &str) -> Result<()> {
        self.ops.delete_dns_policy(project, policy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dns::{DnsOperation, ManagedZone, Policy};

    #[tokio::test]
    async fn test_get_managed_zone() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/dns/v1/projects/test-project/managedZones/my-zone")
            .returning_json(serde_json::to_value(ManagedZone::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let result = dns.get_managed_zone("test-project", "my-zone").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_managed_zones() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/dns/v1/projects/test-project/managedZones?maxResults=100")
            .returning_json(serde_json::json!({
                "managedZones": [{"name": "zone-1", "dnsName": "example.com."}]
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let result = dns.list_managed_zones("test-project").await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "zone-1");
    }

    #[tokio::test]
    async fn test_patch_managed_zone_dnssec() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch("/dns/v1/projects/test-project/managedZones/my-zone")
            .returning_json(serde_json::to_value(DnsOperation::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let dnssec = ManagedZoneDnsSecConfig {
            state: Some("on".to_string()),
            ..Default::default()
        };
        let result = dns
            .patch_managed_zone_dnssec("test-project", "my-zone", dnssec)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_managed_zone_logging() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch("/dns/v1/projects/test-project/managedZones/logging-zone")
            .returning_json(serde_json::to_value(DnsOperation::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let logging = ManagedZoneCloudLoggingConfig {
            enable_logging: Some(true),
        };
        let result = dns
            .patch_managed_zone_logging("test-project", "logging-zone", logging)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_dns_policies() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/dns/v1/projects/test-project/policies?maxResults=100")
            .returning_json(serde_json::json!({
                "policies": [{"name": "my-policy"}]
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let result = dns.list_dns_policies("test-project").await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "my-policy");
    }

    #[tokio::test]
    async fn test_get_dns_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/dns/v1/projects/test-project/policies/my-policy")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let result = dns.get_dns_policy("test-project", "my-policy").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_dns_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/dns/v1/projects/test-project/policies")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let policy = Policy::fixture();
        let result = dns.create_dns_policy("test-project", &policy).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_dns_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch("/dns/v1/projects/test-project/policies/my-policy")
            .returning_json(serde_json::json!({
                "policy": {"name": "my-policy", "enableLogging": true}
            }));

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let patch = Policy {
            name: "my-policy".to_string(),
            enable_logging: Some(true),
            ..Default::default()
        };
        let result = dns
            .patch_dns_policy("test-project", "my-policy", &patch)
            .await
            .unwrap();
        assert_eq!(result.name, "my-policy");
    }

    #[tokio::test]
    async fn test_delete_dns_policy() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/dns/v1/projects/test-project/policies/my-policy")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let dns = DnsClient::new(&client);

        let result = dns.delete_dns_policy("test-project", "my-policy").await;
        assert!(result.is_ok());
    }
}
