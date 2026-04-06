# Backup for GKE Operations

## Backup Plans

### get_backup_plan

**Signature**: `pub async fn get_backup_plan(project: &str, location: &str, backup_plan: &str) -> Result<BackupPlan>`

Gets a backup plan by ID.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `backup_plan` | `&str` | Backup plan ID |

**Returns**: `Result<BackupPlan>`

---

### list_backup_plans

**Signature**: `pub async fn list_backup_plans(project: &str, location: &str) -> Result<Vec<BackupPlan>>`

Lists backup plans in a location.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |

**Returns**: `Result<Vec<BackupPlan>>`

---

### create_backup_plan

**Signature**: `pub async fn create_backup_plan(project: &str, location: &str, backup_plan_id: &str, backup_plan: &BackupPlan) -> Result<()>`

Creates a backup plan. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `backup_plan_id` | `&str` | ID for the new backup plan |
| `backup_plan` | `&BackupPlan` | Backup plan configuration |

**Returns**: `Result<()>`

---

### create_backup_plan_start (LRO)

**Signature**: `pub async fn create_backup_plan_start(project: &str, location: &str, backup_plan_id: &str, backup_plan: &BackupPlan) -> Result<GkeBackupOperation<'a>>`

Non-blocking variant. Returns an operation that can be polled manually.

```rust
let op = client.gkebackup()
    .create_backup_plan_start("my-project", "us-central1", "my-plan", &plan)
    .await?;

// With extended timeout for fresh clusters:
op.with_timeout(Duration::from_secs(900)).wait().await?;
```

---

### delete_backup_plan

**Signature**: `pub async fn delete_backup_plan(project: &str, location: &str, backup_plan: &str) -> Result<()>`

Deletes a backup plan. Blocks until the operation completes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `location` | `&str` | Location (e.g., "us-central1") |
| `backup_plan` | `&str` | Backup plan ID |

**Returns**: `Result<()>`

---

### delete_backup_plan_start (LRO)

**Signature**: `pub async fn delete_backup_plan_start(project: &str, location: &str, backup_plan: &str) -> Result<GkeBackupOperation<'a>>`

Non-blocking variant. Returns an operation that can be polled manually.

```rust
let op = client.gkebackup()
    .delete_backup_plan_start("my-project", "us-central1", "my-plan")
    .await?;
op.wait().await?;
```
