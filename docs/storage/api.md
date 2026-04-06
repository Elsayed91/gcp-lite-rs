# Cloud Storage JSON API

## Overview

The Cloud Storage JSON API allows you to manage buckets and objects. This client provides methods for creating, reading, listing, and deleting storage resources.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let storage = client.storage();
```

## Features

- **Buckets**: Create, list, get, delete, and configure buckets (IAM, CORS, lifecycle)
- **Objects**: Upload, download, list, delete, copy, rewrite, and compose objects
- **Access Control**: Manage IAM policies and public access prevention

## Types

| Type | Description |
|------|-------------|
| `Bucket` | Bucket metadata (name, location, storage class, etc.) |
| `Object` | Object metadata (name, bucket, size, content type, etc.) |
| `Buckets` | List of buckets |
| `Objects` | List of objects |
| `Policy` | IAM policy for buckets |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - bucket or object doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::PreconditionFailed` - e.g., deleting a non-empty bucket
