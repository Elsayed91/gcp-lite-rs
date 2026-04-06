# API Documentation Template

When adding a new API, create `docs/{api_name}/` with three files.

## docs/{api_name}/api.md

```markdown
# {Display Name} API

## Overview

Brief description of what this API does and when you'd use it.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let {accessor} = client.{accessor_name}();
```

## Features

- List the key features/capabilities this client provides
- Note any convenience methods (e.g., read-modify-write for IAM)
- Note LRO support (blocking + _start variants)

## LRO Pattern

**Pattern**: {selflink | name_based | project_name | none}

For LRO operations, two variants are available:
- `{method}()` — blocks until the operation completes
- `{method}_start()` — returns an Operation for manual polling

## Types

| Type | Description |
|------|-------------|
| `{Type}` | Main resource type |
| `{ListType}` | List response wrapper |
| `{LroType}` | Operation response (if LRO) |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` — resource doesn't exist
- `GcpError::PermissionDenied` — insufficient IAM permissions
- `GcpError::ApiNotEnabled` — API not enabled on project
```

## docs/{api_name}/operations.md

```markdown
# {Display Name} Operations

## {Resource Group 1}

### {method_name}

**Signature**: `pub async fn {method}({params}) -> Result<{ReturnType}>`

{Brief description}

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| ... | ... | ... |

**Returns**: `Result<{Type}>`

---

### {method_name}_start (LRO)

**Signature**: `pub async fn {method}_start({params}) -> Result<{Operation}<'a>>`

Non-blocking variant. Returns an operation that can be polled manually.

```rust
let op = client.{api}().{method}_start(project, ...).await?;
op.wait().await?;  // Or poll manually
```
```

## docs/{api_name}/usage.md

```markdown
# {Display Name} Usage Examples

## Basic CRUD

### Create a resource

```rust
use gcp_http_lite::GcpHttpClient;
use gcp_http_lite::types::{api}::{Type};

let client = GcpHttpClient::from_adc().await?;

let resource = {Type} {
    name: "my-resource".to_string(),
    ..Default::default()
};

// Blocking (waits for completion)
client.{api}().create_{resource}(project, &resource).await?;

// Non-blocking (returns operation)
let op = client.{api}().create_{resource}_start(project, &resource).await?;
// Do other work...
op.wait().await?;
```

### Get a resource

```rust
let resource = client.{api}().get_{resource}(project, "my-resource").await?;
println!("Status: {:?}", resource.status);
```

### List resources

```rust
let list = client.{api}().list_{resources}(project).await?;
for item in &list.items {
    println!("{}", item.name);
}
```

### Delete a resource

```rust
client.{api}().delete_{resource}(project, "my-resource").await?;
```

## Testing

```rust
use gcp_http_lite::{GcpHttpClient, MockClient};
use gcp_http_lite::test_support::{ApiMockHelpers};
use gcp_http_lite::types::{api}::*;

#[tokio::test]
async fn test_example() {
    let mut mock = MockClient::new();
    mock.expect_get_{resource}("project", "name")
        .returning_json(serde_json::to_value({Type}::fixture()).unwrap());

    let client = GcpHttpClient::from_mock(mock);
    let result = client.{api}().get_{resource}("project", "name").await;
    assert!(result.is_ok());
}
```
```
