//! Types for the Cloud Logging API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/logging/v2/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Options that change functionality of a sink exporting data to BigQuery.
///
/// **GCP API**: `logging.v2.BigQueryOptions`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/BigQueryOptions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryOptions {
    /// Optional. Whether to use BigQuery's partition tables
    /// (https://docs.cloud.google.com/bigquery/docs/partitioned-tables). By default, Cloud
    /// Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523.
    /// With partitioned tables the date suffix is no longer present and special query syntax
    /// (https://docs.cloud.google.com/bigquery/docs/querying-partitioned-tables) has to be used
    /// instead. In both cases, tables are sharded based on UTC timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_partitioned_tables: Option<bool>,

    /// Output only. True if new timestamp column based partitioning is in use, false if legacy
    /// ingress-time partitioning is in use.All new sinks will have this field set true and will
    /// use timestamp column based partitioning. If use_partitioned_tables is false, this value
    /// has no meaning and will be false. Legacy sinks using partitioned tables will have this
    /// field set to false.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_timestamp_column_partitioning: Option<bool>,
}

impl BigQueryOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            use_partitioned_tables: Some(false),
            uses_timestamp_column_partitioning: Some(false),
        }
    }
}

/// Describes a sink used to export log entries to one of the following destinations: a Cloud
/// Logging log bucket, a Cloud Storage bucket, a BigQuery dataset, a Pub/Sub topic, a Cloud
/// project.A logs filter controls which log entries are exported. The sink must be created
/// within a project, organization, billing account, or folder.
///
/// **GCP API**: `logging.v2.LogSink`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/LogSink>
///
/// ## Coverage
/// 10 of 14 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSink {
    /// Optional. The client-assigned sink identifier, unique within the project.For example:
    /// "my-syslog-errors-to-pubsub".Sink identifiers are limited to 100 characters and can
    /// include only the following characters: upper and lower-case alphanumeric characters,
    /// underscores, hyphens, periods.First character has to be alphanumeric.
    pub name: String,

    /// Required. The export destination: "storage.googleapis.com/[GCS_BUCKET]"
    /// "bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]"
    /// "pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]"
    /// "logging.googleapis.com/projects/[PROJECT_ID]" "logging.googleapis.com/projects/[PROJECT
    /// _ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]" The sink's writer_identity, set when
    /// the sink is created, must have permission to write to the destination or else the log
    /// entries are not exported. For more information, see Route logs to supported destinations
    /// (https://docs.cloud.google.com/logging/docs/export/configure_export_v2).
    pub destination: String,

    /// Optional. An advanced logs filter
    /// (https://docs.cloud.google.com/logging/docs/view/building-queries#queries-by-
    /// expression). The only exported log entries are those that are in the resource owning the
    /// sink and that match the filter.For example:logName="projects/[PROJECT_ID]/logs/[LOG_ID]"
    /// AND severity>=ERROR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Optional. A description of this sink.The maximum length of the description is 8000
    /// characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. If set to true, then this sink is disabled and it does not export any log
    /// entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Output only. An IAM identity—a service account or group—under which Cloud Logging writes
    /// the exported log entries to the sink's destination. This field is either set by
    /// specifying custom_writer_identity or set automatically by sinks.create and sinks.update
    /// based on the value of unique_writer_identity in those methods.Until you grant this
    /// identity write-access to the destination, log entry exports from this sink will fail.
    /// For more information, see Manage access to projects, folders, and organizations
    /// (https://docs.cloud.google.com/iam/docs/granting-roles-to-service-
    /// accounts#granting_access_to_a_service_account_for_a_resource). Consult the destination
    /// service's documentation to determine the appropriate IAM roles to assign to the
    /// identity.Sinks that have a destination that is a log bucket in the same project as the
    /// sink cannot have a writer_identity and no additional permissions are required.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer_identity: Option<String>,

    /// Optional. This field applies only to sinks owned by organizations and folders. If the
    /// field is false, the default, only the logs owned by the sink's parent resource are
    /// available for export. If the field is true, then log entries from all the projects,
    /// folders, and billing accounts contained in the sink's parent resource are also available
    /// for export. Whether a particular log entry from the children is exported depends on the
    /// sink's filter expression.For example, if this field is true, then the filter
    /// resource.type=gce_instance would export all Compute Engine VM instance log entries from
    /// all projects in the sink's parent.To only export entries from certain child projects,
    /// filter on the project part of the log name:logName:("projects/test-project1/" OR
    /// "projects/test-project2/") AND resource.type=gce_instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,

    /// Optional. Options that affect sinks exporting data to BigQuery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigquery_options: Option<BigQueryOptions>,

    /// Output only. The creation timestamp of the sink.This field may not be present for older
    /// sinks.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. The last update timestamp of the sink.This field may not be present for
    /// older sinks.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl LogSink {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-log_sink".into(),
            destination: "test-destination".into(),
            filter: Some("test-filter".into()),
            description: Some("test-description".into()),
            disabled: Some(false),
            writer_identity: Some("test-writer_identity".into()),
            include_children: Some(false),
            bigquery_options: Some(BigQueryOptions::fixture()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
        }
    }
}

/// A description of a label.
///
/// **GCP API**: `logging.v2.LabelDescriptor`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/LabelDescriptor>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDescriptor {
    /// The label key.
    pub key: String,

    /// The type of data that can be assigned to the label.
    ///
    /// **Possible values**:
    /// - `STRING` — A variable-length string. This is the default.
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
/// **GCP API**: `logging.v2.MetricDescriptorMetadata`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/MetricDescriptorMetadata>
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

/// Defines a metric type and its schema. Once a metric descriptor is created, deleting or
/// altering it stops data collection and makes the metric type's existing data unusable.
///
/// **GCP API**: `logging.v2.MetricDescriptor`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/MetricDescriptor>
///
/// ## Coverage
/// 10 of 11 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricDescriptor {
    /// The resource name of the metric descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
}

impl MetricDescriptor {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-metric_descriptor".into()),
            metric_type: Some("test-type".into()),
            labels: vec![],
            metric_kind: Some("test-metric_kind".into()),
            value_type: Some("test-value_type".into()),
            unit: Some("test-unit".into()),
            description: Some("test-description".into()),
            display_name: Some("test-display_name".into()),
            metadata: Some(MetricDescriptorMetadata::fixture()),
            launch_stage: Some("test-launch_stage".into()),
        }
    }
}

/// Describes a logs-based metric. The value of the metric is the number of log entries that
/// match a logs filter in a given time interval.Logs-based metrics can also be used to extract
/// values from logs and create a distribution of the values. The distribution records the
/// statistics of the extracted values along with an optional histogram of the values as
/// specified by the bucket options.
///
/// **GCP API**: `logging.v2.LogMetric`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/LogMetric>
///
/// ## Coverage
/// 9 of 13 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMetric {
    /// Required. The client-assigned metric identifier. Examples: "error_count",
    /// "nginx/requests".Metric identifiers are limited to 100 characters and can include only
    /// the following characters: A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The
    /// forward-slash character (/) denotes a hierarchy of name pieces, and it cannot be the
    /// first character of the name.This field is the [METRIC_ID] part of a metric resource name
    /// in the format "projects/PROJECT_ID/metrics/METRIC_ID". Example: If the resource name of
    /// a metric is "projects/my-project/metrics/nginx%2Frequests", this field's value is
    /// "nginx/requests".
    pub name: String,

    /// Optional. A description of this metric, which is used in documentation. The maximum
    /// length of the description is 8000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Required. An advanced logs filter
    /// (https://cloud.google.com/logging/docs/view/advanced_filters) which is used to match log
    /// entries. Example: "resource.type=gae_app AND severity>=ERROR" The maximum length of the
    /// filter is 20000 characters.
    pub filter: String,

    /// Optional. If set to True, then this metric is disabled and it does not generate any
    /// points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Optional. The metric descriptor associated with the logs-based metric. If unspecified,
    /// it uses a default metric descriptor with a DELTA metric kind, INT64 value type, with no
    /// labels and a unit of "1". Such a metric counts the number of log entries matching the
    /// filter expression.The name, type, and description fields in the metric_descriptor are
    /// output only, and is constructed using the name and description field in the LogMetric.To
    /// create a logs-based metric that records a distribution of log values, a DELTA metric
    /// kind with a DISTRIBUTION value type must be used along with a value_extractor expression
    /// in the LogMetric.Each label in the metric descriptor must have a matching label name as
    /// the key and an extractor expression as the value in the label_extractors map.The
    /// metric_kind and value_type fields in the metric_descriptor cannot be updated once
    /// initially configured. New labels can be added in the metric_descriptor, but existing
    /// labels cannot be modified except for their description.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_descriptor: Option<MetricDescriptor>,

    /// Optional. A value_extractor is required when using a distribution logs-based metric to
    /// extract the values to record from a log entry. Two functions are supported for value
    /// extraction: EXTRACT(field) or REGEXP_EXTRACT(field, regex). The arguments are: field:
    /// The name of the log entry field from which the value is to be extracted. regex: A
    /// regular expression using the Google RE2 syntax
    /// (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data
    /// from the specified log entry field. The value of the field is converted to a string
    /// before applying the regex. It is an error to specify a regex that does not include
    /// exactly one capture group.The result of the extraction must be convertible to a double
    /// type, as the distribution always records double values. If either the extraction or the
    /// conversion to double fails, then those values are not recorded in the
    /// distribution.Example: REGEXP_EXTRACT(jsonPayload.request, ".*quantity=(\d+).*")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_extractor: Option<String>,

    /// Optional. A map from a label key string to an extractor expression which is used to
    /// extract data from a log entry field and assign as the label value. Each label key
    /// specified in the LabelDescriptor must have an associated extractor expression in this
    /// map. The syntax of the extractor expression is the same as for the value_extractor
    /// field.The extracted value is converted to the type defined in the label descriptor. If
    /// either the extraction or the type conversion fails, the label will have a default value.
    /// The default value for a string label is an empty string, for an integer label its 0, and
    /// for a boolean label its false.Note that there are upper bounds on the maximum number of
    /// labels and the number of active time series that are allowed in a project.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub label_extractors: HashMap<String, String>,

    /// Output only. The creation timestamp of the metric.This field may not be present for
    /// older metrics.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. The last update timestamp of the metric.This field may not be present for
    /// older metrics.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl LogMetric {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-log_metric".into(),
            description: Some("test-description".into()),
            filter: "test-filter".into(),
            disabled: Some(false),
            metric_descriptor: Some(MetricDescriptor::fixture()),
            value_extractor: Some("test-value_extractor".into()),
            label_extractors: Default::default(),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
        }
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in
/// your APIs. A typical example is to use it as the request or the response type of an API
/// method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns
/// (google.protobuf.Empty); }
///
/// **GCP API**: `logging.v2.Empty`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/Empty>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingEmpty {}

impl LoggingEmpty {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

/// Result returned from ListSinks.
///
/// **GCP API**: `logging.v2.ListSinksResponse`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/ListSinksResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSinksResponse {
    /// A list of sinks.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<LogSink>,

    /// If there might be more results than appear in this response, then nextPageToken is
    /// included. To get the next set of results, call the same method again using the value of
    /// nextPageToken as pageToken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListSinksResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            sinks: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Result returned from ListLogMetrics.
///
/// **GCP API**: `logging.v2.ListLogMetricsResponse`
/// **Reference**: <https://cloud.google.com/logging/docs/reference/v2/rest/ListLogMetricsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLogMetricsResponse {
    /// A list of logs-based metrics.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<LogMetric>,

    /// If there might be more results than appear in this response, then nextPageToken is
    /// included. To get the next set of results, call this method again using the value of
    /// nextPageToken as pageToken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListLogMetricsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            metrics: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}
