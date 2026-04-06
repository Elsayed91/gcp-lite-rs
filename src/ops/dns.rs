//! Operation contracts for the Cloud DNS API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/dns.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::dns::*;
use crate::{GcpHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Cloud DNS API API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::dns::DnsClient`] instead.
pub struct DnsOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> DnsOps<'a> {
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
        "https://dns.googleapis.com"
    }

    /// Fetches the representation of an existing ManagedZone.
    ///
    /// **GCP API**: `GET dns/v1/projects/{project}/managedZones/{managedZone}`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    /// - `managedZone` — Identifies the managed zone addressed by this request. Can be the managed zone name or ID. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    ///
    /// # Response
    /// [`ManagedZone`]
    #[allow(dead_code)]
    pub(crate) async fn get_managed_zone(
        &self,
        project: &str,
        managed_zone: &str,
    ) -> Result<ManagedZone> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}",
            self.base_url(),
            encode(project),
            encode(managed_zone),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_managed_zone response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Enumerates ManagedZones that have been created but not yet deleted.
    ///
    /// **GCP API**: `GET dns/v1/projects/{project}/managedZones`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `dnsName` — Restricts the list to return only zones with this domain name.
    /// - `maxResults` — Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return.
    /// - `pageToken` — Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list r
    ///
    /// # Response
    /// [`ManagedZonesListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_managed_zones(
        &self,
        project: &str,
        max_results: &str,
        page_token: &str,
        dns_name: &str,
    ) -> Result<ManagedZonesListResponse> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(
            url,
            &[
                ("maxResults", max_results),
                ("pageToken", page_token),
                ("dnsName", dns_name),
            ],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_managed_zones response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Applies a partial update to an existing ManagedZone.
    ///
    /// **GCP API**: `PATCH dns/v1/projects/{project}/managedZones/{managedZone}`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    /// - `managedZone` — Identifies the managed zone addressed by this request. Can be the managed zone name or ID. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    ///
    /// # Request Body
    /// [`ManagedZone`]
    ///
    /// # Response
    /// [`DnsOperation`]
    #[allow(dead_code)]
    pub(crate) async fn patch_managed_zone(
        &self,
        project: &str,
        managed_zone: &str,
        body: &ManagedZone,
    ) -> Result<DnsOperation> {
        let url = format!(
            "{}/dns/v1/projects/{}/managedZones/{}",
            self.base_url(),
            encode(project),
            encode(managed_zone),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_managed_zone response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Enumerates all policies associated with a project.
    ///
    /// **GCP API**: `GET dns/v1/projects/{project}/policies`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `maxResults` — Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return.
    /// - `pageToken` — Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list r
    ///
    /// # Response
    /// [`PoliciesListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_dns_policies(
        &self,
        project: &str,
        max_results: &str,
        page_token: &str,
    ) -> Result<PoliciesListResponse> {
        let url = format!(
            "{}/dns/v1/projects/{}/policies",
            self.base_url(),
            encode(project),
        );
        let url = crate::append_query_params(
            url,
            &[("maxResults", max_results), ("pageToken", page_token)],
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_dns_policies response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Fetches the representation of an existing policy.
    ///
    /// **GCP API**: `GET dns/v1/projects/{project}/policies/{policy}`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    /// - `policy` — User given friendly name of the policy addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn get_dns_policy(&self, project: &str, policy: &str) -> Result<Policy> {
        let url = format!(
            "{}/dns/v1/projects/{}/policies/{}",
            self.base_url(),
            encode(project),
            encode(policy),
        );
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_dns_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Creates a new policy.
    ///
    /// **GCP API**: `POST dns/v1/projects/{project}/policies`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    ///
    /// # Request Body
    /// [`Policy`]
    ///
    /// # Response
    /// [`Policy`]
    #[allow(dead_code)]
    pub(crate) async fn create_dns_policy(&self, project: &str, body: &Policy) -> Result<Policy> {
        let url = format!(
            "{}/dns/v1/projects/{}/policies",
            self.base_url(),
            encode(project),
        );
        let response = self.client.post(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse create_dns_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Applies a partial update to an existing policy.
    ///
    /// **GCP API**: `PATCH dns/v1/projects/{project}/policies/{policy}`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    /// - `policy` — User given friendly name of the policy addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    ///
    /// # Request Body
    /// [`Policy`]
    ///
    /// # Response
    /// [`PoliciesPatchResponse`]
    #[allow(dead_code)]
    pub(crate) async fn patch_dns_policy(
        &self,
        project: &str,
        policy: &str,
        body: &Policy,
    ) -> Result<PoliciesPatchResponse> {
        let url = format!(
            "{}/dns/v1/projects/{}/policies/{}",
            self.base_url(),
            encode(project),
            encode(policy),
        );
        let response = self.client.patch(&url, body).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse patch_dns_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes a previously created policy. Fails if the policy is still being referenced by a
    /// network.
    ///
    /// **GCP API**: `DELETE dns/v1/projects/{project}/policies/{policy}`
    ///
    /// # Path Parameters
    /// - `project` — Identifies the project addressed by this request. *(required)*
    /// - `policy` — User given friendly name of the policy addressed by this request. *(required)*
    ///
    /// # Query Parameters
    /// - `clientOperationId` — For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resou
    #[allow(dead_code)]
    pub(crate) async fn delete_dns_policy(&self, project: &str, policy: &str) -> Result<()> {
        let url = format!(
            "{}/dns/v1/projects/{}/policies/{}",
            self.base_url(),
            encode(project),
            encode(policy),
        );
        let _ = self.client.delete(&url).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_managed_zone() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/dns/v1/projects/test-project/managedZones/test-managedZone")
            .returning_json(serde_json::to_value(ManagedZone::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .get_managed_zone("test-project", "test-managedZone")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_managed_zones() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/dns/v1/projects/test-project/managedZones?maxResults=test-maxResults&pageToken=test-pageToken&dnsName=test-dnsName")
            .returning_json(serde_json::to_value(ManagedZonesListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .list_managed_zones(
                "test-project",
                "test-maxResults",
                "test-pageToken",
                "test-dnsName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_managed_zone() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/dns/v1/projects/test-project/managedZones/test-managedZone")
            .returning_json(serde_json::to_value(DnsOperation::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let body = ManagedZone::fixture();
        let result = ops
            .patch_managed_zone("test-project", "test-managedZone", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_dns_policies() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/dns/v1/projects/test-project/policies?maxResults=test-maxResults&pageToken=test-pageToken")
            .returning_json(serde_json::to_value(PoliciesListResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .list_dns_policies("test-project", "test-maxResults", "test-pageToken")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_dns_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/dns/v1/projects/test-project/policies/test-policy")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops.get_dns_policy("test-project", "test-policy").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_dns_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/dns/v1/projects/test-project/policies")
            .returning_json(serde_json::to_value(Policy::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let body = Policy::fixture();
        let result = ops.create_dns_policy("test-project", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_dns_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/dns/v1/projects/test-project/policies/test-policy")
            .returning_json(serde_json::to_value(PoliciesPatchResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let body = Policy::fixture();
        let result = ops
            .patch_dns_policy("test-project", "test-policy", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_dns_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/dns/v1/projects/test-project/policies/test-policy")
            .returning_json(serde_json::json!({}));

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops.delete_dns_policy("test-project", "test-policy").await;
        assert!(result.is_ok());
    }
}
