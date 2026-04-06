# Kubernetes Engine Usage Examples

## List Clusters

### All locations

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let container = client.container();

let clusters = container.list_clusters("my-project", "-").await?;
for cluster in &clusters {
    println!("{} (location={:?}, status={:?})",
        cluster.name, cluster.location, cluster.status);
}
```

### Specific region

```rust
let clusters = container.list_clusters("my-project", "us-central1").await?;
println!("Found {} clusters in us-central1", clusters.len());
```

## Get Cluster Details

```rust
let cluster = container.get_cluster("my-project", "us-central1", "my-cluster").await?;
println!("Name: {}", cluster.name);
println!("Status: {:?}", cluster.status);
println!("Endpoint: {:?}", cluster.endpoint);
println!("Version: {:?}", cluster.current_master_version);
println!("Node count: {:?}", cluster.current_node_count);
println!("Network: {:?}", cluster.network);
```

## Delete a Cluster

### Blocking (waits for completion)

```rust
container.delete_cluster("my-project", "us-central1", "my-cluster").await?;
println!("Cluster deleted");
```

### Non-blocking (manual polling)

```rust
let op = container.delete_cluster_start("my-project", "us-central1", "my-cluster").await?;
// Do other cleanup work...
op.wait().await?;
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::types::container::{Cluster, ClusterStatus};
use serde_json::json;

#[tokio::test]
async fn test_list_clusters() {
    let mut mock = MockClient::new();

    mock.expect_get("/v1/projects/test-project/locations/-/clusters")
        .returning_json(json!({
            "clusters": [{
                "name": "my-cluster",
                "location": "us-central1",
                "status": "RUNNING",
                "currentMasterVersion": "1.28.3-gke.1200",
                "currentNodeCount": 3
            }]
        }))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let clusters = client.container().list_clusters("test-project", "-").await.unwrap();

    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].name, "my-cluster");
    assert_eq!(clusters[0].status, Some(ClusterStatus::Running));
}
```
