# Cloud Monitoring Usage Examples

## Query CPU Utilization

Query the last hour of CPU utilization for all instances:

```rust
use gcp_lite::GcpHttpClient;
use gcp_lite::api::monitoring::TimeSeriesParams;
use gcp_lite::types::monitoring::TimeInterval;

let client = GcpHttpClient::from_adc().await?;
let monitoring = client.monitoring();

let now = chrono::Utc::now();
let interval = TimeInterval {
    start_time: Some((now - chrono::Duration::hours(1)).to_rfc3339()),
    end_time: Some(now.to_rfc3339()),
};

let response = monitoring
    .list_time_series(
        "my-project",
        "metric.type=\"compute.googleapis.com/instance/cpu/utilization\"",
        &interval,
        None,
    )
    .await?;

for ts in &response.time_series {
    let instance = ts.metric.as_ref()
        .and_then(|m| m.labels.get("instance_name"))
        .map(|s| s.as_str())
        .unwrap_or("?");
    println!("Instance: {}, points: {}", instance, ts.points.len());
}
```

## Query with Aggregation

Aggregate API request counts by service, aligned to 1-hour windows:

```rust
let params = TimeSeriesParams {
    aggregation_alignment_period: Some("3600s".to_string()),
    aggregation_per_series_aligner: Some("ALIGN_RATE".to_string()),
    aggregation_cross_series_reducer: Some("REDUCE_SUM".to_string()),
    aggregation_group_by_fields: Some(vec!["resource.labels.service".to_string()]),
    ..Default::default()
};

let response = monitoring
    .list_time_series(
        "my-project",
        "metric.type=\"serviceruntime.googleapis.com/api/request_count\"",
        &interval,
        Some(&params),
    )
    .await?;

for ts in &response.time_series {
    println!("Service: {:?}", ts.resource.as_ref().map(|r| &r.labels));
}
```

## Discover Available Metrics

Stream all compute-related metric descriptors:

```rust
use futures::StreamExt;
use std::pin::pin;

let monitoring = client.monitoring();

let mut stream = pin!(monitoring.list_metric_descriptors_stream(
    "my-project",
    Some("metric.type = starts_with(\"compute.googleapis.com/\")"),
));

while let Some(result) = stream.next().await {
    let desc = result?;
    println!("{} ({:?})", desc.metric_type.unwrap_or_default(), desc.metric_kind);
}
```

## Check Resource Types

Find what labels a `gce_instance` resource provides:

```rust
let desc = monitoring
    .get_monitored_resource_descriptor("my-project", "gce_instance")
    .await?;

println!("Display name: {:?}", desc.display_name);
for label in &desc.labels {
    println!("  {} ({:?})", label.key, label.value_type);
}
```

## Testing

```rust
use gcp_lite::{GcpHttpClient, MockClient};
use gcp_lite::test_support::MonitoringMockHelpers;
use gcp_lite::types::monitoring::*;

#[tokio::test]
async fn test_get_metric_descriptor() {
    let mut mock = MockClient::new();
    mock.expect_get_metric_descriptor("my-project", "compute.googleapis.com/instance/cpu/utilization")
        .returning_json(serde_json::to_value(MetricDescriptor::fixture()).unwrap());

    let client = GcpHttpClient::from_mock(mock);
    let desc = client.monitoring()
        .get_metric_descriptor("my-project", "compute.googleapis.com/instance/cpu/utilization")
        .await
        .unwrap();
    assert!(desc.metric_type.is_some());
}
```
