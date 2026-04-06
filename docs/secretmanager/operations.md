# Secret Manager Operations

Complete reference for all Secret Manager operations.

## Secret CRUD

### create_secret

```rust
pub async fn create_secret(
    &self,
    project: &str,
    secret_id: &str,
    secret: &Secret,
) -> Result<Secret>
```

Creates a new Secret containing no SecretVersions.

**Parameters:**
- `project` - GCP project ID
- `secret_id` - The ID to assign (not the full resource name)
- `secret` - Secret metadata (must include `replication`)

**Returns:** The created Secret with server-populated fields (name, createTime)

**Example:**
```rust
use gcp_lite::types::secretmanager::{Automatic, Replication, Secret};
use std::collections::HashMap;

let replication = Replication {
    automatic: Some(Automatic::default()),
    ..Default::default()
};

let mut labels = HashMap::new();
labels.insert("env".to_string(), "prod".to_string());

let secret = Secret {
    labels,
    replication: Some(replication),
    ..Default::default()
};

let created = secrets.create_secret("my-project", "api-key", &secret).await?;
```

**Notes:**
- `replication` is required - the API will reject requests without it
- Secret IDs must be unique within a project
- Secret IDs can contain letters, numbers, hyphens, and underscores (max 255 chars)
- This only creates the metadata - use `add_secret_version` to store actual secret data (not yet implemented)

---

### get_secret

```rust
pub async fn get_secret(
    &self,
    project: &str,
    secret: &str,
) -> Result<Secret>
```

Gets metadata for a given Secret.

**Parameters:**
- `project` - GCP project ID
- `secret` - Secret ID (not the full resource name)

**Returns:** The Secret metadata

**Example:**
```rust
let secret = secrets.get_secret("my-project", "api-key").await?;
println!("Created: {:?}", secret.create_time);
println!("Labels: {:?}", secret.labels);
```

**Notes:**
- Returns only metadata, not the actual secret value
- Returns `NotFound` error if the secret doesn't exist

---

### list_secrets

```rust
pub async fn list_secrets(
    &self,
    project: &str,
    filter: Option<&str>,
    page_size: Option<&str>,
    page_token: Option<&str>,
) -> Result<ListSecretsResponse>
```

Lists Secrets in a project.

**Parameters:**
- `project` - GCP project ID
- `filter` - Optional filter expression (e.g., `"labels.env=prod"`)
- `page_size` - Optional page size (max 25000, default determined by server)
- `page_token` - Optional pagination token from previous response

**Returns:** ListSecretsResponse with secrets and pagination info

**Example:**
```rust
// List all secrets
let response = secrets.list_secrets("my-project", None, None, None).await?;
for secret in response.secrets {
    println!("Secret: {}", secret.name);
}

// List with filter
let response = secrets
    .list_secrets("my-project", Some("labels.env=prod"), Some("50"), None)
    .await?;

// Pagination
let mut page_token = None;
loop {
    let response = secrets
        .list_secrets("my-project", None, Some("10"), page_token.as_deref())
        .await?;

    for secret in response.secrets {
        println!("{}", secret.name);
    }

    match response.next_page_token {
        Some(token) if !token.is_empty() => page_token = Some(token),
        _ => break,
    }
}
```

**Notes:**
- Results are sorted by create_time (newest first)
- Empty `secrets` vec if no secrets match
- `total_size` is 0 when a filter is used

---

### patch_secret

```rust
pub async fn patch_secret(
    &self,
    project: &str,
    secret: &str,
    update_mask: &str,
    secret_data: &Secret,
) -> Result<Secret>
```

Updates metadata of an existing Secret.

**Parameters:**
- `project` - GCP project ID
- `secret` - Secret ID (not the full resource name)
- `update_mask` - Field mask specifying which fields to update (e.g., `"labels"`, `"labels,annotations"`)
- `secret_data` - Secret with updated fields (only fields in update_mask are applied)

**Returns:** The updated Secret

**Example:**
```rust
use std::collections::HashMap;

let mut labels = HashMap::new();
labels.insert("env".to_string(), "staging".to_string());

let update = Secret {
    labels,
    ..Default::default()
};

let updated = secrets
    .patch_secret("my-project", "api-key", "labels", &update)
    .await?;
```

**Notes:**
- Only fields listed in `update_mask` are modified
- Other fields are left unchanged (safe for concurrent updates)
- Update mask uses field names from the Secret proto (camelCase, not snake_case)
- Common masks: `"labels"`, `"annotations"`, `"rotation"`, `"expireTime"`
- `replication` cannot be changed after creation

---

### delete_secret

```rust
pub async fn delete_secret(
    &self,
    project: &str,
    secret: &str,
    etag: Option<&str>,
) -> Result<Empty>
```

Deletes a Secret and all of its versions.

**Parameters:**
- `project` - GCP project ID
- `secret` - Secret ID (not the full resource name)
- `etag` - Optional etag for optimistic concurrency control

**Returns:** Empty on success

**Example:**
```rust
// Simple delete
secrets.delete_secret("my-project", "old-api-key", None).await?;

// Delete with etag check
let secret = secrets.get_secret("my-project", "api-key").await?;
secrets.delete_secret("my-project", "api-key", secret.etag.as_deref()).await?;
```

**Notes:**
- Deletes the secret metadata AND all secret versions (data)
- If `etag` is provided, the delete only succeeds if the etag matches (prevents race conditions)
- Returns success even if the secret doesn't exist (idempotent)
- Deletion is immediate and cannot be undone
