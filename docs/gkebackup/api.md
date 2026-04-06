# Backup for GKE API

## Overview

The Backup for GKE API manages backup plans for GKE clusters. Backup plans define what to back up, how often, and how long to retain backups. This client provides full CRUD operations for backup plans.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let backup = client.gkebackup();
```

## Features

- **Backup plan CRUD**: Create, read, list, and delete backup plans
- **LRO support**: Create and delete operations use long-running operations
- **Flexible configuration**: All-namespace or selected-namespace backups, retention policies, schedules

## LRO Pattern

**Pattern**: name_based (standard Google LRO)

GKE Backup operations use the standard Google LRO pattern, polling via `gkebackup.googleapis.com/v1/{name}` and checking `done == true`.

For LRO operations, two variants are available:
- `create_backup_plan()` / `delete_backup_plan()` - block until complete
- `create_backup_plan_start()` / `delete_backup_plan_start()` - return a `GkeBackupOperation` for manual polling

**Note**: On fresh Autopilot clusters, the first backup plan creation can take 10-15 minutes while the backup agent is installed. Use `.with_timeout()` if the default 600s isn't enough.

## Types

| Type | Description |
|------|-------------|
| `BackupPlan` | Backup plan with cluster reference, config, schedule, retention |
| `BackupPlanState` | Plan state enum (Ready, Provisioning, Deactivated, etc.) |
| `BackupConfig` | What to back up (all namespaces or selected) |
| `BackupSchedule` | Cron schedule for automated backups |
| `RetentionPolicy` | How long to keep backups |
| `ListBackupPlansResponse` | List response wrapper |
| `GkeBackupLro` | Operation response for LRO polling |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - backup plan doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::ApiNotEnabled` - GKE Backup API not enabled on project
- `GcpError::OperationFailed` - cluster in Error/Degraded/Stopping state
- `GcpError::OperationTimeout` - LRO exceeded timeout (increase with `.with_timeout()`)
