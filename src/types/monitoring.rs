//! Types for the Cloud Monitoring API API (v3).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/monitoring/v3/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A collection of data points that describes the time-varying values of a metric. A time
/// series is identified by a combination of a fully-specified monitored resource and a fully-
/// specified metric. This type is used for both listing and creating time series.
///
/// **GCP API**: `monitoring.v3.TimeSeries`
/// **Reference**: <https://cloud.google.com/monitoring/api//TimeSeries>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    /// The associated metric. A fully-specified metric used to identify the time series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,

    /// The associated monitored resource. Custom metrics can use only certain monitored
    /// resource types in their time series data. For more information, see Monitored resources
    /// for custom metrics (https://cloud.google.com/monitoring/custom-metrics/creating-
    /// metrics#custom-metric-resources).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<MonitoredResource>,

    /// The metric kind of the time series. When listing time series, this metric kind might be
    /// different from the metric kind of the associated metric if this time series is an
    /// alignment or reduction of other time series.When creating a time series, this field is
    /// optional. If present, it must be the same as the metric kind of the associated metric.
    /// If the associated metric's descriptor must be auto-created, then this field specifies
    /// the metric kind of the new descriptor and must be either GAUGE (the default) or
    /// CUMULATIVE.
    ///
    /// **Possible values**:
    /// - `METRIC_KIND_UNSPECIFIED` — Do not use this default value.
    /// - `GAUGE` — An instantaneous measurement of a value.
    /// - `DELTA` — The change in a value during a time interval.
    /// - `CUMULATIVE` — A value accumulated over a time interval. Cumulative measurements in a time seri...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_kind: Option<String>,

    /// The value type of the time series. When listing time series, this value type might be
    /// different from the value type of the associated metric if this time series is an
    /// alignment or reduction of other time series.When creating a time series, this field is
    /// optional. If present, it must be the same as the type of the data in the points field.
    ///
    /// **Possible values**:
    /// - `VALUE_TYPE_UNSPECIFIED` — Do not use this default value.
    /// - `BOOL` — The value is a boolean. This value type can be used only if the metric kind is G...
    /// - `INT64` — The value is a signed 64-bit integer.
    /// - `DOUBLE` — The value is a double precision floating point number.
    /// - `STRING` — The value is a text string. This value type can be used only if the metric kind ...
    /// - `DISTRIBUTION` — The value is a Distribution.
    /// - `MONEY` — The value is money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,

    /// The data points of this time series. When listing time series, points are returned in
    /// reverse time order.When creating a time series, this field must contain exactly one
    /// point and the point's type must be the same as the value type of the associated metric.
    /// If the associated metric's descriptor must be auto-created, then the value type of the
    /// descriptor is determined by the point's type, which must be BOOL, INT64, DOUBLE, or
    /// DISTRIBUTION.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub points: Vec<Point>,

    /// Output only. The associated monitored resource metadata. When reading a time series,
    /// this field will include metadata labels that are explicitly named in the reduction. When
    /// creating a time series, this field is ignored.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MonitoredResourceMetadata>,

    /// The units in which the metric value is reported. It is only applicable if the value_type
    /// is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored
    /// metric values. This field can only be changed through CreateTimeSeries when it is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// Input only. A detailed description of the time series that will be associated with the
    /// google.api.MetricDescriptor for the metric. Once set, this field cannot be changed
    /// through CreateTimeSeries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl TimeSeries {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric: Some(Metric::fixture()),
            resource: Some(MonitoredResource::fixture()),
            metric_kind: Some("test-metric_kind".into()),
            value_type: Some("test-value_type".into()),
            points: vec![],
            metadata: Some(MonitoredResourceMetadata::fixture()),
            unit: Some("test-unit".into()),
            description: Some("test-description".into()),
        }
    }
}

/// A single data point in a time series.
///
/// **GCP API**: `monitoring.v3.Point`
/// **Reference**: <https://cloud.google.com/monitoring/api//Point>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    /// The time interval to which the data point applies. For GAUGE metrics, the start time is
    /// optional, but if it is supplied, it must equal the end time. For DELTA metrics, the
    /// start and end time should specify a non-zero interval, with subsequent points specifying
    /// contiguous and non-overlapping intervals. For CUMULATIVE metrics, the start and end time
    /// should specify a non-zero interval, with subsequent points specifying the same start
    /// time and increasing end times, until an event resets the cumulative value to zero and
    /// sets a new start time for the following points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TimeInterval>,

    /// The value of the data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TypedValue>,
}

impl Point {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            interval: Some(TimeInterval::fixture()),
            value: Some(TypedValue::fixture()),
        }
    }
}

/// Describes a time interval: Reads: A half-open time interval. It includes the end time but
/// excludes the start time: (startTime, endTime]. The start time must be specified, must be
/// earlier than the end time, and should be no older than the data retention period for the
/// metric. Writes: A closed time interval. It extends from the start time to the end time, and
/// includes both: [startTime, endTime]. Valid time intervals depend on the MetricKind (https://
/// cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors#MetricKind) of the
/// metric value. The end time must not be earlier than the start time, and the end time must
/// not be more than 25 hours in the past or more than five minutes in the future. For GAUGE
/// metrics, the startTime value is technically optional; if no value is specified, the start
/// time defaults to the value of the end time, and the interval represents a single point in
/// time. If both start and end times are specified, they must be identical. Such an interval is
/// valid only for GAUGE metrics, which are point-in-time measurements. The end time of a new
/// interval must be at least a millisecond after the end time of the previous interval. For
/// DELTA metrics, the start time and end time must specify a non-zero interval, with subsequent
/// points specifying contiguous and non-overlapping intervals. For DELTA metrics, the start
/// time of the next interval must be at least a millisecond after the end time of the previous
/// interval. For CUMULATIVE metrics, the start time and end time must specify a non-zero
/// interval, with subsequent points specifying the same start time and increasing end times,
/// until an event resets the cumulative value to zero and sets a new start time for the
/// following points. The new start time must be at least a millisecond after the end time of
/// the previous interval. The start time of a new interval must be at least a millisecond after
/// the end time of the previous interval because intervals are closed. If the start time of a
/// new interval is the same as the end time of the previous interval, then data written at the
/// new start time could overwrite data written at the previous end time.
///
/// **GCP API**: `monitoring.v3.TimeInterval`
/// **Reference**: <https://cloud.google.com/monitoring/api//TimeInterval>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeInterval {
    /// Required. The end of the time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Optional. The beginning of the time interval. The default value for the start time is
    /// the end time. The start time must not be later than the end time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl TimeInterval {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            end_time: Some("test-end_time".into()),
            start_time: Some("test-start_time".into()),
        }
    }
}

/// A single strongly-typed value.
///
/// **GCP API**: `monitoring.v3.TypedValue`
/// **Reference**: <https://cloud.google.com/monitoring/api//TypedValue>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedValue {
    /// A 64-bit integer. Its range is approximately ±9.2x1018.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int64_value: Option<String>,

    /// A 64-bit double-precision floating-point number. Its magnitude is approximately ±10±300
    /// and it has 16 significant digits of precision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,

    /// A variable-length string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,

    /// A Boolean value: true or false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,

    /// A distribution value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_value: Option<serde_json::Value>,
}

impl TypedValue {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            int64_value: Some("test-int64_value".into()),
            string_value: Some("test-string_value".into()),
            bool_value: Some(false),
            ..Default::default()
        }
    }
}

/// A specific metric, identified by specifying values for all of the labels of a
/// MetricDescriptor.
///
/// **GCP API**: `monitoring.v3.Metric`
/// **Reference**: <https://cloud.google.com/monitoring/api//Metric>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    /// An existing metric type, see google.api.MetricDescriptor. For example,
    /// custom.googleapis.com/invoice/paid/amount.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,

    /// The set of label values that uniquely identify this metric. All labels listed in the
    /// MetricDescriptor must be assigned values.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl Metric {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_type: Some("test-type".into()),
            labels: Default::default(),
        }
    }
}

/// An object representing a resource that can be used for monitoring, logging, billing, or
/// other purposes. Examples include virtual machine instances, databases, and storage devices
/// such as disks. The type field identifies a MonitoredResourceDescriptor object that describes
/// the resource's schema. Information in the labels field identifies the actual resource and
/// its attributes according to the schema. For example, a particular Compute Engine VM instance
/// could be represented by the following object, because the MonitoredResourceDescriptor for
/// "gce_instance" has labels "project_id", "instance_id" and "zone": { "type": "gce_instance",
/// "labels": { "project_id": "my-project", "instance_id": "12345678901234", "zone": "us-
/// central1-a" }}
///
/// **GCP API**: `monitoring.v3.MonitoredResource`
/// **Reference**: <https://cloud.google.com/monitoring/api//MonitoredResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredResource {
    /// Required. The monitored resource type. This field must match the type field of a
    /// MonitoredResourceDescriptor object. For example, the type of a Compute Engine VM
    /// instance is gce_instance. For a list of types, see Monitoring resource types
    /// (https://cloud.google.com/monitoring/api/resources) and Logging resource types
    /// (https://cloud.google.com/logging/docs/api/v2/resource-list).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Required. Values for all of the labels listed in the associated monitored resource
    /// descriptor. For example, Compute Engine VM instances use the labels "project_id",
    /// "instance_id", and "zone".
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl MonitoredResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_type: Some("test-type".into()),
            labels: Default::default(),
        }
    }
}

/// Auxiliary metadata for a MonitoredResource object. MonitoredResource objects contain the
/// minimum set of information to uniquely identify a monitored resource instance. There is some
/// other useful auxiliary metadata. Monitoring and Logging use an ingestion pipeline to extract
/// metadata for cloud resources of all types, and store the metadata in this message.
///
/// **GCP API**: `monitoring.v3.MonitoredResourceMetadata`
/// **Reference**: <https://cloud.google.com/monitoring/api//MonitoredResourceMetadata>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels. System labels are a kind of
    /// metadata extracted by Google, including "machine_image", "vpc", "subnet_id",
    /// "security_group", "name", etc. System label values can be only strings, Boolean values,
    /// or a list of strings. For example: { "name": "my-test-instance", "security_group": ["a",
    /// "b", "c"], "spot_instance": false }
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_labels: Option<serde_json::Value>,

    /// Output only. A map of user-defined metadata labels.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub user_labels: HashMap<String, String>,
}

impl MonitoredResourceMetadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            system_labels: Default::default(),
            user_labels: Default::default(),
        }
    }
}

/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or
/// altering it stops data collection and makes the metric type's existing data unusable.
///
/// **GCP API**: `monitoring.v3.MetricDescriptor`
/// **Reference**: <https://cloud.google.com/monitoring/api//MetricDescriptor>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricDescriptor {
    /// The resource name of the metric descriptor.
    pub name: String,

    /// The metric type, including its DNS name prefix. The type is not URL-encoded. All user-
    /// defined metric types have the DNS name custom.googleapis.com or external.googleapis.com.
    /// Metric types should use a natural hierarchical grouping. For example:
    /// "custom.googleapis.com/invoice/paid/amount" "external.googleapis.com/prometheus/up"
    /// "appengine.googleapis.com/http/server/response_latencies"
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,

    /// The set of labels that can be used to describe a specific instance of this metric type.
    /// For example, the appengine.googleapis.com/http/server/response_latencies metric type has
    /// a label for the HTTP response code, response_code, so you can look at latencies for
    /// successful responses or just for responses that failed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelDescriptor>,

    /// Whether the metric records instantaneous values, changes to a value, etc. Some
    /// combinations of metric_kind and value_type might not be supported.
    ///
    /// **Possible values**:
    /// - `METRIC_KIND_UNSPECIFIED` — Do not use this default value.
    /// - `GAUGE` — An instantaneous measurement of a value.
    /// - `DELTA` — The change in a value during a time interval.
    /// - `CUMULATIVE` — A value accumulated over a time interval. Cumulative measurements in a time seri...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_kind: Option<String>,

    /// Whether the measurement is an integer, a floating-point number, etc. Some combinations
    /// of metric_kind and value_type might not be supported.
    ///
    /// **Possible values**:
    /// - `VALUE_TYPE_UNSPECIFIED` — Do not use this default value.
    /// - `BOOL` — The value is a boolean. This value type can be used only if the metric kind is G...
    /// - `INT64` — The value is a signed 64-bit integer.
    /// - `DOUBLE` — The value is a double precision floating point number.
    /// - `STRING` — The value is a text string. This value type can be used only if the metric kind ...
    /// - `DISTRIBUTION` — The value is a Distribution.
    /// - `MONEY` — The value is money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,

    /// The units in which the metric value is reported. It is only applicable if the value_type
    /// is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored
    /// metric values.Different systems might scale the values to be more easily displayed (so a
    /// value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed
    /// as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in
    /// thousands of bytes, no matter how it might be displayed.If you want a custom metric to
    /// record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE
    /// metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005
    /// CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom
    /// metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric
    /// whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use
    /// Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The
    /// Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic
    /// units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes
    /// (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa
    /// (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p
    /// pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi
    /// (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar
    /// also includes these connectors: / division or ratio (as an infix operator). For
    /// examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric
    /// unit; rates should always be computed at query time from the underlying cumulative or
    /// delta value). . multiplication or composition (as an infix operator). For examples,
    /// GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { "."
    /// Component } { "/" Component } ; Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ] |
    /// Annotation | "1" ; Annotation = "{" NAME "}" ; Notes: Annotation is just a comment if it
    /// follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For
    /// examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank
    /// printable ASCII characters not containing { or }. 1 represents a unitary dimensionless
    /// unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is
    /// typically used when none of the basic units are appropriate. For example, "new users per
    /// day" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean "5 new
    /// users). Alternatively, "thousands of page views per day" would be represented as 1000/d
    /// or k1/d or k{page_views}/d (and a metric value of 5.3 would mean "5300 page views per
    /// day"). % represents dimensionless value of 1/100, and annotates values giving a
    /// percentage (so the metric values are typically in the range of 0..100, and a metric
    /// value 3 means "3 percent"). 10^2.% indicates a metric contains a ratio, typically in the
    /// range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric
    /// value 0.03 means "3 percent").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// A detailed description of the metric, which can be used in documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A concise name for the metric, which can be displayed in user interfaces. Use sentence
    /// case without an ending period, for example "Request count". This field is optional but
    /// it is recommended to be set for any metrics associated with user-visible concepts, such
    /// as Quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional. Metadata which can be used to guide usage of the metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetricDescriptorMetadata>,

    /// Optional. The launch stage of the metric definition.
    ///
    /// **Possible values**:
    /// - `LAUNCH_STAGE_UNSPECIFIED` — Do not use this default value.
    /// - `UNIMPLEMENTED` — The feature is not yet implemented. Users can not use it.
    /// - `PRELAUNCH` — Prelaunch features are hidden from users and are only visible internally.
    /// - `EARLY_ACCESS` — Early Access features are limited to a closed group of testers. To use these fea...
    /// - `ALPHA` — Alpha is a limited availability test for releases before they are cleared for wi...
    /// - `BETA` — Beta is the point at which we are ready to open a release for any customer to us...
    /// - `GA` — GA features are open to all developers and are considered stable and fully quali...
    /// - `DEPRECATED` — Deprecated features are scheduled to be shut down and removed. For more informat...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_stage: Option<String>,

    /// Read-only. If present, then a time series, which is identified partially by a metric
    /// type and a MonitoredResourceDescriptor, that is associated with this metric type can
    /// only be associated with one of the monitored resource types listed here.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub monitored_resource_types: Vec<String>,
}

impl MetricDescriptor {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-metric_descriptor".into(),
            metric_type: Some("test-type".into()),
            labels: vec![],
            metric_kind: Some("test-metric_kind".into()),
            value_type: Some("test-value_type".into()),
            unit: Some("test-unit".into()),
            description: Some("test-description".into()),
            display_name: Some("test-display_name".into()),
            metadata: Some(MetricDescriptorMetadata::fixture()),
            launch_stage: Some("test-launch_stage".into()),
            monitored_resource_types: vec![],
        }
    }
}

/// A description of a label.
///
/// **GCP API**: `monitoring.v3.LabelDescriptor`
/// **Reference**: <https://cloud.google.com/monitoring/api//LabelDescriptor>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDescriptor {
    /// The key for this label. The key must meet the following criteria: Does not exceed 100
    /// characters. Matches the following regular expression: [a-zA-Z][a-zA-Z0-9_]* The first
    /// character must be an upper- or lower-case letter. The remaining characters must be
    /// letters, digits, or underscores.
    pub key: String,

    /// The type of data that can be assigned to the label.
    ///
    /// **Possible values**:
    /// - `STRING` — A variable-length string, not to exceed 1,024 characters. This is the default va...
    /// - `BOOL` — Boolean; true or false.
    /// - `INT64` — A 64-bit signed integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,

    /// A human-readable description for the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl LabelDescriptor {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key: "test-key".into(),
            value_type: Some("test-value_type".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Additional annotations that can be used to guide the usage of a metric.
///
/// **GCP API**: `monitoring.v3.MetricDescriptorMetadata`
/// **Reference**: <https://cloud.google.com/monitoring/api//MetricDescriptorMetadata>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricDescriptorMetadata {
    /// Deprecated. Must use the MetricDescriptor.launch_stage instead.
    ///
    /// **Possible values**:
    /// - `LAUNCH_STAGE_UNSPECIFIED` — Do not use this default value.
    /// - `UNIMPLEMENTED` — The feature is not yet implemented. Users can not use it.
    /// - `PRELAUNCH` — Prelaunch features are hidden from users and are only visible internally.
    /// - `EARLY_ACCESS` — Early Access features are limited to a closed group of testers. To use these fea...
    /// - `ALPHA` — Alpha is a limited availability test for releases before they are cleared for wi...
    /// - `BETA` — Beta is the point at which we are ready to open a release for any customer to us...
    /// - `GA` — GA features are open to all developers and are considered stable and fully quali...
    /// - `DEPRECATED` — Deprecated features are scheduled to be shut down and removed. For more informat...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_stage: Option<String>,

    /// The sampling period of metric data points. For metrics which are written periodically,
    /// consecutive data points are stored at this time interval, excluding data loss due to
    /// errors. Metrics with a higher granularity have a smaller sampling period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_period: Option<String>,

    /// The delay of data points caused by ingestion. Data points older than this age are
    /// guaranteed to be ingested and available to be read, excluding data loss due to errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_delay: Option<String>,
}

impl MetricDescriptorMetadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            launch_stage: Some("test-launch_stage".into()),
            sample_period: Some("test-sample_period".into()),
            ingest_delay: Some("test-ingest_delay".into()),
        }
    }
}

/// The ListTimeSeries response.
///
/// **GCP API**: `monitoring.v3.ListTimeSeriesResponse`
/// **Reference**: <https://cloud.google.com/monitoring/api//ListTimeSeriesResponse>
///
/// ## Coverage
/// 3 of 5 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTimeSeriesResponse {
    /// One or more time series that match the filter included in the request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_series: Vec<TimeSeries>,

    /// If there are more results than have been returned, then this field is set to a non-empty
    /// value. To see the additional results, use that value as page_token in the next call to
    /// this method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The unit in which all time_series point values are reported. unit follows the UCUM
    /// format for units as seen in https://unitsofmeasure.org/ucum.html. If different
    /// time_series have different units (for example, because they come from different metric
    /// types, or a unit is absent), then unit will be "{not_a_unit}".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl ListTimeSeriesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            time_series: vec![],
            next_page_token: Some("test-next_page_token".into()),
            unit: Some("test-unit".into()),
        }
    }
}

/// An object that describes the schema of a MonitoredResource object using a type name and a
/// set of labels. For example, the monitored resource descriptor for Google Compute Engine VM
/// instances has a type of "gce_instance" and specifies the use of the labels "instance_id" and
/// "zone" to identify particular VM instances.Different APIs can support different monitored
/// resource types. APIs generally provide a list method that returns the monitored resource
/// descriptors used by the API.
///
/// **GCP API**: `monitoring.v3.MonitoredResourceDescriptor`
/// **Reference**: <https://cloud.google.com/monitoring/api//MonitoredResourceDescriptor>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredResourceDescriptor {
    /// Optional. The resource name of the monitored resource descriptor:
    /// "projects/{project_id}/monitoredResourceDescriptors/{type}" where {type} is the value of
    /// the type field in this object and {project_id} is a project ID that provides API-
    /// specific context for accessing the type. APIs that do not use project information can
    /// use the resource name format "monitoredResourceDescriptors/{type}".
    pub name: String,

    /// Required. The monitored resource type. For example, the type "cloudsql_database"
    /// represents databases in Google Cloud SQL. For a list of types, see Monitored resource
    /// types (https://cloud.google.com/monitoring/api/resources) and Logging resource types
    /// (https://cloud.google.com/logging/docs/api/v2/resource-list).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Optional. A concise name for the monitored resource type that might be displayed in user
    /// interfaces. It should be a Title Cased Noun Phrase, without any article or other
    /// determiners. For example, "Google Cloud SQL Database".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional. A detailed description of the monitored resource type that might be used in
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Required. A set of labels used to describe instances of this monitored resource type.
    /// For example, an individual Google Cloud SQL database is identified by values for the
    /// labels "database_id" and "zone".
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<LabelDescriptor>,

    /// Optional. The launch stage of the monitored resource definition.
    ///
    /// **Possible values**:
    /// - `LAUNCH_STAGE_UNSPECIFIED` — Do not use this default value.
    /// - `UNIMPLEMENTED` — The feature is not yet implemented. Users can not use it.
    /// - `PRELAUNCH` — Prelaunch features are hidden from users and are only visible internally.
    /// - `EARLY_ACCESS` — Early Access features are limited to a closed group of testers. To use these fea...
    /// - `ALPHA` — Alpha is a limited availability test for releases before they are cleared for wi...
    /// - `BETA` — Beta is the point at which we are ready to open a release for any customer to us...
    /// - `GA` — GA features are open to all developers and are considered stable and fully quali...
    /// - `DEPRECATED` — Deprecated features are scheduled to be shut down and removed. For more informat...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_stage: Option<String>,
}

impl MonitoredResourceDescriptor {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-monitored_resource_descriptor".into(),
            resource_type: Some("test-type".into()),
            display_name: Some("test-display_name".into()),
            description: Some("test-description".into()),
            labels: vec![],
            launch_stage: Some("test-launch_stage".into()),
        }
    }
}

/// Documentation that is included in the notifications and incidents pertaining to this policy.
///
/// **GCP API**: `monitoring.v3.Documentation`
/// **Reference**: <https://cloud.google.com/monitoring/api//Documentation>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    /// The body of the documentation, interpreted according to mime_type. The content may not
    /// exceed 8,192 Unicode characters and may not exceed more than 10,240 bytes when encoded
    /// in UTF-8 format, whichever is smaller. This text can be templatized by using variables
    /// (https://cloud.google.com/monitoring/alerts/doc-variables#doc-vars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Optional. The subject line of the notification. The subject line may not exceed 10,240
    /// bytes. In notifications generated by this policy, the contents of the subject line after
    /// variable expansion will be truncated to 255 bytes or shorter at the latest UTF-8
    /// character boundary. The 255-byte limit is recommended by this thread
    /// (https://stackoverflow.com/questions/1592291/what-is-the-email-subject-length-limit). It
    /// is both the limit imposed by some third-party ticketing products and it is common to
    /// define textual fields in databases as VARCHAR(255).The contents of the subject line can
    /// be templatized by using variables (https://cloud.google.com/monitoring/alerts/doc-
    /// variables#doc-vars). If this field is missing or empty, a default subject line will be
    /// generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// The format of the content field. Presently, only the value "text/markdown" is supported.
    /// See Markdown (https://en.wikipedia.org/wiki/Markdown) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl Documentation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            content: Some("test-content".into()),
            subject: Some("test-subject".into()),
            mime_type: Some("test-mime_type".into()),
        }
    }
}

/// A condition is a true/false test that determines when an alerting policy should open an
/// incident. If a condition evaluates to true, it signifies that something is wrong.
///
/// **GCP API**: `monitoring.v3.Condition`
/// **Reference**: <https://cloud.google.com/monitoring/api//Condition>
///
/// ## Coverage
/// 6 of 8 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Required if the condition exists. The unique resource name for this condition. Its
    /// format is:
    /// projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]
    /// [CONDITION_ID] is assigned by Cloud Monitoring when the condition is created as part of
    /// a new or updated alerting policy.When calling the alertPolicies.create method, do not
    /// include the name field in the conditions of the requested alerting policy. Cloud
    /// Monitoring creates the condition identifiers and includes them in the new policy.When
    /// calling the alertPolicies.update method to update a policy, including a condition name
    /// causes the existing condition to be updated. Conditions without names are added to the
    /// updated policy. Existing conditions are deleted if they are not updated.Best practice is
    /// to preserve [CONDITION_ID] if you make only small changes, such as those to condition
    /// thresholds, durations, or trigger values. Otherwise, treat the change as a new condition
    /// and let the existing condition be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A short name or phrase used to identify the condition in dashboards, notifications, and
    /// incidents. To avoid confusion, don't use the same display name for multiple conditions
    /// in the same policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// A condition that compares a time series against a threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_threshold: Option<serde_json::Value>,

    /// A condition that checks that a time series continues to receive new data points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_absent: Option<serde_json::Value>,

    /// A condition that checks for log messages matching given constraints. If set, no other
    /// conditions can be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_matched_log: Option<serde_json::Value>,

    /// A condition that uses the Monitoring Query Language to define alerts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_monitoring_query_language: Option<serde_json::Value>,
}

impl Condition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-condition".into()),
            display_name: Some("test-display_name".into()),
            ..Default::default()
        }
    }
}

/// A description of the conditions under which some aspect of your system is considered to be
/// "unhealthy" and the ways to notify people or services about this state. For an overview of
/// alerting policies, see Introduction to Alerting
/// (https://cloud.google.com/monitoring/alerts/).
///
/// **GCP API**: `monitoring.v3.AlertPolicy`
/// **Reference**: <https://cloud.google.com/monitoring/api//AlertPolicy>
///
/// ## Coverage
/// 9 of 13 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertPolicy {
    /// Identifier. Required if the policy exists. The resource name for this policy. The format
    /// is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is
    /// assigned by Cloud Monitoring when the policy is created. When calling the
    /// alertPolicies.create method, do not include the name field in the alerting policy passed
    /// as part of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A short name or phrase used to identify the policy in dashboards, notifications, and
    /// incidents. To avoid confusion, don't use the same display name for multiple policies in
    /// the same project. The name is limited to 512 Unicode characters.The convention for the
    /// display_name of a PrometheusQueryLanguageCondition is "{rule group name}/{alert name}",
    /// where the {rule group name} and {alert name} should be taken from the corresponding
    /// Prometheus configuration file. This convention is not enforced. In any case the
    /// display_name is not a unique key of the AlertPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Whether or not the policy is enabled. On write, the default interpretation if unset is
    /// that the policy is enabled. On read, clients should not make any assumption about the
    /// state if it has not been populated. The field should always be populated on List and Get
    /// operations, unless a field projection has been specified that strips it out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// A list of conditions for the policy. The conditions are combined by AND or OR according
    /// to the combiner field. If the combined conditions evaluate to true, then an incident is
    /// created. A policy can have from one to six conditions. If
    /// condition_time_series_query_language is present, it must be the only condition. If
    /// condition_monitoring_query_language is present, it must be the only condition.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,

    /// Identifies the notification channels to which notifications should be sent when
    /// incidents are opened or closed or when new violations occur on an already opened
    /// incident. Each element of this array corresponds to the name field in each of the
    /// NotificationChannel objects that are returned from the ListNotificationChannels method.
    /// The format of the entries in this field is:
    /// projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notification_channels: Vec<String>,

    /// How to combine the results of multiple conditions to determine if an incident should be
    /// opened. If condition_time_series_query_language is present, this must be
    /// COMBINE_UNSPECIFIED.
    ///
    /// **Possible values**:
    /// - `COMBINE_UNSPECIFIED` — An unspecified combiner.
    /// - `AND` — Combine conditions using the logical AND operator. An incident is created only i...
    /// - `OR` — Combine conditions using the logical OR operator. An incident is created if any ...
    /// - `AND_WITH_MATCHING_RESOURCE` — Combine conditions using logical AND operator, but unlike the regular AND option...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combiner: Option<String>,

    /// User-supplied key/value data to be used for organizing and identifying the AlertPolicy
    /// objects.The field can contain up to 64 entries. Each key and value is limited to 63
    /// Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain
    /// only lowercase letters, numerals, underscores, and dashes. Keys must begin with a
    /// letter.Note that Prometheus {alert name} is a valid Prometheus label names
    /// (https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels), whereas
    /// Prometheus {rule group} is an unrestricted UTF-8 string. This means that they cannot be
    /// stored as-is in user labels, because they may contain characters that are not allowed in
    /// user-label values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_labels: Option<std::collections::HashMap<String, String>>,

    /// Documentation that is included with notifications and incidents related to this policy.
    /// Best practice is for the documentation to include information to help responders
    /// understand, mitigate, escalate, and correct the underlying problems detected by the
    /// alerting policy. Notification channels that have limited capacity might not show this
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,

    /// Optional. The severity of an alerting policy indicates how important incidents generated
    /// by that policy are. The severity level will be displayed on the Incident detail page and
    /// in notifications.
    ///
    /// **Possible values**:
    /// - `SEVERITY_UNSPECIFIED` — No severity is specified. This is the default value.
    /// - `CRITICAL` — This is the highest severity level. Use this if the problem could cause signific...
    /// - `ERROR` — This is the medium severity level. Use this if the problem could cause minor dam...
    /// - `WARNING` — This is the lowest severity level. Use this if the problem is not causing any da...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl AlertPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-alert_policy".into()),
            display_name: Some("test-display_name".into()),
            enabled: Some(false),
            conditions: vec![],
            notification_channels: vec![],
            combiner: Some("test-combiner".into()),
            user_labels: Default::default(),
            documentation: Some(Documentation::fixture()),
            severity: Some("test-severity".into()),
        }
    }
}

/// A NotificationChannel is a medium through which an alert is delivered when a policy
/// violation is detected. Examples of channels include email, SMS, and third-party messaging
/// applications. Fields containing sensitive information like authentication tokens or contact
/// info are only partially populated on retrieval.
///
/// **GCP API**: `monitoring.v3.NotificationChannel`
/// **Reference**: <https://cloud.google.com/monitoring/api//NotificationChannel>
///
/// ## Coverage
/// 7 of 10 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationChannel {
    /// Identifier. The full REST resource name for this channel. The format is:
    /// projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is
    /// automatically assigned by the server on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An optional human-readable name for this notification channel. It is recommended that
    /// you specify a non-empty and unique name in order to make it easier to identify the
    /// channels in your project, though this is not enforced. The display name is limited to
    /// 512 Unicode characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The type of the notification channel. This field matches the value of the
    /// NotificationChannelDescriptor.type field.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,

    /// An optional human-readable description of this notification channel. This description
    /// may provide additional details, beyond the display name, for the channel. This may not
    /// exceed 1024 Unicode characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether notifications are forwarded to the described channel. This makes it possible to
    /// disable delivery of notifications to a particular channel without removing the channel
    /// from all alerting policies that reference the channel. This is a more convenient
    /// approach when the change is temporary and you want to receive notifications from the
    /// same set of alerting policies on the channel at some point in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Configuration fields that define the channel and its behavior. The permissible and
    /// required labels are specified in the NotificationChannelDescriptor.labels of the
    /// NotificationChannelDescriptor corresponding to the type field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// Indicates whether this channel has been verified or not. On a ListNotificationChannels
    /// or GetNotificationChannel operation, this field is expected to be populated.If the value
    /// is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires
    /// verification and lacks verification); otherwise, it is assumed that the channel works.If
    /// the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type
    /// that does not require verification or that this specific channel has been exempted from
    /// verification because it was created prior to verification being required for channels of
    /// this type.This field cannot be modified using a standard UpdateNotificationChannel
    /// operation. To change the value of this field, you must call VerifyNotificationChannel.
    ///
    /// **Possible values**:
    /// - `VERIFICATION_STATUS_UNSPECIFIED` — Sentinel value used to indicate that the state is unknown, omitted, or is not ap...
    /// - `UNVERIFIED` — The channel has yet to be verified and requires verification to function. Note t...
    /// - `VERIFIED` — It has been proven that notifications can be received on this notification chann...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

impl NotificationChannel {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-notification_channel".into()),
            display_name: Some("test-display_name".into()),
            channel_type: Some("test-type".into()),
            description: Some("test-description".into()),
            enabled: Some(false),
            labels: Default::default(),
            verification_status: Some("test-verification_status".into()),
        }
    }
}

/// The protocol for the ListAlertPolicies response.
///
/// **GCP API**: `monitoring.v3.ListAlertPoliciesResponse`
/// **Reference**: <https://cloud.google.com/monitoring/api//ListAlertPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAlertPoliciesResponse {
    /// The returned alert policies.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alert_policies: Vec<AlertPolicy>,

    /// If there might be more results than were returned, then this field is set to a non-empty
    /// value. To see the additional results, use that value as page_token in the next call to
    /// this method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The total number of alert policies in all pages. This number is only an estimate, and
    /// may change in subsequent pages. https://aip.dev/158
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}

impl ListAlertPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alert_policies: vec![],
            next_page_token: Some("test-next_page_token".into()),
            total_size: Some(100),
        }
    }
}

/// The ListNotificationChannels response.
///
/// **GCP API**: `monitoring.v3.ListNotificationChannelsResponse`
/// **Reference**: <https://cloud.google.com/monitoring/api//ListNotificationChannelsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationChannelsResponse {
    /// The notification channels defined for the specified project.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notification_channels: Vec<NotificationChannel>,

    /// If not empty, indicates that there may be more results that match the request. Use the
    /// value in the page_token field in a subsequent request to fetch the next set of results.
    /// If empty, all results have been returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The total number of notification channels in all pages. This number is only an estimate,
    /// and may change in subsequent pages. https://aip.dev/158
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}

impl ListNotificationChannelsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            notification_channels: vec![],
            next_page_token: Some("test-next_page_token".into()),
            total_size: Some(100),
        }
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in
/// your APIs. A typical example is to use it as the request or the response type of an API
/// method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns
/// (google.protobuf.Empty); }
///
/// **GCP API**: `monitoring.v3.Empty`
/// **Reference**: <https://cloud.google.com/monitoring/api//Empty>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringEmpty {}

impl MonitoringEmpty {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing MetricDescriptor resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMetricDescriptorsResponse {
    /// A list of MetricDescriptor resources.
    #[serde(default)]
    pub metric_descriptors: Vec<MetricDescriptor>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListMetricDescriptorsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metric_descriptors: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing MonitoredResourceDescriptor resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// A list of MonitoredResourceDescriptor resources.
    #[serde(default)]
    pub resource_descriptors: Vec<MonitoredResourceDescriptor>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListMonitoredResourceDescriptorsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource_descriptors: vec![],
            next_page_token: None,
        }
    }
}
