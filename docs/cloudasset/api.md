# Cloud Asset API

## Overview

The Cloud Asset API allows you to search and analyze Google Cloud resources and IAM policies. This client provides methods for listing assets, searching resources, and analyzing IAM policies across your organization.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let cloud_asset = client.cloud_asset();
```

## Features

- **Asset Inventory**: List assets with history and metadata
- **Resource Search**: Search across all resources with powerful filtering
- **IAM Policy Search**: Search and analyze IAM policies across projects
- **Pagination**: Flexible pagination support (single page, stream, or collect all)

## Types

| Type | Description |
|------|-------------|
| `Asset` | Representation of a GCP asset (resource + IAM policy) |
| `ResourceSearchResult` | Result from resource search |
| `IamPolicySearchResult` | Result from IAM policy search |
| `ListAssetsOptions` | Configuration for listing assets |
| `SearchOptions` | Configuration for searching resources |
| `SearchIamPoliciesOptions` | Configuration for searching IAM policies |

## Error Handling

Common errors for this API:
- `GcpError::InvalidArgument` - invalid query syntax or parameters
- `GcpError::PermissionDenied` - insufficient permissions (requires `cloudasset.assets.searchAllResources`, etc.)
- `GcpError::Unauthenticated` - missing or invalid credentials
