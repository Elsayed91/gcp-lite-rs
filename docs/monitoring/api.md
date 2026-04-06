# Cloud Monitoring API

## Overview

The Cloud Monitoring API provides read-only access to GCP metrics data. Use it to query time series (CPU, memory, request counts, etc.), discover available metric types, and explore monitored resource types.

Designed for policy engines and monitoring dashboards that need to evaluate GCP resource metrics programmatically.

## Client Access

```rust
let client = GcpHttpClient::from_adc().await?;
let monitoring = client.monitoring();
```

## Features

- **Time Series Queries**: List time series with filtering, time intervals, and aggregation (alignment, cross-series reduction, group-by)
- **Metric Discovery**: List and get metric descriptors to find available metrics
- **Resource Discovery**: List and get monitored resource descriptors to find resource types
- **Pagination**: All list operations support single-page and auto-paginating stream variants
- **Aggregation**: Full support for alignment periods, per-series aligners, cross-series reducers, and group-by fields

## Types

| Type | Description |
|------|-------------|
| `TimeSeries` | A collection of data points for a metric/resource pair |
| `Point` | A single data point with interval and value |
| `TimeInterval` | Start and end time for queries |
| `TypedValue` | Typed metric value (double, int64, string, bool, distribution) |
| `Metric` | Metric type and labels |
| `MonitoredResource` | Resource type and labels |
| `MetricDescriptor` | Metadata about a metric type (kind, value type, labels, unit) |
| `MonitoredResourceDescriptor` | Metadata about a resource type (labels, description) |
| `LabelDescriptor` | A label key with type and description |
| `TimeSeriesParams` | Optional aggregation and pagination parameters for time series queries |
| `ListTimeSeriesResponse` | Response wrapper for time series list |
| `ListMetricDescriptorsResponse` | Response wrapper for metric descriptor list |
| `ListMonitoredResourceDescriptorsResponse` | Response wrapper for resource descriptor list |

## Error Handling

Common errors for this API:
- `GcpError::NotFound` - metric descriptor or resource type doesn't exist
- `GcpError::InvalidArgument` - invalid filter syntax, missing required interval fields
- `GcpError::PermissionDenied` - insufficient IAM permissions (`monitoring.timeSeries.list`)
- `GcpError::ApiNotEnabled` - Monitoring API not enabled on project
