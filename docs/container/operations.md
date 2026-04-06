# Kubernetes Engine Operations

## Clusters

### list_clusters

**Signature**: `pub async fn list_clusters(project: &str, location: &str) -> Result<Vec<Cluster>>`

Lists clusters in a location. Use `"-"` as location to list across all locations.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") or "-" for all |

**Returns**: `Result<Vec<Cluster>>`

---

### get_cluster

**Signature**: `pub async fn get_cluster(project: &str, location: &str, cluster: &str) -> Result<Cluster>`

Gets a cluster by name.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `cluster` | `&str` | Cluster name |

**Returns**: `Result<Cluster>`

---

### delete_cluster

**Signature**: `pub async fn delete_cluster(project: &str, location: &str, cluster: &str) -> Result<()>`

Deletes a cluster. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `cluster` | `&str` | Cluster name |

**Returns**: `Result<()>`

---

### delete_cluster_start (LRO)

**Signature**: `pub async fn delete_cluster_start(project: &str, location: &str, cluster: &str) -> Result<ContainerOperation<'a>>`

Non-blocking variant. Returns an operation that can be polled manually.

```rust
let op = client.container().delete_cluster_start("my-project", "us-central1", "my-cluster").await?;
// Do other work...
op.wait().await?;

// Or with custom timeout:
let op = client.container().delete_cluster_start("my-project", "us-central1", "my-cluster").await?;
op.with_timeout(Duration::from_secs(900)).wait().await?;
```
