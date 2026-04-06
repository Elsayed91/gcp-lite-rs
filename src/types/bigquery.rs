//! Types for the BigQuery API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/bigquery/v2/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identifier for a dataset.
///
/// **GCP API**: `bigquery.v2.DatasetReference`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/DatasetReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID must contain
    /// only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024
    /// characters.
    pub dataset_id: String,

    /// Optional. The ID of the project containing this dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl DatasetReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dataset_id: "test-dataset_id".into(),
            project_id: Some("test-project_id".into()),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.TableReference`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableReference {
    /// Required. The ID of the dataset containing this table.
    pub dataset_id: String,

    /// Required. The ID of the project containing this table.
    pub project_id: String,

    /// Required. The ID of the table. The ID can contain Unicode characters in category L
    /// (letter), M (mark), N (number), Pc (connector, including underscore), Pd (dash), and Zs
    /// (space). For more information, see [General
    /// Category](https://wikipedia.org/wiki/Unicode_character_property#General_Category). The
    /// maximum length is 1,024 characters. Certain operations allow suffixing of the table ID
    /// with a partition decorator, such as `sample_table$20190123`.
    pub table_id: String,
}

impl TableReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dataset_id: "test-dataset_id".into(),
            project_id: "test-project_id".into(),
            table_id: "test-table_id".into(),
        }
    }
}

/// A job reference is a fully qualified identifier for referring to a job.
///
/// **GCP API**: `bigquery.v2.JobReference`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReference {
    /// Required. The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9),
    /// underscores (_), or dashes (-). The maximum length is 1,024 characters.
    pub job_id: String,

    /// Optional. The geographic location of the job. The default value is US. For more
    /// information about BigQuery locations, see:
    /// https://cloud.google.com/bigquery/docs/locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Required. The ID of the project containing this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl JobReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            job_id: "test-job_id".into(),
            location: Some("test-location".into()),
            project_id: Some("test-project_id".into()),
        }
    }
}

/// Error details.
///
/// **GCP API**: `bigquery.v2.ErrorProto`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/ErrorProto>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProto {
    /// Debugging information. This property is internal to Google and should not be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<String>,

    /// Specifies where the error occurred, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// A human-readable description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// A short error code that summarizes the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ErrorProto {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            debug_info: Some("test-debug_info".into()),
            location: Some("test-location".into()),
            message: Some("test-message".into()),
            reason: Some("test-reason".into()),
        }
    }
}

/// Schema of a table
///
/// **GCP API**: `bigquery.v2.TableSchema`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableSchema>
///
/// ## Coverage
/// 1 of 2 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<TableFieldSchema>,
}

impl TableSchema {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { fields: vec![] }
    }
}

/// A field in TableSchema
///
/// **GCP API**: `bigquery.v2.TableFieldSchema`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableFieldSchema>
///
/// ## Coverage
/// 5 of 17 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFieldSchema {
    /// Optional. The field description. The maximum length is 1,024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. Describes the nested schema fields if the type property is set to RECORD.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<TableFieldSchema>,

    /// Optional. The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The
    /// default value is NULLABLE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Required. The field name. The name must contain only letters (a-z, A-Z), numbers (0-9),
    /// or underscores (_), and must start with a letter or underscore. The maximum length is
    /// 300 characters.
    pub name: String,

    /// Required. The field data type. Possible values include:
    /// * STRING
    /// * BYTES
    /// * INTEGER (or INT64)
    /// * FLOAT (or FLOAT64)
    /// * BOOLEAN (or BOOL)
    /// * TIMESTAMP
    /// * DATE
    /// * TIME
    /// * DATETIME
    /// * GEOGRAPHY
    /// * NUMERIC
    /// * BIGNUMERIC
    /// * JSON
    /// * RECORD (or STRUCT)
    /// * RANGE Use of RECORD/STRUCT indicates that the field contains a nested schema.
    #[serde(rename = "type")]
    pub field_type: String,
}

impl TableFieldSchema {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            description: Some("test-description".into()),
            fields: vec![],
            mode: Some("test-mode".into()),
            name: "test-table_field_schema".into(),
            field_type: "test-type".into(),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.TableRow`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableRow>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    /// Represents a single row in the result set, consisting of one or more fields.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub f: Vec<TableCell>,
}

impl TableRow {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { f: vec![] }
    }
}

///
/// **GCP API**: `bigquery.v2.TableCell`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableCell>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    /// The `v` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<serde_json::Value>,
}

impl TableCell {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// Configures the access a dataset defined in an external metadata storage.
///
/// **GCP API**: `bigquery.v2.ExternalDatasetReference`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/ExternalDatasetReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDatasetReference {
    /// Required. The connection id that is used to access the external_source. Format:
    /// projects/{project_id}/locations/{location_id}/connections/{connection_id}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,

    /// Required. External source that backs this dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source: Option<String>,
}

impl ExternalDatasetReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            connection: Some("test-connection".into()),
            external_source: Some("test-external_source".into()),
        }
    }
}

/// Represents a BigQuery dataset.
///
/// **GCP API**: `bigquery.v2.Dataset`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/Dataset>
///
/// ## Coverage
/// 15 of 30 fields included.
/// Omitted fields:
/// - `defaultCollation` — Rarely used
/// - `defaultEncryptionConfiguration` — CMEK — separate concern
/// - `defaultRoundingMode` — Rarely used
/// - `externalCatalogDatasetOptions` — External catalog — uncommon
/// - `externalDatasetReference` — External dataset — uncommon
/// - `isCaseInsensitive` — Rarely used
/// - `linkedDatasetMetadata` — Linked datasets — uncommon
/// - `linkedDatasetSource` — Linked datasets — uncommon
/// - `maxTimeTravelHours` — Advanced feature
/// - `resourceTags` — Resource tags — separate concern
/// - `restrictions` — Restrictions — uncommon
/// - `satisfiesPzi` — Compliance flags
/// - `satisfiesPzs` — Compliance flags
/// - `storageBillingModel` — Billing model — uncommon
/// - `tags` — Tags — separate concern
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    /// Optional. An array of objects that define dataset access for one or more entities. You
    /// can set this property when inserting or updating a dataset in order to control who is
    /// allowed to access the data. If unspecified at dataset creation time, BigQuery adds
    /// default dataset access for the following entities: access.specialGroup: projectReaders;
    /// access.role: READER; access.specialGroup: projectWriters; access.role: WRITER;
    /// access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset
    /// creator email]; access.role: OWNER; If you patch a dataset, then this field is
    /// overwritten by the patched dataset's access field. To add entities, you must supply the
    /// entire existing access array in addition to any new entities that you want to add.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access: Vec<serde_json::Value>,

    /// Required. A reference that identifies the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_reference: Option<DatasetReference>,

    /// Optional. A user-friendly description of the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. A descriptive name for the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,

    /// Output only. The fully-qualified unique name of the dataset in the format
    /// projectId:datasetId. The dataset name without the project name is given in the datasetId
    /// field. When creating a new dataset, leave this field blank, and instead specify the
    /// datasetId field.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. The resource type.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The labels associated with this dataset. You can use these to organize and group your
    /// datasets. You can set this property when inserting or updating a dataset. See [Creating
    /// and Updating Dataset Labels](https://cloud.google.com/bigquery/docs/creating-managing-
    /// labels#creating_and_updating_dataset_labels) for more information.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// The geographic location where the dataset should reside. See
    /// https://cloud.google.com/bigquery/docs/locations for supported locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Output only. The time when this dataset was created, in milliseconds since the epoch.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// Output only. The date when this dataset was last modified, in milliseconds since the
    /// epoch.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,

    /// Optional. The default lifetime of all tables in the dataset, in milliseconds. The
    /// minimum lifetime value is 3600000 milliseconds (one hour). To clear an existing default
    /// expiration with a PATCH request, set to 0. Once this property is set, all newly-created
    /// tables in the dataset will have an expirationTime property set to the creation time plus
    /// the value in this property, and changing the value will only affect new tables, not
    /// existing ones. When the expirationTime for a given table is reached, that table will be
    /// deleted automatically. If a table's expirationTime is modified or removed before the
    /// table expires, or if you provide an explicit expirationTime when creating a table, that
    /// value takes precedence over the default expiration time indicated by this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_table_expiration_ms: Option<String>,

    /// This default partition expiration, expressed in milliseconds. When new time-partitioned
    /// tables are created in a dataset where this property is set, the table will inherit this
    /// value, propagated as the `TimePartitioning.expirationMs` property on the new table. If
    /// you set `TimePartitioning.expirationMs` explicitly when creating a table, the
    /// `defaultPartitionExpirationMs` of the containing dataset is ignored. When creating a
    /// partitioned table, if `defaultPartitionExpirationMs` is set, the
    /// `defaultTableExpirationMs` value is ignored and the table will not be inherit a table
    /// expiration deadline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_partition_expiration_ms: Option<String>,

    /// Output only. A hash of the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Output only. A URL that can be used to access the resource again. You can use this URL
    /// in Get or Update requests to the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. Same as `type` in `ListFormatDataset`. The type of the dataset, one of:
    /// * DEFAULT
    /// - only accessible by owner and authorized accounts,
    /// * PUBLIC
    /// - accessible by everyone,
    /// * LINKED
    /// - linked dataset,
    /// * EXTERNAL
    /// - dataset with definition in external metadata catalog.
    ///
    /// *Output-only field.*
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
}

impl Dataset {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access: vec![],
            dataset_reference: Some(DatasetReference::fixture()),
            description: Some("test-description".into()),
            friendly_name: Some("test-friendly_name".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            labels: Default::default(),
            location: Some("test-location".into()),
            creation_time: Some("test-creation_time".into()),
            last_modified_time: Some("test-last_modified_time".into()),
            default_table_expiration_ms: Some("test-default_table_expiration_ms".into()),
            default_partition_expiration_ms: Some("test-default_partition_expiration_ms".into()),
            etag: Some("test-etag".into()),
            self_link: Some("test-self_link".into()),
            type_value: Some("test-type".into()),
        }
    }
}

/// Response format for a page of results when listing datasets.
///
/// **GCP API**: `bigquery.v2.DatasetList`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/DatasetList>
///
/// ## Coverage
/// 4 of 5 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetList {
    /// An array of the dataset resources in the project. Each resource contains basic
    /// information. For full information about a particular dataset resource, use the Datasets:
    /// get method. This property is omitted when there are no datasets in the project.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub datasets: Vec<DatasetListItem>,

    /// Output only. A hash value of the results page. You can use this property to determine if
    /// the page has changed since the last request.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Output only. The resource type. This property always returns the value
    /// "bigquery#datasetList"
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A token that can be used to request the next results page. This property is omitted on
    /// the final results page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl DatasetList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            datasets: vec![],
            etag: Some("test-etag".into()),
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.Table`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/Table>
///
/// ## Coverage
/// 21 of 52 fields included.
/// Omitted fields:
/// - `biglakeConfiguration` — BigLake — uncommon
/// - `cloneDefinition` — Clone metadata — uncommon
/// - `defaultCollation` — Rarely used
/// - `defaultRoundingMode` — Rarely used
/// - `encryptionConfiguration` — CMEK — separate concern
/// - `externalCatalogTableOptions` — External catalog — uncommon
/// - `externalDataConfiguration` — External data — separate concern
/// - `managedTableType` — Rarely used
/// - `materializedView` — Materialized views — separate concern
/// - `materializedViewStatus` — Materialized views — separate concern
/// - `maxStaleness` — Materialized views — separate concern
/// - `model` — Deprecated
/// - `numActiveLogicalBytes` — Detailed storage metrics
/// - `numActivePhysicalBytes` — Detailed storage metrics
/// - `numCurrentPhysicalBytes` — Detailed storage metrics
/// - `numLongTermBytes` — Detailed storage metrics
/// - `numLongTermLogicalBytes` — Detailed storage metrics
/// - `numLongTermPhysicalBytes` — Detailed storage metrics
/// - `numPhysicalBytes` — Detailed storage metrics
/// - `numTimeTravelPhysicalBytes` — Detailed storage metrics
/// - `numTotalLogicalBytes` — Detailed storage metrics
/// - `numTotalPhysicalBytes` — Detailed storage metrics
/// - `partitionDefinition` — Detailed partitioning info
/// - `replicas` — Replication — uncommon
/// - `resourceTags` — Resource tags — separate concern
/// - `restrictions` — Restrictions — uncommon
/// - `snapshotDefinition` — Snapshots — uncommon
/// - `streamingBuffer` — Streaming info — separate concern
/// - `tableConstraints` — Constraints — uncommon
/// - `tableReplicationInfo` — Replication — uncommon
/// - `view` — View definition — separate concern
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    /// Required. Reference describing the ID of this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_reference: Option<TableReference>,

    /// Optional. A user-friendly description of this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional. A descriptive name for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,

    /// Output only. An opaque ID uniquely identifying the table.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The type of resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The labels associated with this table. You can use these to organize and group your
    /// tables. Label keys and values can be no longer than 63 characters, can only contain
    /// lowercase letters, numeric characters, underscores and dashes. International characters
    /// are allowed. Label values are optional. Label keys must start with a letter and each
    /// label in the list must have a different key.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. The geographic location where the table resides. This value is inherited
    /// from the dataset.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Output only. The time when this table was created, in milliseconds since the epoch.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// Output only. The time when this table was last modified, in milliseconds since the
    /// epoch.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,

    /// Optional. The time when this table expires, in milliseconds since the epoch. If not
    /// present, the table will persist indefinitely. Expired tables will be deleted and their
    /// storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset
    /// can be used to set a default expirationTime on newly created tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,

    /// Optional. Describes the schema of this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,

    /// Output only. The size of this table in logical bytes, excluding any data in the
    /// streaming buffer.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_bytes: Option<String>,

    /// Output only. The number of rows of data in this table, excluding any data in the
    /// streaming buffer.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows: Option<String>,

    /// Output only. The number of partitions present in the table or materialized view. This
    /// data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_partitions: Option<String>,

    /// Output only. Describes the table type. The following values are supported:
    /// * `TABLE`: A normal BigQuery table.
    /// * `VIEW`: A virtual table defined by a SQL query.
    /// * `EXTERNAL`: A table that references data stored in an external storage system, such as
    ///   Google Cloud Storage.
    /// * `MATERIALIZED_VIEW`: A precomputed view defined by a SQL query.
    /// * `SNAPSHOT`: An immutable BigQuery table that preserves the contents of a base table at
    ///   a particular time. See additional information on [table
    ///   snapshots](https://cloud.google.com/bigquery/docs/table-snapshots-intro). The default
    ///   value is `TABLE`.
    ///
    /// *Output-only field.*
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,

    /// Output only. A hash of this resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Output only. A URL that can be used to access this resource again.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Clustering specification for the table. Must be specified with time-based partitioning,
    /// data in the table will be first partitioned and subsequently clustered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,

    /// If specified, configures time-based partitioning for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,

    /// If specified, configures range partitioning for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,

    /// Optional. If set to true, queries over this table require a partition filter that can be
    /// used for partition elimination to be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_partition_filter: Option<bool>,
}

impl Table {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            table_reference: Some(TableReference::fixture()),
            description: Some("test-description".into()),
            friendly_name: Some("test-friendly_name".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            labels: Default::default(),
            location: Some("test-location".into()),
            creation_time: Some("test-creation_time".into()),
            last_modified_time: Some("test-last_modified_time".into()),
            expiration_time: Some("test-expiration_time".into()),
            schema: Some(TableSchema::fixture()),
            num_bytes: Some("test-num_bytes".into()),
            num_rows: Some("test-num_rows".into()),
            num_partitions: Some("test-num_partitions".into()),
            type_value: Some("test-type".into()),
            etag: Some("test-etag".into()),
            self_link: Some("test-self_link".into()),
            clustering: Some(Clustering::fixture()),
            time_partitioning: Some(TimePartitioning::fixture()),
            range_partitioning: Some(RangePartitioning::fixture()),
            require_partition_filter: Some(false),
        }
    }
}

/// Partial projection of the metadata for a given table in a list response.
///
/// **GCP API**: `bigquery.v2.TableList`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TableList>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableList {
    /// A hash of this page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The type of list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A token to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// Tables in the requested dataset.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<TableListItem>,

    /// The total number of tables in the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i32>,
}

impl TableList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            etag: Some("test-etag".into()),
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
            tables: vec![],
            total_items: Some(100),
        }
    }
}

/// Configures table clustering.
///
/// **GCP API**: `bigquery.v2.Clustering`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/Clustering>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clustering {
    /// One or more fields on which data should be clustered. Only top-level, non-repeated,
    /// simple-type fields are supported. The ordering of the clustering fields should be
    /// prioritized from most to least important for filtering purposes. For additional
    /// information, see [Introduction to clustered
    /// tables](https://cloud.google.com/bigquery/docs/clustered-tables#limitations).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,
}

impl Clustering {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { fields: vec![] }
    }
}

///
/// **GCP API**: `bigquery.v2.TimePartitioning`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/TimePartitioning>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePartitioning {
    /// Optional. Number of milliseconds for which to keep the storage for a partition. A
    /// wrapper is used here because 0 is an invalid value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ms: Option<String>,

    /// Optional. If not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if
    /// set, the table is partitioned by this field. The field must be a top-level TIMESTAMP or
    /// DATE field. Its mode must be NULLABLE or REQUIRED. A wrapper is used here because an
    /// empty string is an invalid value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// Required. The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one
    /// partition per day, hour, month, and year, respectively.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_type: Option<String>,
}

impl TimePartitioning {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expiration_ms: Some("test-expiration_ms".into()),
            field: Some("test-field".into()),
            partition_type: Some("test-type".into()),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.RangePartitioning`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/RangePartitioning>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangePartitioning {
    /// Required. The name of the column to partition the table on. It must be a top-level,
    /// INT64 column whose mode is NULLABLE or REQUIRED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// [Experimental] Defines the ranges for range partitioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangePartitioningRange>,
}

impl RangePartitioning {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            field: Some("test-field".into()),
            range: Default::default(),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.Job`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/Job>
///
/// ## Coverage
/// 9 of 11 fields included.
/// Omitted fields:
/// - `jobCreationReason` — Rarely inspected
/// - `principal_subject` — Identity info — rarely needed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// Required. Describes the job configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<JobConfiguration>,

    /// Output only. A hash of this resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Output only. Opaque ID field of the job.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Optional. Reference describing the unique-per-user name of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,

    /// Output only. The type of the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Output only. A URL that can be used to access the resource again.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. Information about the job, including starting time and ending time of the
    /// job.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,

    /// Output only. The status of this job. Examine this value when polling an asynchronous job
    /// to see if the job is complete.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,

    /// Output only. Email address of the user who ran the job.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}

impl Job {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            configuration: Some(JobConfiguration::fixture()),
            etag: Some("test-etag".into()),
            id: Some("test-id".into()),
            job_reference: Some(JobReference::fixture()),
            kind: Some("test-kind".into()),
            self_link: Some("test-self_link".into()),
            statistics: Some(JobStatistics::fixture()),
            status: Some(JobStatus::fixture()),
            user_email: Some("test-user_email".into()),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.JobConfiguration`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobConfiguration>
///
/// ## Coverage
/// 4 of 10 fields included.
/// Omitted fields:
/// - `copy` — Copy jobs — not in scope
/// - `extract` — Extract jobs — not in scope
/// - `load` — Load jobs — not in scope
/// - `jobTimeoutMs` — Rarely set
/// - `maxSlots` — Rate limiting — rarely used
/// - `reservation` — Reservation — rarely used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfiguration {
    /// Optional. If set, don't actually run this job. A valid query will return a mostly empty
    /// response with some processing statistics, while an invalid query will return the same
    /// error it would if it wasn't a dry run. Behavior of non-query jobs is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Output only. The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,

    /// The labels associated with this job. You can use these to organize and group your jobs.
    /// Label keys and values can be no longer than 63 characters, can only contain lowercase
    /// letters, numeric characters, underscores and dashes. International characters are
    /// allowed. Label values are optional. Label keys must start with a letter and each label
    /// in the list must have a different key.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// [Pick one] Configures a query job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<JobConfigurationQuery>,
}

impl JobConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dry_run: Some(false),
            job_type: Some("test-job_type".into()),
            labels: Default::default(),
            query: Some(JobConfigurationQuery::fixture()),
        }
    }
}

/// JobConfigurationQuery configures a BigQuery query job.
///
/// **GCP API**: `bigquery.v2.JobConfigurationQuery`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobConfigurationQuery>
///
/// ## Coverage
/// 10 of 28 fields included.
/// Omitted fields:
/// - `allowLargeResults` — Legacy SQL only
/// - `clustering` — Advanced partitioning
/// - `connectionProperties` — Connection properties — uncommon
/// - `continuous` — Continuous queries — uncommon
/// - `createSession` — Sessions — uncommon
/// - `destinationEncryptionConfiguration` — CMEK — separate concern
/// - `flattenResults` — Legacy SQL only
/// - `maximumBillingTier` — Deprecated
/// - `preserveNulls` — Deprecated
/// - `queryParameters` — Parameterized queries — can extend later
/// - `rangePartitioning` — Advanced partitioning
/// - `schemaUpdateOptions` — Schema updates — uncommon
/// - `scriptOptions` — Script options — uncommon
/// - `systemVariables` — System variables — read-only
/// - `tableDefinitions` — External tables — uncommon
/// - `timePartitioning` — Advanced partitioning
/// - `userDefinedFunctionResources` — UDFs — uncommon
/// - `writeIncrementalResults` — Incremental results — uncommon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfigurationQuery {
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate
    /// whether the query uses legacy SQL or GoogleSQL.
    pub query: String,

    /// Optional. Specifies whether to use BigQuery's legacy SQL dialect for this query. The
    /// default value is true. If set to false, the query uses BigQuery's
    /// [GoogleSQL](https://docs.cloud.google.com/bigquery/docs/introduction-sql). When
    /// useLegacySql is set to false, the value of flattenResults is ignored; query will be run
    /// as if flattenResults is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,

    /// Optional. Whether to look for the result in the query cache. The query cache is a best-
    /// effort cache that will be flushed whenever tables in the query are modified. Moreover,
    /// the query cache is only available when a query does not have a destination table
    /// specified. The default value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,

    /// Optional. Specifies the default dataset to use for unqualified table names in the query.
    /// This setting does not alter behavior of unqualified dataset names. Setting the system
    /// variable `@@dataset_id` achieves the same behavior. See
    /// https://cloud.google.com/bigquery/docs/reference/system-variables for more information
    /// on system variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dataset: Option<DatasetReference>,

    /// Optional. Describes the table where the query results should be stored. This property
    /// must be set for large results that exceed the maximum response size. For queries that
    /// produce anonymous (cached) results, this field will be populated by BigQuery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<TableReference>,

    /// Optional. Specifies whether the job is allowed to create new tables. The following
    /// values are supported:
    /// * CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// * CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is
    ///   returned in the job result. The default value is CREATE_IF_NEEDED. Creation,
    ///   truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,

    /// Optional. Specifies the action that occurs if the destination table already exists. The
    /// following values are supported:
    /// * WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the data, removes the
    ///   constraints, and uses the schema from the query result.
    /// * WRITE_TRUNCATE_DATA: If the table already exists, BigQuery overwrites the data, but
    ///   keeps the constraints and schema of the existing table.
    /// * WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// * WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is
    ///   returned in the job result. The default value is WRITE_EMPTY. Each action is atomic
    ///   and only occurs if BigQuery is able to complete the job successfully. Creation,
    ///   truncation and append actions occur as one atomic update upon job completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,

    /// Optional. Specifies a priority for the query. Possible values include INTERACTIVE and
    /// BATCH. The default value is INTERACTIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Limits the bytes billed for this job. Queries that will have bytes billed beyond this
    /// limit will fail (without incurring a charge). If unspecified, this will be set to your
    /// project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,

    /// GoogleSQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to
    /// use named (@myparam) query parameters in this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<String>,
}

impl JobConfigurationQuery {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            query: "test-query".into(),
            use_legacy_sql: Some(false),
            use_query_cache: Some(false),
            default_dataset: Some(DatasetReference::fixture()),
            destination_table: Some(TableReference::fixture()),
            create_disposition: Some("test-create_disposition".into()),
            write_disposition: Some("test-write_disposition".into()),
            priority: Some("test-priority".into()),
            maximum_bytes_billed: Some("test-maximum_bytes_billed".into()),
            parameter_mode: Some("test-parameter_mode".into()),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.JobStatus`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobStatus>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    /// Output only. Final error result of the job. If present, indicates that the job has
    /// completed and was unsuccessful.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_result: Option<ErrorProto>,

    /// Output only. The first errors encountered during the running of the job. The final
    /// message includes the number of errors that caused the process to stop. Errors here do
    /// not necessarily mean that the job has not completed or was unsuccessful.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorProto>,

    /// Output only. Running state of the job. Valid states include 'PENDING', 'RUNNING', and
    /// 'DONE'.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl JobStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            error_result: Some(ErrorProto::fixture()),
            errors: vec![],
            state: Some("test-state".into()),
        }
    }
}

/// Statistics for a single job execution.
///
/// **GCP API**: `bigquery.v2.JobStatistics`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobStatistics>
///
/// ## Coverage
/// 7 of 23 fields included.
/// Omitted fields:
/// - `completionRatio` — For load/extract jobs
/// - `copy` — Copy job stats
/// - `dataMaskingStatistics` — Data masking stats
/// - `edition` — Edition info
/// - `extract` — Extract job stats
/// - `finalExecutionDurationMs` — Final execution duration
/// - `load` — Load job stats
/// - `query` — Query-specific stats (JobStatistics2) — large nested type
/// - `quotaDeferments` — Quota deferments
/// - `reservationGroupPath` — Reservation info
/// - `reservationUsage` — Deprecated
/// - `reservation_id` — Reservation info
/// - `rowLevelSecurityStatistics` — RLS stats
/// - `scriptStatistics` — Script stats
/// - `sessionInfo` — Session info
/// - `transactionInfo` — Transaction info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics {
    /// Output only. Creation time of this job, in milliseconds since the epoch. This field will
    /// be present on all jobs.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// Output only. End time of this job, in milliseconds since the epoch. This field will be
    /// present whenever a job is in the DONE state.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Output only. Start time of this job, in milliseconds since the epoch. This field will be
    /// present when the job transitions from the PENDING state to either RUNNING or DONE.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Output only. Total bytes processed for the job.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,

    /// Output only. Slot-milliseconds for the job.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_slot_ms: Option<String>,

    /// Output only. Number of child jobs executed.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_child_jobs: Option<String>,

    /// Output only. If this is a child job, specifies the job ID of the parent.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<String>,
}

impl JobStatistics {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            creation_time: Some("test-creation_time".into()),
            end_time: Some("test-end_time".into()),
            start_time: Some("test-start_time".into()),
            total_bytes_processed: Some("test-total_bytes_processed".into()),
            total_slot_ms: Some("test-total_slot_ms".into()),
            num_child_jobs: Some("test-num_child_jobs".into()),
            parent_job_id: Some("test-parent_job_id".into()),
        }
    }
}

/// JobList is the response format for a jobs.list call.
///
/// **GCP API**: `bigquery.v2.JobList`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobList>
///
/// ## Coverage
/// 4 of 5 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    /// A hash of this page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// List of jobs that were requested.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub jobs: Vec<JobListItem>,

    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A token to request the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl JobList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            etag: Some("test-etag".into()),
            jobs: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Describes the format of the jobs.query request.
///
/// **GCP API**: `bigquery.v2.QueryRequest`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/QueryRequest>
///
/// ## Coverage
/// 11 of 25 fields included.
/// Omitted fields:
/// - `connectionProperties` — Connection properties — uncommon
/// - `continuous` — Continuous queries — uncommon
/// - `createSession` — Sessions — uncommon
/// - `destinationEncryptionConfiguration` — CMEK — separate concern
/// - `formatOptions` — Format options — uncommon
/// - `jobCreationMode` — Job creation mode — advanced
/// - `jobTimeoutMs` — Job timeout — use timeoutMs instead
/// - `kind` — Always bigquery#queryRequest
/// - `maxSlots` — Rate limiting — rarely used
/// - `preserveNulls` — Deprecated
/// - `queryParameters` — Parameterized queries — can extend later
/// - `requestId` — Idempotency — can extend later
/// - `reservation` — Reservation — rarely used
/// - `writeIncrementalResults` — Incremental results — uncommon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    /// Required. A query string to execute, using Google Standard SQL or legacy SQL syntax.
    /// Example: "SELECT COUNT(f1) FROM myProjectId.myDatasetId.myTableId".
    pub query: String,

    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value
    /// is true. If set to false, the query uses BigQuery's
    /// [GoogleSQL](https://docs.cloud.google.com/bigquery/docs/introduction-sql). When
    /// useLegacySql is set to false, the value of flattenResults is ignored; query will be run
    /// as if flattenResults is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,

    /// Optional. Whether to look for the result in the query cache. The query cache is a best-
    /// effort cache that will be flushed whenever tables in the query are modified. The default
    /// value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,

    /// Optional. Specifies the default datasetId and projectId to assume for any unqualified
    /// table names in the query. If not set, all table names in the query string must be
    /// qualified in the format 'datasetId.tableId'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dataset: Option<DatasetReference>,

    /// Optional. If set to true, BigQuery doesn't run the job. Instead, if the query is valid,
    /// BigQuery returns statistics about the job such as how many bytes would be processed. If
    /// the query is invalid, an error returns. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// Optional. The maximum number of rows of data to return per page of results. Setting this
    /// flag to a small value such as 1000 and then paging through results might improve
    /// reliability when the query result set is large. In addition to this limit, responses are
    /// also limited to 10 MB. By default, there is no maximum row count, and only the byte
    /// limit applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,

    /// Optional. Optional: Specifies the maximum amount of time, in milliseconds, that the
    /// client is willing to wait for the query to complete. By default, this limit is 10
    /// seconds (10,000 milliseconds). If the query is complete, the jobComplete field in the
    /// response is true. If the query has not yet completed, jobComplete is false. You can
    /// request a longer timeout period in the timeoutMs field. However, the call is not
    /// guaranteed to wait for the specified timeout; it typically returns after around 200
    /// seconds (200,000 milliseconds), even if the query is not complete. If jobComplete is
    /// false, you can continue to wait for the query to complete by calling the getQueryResults
    /// method until the jobComplete field in the getQueryResults response is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,

    /// The geographic location where the job should run. For more information, see how to
    /// [specify locations](https://cloud.google.com/bigquery/docs/locations#specify_locations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Optional. Limits the bytes billed for this query. Queries with bytes billed above this
    /// limit will fail (without incurring a charge). If unspecified, the project default is
    /// used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,

    /// GoogleSQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to
    /// use named (@myparam) query parameters in this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<String>,

    /// Optional. The labels associated with this query. Labels can be used to organize and
    /// group query jobs. Label keys and values can be no longer than 63 characters, can only
    /// contain lowercase letters, numeric characters, underscores and dashes. International
    /// characters are allowed. Label keys must start with a letter and each label in the list
    /// must have a different key.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl QueryRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            query: "test-query".into(),
            use_legacy_sql: Some(false),
            use_query_cache: Some(false),
            default_dataset: Some(DatasetReference::fixture()),
            dry_run: Some(false),
            max_results: Some(100),
            timeout_ms: Some(100),
            location: Some("test-location".into()),
            maximum_bytes_billed: Some("test-maximum_bytes_billed".into()),
            parameter_mode: Some("test-parameter_mode".into()),
            labels: Default::default(),
        }
    }
}

///
/// **GCP API**: `bigquery.v2.QueryResponse`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/QueryResponse>
///
/// ## Coverage
/// 11 of 21 fields included.
/// Omitted fields:
/// - `creationTime` — Rarely needed from query response
/// - `dmlStats` — DML statistics — uncommon
/// - `endTime` — Rarely needed
/// - `jobCreationReason` — Rarely inspected
/// - `location` — Can get from jobReference
/// - `queryId` — Internal query ID
/// - `sessionInfo` — Session info
/// - `startTime` — Rarely needed
/// - `totalBytesBilled` — Billing info
/// - `totalSlotMs` — Slot usage info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    /// Whether the query has completed or not. If rows or totalRows are present, this will
    /// always be true. If this is false, totalRows will not be available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_complete: Option<bool>,

    /// Reference to the Job that was created to run the query. This field will be present even
    /// if the original request timed out, in which case GetQueryResults can be used to read the
    /// results once the query has completed. Since this API only returns the first page of
    /// results, subsequent pages can be fetched via the same mechanism (GetQueryResults). If
    /// job_creation_mode was set to `JOB_CREATION_OPTIONAL` and the query completes without
    /// creating a job, this field will be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,

    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// An object with as many results as can be contained within the maximum permitted reply
    /// size. To get any additional rows, you can call GetQueryResults and specify the
    /// jobReference returned above.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<TableRow>,

    /// The schema of the results. Present only when the query completes successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,

    /// The total number of bytes processed for this query. If this query was a dry run, this is
    /// the number of bytes that would be processed if the query were run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_processed: Option<String>,

    /// The total number of rows in the complete query result set, which can be more than the
    /// number of rows in this single page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows: Option<String>,

    /// Whether the query result was fetched from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit: Option<bool>,

    /// Output only. The first errors or warnings encountered during the running of the job. The
    /// final message includes the number of errors that caused the process to stop. Errors here
    /// do not necessarily mean that the job has completed or was unsuccessful. For more
    /// information about error messages, see [Error
    /// messages](https://cloud.google.com/bigquery/docs/error-messages).
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorProto>,

    /// A token used for paging results. A non-empty token indicates that additional results are
    /// available. To see additional results, query the [`jobs.getQueryResults`](https://cloud.g
    /// oogle.com/bigquery/docs/reference/rest/v2/jobs/getQueryResults) method. For more
    /// information, see [Paging through table
    /// data](https://cloud.google.com/bigquery/docs/paging-results).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// Output only. The number of rows affected by a DML statement. Present only for DML
    /// statements INSERT, UPDATE or DELETE.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_dml_affected_rows: Option<String>,
}

impl QueryResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            job_complete: Some(false),
            job_reference: Some(JobReference::fixture()),
            kind: Some("test-kind".into()),
            rows: vec![],
            schema: Some(TableSchema::fixture()),
            total_bytes_processed: Some("test-total_bytes_processed".into()),
            total_rows: Some("test-total_rows".into()),
            cache_hit: Some(false),
            errors: vec![],
            page_token: Some("test-page_token".into()),
            num_dml_affected_rows: Some("test-num_dml_affected_rows".into()),
        }
    }
}

/// Describes format of a jobs cancellation response.
///
/// **GCP API**: `bigquery.v2.JobCancelResponse`
/// **Reference**: <https://cloud.google.com/bigquery/docs/reference/rest/v2/JobCancelResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCancelResponse {
    /// The final state of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,

    /// The resource type of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl JobCancelResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            job: Some(Job::fixture()),
            kind: Some("test-kind".into()),
        }
    }
}

// ======================================================================
// Inline struct types (from array/object fields)
// ======================================================================

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `bigquery.v2` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetListItem {
    /// The dataset reference. Use this property to access specific parts of the dataset's ID,
    /// such as project ID or dataset ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_reference: Option<DatasetReference>,

    /// Output only. Reference to a read-only external dataset defined in data catalogs outside
    /// of BigQuery. Filled out when the dataset type is EXTERNAL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_dataset_reference: Option<ExternalDatasetReference>,

    /// An alternate name for the dataset. The friendly name is purely decorative in nature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,

    /// The fully-qualified, unique, opaque ID of the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The resource type. This property always returns the value "bigquery#dataset"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The labels associated with this dataset. You can use these to organize and group your
    /// datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    /// The geographic location where the dataset resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `bigquery.v2` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableListItem {
    /// Clustering specification for this table, if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Clustering>,

    /// Output only. The time when this table was created, in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// The time when this table expires, in milliseconds since the epoch. If not present, the
    /// table will persist indefinitely. Expired tables will be deleted and their storage
    /// reclaimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,

    /// The user-friendly name for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,

    /// An opaque ID of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The labels associated with this table. You can use these to organize and group your
    /// tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    /// The range partitioning for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<RangePartitioning>,

    /// Optional. If set to true, queries including this table must specify a partition filter.
    /// This filter is used for partition elimination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_partition_filter: Option<bool>,

    /// A reference uniquely identifying table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_reference: Option<TableReference>,

    /// The time-based partitioning for this table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<TimePartitioning>,

    /// The type of table.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,

    /// Information about a logical view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<serde_json::Value>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `bigquery.v2` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangePartitioningRange {
    /// [Experimental] The end of range partitioning, exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// [Experimental] The width of each interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// [Experimental] The start of range partitioning, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `bigquery.v2` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListItem {
    /// Required. Describes the job configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<JobConfiguration>,

    /// A result object that will be present only if the job has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_result: Option<ErrorProto>,

    /// Unique opaque ID of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Unique opaque ID of the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<JobReference>,

    /// The resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// [Full-projection-only] String representation of identity of requesting party. Populated
    /// for both first- and third-party identities. Only present for APIs that support third-
    /// party identities.
    #[serde(rename = "principal_subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_subject: Option<String>,

    /// Running state of the job. When the state is DONE, errorResult can be checked to
    /// determine whether the job succeeded or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Output only. Information about the job, including starting time and ending time of the
    /// job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,

    /// [Full-projection-only] Describes the status of this job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,

    /// [Full-projection-only] Email address of the user who ran the job.
    #[serde(rename = "user_email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}
