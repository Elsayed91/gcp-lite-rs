//! Operation contracts for the Kubernetes Engine API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/container.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::container::*;
use crate::{GcpHttpClient, Result};

/// Raw HTTP operations for the Kubernetes Engine API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the GCP Discovery Document.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::container::ContainerClient`] instead.
pub struct ContainerOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> ContainerOps<'a> {
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
        "https://container.googleapis.com"
    }

    /// Lists all clusters owned by a project in either the specified zone or all zones.
    ///
    /// **GCP API**: `GET v1/{+parent}/clusters`
    ///
    /// # Path Parameters
    /// - `parent` — The parent (project and location) where the clusters will be listed. Specified in the format `projects/*/locations/*`. L *(required)*
    ///
    /// # Query Parameters
    /// - `projectId` — Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/
    /// - `zone` — Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which
    ///
    /// # Response
    /// [`ListClustersResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_clusters(&self, parent: &str) -> Result<ListClustersResponse> {
        let url = format!("{}/v1/{}/clusters", self.base_url(), parent,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse list_clusters response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Gets the details of a specific cluster.
    ///
    /// **GCP API**: `GET v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — The name (project, location, cluster) of the cluster to retrieve. Specified in the format `projects/*/locations/*/cluste *(required)*
    ///
    /// # Query Parameters
    /// - `clusterId` — Deprecated. The name of the cluster to retrieve. This field has been deprecated and replaced by the name field.
    /// - `projectId` — Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/
    /// - `zone` — Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which
    ///
    /// # Response
    /// [`Cluster`]
    #[allow(dead_code)]
    pub(crate) async fn get_cluster(&self, name: &str) -> Result<Cluster> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.get(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse get_cluster response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }

    /// Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls
    /// and routes that were configured during cluster creation are also deleted. Other Google
    /// Compute Engine resources that might be in use by the cluster, such as load balancer
    /// resources, are not deleted if they weren't present when the cluster was initially
    /// created.
    ///
    /// **GCP API**: `DELETE v1/{+name}`
    ///
    /// # Path Parameters
    /// - `name` — The name (project, location, cluster) of the cluster to delete. Specified in the format `projects/*/locations/*/clusters *(required)*
    ///
    /// # Query Parameters
    /// - `clusterId` — Deprecated. The name of the cluster to delete. This field has been deprecated and replaced by the name field.
    /// - `projectId` — Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/
    /// - `zone` — Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which
    ///
    /// # Response
    /// [`ContainerLro`]
    #[allow(dead_code)]
    pub(crate) async fn delete_cluster(&self, name: &str) -> Result<ContainerLro> {
        let url = format!("{}/v1/{}", self.base_url(), name,);
        let response = self.client.delete(&url).await?;
        serde_json::from_slice(&response).map_err(|e| crate::GcpError::InvalidResponse {
            message: format!("Failed to parse delete_cluster response: {e}"),
            body: Some(String::from_utf8_lossy(&response).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-parent/clusters")
            .returning_json(serde_json::to_value(ListClustersResponse::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ContainerOps::new(&client);

        let result = ops.list_clusters("test-parent").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/test-name")
            .returning_json(serde_json::to_value(Cluster::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ContainerOps::new(&client);

        let result = ops.get_cluster("test-name").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/test-name")
            .returning_json(serde_json::to_value(ContainerLro::fixture()).unwrap());

        let client = crate::GcpHttpClient::from_mock(mock);
        let ops = ContainerOps::new(&client);

        let result = ops.delete_cluster("test-name").await;
        assert!(result.is_ok());
    }
}
