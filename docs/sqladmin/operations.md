# Cloud SQL Admin Operations

## Instances

### list_instances

**Signature**: `pub async fn list_instances(project: &str) -> Result<Vec<DatabaseInstance>>`

Lists all Cloud SQL instances in a project.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |

**Returns**: `Result<Vec<DatabaseInstance>>`

---

### get_instance

**Signature**: `pub async fn get_instance(project: &str, instance: &str) -> Result<DatabaseInstance>`

Gets details of a specific instance.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |

**Returns**: `Result<DatabaseInstance>`

---

### create_instance / create_instance_start (LRO)

**Signature**: `pub async fn create_instance(project: &str, instance: &DatabaseInstance) -> Result<()>`

Creates a new Cloud SQL instance. Takes ~5-10 minutes.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&DatabaseInstance` | Instance configuration |

```rust
let op = sql.create_instance_start("my-project", &instance).await?;
op.wait().await?;
```

---

### delete_instance / delete_instance_start (LRO)

**Signature**: `pub async fn delete_instance(project: &str, instance: &str) -> Result<()>`

Deletes an instance. Instance names cannot be reused for ~1 week after deletion.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |

---

### update_instance / update_instance_start (LRO)

**Signature**: `pub async fn update_instance(project: &str, instance: &str, body: &DatabaseInstance) -> Result<()>`

Updates an instance's settings (tier, flags, etc.) using patch semantics.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |
| `body` | `&DatabaseInstance` | Fields to update |

---

### restart_instance / restart_instance_start (LRO)

**Signature**: `pub async fn restart_instance(project: &str, instance: &str) -> Result<()>`

Restarts a Cloud SQL instance.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |

---

## Instance Actions

### clone_instance / clone_instance_start (LRO)

**Signature**: `pub async fn clone_instance(project: &str, instance: &str, body: &InstancesCloneRequest) -> Result<()>`

Clones an instance to a new instance.

---

### failover_instance / failover_instance_start (LRO)

**Signature**: `pub async fn failover_instance(project: &str, instance: &str, body: &InstancesFailoverRequest) -> Result<()>`

Triggers a failover for a high-availability instance.

---

### promote_replica / promote_replica_start (LRO)

**Signature**: `pub async fn promote_replica(project: &str, instance: &str) -> Result<()>`

Promotes a read replica to a standalone primary instance.

---

### reset_ssl_config / reset_ssl_config_start (LRO)

**Signature**: `pub async fn reset_ssl_config(project: &str, instance: &str) -> Result<()>`

Resets the SSL configuration for an instance.

---

### import_instance / import_instance_start (LRO)

**Signature**: `pub async fn import_instance(project: &str, instance: &str, body: &InstancesImportRequest) -> Result<()>`

Imports data into an instance from a Cloud Storage file.

---

### export_instance / export_instance_start (LRO)

**Signature**: `pub async fn export_instance(project: &str, instance: &str, body: &InstancesExportRequest) -> Result<()>`

Exports data from an instance to a Cloud Storage file.

---

## Databases

### list_databases

**Signature**: `pub async fn list_databases(project: &str, instance: &str) -> Result<Vec<Database>>`

Lists databases in an instance.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |

**Returns**: `Result<Vec<Database>>`

---

### get_database

**Signature**: `pub async fn get_database(project: &str, instance: &str, database: &str) -> Result<Database>`

Gets a specific database.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |
| `database` | `&str` | Database name |

**Returns**: `Result<Database>`

---

### create_database / create_database_start (LRO)

**Signature**: `pub async fn create_database(project: &str, instance: &str, database: &Database) -> Result<()>`

Creates a database in an instance.

---

### delete_database / delete_database_start (LRO)

**Signature**: `pub async fn delete_database(project: &str, instance: &str, database: &str) -> Result<()>`

Deletes a database from an instance.

---

### update_database / update_database_start (LRO)

**Signature**: `pub async fn update_database(project: &str, instance: &str, database_name: &str, body: &Database) -> Result<()>`

Updates a database using patch semantics. Note: PostgreSQL does not allow changing charset/collation after creation.

---

## Users

### list_users

**Signature**: `pub async fn list_users(project: &str, instance: &str) -> Result<Vec<User>>`

Lists users in an instance.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `instance` | `&str` | Instance name |

**Returns**: `Result<Vec<User>>`

---

### get_user

**Signature**: `pub async fn get_user(project: &str, instance: &str, name: &str) -> Result<User>`

Gets a specific user.

---

### create_user / create_user_start (LRO)

**Signature**: `pub async fn create_user(project: &str, instance: &str, user: &User) -> Result<()>`

Creates a user in an instance.

---

### delete_user / delete_user_start (LRO)

**Signature**: `pub async fn delete_user(project: &str, instance: &str, name: &str, host: &str) -> Result<()>`

Deletes a user. For PostgreSQL, pass `""` for `host`.

---

### update_user / update_user_start (LRO)

**Signature**: `pub async fn update_user(project: &str, instance: &str, name: &str, host: &str, body: &User) -> Result<()>`

Updates a user (e.g., password change). For PostgreSQL, pass `""` for `host`.

---

## Operations

### list_operations

**Signature**: `pub async fn list_operations(project: &str) -> Result<Vec<OperationResponse>>`

Lists operations for a project.

---

### get_operation

**Signature**: `pub async fn get_operation(project: &str, operation: &str) -> Result<OperationResponse>`

Gets a specific operation by name.
