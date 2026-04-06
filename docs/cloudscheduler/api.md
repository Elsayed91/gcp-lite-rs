# Cloud Scheduler API

## Overview

Cloud Scheduler is a fully managed enterprise-grade cron job scheduler. It allows you to schedule virtually any job, including batch, big data jobs, cloud infrastructure operations, and more.

This client provides methods for creating, managing, and controlling scheduled jobs.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let scheduler = client.scheduler();
```

## Features

- **Job CRUD**: Create, read, update, and delete scheduled jobs
- **Job Control**: Pause, resume, and force-run jobs on demand
- **Flexible Targets**: HTTP endpoints, Pub/Sub topics, and App Engine handlers
- **Cron Scheduling**: Standard cron expression support with timezone configuration

## LRO Pattern

**Pattern**: none (all operations are synchronous)

All Cloud Scheduler job operations return immediately with the result.

## Types

| Type | Description |
|------|-------------|
| `Job` | A scheduled job with target, schedule, and configuration |
| `HttpTarget` | HTTP endpoint target configuration |
| `PubsubTarget` | Pub/Sub topic target configuration |
| `AppEngineHttpTarget` | App Engine handler target configuration |
| `RetryConfig` | Retry policy configuration |
| `JobState` | Job state enum (Enabled, Paused, Disabled, etc.) |
| `JobStatus` | Job status with error details |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - job doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions  
- `GcpError::ApiNotEnabled` - Cloud Scheduler API not enabled on project
- `GcpError::InvalidArgument` - invalid job configuration (e.g., bad cron expression)
