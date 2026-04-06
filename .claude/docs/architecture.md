# Architecture Reference

## Project Structure

gcp-lite-rs is a standalone Rust crate for GCP API access:

```
gcp-lite/
├── Cargo.toml
├── src/
│   ├── types/*.rs          # Generated types (codegen)
│   ├── ops/*.rs            # Generated HTTP operations (codegen)
│   ├── test_support/*.rs   # Generated mock helpers (codegen)
│   ├── api/*.rs            # Hand-written API wrappers
│   ├── client.rs           # GcpHttpClient
│   ├── operation.rs        # LRO polling
│   └── lib.rs              # Crate root
├── codegen/                # Codegen pipeline (manifests, scripts)
├── tests/integration/      # Integration tests
└── docs/                   # Per-API documentation
```

## Three-Layer Design

The crate uses a strict three-layer architecture where each layer has clear ownership and responsibilities.

### Layer 1: Types (`src/types/*.rs`)

**Owner**: Codegen (never edit manually)

Generated Rust structs and enums from GCP Discovery Document schemas.

Properties:
- Derive `Default`, `Serialize`, `Deserialize`, `Debug`, `Clone`
- `#[serde(rename_all = "camelCase")]` on all structs
- `Option<T>` fields get `#[serde(skip_serializing_if = "Option::is_none")]`
- `Vec<T>` fields get `#[serde(default)]` + `#[serde(skip_serializing_if = "Vec::is_empty")]`
- Enums use `#[serde(other)] Unknown` variant for forward-compat
- Every type gets a `fixture()` method for test data (feature-gated)
- LRO response types get `fixture_pending()` and `fixture_done()`

Example:
```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub name: String,  // required fields are non-Optional

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}
```

### Layer 2: Ops (`src/ops/*.rs`)

**Owner**: Codegen (never edit manually)

Raw HTTP operations with correct URLs, methods, and parameter ordering.

Properties:
- Visibility: `pub(crate)` (internal only, not part of public API)
- Each ops struct wraps a `&GcpHttpClient` reference
- Functions construct URLs, serialize bodies, call HTTP methods, deserialize responses
- Each operation gets a unit test validating URL/method/serialization
- `#[cfg(test)]` base_url override for mock testing

Example:
```rust
pub(crate) struct ComputeOps<'a> {
    pub(crate) client: &'a GcpHttpClient,
}

impl<'a> ComputeOps<'a> {
    pub(crate) async fn create_disk(
        &self, project: &str, zone: &str, body: &Disk
    ) -> Result<OperationResponse> {
        let url = format!("{}/projects/{}/zones/{}/disks",
            Self::base_url(), project, zone);
        let response = self.client.post(&url, body).await?;
        Ok(serde_json::from_slice(&response)?)
    }
}
```

### Layer 3: API (`src/api/*.rs`)

**Owner**: Developer (hand-written, scaffolded once by codegen)

Ergonomic wrappers that provide the public interface.

Responsibilities:
- Resource name path construction (e.g., `format!("projects/{}/serviceAccounts/{}", project, email)`)
- LRO polling: dual methods for long-running operations
- Convenience methods (e.g., `add_iam_policy_binding` does read-modify-write)
- Section-grouped methods with separator comments

Structure:
```rust
pub struct IamClient<'a> {
    ops: IamOps<'a>,
}

impl<'a> IamClient<'a> {
    pub(crate) fn new(client: &'a GcpHttpClient) -> Self {
        Self { ops: IamOps::new(client) }
    }

    // ── Service Accounts ─────────────────────────────────────────────

    pub async fn get_service_account(
        &self, project: &str, email: &str,
    ) -> Result<ServiceAccount> {
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        self.ops.get_service_account(&name).await
    }
}
```

### Layer 4: Test Support (`src/test_support/*.rs`)

**Owner**: Codegen (never edit manually)

MockClient extension traits that provide ergonomic test setup.

Properties:
- Feature-gated behind `#[cfg(any(test, feature = "test-support"))]`
- One trait per API (e.g., `ComputeMockHelpers`)
- Methods like `mock.expect_create_disk("project", "zone")` eliminate manual URL construction
- Return a builder that supports `.returning_json()`, `.returning_json_sequence()`, `.times(n)`

## Module Registration

When a new API is added, it must be registered in three places:

### 1. `src/api/mod.rs`
```rust
pub mod compute;
pub mod iam;
pub mod new_api;  // <-- add module

pub use compute::ComputeClient;
pub use iam::IamClient;
pub use new_api::NewApiClient;  // <-- re-export client struct
```

### 2. `src/client.rs` (accessor method)
```rust
impl GcpHttpClient {
    /// Access the New API
    pub fn new_api(&self) -> crate::api::NewApiClient<'_> {
        crate::api::NewApiClient::new(self)
    }
}
```

### 3. `src/lib.rs` (if new Operation type)
```rust
pub use operation::{Operation, PollConfig, ServiceUsageOperation, NewApiOperation};
```

## Error Handling

`GcpError` enum (uses `thiserror` v2, derives `Clone`) with variants:
- `Auth` — 401 Unauthorized
- `PermissionDenied` — 403 Forbidden
- `NotFound` — 404 Not Found
- `RateLimited` — 429 Too Many Requests (with `retry_after`)
- `ApiNotEnabled` — 403 with "API not enabled" message
- `QuotaExceeded` — Quota/billing errors
- `InvalidArgument` — 400 Bad Request
- `ServerError` — 5xx errors (with `retryable` flag)
- `OperationTimeout` — LRO polling exceeded timeout
- `OperationFailed` — LRO completed with error
- `Network` — Connection/DNS failures
- `InvalidResponse` — Unparseable response

Key methods: `is_retryable()`, `status_code()`, `retry_after()`

## Codegen Pipeline

See `.claude/docs/codegen-reference.md` for the full pipeline, manifest format, and tools.
