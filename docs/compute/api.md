# Compute Engine API

## Overview

The Compute Engine API allows you to create and manage virtual machines, disks, snapshots, and networking resources on Google Cloud.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let compute = client.compute();
```

## Features

- **Instances**: Create, start, stop, reset, and delete VMs
- **Disks**: Manage persistent disks (create, delete, resize)
- **Snapshots**: Create and manage disk snapshots
- **Addresses**: Reserve and release static IP addresses
- **Routers**: Manage Cloud Routers and NAT gateways
- **Backend Services**: Manage load balancing backend services (global & regional)
- **LRO Support**: Full support for long-running operations (blocking or manual polling)

## Types

| Type | Description |
|------|-------------|
| `Instance` | Virtual machine instance |
| `Disk` | Persistent disk |
| `Snapshot` | Disk snapshot |
| `Address` | Static IP address |
| `Router` | Cloud Router configuration |
| `BackendService` | Load balancer backend service |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - resource doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::QuotaExceeded` - exceeded project quotas (e.g., CPUs, IPs)
