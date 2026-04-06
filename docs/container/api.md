# Kubernetes Engine (GKE) API

## Overview

The Kubernetes Engine API manages GKE clusters. This client provides methods for listing, inspecting, and deleting clusters. Cluster creation is not included (use `gcloud container clusters create-auto` or Terraform).

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let container = client.container();
```

## Features

- **List clusters**: Across all locations or in a specific region/zone
- **Get cluster details**: Status, endpoint, version, node count, network config
- **Delete clusters**: With LRO polling (blocking + `_start` variants)

## LRO Pattern

**Pattern**: selfLink (status-string polling)

GKE Container operations return a short operation name and a `selfLink` URL. Polling uses the `selfLink` URL and checks `status == "DONE"` (same status-string pattern as Compute Engine).

For LRO operations, two variants are available:
- `delete_cluster()` - blocks until the operation completes
- `delete_cluster_start()` - returns a `ContainerOperation` for manual polling

## Types

| Type | Description |
|------|-------------|
| `Cluster` | GKE cluster with status, endpoint, version, network config |
| `ClusterStatus` | Cluster state enum (Running, Provisioning, Stopping, etc.) |
| `ListClustersResponse` | List response wrapper |
| `ContainerLro` | Operation response with selfLink for polling |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - cluster doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::ApiNotEnabled` - Container API not enabled on project
- `GcpError::InvalidArgument` - cluster has an incompatible operation running
