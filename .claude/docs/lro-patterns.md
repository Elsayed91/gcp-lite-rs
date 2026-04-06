# GCP LRO (Long-Running Operation) Patterns

## Overview

GCP APIs use different patterns for long-running operations (LROs). Each pattern requires a different polling mechanism. This document catalogs all known patterns and how to implement them.

## Pattern Catalog

### 1. `selflink` — Compute Engine Style

**Used by**: Compute Engine

**How it works**:
- Operation response includes a `selfLink` URL
- Poll by GET-ing the selfLink URL directly
- Done when `status == "DONE"`
- Error in `error.errors[0].message`

**Manifest config**:
```toml
[api.lro]
pattern = "selflink"
response_type = "OperationResponse"
poll_config = "disk_operation"
```

**Response format**:
```json
{
  "name": "operation-123",
  "status": "RUNNING",
  "selfLink": "https://compute.googleapis.com/compute/v1/projects/my-project/zones/us-central1-a/operations/operation-123",
  "targetLink": "https://compute.googleapis.com/compute/v1/projects/my-project/zones/us-central1-a/disks/my-disk"
}
```

**Implementation**: Uses `Operation` struct in `src/operation.rs`. Polls via the full selfLink URL.

**Required type in manifest**:
```toml
[[types]]
schema = "Operation"
rust_name = "OperationResponse"
include_fields = ["name", "operationType", "status", "targetLink", "selfLink"]
```

### 2. `name_based` — Google LRO Style

**Used by**: Service Usage, Cloud Resource Manager, Cloud Functions, Cloud Build

**How it works**:
- Operation response includes a `name` field (e.g., `operations/acf.123`)
- Poll by GET-ing `https://{service}.googleapis.com/{version}/{name}`
- Done when `done == true`
- Error in `error` object (standard Google RPC Status)

**Manifest config**:
```toml
[api.lro]
pattern = "name_based"
response_type = "ServiceUsageLro"
poll_config = "service_usage_operation"
base_url = "https://serviceusage.googleapis.com"
api_version = "v1"
```

**Response format**:
```json
{
  "name": "operations/acf.123456",
  "done": false,
  "metadata": { "@type": "..." }
}
```

When done:
```json
{
  "name": "operations/acf.123456",
  "done": true,
  "response": { "@type": "...", ... }
}
```

**Implementation**: Each API gets its own Operation struct (e.g., `ServiceUsageOperation`, `ResourceManagerOperation`) to avoid name collisions and because each has a different base URL.

**Required type in manifest**:
```toml
[[types]]
schema = "Operation"
rust_name = "ServiceUsageLro"  # Unique name to avoid collision
include_fields = ["name", "done", "error", "metadata", "response"]
```

### 3. `project_name` — Cloud SQL Style

**Used by**: Cloud SQL

**How it works**:
- Operation response includes `name` and `targetProject`
- Poll by GET-ing `https://sqladmin.googleapis.com/v1/projects/{project}/operations/{name}`
- Done when `status == "DONE"`

**Manifest config**:
```toml
[api.lro]
pattern = "project_name"
response_type = "SqlOperation"
poll_config = "sql_operation"
base_url = "https://sqladmin.googleapis.com"
api_version = "v1"
```

**Response format**:
```json
{
  "name": "e3b0c442-98fc-1c14-b39f-4c2426d92042",
  "status": "RUNNING",
  "targetProject": "my-project",
  "operationType": "DELETE"
}
```

**Polling URL**: `https://sqladmin.googleapis.com/v1/projects/{targetProject}/operations/{name}`

### 4. `project_location_name` — Regional Operations Style

**Used by**: GKE, Cloud Run, Cloud Functions (v2), Cloud Scheduler (some ops)

**How it works**:
- Operation response includes `name` in format `projects/{project}/locations/{location}/operations/{id}`
- Poll by GET-ing `https://{service}.googleapis.com/{version}/{name}`
- Done when `done == true` or `status == "DONE"` (varies by API)

**Manifest config**:
```toml
[api.lro]
pattern = "project_location_name"
response_type = "GkeOperation"
poll_config = "gke_operation"
base_url = "https://container.googleapis.com"
api_version = "v1"
```

### 5. `none` — No LROs (All Synchronous)

**Used by**: IAM, Billing, Recommender, Cloud Asset

**How it works**: All operations return immediately. No operation polling needed.

**Manifest config**: Simply omit the `[api.lro]` section entirely.

## Initially-Done LROs (CRITICAL)

**Every LRO helper MUST check if the initial response is already done before constructing a polling operation.** GCP returns `done: true` (or `status: "DONE"`) from the initial POST when the operation is a no-op — for example, enabling an already-enabled service, or deleting an already-deleted resource.

When this happens, the operation `name` in the response may be a placeholder (e.g., `"DONE_OPERATION"`) that GCP will reject if you try to poll it. The `selfLink` may also be missing or invalid.

### What the API helper MUST do

For **name-based LROs** (`done: bool` + `error: Option<Value>`):
```rust
fn lro_operation(&self, lro: ServiceUsageLro) -> Result<ServiceUsageOperation<'a>> {
    // 1. If done + error -> fail fast
    if lro.done
        && let Some(error) = &lro.error
    {
        return Err(/* extract error message and code */);
    }
    // 2. Pass lro.done to the operation constructor
    Ok(ServiceUsageOperation::new(
        self.ops.client,
        lro.name,
        config.initial_interval(),
        config.timeout(),
        lro.done,  // <- short-circuits wait() if true
    ))
}
```

For **status-based LROs** (`status: Option<String>`):
```rust
fn zonal_operation(&self, op: OperationResponse) -> Result<Operation<'a>> {
    let initially_done = op.status.as_deref() == Some("DONE");
    // ... construct Operation with initially_done flag
}
```

### What the Operation struct MUST do

Every operation struct has an `initially_done: bool` field. `wait()` short-circuits:
```rust
pub async fn wait(self) -> Result<()> {
    if self.initially_done {
        return Ok(());
    }
    poll_operation(&self, ...).await
}
```

### When this matters

Any idempotent GCP operation can return an immediately-done LRO:
- **Enable API** on an already-enabled service
- **Delete** an already-deleted resource (some APIs)
- **Create** when the resource already exists in desired state (some APIs)
- **Any mutate** that results in no actual change

### Unit test requirement

Every API with LROs MUST have a unit test for the already-done case:
```rust
#[tokio::test]
async fn test_enable_already_enabled_service_succeeds() {
    let mut mock = MockClient::new();
    // Return done:true immediately — no polling should happen
    mock.expect_post("/v1/projects/my-project/services/compute.googleapis.com:enable")
        .returning_json(json!({"name": "DONE_OPERATION", "done": true}))
        .times(1);
    // NO expect_get — if polling happens, the test fails
    let client = GcpHttpClient::from_mock(mock);
    let result = client.service_usage()
        .enable_service("my-project", "compute.googleapis.com").await;
    assert!(result.is_ok());
}
```

## How to Identify an API's LRO Pattern

1. Check the GCP REST API reference for the API
2. Look at any `create`, `delete`, `update`, or `patch` method
3. Check the response type:
   - If it returns an `Operation` with `selfLink` -> `selflink`
   - If it returns an `Operation` with `name` and `done` fields -> `name_based`
   - If operations have a project-scoped endpoint -> `project_name`
   - If operations have a regional endpoint -> `project_location_name`
   - If all methods return the resource directly -> `none`
4. Alternatively, run `python3 codegen/bootstrap.py {api} {version}` and check the auto-detected pattern

## Adding a New LRO Type to operation.rs

When adding an API with a new LRO pattern (or same pattern but different base URL), create a new type in `src/operation.rs`:

### Step 1: Define the struct
```rust
/// A long-running {ApiName} operation.
///
/// Polls via `{base_url}/{version}/{name}` until `done == true`.
pub struct {ApiName}Operation<'a> {
    client: &'a GcpHttpClient,
    operation_name: String,
    initial_interval: Duration,
    timeout: Duration,
    initially_done: bool,
}
```

### Step 2: Implement constructor + wait + build_url
```rust
impl<'a> {ApiName}Operation<'a> {
    pub fn new(
        client: &'a GcpHttpClient,
        operation_name: String,
        initial_interval: Duration,
        timeout: Duration,
        initially_done: bool,
    ) -> Self {
        Self { client, operation_name, initial_interval, timeout, initially_done }
    }

    pub async fn wait(self) -> Result<()> {
        if self.initially_done {
            return Ok(());
        }
        poll_operation(&self, self.client, self.initial_interval, self.timeout).await
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build_url(&self, path: &str) -> String {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return format!("{}{}", base.trim_end_matches('/'), path);
            }
        }
        format!("https://{service}.googleapis.com{}", path)
    }
}
```

### Step 3: Implement OperationPoller trait
```rust
impl<'a> OperationPoller for {ApiName}Operation<'a> {
    fn build_poll_url(&self, _client: &GcpHttpClient) -> String {
        self.build_url(&format!("/{version}/{}", self.operation_name))
    }

    fn parse_status(&self, response: &[u8]) -> Result<PollStatus> {
        // For name_based pattern, use LroStatus (done/error)
        // For selflink pattern, use ComputeOperationStatus (status/error)
        let status: LroStatus = serde_json::from_slice(response)
            .map_err(|e| GcpError::InvalidResponse {
                message: format!("Failed to parse operation status: {}", e),
                body: Some(String::from_utf8_lossy(response).to_string()),
            })?;

        if status.done {
            if let Some(error) = status.error {
                let message = error.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error").to_string();
                let code = error.get("code")
                    .and_then(|v| v.as_str().map(String::from)
                        .or_else(|| v.as_i64().map(|n| n.to_string())));
                Ok(PollStatus::Failed { message, code })
            } else {
                Ok(PollStatus::Done)
            }
        } else {
            Ok(PollStatus::InProgress)
        }
    }

    fn operation_id(&self) -> &str {
        &self.operation_name
    }
}
```

### Step 4: Add PollConfig preset
```rust
impl PollConfig {
    /// Configuration for {api_name} operations ({timeout} seconds).
    pub fn {api_name}_operation() -> Self {
        Self {
            initial_interval: Duration::from_secs(1),
            timeout: Duration::from_secs(300),
        }
    }
}
```

### Step 5: Export from lib.rs
```rust
pub use operation::{Operation, PollConfig, ServiceUsageOperation, {ApiName}Operation};
```

## Existing PollConfig Presets

| Preset | Initial Interval | Timeout | Used By |
|--------|-----------------|---------|---------|
| `disk_operation()` | 1s | 5 min | Compute Engine |
| `service_usage_operation()` | 500ms | 2 min | Service Usage |
| `project_operation()` | 2s | 10 min | Resource Manager |

## Naming Conventions

- LRO type in manifest: `{ApiDisplay}Lro` (e.g., `ServiceUsageLro`, `ProjectsLro`)
- Rust operation struct: `{ApiName}Operation` (e.g., `ServiceUsageOperation`)
- PollConfig preset: `{api_name}_operation()` (e.g., `service_usage_operation()`)
- Exception: Compute uses `OperationResponse` (historical) and `Operation` struct
