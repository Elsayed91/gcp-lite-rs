# IAM API

## Overview

The IAM API allows you to manage service accounts and keys. This client provides methods for creating, deleting, and listing service accounts and their associated keys.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let iam = client.iam();
```

## Features

- **Service Accounts**: Create, get, list, and delete service accounts
- **Service Account Keys**: Create, list, and delete keys

## Types

| Type | Description |
|------|-------------|
| `ServiceAccount` | Service account metadata (email, display name, etc.) |
| `ServiceAccountKey` | Key metadata and private key data (if created) |
| `CreateServiceAccountRequest` | Configuration for creating a service account |
| `CreateServiceAccountKeyRequest` | Configuration for creating a key |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - service account or key doesn't exist
- `GcpError::PermissionDenied` - insufficient permissions
- `GcpError::QuotaExceeded` - exceeded key limit (10 per account)
