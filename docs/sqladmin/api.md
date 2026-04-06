# Cloud SQL Admin API

## Overview

The Cloud SQL Admin API manages Cloud SQL instances, databases, and users. This client provides full CRUD operations for all three resource types, plus instance actions like restart, clone, failover, and import/export.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let sql = client.sqladmin();
```

## Features

- **Instance management**: Create, get, list, update, delete, restart
- **Instance actions**: Clone, failover, promote replica, reset SSL, import, export
- **Database CRUD**: Create, get, list, update, delete databases within an instance
- **User CRUD**: Create, get, list, update, delete users within an instance
- **Operation management**: List and inspect operations
- **LRO support**: All mutating operations have blocking + `_start` variants

## LRO Pattern

**Pattern**: project_name (polls via `/v1/projects/{project}/operations/{name}`)

Cloud SQL operations use a `status` string field. Polling checks `status == "DONE"`. Timeout is 15 minutes with 2-second initial polling interval.

For LRO operations, two variants are available:
- `create_instance()` - blocks until the operation completes
- `create_instance_start()` - returns a `SqlOperation` for manual polling

## Types

| Type | Description |
|------|-------------|
| `DatabaseInstance` | Cloud SQL instance with settings, version, region, state |
| `Settings` | Instance configuration (tier, disk, backup, IP, maintenance) |
| `Database` | Database within an instance |
| `User` | User account within an instance |
| `OperationResponse` | Operation status for LRO polling |
| `SqlInstanceType` | Instance type enum (CloudSqlInstance, ReadReplica, etc.) |
| `SqlBackendType` | Backend type enum (FirstGen, SecondGen, External) |
| `InstanceState` | Instance state enum (Runnable, Suspended, Maintenance, etc.) |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - instance, database, or user doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::ApiNotEnabled` - Cloud SQL Admin API not enabled on project
- `GcpError::OperationFailed` - LRO completed with an error (e.g., quota exceeded)
- `GcpError::InvalidArgument` - invalid instance configuration or parameters
