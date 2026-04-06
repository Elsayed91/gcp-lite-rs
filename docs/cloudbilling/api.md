# Cloud Billing API

## Overview

The Cloud Billing API allows you to programmatically manage billing for your Google Cloud projects. This client provides methods to get and update the billing account associated with a project.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let billing = client.billing();
```

## Features

- **Get billing info**: Retrieve the billing configuration for a project
- **Update billing info**: Link a project to a billing account or disable billing

## Types

| Type | Description |
|------|-------------|
| `ProjectBillingInfo` | Billing configuration for a project (billing account name, enabled status) |

## Error Handling

Common errors for this API:
- `GcpError::PermissionDenied` - insufficient IAM permissions (requires `resourcemanager.projects.get` and billing permissions)
- `GcpError::NotFound` - project or billing account doesn't exist
- `GcpError::InvalidArgument` - invalid billing account format
