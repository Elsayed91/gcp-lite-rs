# Service Usage API

## Overview

The Service Usage API allows you to list, enable, and disable Google Cloud APIs for your projects.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let service_usage = client.service_usage();
```

## Features

- **Service Management**: Enable and disable APIs
- **Batch Operations**: Enable multiple APIs at once
- **Discovery**: List available APIs and check their status
- **LRO Support**: Long-running operations for enabling/disabling services

## Types

| Type | Description |
|------|-------------|
| `ServiceState` | API status (ENABLED/DISABLED) |
| `ServiceStateEnum` | Enum for status values |
| `BatchEnableServicesRequest` | Request for batch enabling |
| `DisableServiceRequest` | Request for disabling (with check options) |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - service not found (e.g., typo in API name)
- `GcpError::PermissionDenied` - insufficient permissions (requires `serviceusage.services.enable`, etc.)
- `GcpError::FailedPrecondition` - disabling a service that other enabled services depend on
