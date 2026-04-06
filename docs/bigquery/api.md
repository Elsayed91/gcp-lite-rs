# BigQuery API

## Overview

The BigQuery API allows you to manage datasets, tables, and jobs, as well as run SQL queries. This client provides methods for interacting with BigQuery resources and executing analytical workloads.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let bigquery = client.bigquery();
```

## Features

- **Datasets**: List, get, and filter datasets
- **Tables**: List, get, and patch tables
- **Jobs**: List, get, and insert (start) asynchronous jobs
- **Querying**: Run synchronous SQL queries
- **Pagination**: List methods automatically paginate to collect all results

## Types

| Type | Description |
|------|-------------|
| `Dataset` | Dataset metadata (ID, location, labels, etc.) |
| `Table` | Table metadata (schema, partitioning, expiration) |
| `Job` | Asynchronous job configuration and status |
| `QueryRequest` | Configuration for synchronous queries |
| `QueryResponse` | Results of a synchronous query |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - dataset, table, or job doesn't exist
- `GcpError::PermissionDenied` - insufficient IAM permissions
- `GcpError::BadRequest` - invalid SQL query or configuration
