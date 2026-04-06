# Secret Manager API

API client for Google Cloud Secret Manager v1.

## Overview

The Secret Manager API provides secure storage and management of sensitive data like API keys, passwords, and certificates. This implementation covers secret metadata CRUD operations (create, read, update, delete).

## Client Access

```rust
use gcp_lite::GcpHttpClient;

let client = GcpHttpClient::from_adc().await?;
let secrets = client.secret_manager();
```

## Client Struct

```rust
pub struct SecretManagerClient<'a>
```

Created via `GcpHttpClient::secret_manager()`. Wraps `SecretmanagerOps` from the ops layer.

## LRO Pattern

**None** - All Secret CRUD operations are synchronous and return immediately.

## Type System

All types are in `gcp_lite::types::secretmanager`.

### Core Types

#### Secret
```rust
pub struct Secret {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub etag: Option<String>,
    pub replication: Option<Replication>,
    pub create_time: Option<String>,
    pub expire_time: Option<String>,
    pub rotation: Option<Rotation>,
}
```

The main resource type representing secret metadata. **Does not contain the actual secret data** - that's stored in SecretVersions (not yet implemented in this client).

**Key fields:**
- `name` - Resource name: `projects/{project}/secrets/{secret}`
- `labels` - User-defined key-value pairs for organization
- `replication` - **Required on create** - automatic or user-managed replication
- `etag` - Used for optimistic concurrency control

#### Replication
```rust
pub struct Replication {
    pub automatic: Option<Automatic>,
    pub user_managed: Option<UserManaged>,
}
```

Defines where secret data is replicated. Exactly one of `automatic` or `user_managed` must be set.

#### Automatic
```rust
pub struct Automatic {
    pub customer_managed_encryption: Option<CustomerManagedEncryption>,
}
```

Automatic replication across all GCP regions. Simplest option for most use cases.

#### UserManaged
```rust
pub struct UserManaged {
    pub replicas: Vec<Replica>,
}
```

Explicit control over which regions store the secret.

#### ListSecretsResponse
```rust
pub struct ListSecretsResponse {
    pub secrets: Vec<Secret>,
    pub next_page_token: Option<String>,
    pub total_size: Option<i32>,
}
```

Response from `list_secrets`. Contains paginated results.

#### Empty
```rust
pub struct Empty {}
```

Response from `delete_secret`.

## Resource Names

Secrets use hierarchical resource names:

```
projects/{project}/secrets/{secret}
```

The API layer accepts separate `project` and `secret` parameters and constructs the full resource name internally.

**Note:** GCP may return project numbers instead of project IDs in resource names (e.g., `projects/123456789/secrets/my-secret` instead of `projects/my-project/secrets/my-secret`). Both are valid.
