# Cloud Resource Manager API

## Overview

The Cloud Resource Manager API allows you to manage Google Cloud projects and their IAM policies. This client provides methods for creating, updating, moving, and deleting projects, as well as managing access control.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let projects = client.projects();
```

## Features

- **Project Management**: Create, delete, undelete, move, and update projects
- **Querying**: Get, list, and search for projects
- **IAM**: Get/set policies, add/remove bindings, test permissions
- **LRO Support**: Long-running operations for project lifecycle management

## Types

| Type | Description |
|------|-------------|
| `Project` | Project metadata (ID, display name, state, labels) |
| `IamPolicy` | IAM policy structure (version, bindings, etag) |
| `IamBinding` | Role binding (role, members, condition) |
| `ProjectState` | Project lifecycle state (Active, DeleteRequested) |
| `ResourceManagerOperation` | Handle for polling long-running operations |

## Error Handling

Common errors for this API:
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::NotFound` - project doesn't exist
- `GcpError::PreconditionFailed` - e.g., deleting a project that is already deleted
- `GcpError::Aborted` - etag conflict when updating IAM policies (handled automatically by `add/remove_iam_policy_binding`)
