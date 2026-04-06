# Codegen Reference

## Tools

### bootstrap.py — Draft a New Manifest

```bash
python3 codegen/bootstrap.py {api_name} {version}
```

- Fetches the GCP Discovery Document for the API
- Auto-detects the LRO pattern
- Generates a draft manifest at `codegen/manifests/{api_name}.toml`
- Includes ALL schemas and operations from the discovery doc
- Developer then curates (selects which types/operations to include)

For APIs with non-standard discovery doc URLs, `CUSTOM_URLS` in bootstrap.py maps API names to URLs.

### codegen CLI — Generate and Apply Rust Code

```bash
# Generate + apply (primary workflow):
cd codegen && uv run python -m codegen.cli apply

# Single API:
cd codegen && uv run python -m codegen.cli apply --api compute

# Dry-run (preview changes):
cd codegen && uv run python -m codegen.cli apply --dry-run
```

The pipeline uses a multi-provider architecture:
1. **Provider plugin** reads the manifest and discovery doc, producing provider-agnostic IR
2. **Shared emitter** converts IR to Rust source code
3. **Apply** smart-merges generated code into `src/`

Generates to `codegen/generated/`:
- `types/{api}.rs` — Rust structs with serde, fixtures, tests
- `ops/{api}.rs` — HTTP operations with tests
- `api/{api}.rs` — API client scaffold (only used for NEW APIs)
- `test_support/{api}_mock_helpers.rs` — MockClient extension traits
- `registration/types_mod.rs` — Updated `src/types/mod.rs`
- `registration/ops_mod.rs` — Updated `src/ops/mod.rs`
- `registration/api_mod.rs` — Updated `src/api/mod.rs`
- `registration/client_accessors.rs` — Accessor methods for `src/client.rs`

Apply handles all copying and merging automatically:
1. **Overwrites** `src/types/`, `src/ops/`, `src/test_support/` (including `mod.rs`)
2. **Merges** new `pub mod` and `pub use` lines into `src/api/mod.rs` (never removes existing lines)
3. **Replaces** the accessor block in `src/client.rs` between marker comments
4. **Copies** API scaffolds only for new APIs not yet in `src/api/`

### extend.py — Browse, Add, and Diff Manifests

```bash
# Show what's available but not in the manifest
python3 codegen/extend.py compute --available-types
python3 codegen/extend.py compute --available-ops

# Generate ready-to-paste manifest entry
python3 codegen/extend.py compute --add-type Snapshot
python3 codegen/extend.py compute --add-op snapshots.insert

# Audit format:"byte" fields
python3 codegen/extend.py compute --audit
python3 codegen/extend.py --audit-all

# Diff manifest against discovery doc (detect upstream API changes)
python3 codegen/extend.py compute --diff
python3 codegen/extend.py --diff-all
```

- `--available-types`: Lists schemas in discovery doc not in the manifest
- `--available-ops`: Lists methods in discovery doc not in the manifest
- `--add-type <Name>`: Generates a `[[types]]` entry with all fields, overrides for reserved words, `format:"byte"` fields, and enum suggestions
- `--add-op <resource.method>`: Generates an `[[operations]]` entry with inferred `rust_name`, LRO detection, list response detection, and query params
- `--audit`: Reports `format:"byte"` fields missing `format = "bytes"` overrides
- `--audit-all`: Audits all manifests
- `--diff`: Compares manifest against discovery doc showing: removed schemas/fields/methods, new fields added upstream to schemas we use, new methods on resources we use, query param drift
- `--diff-all`: Diffs all manifests

### reference.py — Full Reference Manifests

```bash
python3 codegen/reference.py               # Generate all
python3 codegen/reference.py compute        # Generate one API
python3 codegen/reference.py --check        # Verify references are up-to-date
```

Generates `codegen/reference/{api}.full.toml` — one per API — containing **every** schema and operation from the discovery doc, fully annotated:

- All fields with type info, format, enums, `$ref` targets
- `[IN MANIFEST]` tags for schemas/operations already in the curated manifest
- Warnings for `format:"byte"` fields and reserved words
- Suggested field overrides
- Coverage summary (schemas and operations in manifest vs discovery)

These files are committed to git and serve as browseable references when extending manifests. Regenerate after fetching new discovery docs to catch upstream API changes.

### verify.py — Validate Manifests

```bash
python3 codegen/verify.py
```

- Validates all manifests against their discovery documents
- Checks schema existence, field references
- Reports coverage statistics
- Runs `cargo check` and `cargo test`

### fetch_discovery.py — Manage Discovery Doc Cache

```bash
python3 codegen/fetch_discovery.py           # Fetch all (skip cached)
python3 codegen/fetch_discovery.py --refresh  # Force re-download
python3 codegen/fetch_discovery.py --list     # List available APIs
```

## TOML Manifest Format

### [api] Section (Required)

```toml
[api]
name = "compute"                    # Module name (snake_case, used for file names)
display_name = "Compute Engine"     # Human-readable name
version = "v1"                      # API version
discovery_url = "https://..."       # GCP Discovery Document URL
base_url = "https://compute.googleapis.com/compute/v1"  # Base URL for API requests
doc_url = "https://cloud.google.com/compute/docs/reference/rest/v1"  # Reference docs
```

### [api.client] Section (Required)

```toml
[api.client]
accessor_name = "compute"           # Method name on GcpHttpClient (client.compute())
client_struct = "ComputeClient"     # Rust struct name for the API client
```

### [api.lro] Section (Optional — omit for sync-only APIs)

```toml
[api.lro]
pattern = "selflink"                # LRO polling pattern (see lro-patterns.md)
response_type = "OperationResponse" # Rust type for operation response
poll_config = "disk_operation"      # PollConfig preset method name

# Required for name_based patterns:
base_url = "https://serviceusage.googleapis.com"  # Base URL for polling
api_version = "v1"                  # API version for polling URL
```

### [api.path_helpers] Section (Optional)

```toml
[api.path_helpers]
service_account_name = "projects/{project}/serviceAccounts/{email}"
project_name = "projects/{project}"
key_name = "projects/{project}/serviceAccounts/{email}/keys/{key_id}"
```

Generates helper functions for constructing resource paths.

### [[types]] Array (Required — at least one)

Each `[[types]]` entry defines a type to generate.

```toml
[[types]]
schema = "Disk"                     # Discovery Document schema name (exact match)
rust_name = "OperationResponse"     # Optional: rename the Rust struct (default: schema name)
include_fields = [                  # Optional: whitelist of fields (omit for ALL fields)
    "name", "sizeGb", "type",
]

[types.field_overrides]             # Optional: per-field customizations
name = { required = true }                              # Make non-optional (String instead of Option<String>)
type = { rust_name = "disk_type", serde_rename = "type" }  # Rename reserved word
status = { enum_type = "DiskStatus" }                   # Generate typed enum
error = { rust_type = "serde_json::Value" }             # Force specific Rust type
items = { inline_struct = "MetadataItem" }              # Generate inline struct for nested object
body = { format = "bytes" }                             # Base64 encode/decode on wire

[types.omitted]                     # Optional: document why fields were excluded
sourceDisk = "Source disk cloning - uncommon operation"
```

#### Field Override Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `required` | `bool` | Make field non-optional (`String` instead of `Option<String>`) | `{ required = true }` |
| `rust_name` | `string` | Rename the Rust field (use with `serde_rename`) | `{ rust_name = "disk_type" }` |
| `serde_rename` | `string` | Set `#[serde(rename = "...")]` on the field | `{ serde_rename = "type" }` |
| `enum_type` | `string` | Generate a typed enum for this field | `{ enum_type = "DiskStatus" }` |
| `rust_type` | `string` | Force a specific Rust type | `{ rust_type = "serde_json::Value" }` |
| `inline_struct` | `string` | Generate an inline struct for nested objects | `{ inline_struct = "MetadataItem" }` |
| `format` | `string` | Wire format handling. `"bytes"` = base64 encode/decode | `{ format = "bytes" }` |

#### Common Field Override Patterns

**Base64-encoded byte fields** (body, data, etc. with `format: "byte"` in discovery doc):
```toml
body = { format = "bytes" }
data = { format = "bytes" }
```
Codegen emits `#[serde(serialize_with, deserialize_with)]` attributes that transparently base64-encode on serialization and decode on deserialization. Users work with raw strings; the wire format is handled automatically. The helpers live in `src/serde_base64.rs`.

**IMPORTANT**: Codegen now **warns** when a field has `format: "byte"` in the GCP Discovery Document but no `format = "bytes"` override in the manifest. These warnings must be reviewed when adding or extending APIs.

**Reserved words** (type, match, ref, etc.):
```toml
type = { rust_name = "disk_type", serde_rename = "type" }
interface = { rust_name = "interface_type", serde_rename = "interface" }
```

**Status enums**:
```toml
status = { enum_type = "DiskStatus" }
state = { enum_type = "ProjectState" }
```

**Required name fields**:
```toml
name = { required = true }
```

### [[operations]] Array (Required — at least one)

Each `[[operations]]` entry defines an HTTP operation to generate.

```toml
[[operations]]
discovery_resource = "disks"        # Resource path in discovery doc
discovery_method = "insert"         # Method name in discovery doc
rust_name = "create_disk"           # Rust function name
is_lro = true                       # Marks as long-running operation
description = "Create a disk"       # Optional: doc comment

# For nested resources, use dotted path instead of discovery_resource:
discovery_method = "projects.serviceAccounts.create"

# For list operations:
list_response = { type_name = "DiskList", items_field = "items", item_type = "Disk" }

# For operations with query parameters:
query_params = ["parent", "filter"]
```

#### Operation Fields

| Field | Required | Description |
|-------|----------|-------------|
| `discovery_resource` | One of this or dotted `discovery_method` | Resource path (e.g., "disks") |
| `discovery_method` | Yes | Method name (e.g., "insert") or dotted path (e.g., "projects.serviceAccounts.create") |
| `rust_name` | Yes | Rust function name (snake_case) |
| `is_lro` | No | Whether this is a long-running operation (default: false) |
| `description` | No | Doc comment for the generated function |
| `list_response` | No | For list operations: `{ type_name, items_field, item_type }` |
| `query_params` | No | Array of query parameter names to add to function signature |

#### `repeated: true` Query Params

Codegen handles `repeated: true` params natively. When a discovery doc marks a query param as `repeated: true`, codegen generates `&[&str]` instead of `&str` and builds the URL with loop-based repeated params (`?key=A&key=B`). Just include the param in the manifest's `query_params` like any other — no special handling needed in the API layer.

### Naming Conventions

| Thing | Convention | Example |
|-------|-----------|---------|
| Module name | snake_case | `cloud_sql`, `service_usage` |
| Client struct | PascalCase + "Client" | `CloudSqlClient`, `IamClient` |
| Accessor method | snake_case (short) | `client.cloud_sql()`, `client.iam()` |
| Operation function | snake_case verb_noun | `create_disk`, `list_instances` |
| LRO response type | PascalCase + "Lro" | `ServiceUsageLro`, `ProjectsLro` |
| Operation struct | PascalCase + "Operation" | `ServiceUsageOperation` |
| List response | PascalCase + "List" | `DiskList`, `InstanceList` |
| Enum type | PascalCase + field context | `DiskStatus`, `ProjectState` |

## Generated Code Patterns

### Fixture Methods

Every type gets:
```rust
#[cfg(any(test, feature = "test-support"))]
impl Disk {
    pub fn fixture() -> Self {
        Self {
            name: "test-disk".to_string(),
            size_gb: Some("100".to_string()),
            ..Default::default()
        }
    }
}
```

LRO types get additional fixtures:
```rust
impl ServiceUsageLro {
    pub fn fixture() -> Self { /* done: true */ }
    pub fn fixture_pending() -> Self { /* done: false */ }
    pub fn fixture_done() -> Self { /* done: true */ }
}
```

### Operation Unit Tests

Each operation gets a test:
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_create_disk() {
        let mut mock = MockClient::new();
        mock.expect_post("/compute/v1/projects/test-project/zones/us-central1-a/disks")
            .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap())
            .times(1);
        let client = GcpHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);
        let result = ops.create_disk("test-project", "us-central1-a", &Disk::fixture()).await;
        assert!(result.is_ok());
    }
}
```

### MockClient Helper Traits

Extension traits for ergonomic test setup:
```rust
pub trait ComputeMockHelpers {
    fn expect_create_disk(&mut self, project: &str, zone: &str) -> &mut MockExpectation;
    fn expect_get_disk(&mut self, project: &str, zone: &str, disk: &str) -> &mut MockExpectation;
    // ...
}
```
