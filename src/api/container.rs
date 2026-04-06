//! Kubernetes Engine (GKE) API client.
//!
//! Provides ergonomic wrappers over generated ops for GKE cluster management.

use crate::operation::{ContainerOperation, PollConfig};
use crate::ops::container::ContainerOps;
use crate::types::container::{Cluster, ContainerLro};
use crate::{GcpHttpClient, Result};

/// Client for the Kubernetes Engine API.
pub struct ContainerClient<'a> {
    ops: ContainerOps<'a>,
}

impl<'a> ContainerClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self {
            ops: ContainerOps::new(client),
        }
    }

    // ── Clusters ──────────────────────────────────────────────────────

    /// List clusters in a location.
    ///
    /// Use `"-"` as `location` to list across all locations.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1") or "-" for all
    pub async fn list_clusters(&self, project: &str, location: &str) -> Result<Vec<Cluster>> {
        let parent = format!("projects/{}/locations/{}", project, location);
        let response = self.ops.list_clusters(&parent).await?;
        Ok(response.clusters)
    }

    /// Get a cluster by name.
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `cluster` - The cluster name
    pub async fn get_cluster(
        &self,
        project: &str,
        location: &str,
        cluster: &str,
    ) -> Result<Cluster> {
        let name = format!(
            "projects/{}/locations/{}/clusters/{}",
            project, location, cluster
        );
        self.ops.get_cluster(&name).await
    }

    /// Delete a cluster (blocks until complete).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `cluster` - The cluster name
    pub async fn delete_cluster(&self, project: &str, location: &str, cluster: &str) -> Result<()> {
        let op = self
            .delete_cluster_start(project, location, cluster)
            .await?;
        op.wait().await
    }

    /// Delete a cluster (returns operation for manual polling).
    ///
    /// # Arguments
    /// * `project` - The GCP project ID
    /// * `location` - The location (e.g., "us-central1")
    /// * `cluster` - The cluster name
    pub async fn delete_cluster_start(
        &self,
        project: &str,
        location: &str,
        cluster: &str,
    ) -> Result<ContainerOperation<'a>> {
        let name = format!(
            "projects/{}/locations/{}/clusters/{}",
            project, location, cluster
        );
        let lro = self.ops.delete_cluster(&name).await?;
        self.container_operation(lro)
    }

    // ── Helpers ───────────────────────────────────────────────────────

    fn container_operation(&self, lro: ContainerLro) -> Result<ContainerOperation<'a>> {
        let initially_done = lro.status.as_deref() == Some("DONE");
        let operation_url = lro
            .self_link
            .ok_or_else(|| crate::GcpError::InvalidResponse {
                message: "Container operation missing selfLink".to_string(),
                body: None,
            })?;
        let config = PollConfig::container_operation();
        Ok(ContainerOperation::new(
            self.ops.client,
            operation_url,
            config.initial_interval(),
            config.timeout(),
            initially_done,
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_list_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/-/clusters")
            .returning_json(json!({
                "clusters": [
                    {
                        "name": "my-cluster",
                        "location": "us-central1",
                        "status": "RUNNING",
                        "currentMasterVersion": "1.28.3-gke.1200",
                        "currentNodeCount": 3
                    }
                ]
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let container = client.container();

        let clusters = container.list_clusters("test-project", "-").await.unwrap();
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].name, "my-cluster");
        assert_eq!(
            clusters[0].status,
            Some(crate::types::container::ClusterStatus::Running)
        );
        assert_eq!(clusters[0].current_node_count, Some(3));
    }

    #[tokio::test]
    async fn test_list_clusters_empty() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/clusters")
            .returning_json(json!({}))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let clusters = client
            .container()
            .list_clusters("test-project", "us-central1")
            .await
            .unwrap();
        assert!(clusters.is_empty());
    }

    #[tokio::test]
    async fn test_get_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/locations/us-central1/clusters/my-cluster")
            .returning_json(json!({
                "name": "my-cluster",
                "location": "us-central1",
                "status": "RUNNING",
                "endpoint": "35.192.0.1",
                "currentMasterVersion": "1.28.3-gke.1200",
                "currentNodeCount": 3,
                "createTime": "2024-01-15T10:00:00Z",
                "network": "default",
                "subnetwork": "default"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let container = client.container();

        let cluster = container
            .get_cluster("test-project", "us-central1", "my-cluster")
            .await
            .unwrap();

        assert_eq!(cluster.name, "my-cluster");
        assert_eq!(cluster.location, Some("us-central1".to_string()));
        assert_eq!(cluster.endpoint, Some("35.192.0.1".to_string()));
        assert_eq!(cluster.current_node_count, Some(3));
        assert_eq!(cluster.network, Some("default".to_string()));
    }

    #[tokio::test]
    async fn test_delete_cluster_start() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/v1/projects/test-project/locations/us-central1/clusters/my-cluster")
            .returning_json(json!({
                "name": "operations/op-123",
                "status": "RUNNING",
                "operationType": "DELETE_CLUSTER",
                "selfLink": "https://container.googleapis.com/v1/projects/test-project/locations/us-central1/operations/op-123",
                "targetLink": "https://container.googleapis.com/v1/projects/test-project/locations/us-central1/clusters/my-cluster"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let container = client.container();

        let op = container
            .delete_cluster_start("test-project", "us-central1", "my-cluster")
            .await;
        assert!(op.is_ok());
    }

    // ── Initially-Done LRO Tests ──────────────────────────────────────

    #[tokio::test]
    async fn test_delete_cluster_already_done_skips_polling() {
        let mut mock = crate::MockClient::new();

        // GCP returns status:"DONE" for already-deleted cluster
        mock.expect_delete("/v1/projects/test-project/locations/us-central1/clusters/gone-cluster")
            .returning_json(json!({
                "name": "operations/op-done",
                "status": "DONE",
                "operationType": "DELETE_CLUSTER",
                "selfLink": "https://container.googleapis.com/v1/projects/test-project/locations/us-central1/operations/op-done"
            }))
            .times(1);

        // NO expect_get — if polling happens, the mock panics
        let client = crate::GcpHttpClient::from_mock(mock);

        let op = client
            .container()
            .delete_cluster_start("test-project", "us-central1", "gone-cluster")
            .await;
        assert!(op.is_ok());

        let wait_result = op.unwrap().wait().await;
        assert!(wait_result.is_ok());
    }
}
