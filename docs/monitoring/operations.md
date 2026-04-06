# Cloud Monitoring Operations

## Time Series

### list_time_series

**Signature**: `pub async fn list_time_series(project: &str, filter: &str, interval: &TimeInterval, params: Option<&TimeSeriesParams>) -> Result<ListTimeSeriesResponse>`

Lists time series matching a filter within a time interval. Supports aggregation (alignment, cross-series reduction, group-by).

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `filter` | `&str` | Monitoring filter (e.g., `metric.type="compute.googleapis.com/instance/cpu/utilization"`) |
| `interval` | `&TimeInterval` | Time range; `end_time` is required, `start_time` is optional |
| `params` | `Option<&TimeSeriesParams>` | Optional aggregation, pagination, and ordering |

**Returns**: `Result<ListTimeSeriesResponse>`

---

### list_time_series_stream

**Signature**: `pub fn list_time_series_stream(project: &str, filter: &str, interval: &TimeInterval, params: Option<TimeSeriesParams>) -> impl Stream<Item = Result<TimeSeries>>`

Returns an async stream of time series, handling pagination automatically.

---

## Metric Descriptors

### list_metric_descriptors

**Signature**: `pub async fn list_metric_descriptors(project: &str, filter: Option<&str>, page_token: Option<&str>) -> Result<ListMetricDescriptorsResponse>`

Lists metric descriptors. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `filter` | `Option<&str>` | Optional filter (e.g., `metric.type = starts_with("compute.googleapis.com/")`) |
| `page_token` | `Option<&str>` | Pagination token |

**Returns**: `Result<ListMetricDescriptorsResponse>`

---

### list_metric_descriptors_stream

**Signature**: `pub fn list_metric_descriptors_stream(project: &str, filter: Option<&str>) -> impl Stream<Item = Result<MetricDescriptor>>`

Returns an async stream of metric descriptors, handling pagination automatically.

---

### get_metric_descriptor

**Signature**: `pub async fn get_metric_descriptor(project: &str, metric_type: &str) -> Result<MetricDescriptor>`

Gets a single metric descriptor.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `metric_type` | `&str` | Metric type (e.g., "compute.googleapis.com/instance/cpu/utilization") |

**Returns**: `Result<MetricDescriptor>`

---

## Monitored Resource Descriptors

### list_monitored_resource_descriptors

**Signature**: `pub async fn list_monitored_resource_descriptors(project: &str, filter: Option<&str>, page_token: Option<&str>) -> Result<ListMonitoredResourceDescriptorsResponse>`

Lists monitored resource descriptors. Returns a single page.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `filter` | `Option<&str>` | Optional filter |
| `page_token` | `Option<&str>` | Pagination token |

**Returns**: `Result<ListMonitoredResourceDescriptorsResponse>`

---

### list_monitored_resource_descriptors_stream

**Signature**: `pub fn list_monitored_resource_descriptors_stream(project: &str, filter: Option<&str>) -> impl Stream<Item = Result<MonitoredResourceDescriptor>>`

Returns an async stream of monitored resource descriptors, handling pagination automatically.

---

### get_monitored_resource_descriptor

**Signature**: `pub async fn get_monitored_resource_descriptor(project: &str, resource_type: &str) -> Result<MonitoredResourceDescriptor>`

Gets a single monitored resource descriptor.

| Parameter | Type | Description |
|-----------|------|-------------|
| `project` | `&str` | GCP project ID |
| `resource_type` | `&str` | Resource type (e.g., "gce_instance", "gcs_bucket") |

**Returns**: `Result<MonitoredResourceDescriptor>`
