# Recommender API

## Overview

The Recommender API allows you to access insights and recommendations for your Google Cloud resources, helping you optimize for cost, security, performance, and reliability.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let recommender = client.recommender();
```

## Features

- **Recommendations**: List recommendations for specific services and regions
- **Pagination**: Flexible pagination support (single page, stream, or collect all)
- **Impact Analysis**: View cost, security, and sustainability impact projections

## Types

| Type | Description |
|------|-------------|
| `Recommendation` | A suggested improvement (with impact and operations) |
| `Impact` | Quantified effect of applying the recommendation |
| `OperationGroup` | Set of operations to apply the recommendation |
| `ListRecommendationsOptions` | Filtering and pagination options |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - project, location, or recommender ID not found
- `GcpError::PermissionDenied` - insufficient IAM permissions (e.g., `recommender.computeInstanceMachineTypes.get`)
