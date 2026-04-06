# GCP Testing Guide

## Integration-First Development

**Core principle**: Integration tests drive API development. Unit tests encode proven behavior.

For the shared cross-provider methodology (structure, fixture management, CLI helpers, checklist), see **[`integration-testing-methodology.md`](integration-testing-methodology.md)**. This guide covers GCP-specific patterns.

### Why Integration-First?

GCP APIs have quirks that are impossible to predict from discovery docs alone:
- Some create operations require body fields that seem redundant with URL parameters (GCS copy needs `bucket` + `name` in body)
- Some operations use different base URLs than the rest of the API (GCS uploads: `/upload/storage/v1` vs `/storage/v1`)
- Some APIs reject empty query parameter values (`?generation=` -> 400 error)
- Path parameter encoding varies between APIs (`{param}` vs `{+param}`)

Writing unit tests based on assumptions about these behaviors leads to tests that pass against mocks but fail against the real API. Then you debug both the implementation AND the tests simultaneously.

### The Workflow

For each operation group (cluster of related operations on one resource type):

1. **Write API methods** for this group only
2. **Write integration test** that exercises the real GCP API
3. **Run the integration test** — fix failures until it passes
4. **Write unit tests** that encode the now-proven-correct behavior (URLs, bodies, params)
5. **Commit** this group before moving to the next

### Layer-by-Layer Integration Testing

Integration-test each operation independently against the real GCP API before composing them. This prevents bugs from compounding:

1. **Integration-test each operation in isolation first** — verify create, get, list, delete each separately against the real API
2. **Then integration-test combinations** — create -> get -> list -> delete lifecycle
3. **Use `gcloud` CLI freely** for setup/teardown when the API method doesn't exist yet. This is incremental — don't block testing operation B because operation A isn't implemented. Use `gcloud` to create prerequisites.

### Operation Group Ordering

Order groups from simplest to most complex:
- **First**: Basic CRUD (create, get, list, delete) — these are foundational
- **Then**: IAM/policy operations — build on CRUD for setup
- **Then**: Advanced operations (copy, rewrite, compose) — build on basic CRUD
- **Last**: LRO operations — most complex, benefit from all previous groups being solid

Each group's integration test can use previously-validated groups for setup/teardown. For operations not yet implemented, use gcloud CLI as fallback.

### Example: Storage API Groups

```
Group 1: Bucket CRUD -> gcloud for cleanup fallback
Group 2: Bucket IAM -> uses Group 1's create_bucket/delete_bucket
Group 3: Object basics -> uses Group 1 for bucket setup, gcloud for object cleanup
Group 4: Object advanced -> uses Groups 1-3 for setup
```

### Edge Cases That MUST Be Integration-Tested

These are the cases that mock tests CANNOT catch. A mock test encodes what the developer *thinks* is correct. Only a real API call reveals whether it actually is.

#### Multi-Value Parameters
For any list/filter/search parameter that accepts multiple values:
- **Test with 0 values** (omitted)
- **Test with 1 value** (may work by accident even if serialization is wrong)
- **Test with 2+ values** — this is the critical case that catches serialization bugs

**Real-world example**: Cloud Asset `list_assets` requires `assetTypes` as repeated query params (`?assetTypes=A&assetTypes=B`). Comma-joining (`?assetTypes=A,B`) silently fails. A single-value test (`?assetTypes=A`) passes fine either way. Only a 2+ value integration test catches this.

#### Discovery Doc `repeated: true` Params
Before writing any integration test, check the discovery doc for query params marked `"repeated": true`. These REQUIRE repeated query params and CANNOT use comma-separated values. The codegen only supports single-value params, so these must be:
1. **Omitted from the manifest** `query_params`
2. **Handled in the API layer** with manual URL construction
3. **Integration-tested with 2+ values** to verify correctness

See `src/api/cloudasset.rs:list_assets_raw` for the reference implementation.

#### Idempotent / No-Op Operations (CRITICAL)

Any GCP operation that can be a no-op MUST be tested in its no-op state. GCP returns immediately-done LROs with placeholder operation names when nothing changes. If the code blindly polls these, it crashes.

For every LRO operation, test:
- **The already-in-desired-state case** — e.g., enabling an already-enabled API, deleting an already-deleted resource
- **The `done: true` initial response** — verify no polling attempt is made (set no `expect_get` for the poll URL; the mock will panic if polling happens)
- **The `done: true` + error initial response** — verify the error propagates immediately

This is not theoretical. An already-enabled service returning `{"name": "DONE_OPERATION", "done": true}` caused a production failure when our code tried to poll `/v1/DONE_OPERATION` and GCP rejected it.

```rust
#[tokio::test]
async fn test_enable_already_enabled_service_succeeds() {
    let mut mock = crate::MockClient::new();
    mock.expect_post("/v1/projects/p/services/s:enable")
        .returning_json(json!({"name": "DONE_OPERATION", "done": true}))
        .times(1);
    // NO expect_get — polling must not happen
    let client = crate::GcpHttpClient::from_mock(mock);
    let result = client.service_usage().enable_service("p", "s").await;
    assert!(result.is_ok());
}
```

#### Non-Interference Tests for Convenience Methods (CRITICAL)

Any convenience method that does a read-modify-write cycle MUST be tested for non-interference. The pattern:

1. Set field A on the resource (e.g., router description)
2. Call the convenience method that modifies field B (e.g., delete NAT gateway)
3. Verify field A is unchanged after the method returns

This catches bugs where the method fetches the full resource, modifies one field, and patches everything back — silently overwriting concurrent changes to other fields.

**Reference implementations**:
- `test_router_nat_lifecycle` — sets router description, adds NAT, deletes NAT via `delete_nat_gateway`, verifies description survives
- `test_public_access_prevention` — sets storage_class, calls `set_public_access_prevention`, verifies storage_class survives

#### Error Case Testing (REQUIRED for every API)

Every integration test MUST include at least one error case:
- **Get non-existent resource** — verifies error parsing works correctly
- **Operations on deleted resources** — verifies proper 404 handling
- **Delete already-deleted resource** — verifies idempotent error handling

#### Other Edge Cases to Cover
- **Special characters** in string params (spaces, slashes, unicode)
- **Empty/missing optional params** — some APIs reject `?param=` with 400
- **List with None/empty filters** — verify no-filter case works
- **Boundary values** — page_size=0, page_size=1, page_size=max

## Test Categories

### 1. Unit Tests (in `src/api/{api}.rs`)

Hand-written tests in `#[cfg(test)] mod tests` blocks within API client files.

Pattern:
```rust
#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_get_resource() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/v1/projects/test-project/resources/my-resource")
            .returning_json(json!({
                "name": "my-resource",
                "status": "ACTIVE"
            }))
            .times(1);

        let client = crate::GcpHttpClient::from_mock(mock);
        let api = client.my_api();

        let result = api.get_resource("test-project", "my-resource").await;
        assert!(result.is_ok());
        let resource = result.unwrap();
        assert_eq!(resource.name, "my-resource".to_string());
    }
}
```

### What to Test in Unit Tests

**Avoid trivial tests.** Do not write tests that just assert `result.is_ok()` — always verify actual returned data (field values, list lengths, resource names, LRO polling sequences). Every test should demonstrate meaningful behavior.

For each API method, write at minimum:
1. **Happy path** — method returns expected data, verify field values
2. **Lifecycle test** — create -> get -> list -> delete flow, verify returned data at each step
3. **LRO tests** — verify both blocking and _start variants work with polling sequences

For LRO operations, test the polling sequence:
```rust
#[tokio::test]
async fn test_enable_service_blocks_until_complete() {
    let mut mock = crate::MockClient::new();

    // Initial LRO response
    mock.expect_post("/v1/projects/my-project/services/compute.googleapis.com:enable")
        .returning_json(json!({
            "name": "operations/acf.123456",
            "done": false
        }))
        .times(1);

    // Polling sequence: pending -> done
    mock.expect_get("/v1/operations/acf.123456")
        .returning_json_sequence(vec![
            json!({ "name": "operations/acf.123456", "done": false }),
            json!({ "name": "operations/acf.123456", "done": true }),
        ])
        .times(2);

    let client = crate::GcpHttpClient::from_mock(mock);
    let result = client.service_usage().enable_service("my-project", "compute.googleapis.com").await;
    assert!(result.is_ok());
}
```

### 2. Generated Operation Tests (in `src/ops/{api}.rs`)

Auto-generated tests that validate URL construction, HTTP method, and serialization/deserialization for each operation. These are created by codegen and should never be edited.

### 3. Generated Fixture Validation (in `src/fixture_tests.rs`)

Auto-generated tests that ensure every `fixture()` method produces valid data that round-trips through serde.

### 4. MockClient Helper Tests

The generated `src/test_support/{api}_mock_helpers.rs` provide ergonomic test setup:

```rust
use gcp_http_lite::test_support::ComputeMockHelpers;
use gcp_http_lite::types::compute::*;

#[tokio::test]
async fn test_with_helpers() {
    let mut mock = MockClient::new();

    // Instead of manual URL construction:
    mock.expect_create_disk("test-project", "us-central1-a")
        .returning_json(serde_json::to_value(OperationResponse::fixture()).unwrap());

    let client = GcpHttpClient::from_mock(mock);
    let disk = Disk::fixture();
    let result = client.compute().create_disk_start("test-project", "us-central1-a", &disk).await;
    assert!(result.is_ok());
}
```

## Mock Integration Testing

Mock integration tests exercise multi-step workflows with MockClient — no network required. These live in `tests/mock_integration/` and test realistic scenarios that span multiple API calls.

### When to Write Mock Integration Tests

- Full CRUD lifecycle: create -> get -> list -> update -> delete
- LRO sequences: start operation -> poll pending -> poll done -> verify result
- Error recovery: create fails -> retry -> succeeds
- Multi-API workflows: create resource -> set IAM policy -> verify

### Pattern

```rust
//! Mock integration tests for {API_NAME} API workflows
//!
//! These test realistic multi-step scenarios using MockClient.
//! No GCP credentials needed.

use gcp_http_lite::{GcpHttpClient, MockClient};
use gcp_http_lite::types::{api_name}::*;
use serde_json::json;

#[tokio::test]
async fn test_full_crud_lifecycle() {
    let mut mock = MockClient::new();

    // Setup: expect create, get, list, delete in sequence
    mock.expect_post("/v1/projects/test-project/resources")
        .returning_json(json!({
            "name": "my-resource",
            "status": "ACTIVE"
        }))
        .times(1);

    mock.expect_get("/v1/projects/test-project/resources/my-resource")
        .returning_json(json!({
            "name": "my-resource",
            "status": "ACTIVE"
        }))
        .times(1);

    mock.expect_get("/v1/projects/test-project/resources")
        .returning_json(json!({
            "resources": [{"name": "my-resource", "status": "ACTIVE"}]
        }))
        .times(1);

    mock.expect_delete("/v1/projects/test-project/resources/my-resource")
        .returning_json(json!({}))
        .times(1);

    let client = GcpHttpClient::from_mock(mock);
    let api = client.my_api();

    // Create
    let created = api.create_resource("test-project", &resource).await.unwrap();
    assert_eq!(created.name, "my-resource".to_string());

    // Get
    let fetched = api.get_resource("test-project", "my-resource").await.unwrap();
    assert_eq!(fetched.status, Some("ACTIVE".to_string()));

    // List
    let list = api.list_resources("test-project").await.unwrap();
    assert!(!list.resources.is_empty());

    // Delete
    api.delete_resource("test-project", "my-resource").await.unwrap();
}
```

### Mock Integration Test Checklist

- Tests live in `tests/mock_integration/{api_name}.rs`
- Each test covers a realistic multi-step workflow
- All assertions verify actual data, not just success
- LRO workflows test the full polling sequence
- Error scenarios test recovery and proper error types

## Property Testing (Optional)

Use property-based testing where applicable to verify invariants across random inputs. Add `proptest` as a dev dependency if not already present.

### Good Candidates for Property Tests

- **Serde round-trip**: any type should survive serialize -> deserialize
- **URL construction**: resource name formatting should handle edge cases
- **Enum coverage**: all variants should serialize/deserialize correctly

### Pattern

```rust
#[cfg(test)]
mod prop_tests {
    use proptest::prelude::*;
    use serde_json;

    proptest! {
        #[test]
        fn serde_round_trip_resource(
            name in "[a-z][a-z0-9-]{2,20}",
            project in "[a-z][a-z0-9-]{5,20}",
        ) {
            let resource = Resource {
                name: Some(name.clone()),
                project: Some(project.clone()),
                ..Default::default()
            };
            let json = serde_json::to_string(&resource).unwrap();
            let decoded: Resource = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded.name, Some(name));
            assert_eq!(decoded.project, Some(project));
        }
    }
}
```

## Snapshot Testing (Optional)

Use snapshot testing where applicable to catch unexpected changes in serialized output. Add `insta` as a dev dependency if not already present.

### Good Candidates for Snapshot Tests

- **Request body serialization**: verify the exact JSON sent to GCP
- **URL patterns**: verify constructed URLs match expected patterns
- **Fixture data**: verify fixture() output is stable

### Pattern

```rust
#[cfg(test)]
mod snapshot_tests {
    use insta::assert_json_snapshot;

    #[test]
    fn snapshot_create_resource_body() {
        let resource = Resource {
            name: Some("test-resource".to_string()),
            labels: vec![("env".to_string(), "test".to_string())].into_iter().collect(),
            ..Default::default()
        };
        assert_json_snapshot!(resource);
    }
}
```

## Integration Testing

### Running Integration Tests

```bash
# All integration tests
GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=your-project-id \
  cargo nextest run --test integration -- --ignored --test-threads=1 --nocapture

# Single API
GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=your-project-id \
  cargo nextest run --test integration {api_name} -- --ignored --test-threads=1 --nocapture
```

### Prerequisites

- `gcloud` CLI installed and authenticated (`gcloud auth login`)
- ADC configured (`gcloud auth application-default login`)
- Sufficient IAM permissions in the test project
- Required GCP APIs enabled

### Core Patterns

#### 1. Deterministic Resource Names

Use constants, never random names. Enables reliable cleanup across test runs.

```rust
const TEST_SA_NAME: &str = "iam-integ-test";
const TEST_LOCATION: &str = "us-central1";
const TEST_RESOURCE_NAME: &str = "{api}-integ-test";
```

#### 2. Always-Cleanup Pattern

Never let test failures leave resources behind:

```rust
#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_resource_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== Resource Lifecycle Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;

    // 1. Pre-cleanup (handle leftovers from previous failed runs)
    gcloud_delete_resource(&project_id, TEST_RESOURCE_NAME);
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 2. Run test logic in a helper that returns Result
    let result = run_resource_tests(&client, &project_id).await;

    // 3. Always cleanup, regardless of pass/fail
    gcloud_delete_resource(&project_id, TEST_RESOURCE_NAME);

    // 4. Propagate the test result AFTER cleanup
    result?;
    println!("\nAll tests passed!");
    Ok(())
}
```

#### 3. Fixture Creation

See [shared methodology](integration-testing-methodology.md#fixture-management-library-first-cli-as-fallback) for the Fixture Rule, CLI helper patterns, and rules.

#### 4. gcloud CLI for Verification

```rust
fn resource_exists(project_id: &str, name: &str) -> bool {
    let output = Command::new("gcloud")
        .args([
            "{service}", "{resource-type}", "describe", name,
            "--project", project_id,
            "--quiet",
        ])
        .output();

    output.map(|o| o.status.success()).unwrap_or(false)
}
```

#### 5. Step-Numbered Output

See [shared methodology](integration-testing-methodology.md#output-step-numbered-println).

#### 6. Eventual Consistency Handling

GCP APIs are eventually consistent. Add delays after mutations:
- After creating a resource: 3-5s before GET/LIST
- After deleting a resource: 2-3s before verifying deletion
- After IAM changes: 5-10s for propagation

```rust
async fn wait_for_consistency() {
    tokio::time::sleep(Duration::from_secs(5)).await;
}
```

#### 7. Error Case Testing

Always test error paths:

```rust
println!("\n[N/N] Getting non-existent resource (expect error)...");
let result = client.api().get_resource(project_id, "nonexistent").await;
assert!(result.is_err(), "Should error on non-existent resource");
println!("  Correctly received error");
```

#### 8. Delete Verification

```rust
client.api().delete_resource(project_id, name).await?;
tokio::time::sleep(Duration::from_secs(2)).await;

let result = client.api().get_resource(project_id, name).await;
assert!(result.is_err(), "Deleted resource should return error on get");
println!("  Verified: resource deleted");
```

#### 9. API Enablement Checks

```rust
async fn ensure_apis_enabled(
    client: &GcpHttpClient, project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let apis = ["{api}.googleapis.com"];
    for api in &apis {
        let enabled = client.service_usage().is_service_enabled(project_id, api).await?;
        if !enabled {
            println!("  Enabling {}...", api);
            client.service_usage().enable_service(project_id, api).await?;
        }
    }
    Ok(())
}
```

### Integration Test Template

```rust
//! Integration tests for {API_NAME} API
//!
//! Run with:
//!   GOOGLE_AUTH_USE_GCLOUD=1 GCLOUD_PROJECT_ID=<project> \
//!     cargo nextest run --test integration {api_name} -- --ignored --nocapture

use gcp_http_lite::GcpHttpClient;
use std::env;
use std::process::Command;
use std::time::Duration;

const TEST_RESOURCE_NAME: &str = "{api}-integ-test";

fn gcloud_delete_resource(project_id: &str, name: &str) {
    let _ = Command::new("gcloud")
        .args([
            "{service}", "{resource-type}", "delete", name,
            "--project", project_id,
            "--quiet",
        ])
        .output();
}

#[tokio::test]
#[ignore = "requires GCLOUD_PROJECT_ID and ADC credentials"]
async fn test_{api}_resource_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = env::var("GCLOUD_PROJECT_ID").expect("GCLOUD_PROJECT_ID must be set");

    println!("\n=== {API_NAME} Lifecycle Test ===");
    println!("Project: {}", project_id);

    let client = GcpHttpClient::from_adc().await?;

    // Pre-cleanup
    gcloud_delete_resource(&project_id, TEST_RESOURCE_NAME);
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Run tests, always cleanup
    let result = run_tests(&client, &project_id).await;
    gcloud_delete_resource(&project_id, TEST_RESOURCE_NAME);

    result?;
    println!("\nAll {API_NAME} tests passed!");
    Ok(())
}

async fn run_tests(
    client: &GcpHttpClient,
    project_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [1/N] CREATE
    println!("\n[1/5] Creating resource...");
    // client.api().create_resource(project_id, ...).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    // [2/N] GET
    println!("\n[2/5] Getting resource...");
    // let fetched = client.api().get_resource(project_id, ...).await?;

    // [3/N] LIST
    println!("\n[3/5] Listing resources...");
    // let list = client.api().list_resources(project_id).await?;

    // [4/N] DELETE
    println!("\n[4/5] Deleting resource...");
    // client.api().delete_resource(project_id, ...).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // [5/N] Error case
    println!("\n[5/5] Getting non-existent resource (expect error)...");
    // let result = client.api().get_resource(project_id, "nonexistent").await;
    // assert!(result.is_err());

    Ok(())
}
```

### Adding Integration Test to Taskfile.yml

```yaml
test:integration:{api_name}:
  desc: {API Display Name} integration tests
  cmds:
    - cargo nextest run --test integration {api_name} -- --ignored --test-threads=1 --nocapture
```

### Checklist for Integration Tests

See [shared methodology](integration-testing-methodology.md#what-to-test-comprehensive-checklist) for the full checklist. GCP-specific additions:
- Tests multi-value query params with 2+ values (catches serialization bugs)
- Tests `repeated: true` discovery doc params with manual URL construction
- Tests idempotent/no-op LRO cases (initially-done operations)
- Tests non-interference for read-modify-write convenience methods
- Handles eventual consistency with GCP-appropriate delays
