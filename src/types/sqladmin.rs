//! Types for the Cloud SQL Admin API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/sqladmin/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `sqladmin.v1.DatabaseInstance.backendType`.
///
/// **GCP API**: `sqladmin.v1.DatabaseInstance.backendType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SqlBackendType {
    /// This is an unknown backend type for instance.
    SqlBackendTypeUnspecified,

    /// V1 speckle instance.
    FirstGen,

    /// V2 speckle instance.
    SecondGen,

    /// On premises instance.
    External,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `sqladmin.v1.DatabaseInstance.instanceType`.
///
/// **GCP API**: `sqladmin.v1.DatabaseInstance.instanceType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SqlInstanceType {
    /// This is an unknown Cloud SQL instance type.
    SqlInstanceTypeUnspecified,

    /// A regular Cloud SQL instance that is not replicating from a primary instance.
    CloudSqlInstance,

    /// An instance running on the customer's premises that is not managed by Cloud SQL.
    OnPremisesInstance,

    /// A Cloud SQL instance acting as a read-replica.
    ReadReplicaInstance,

    /// A Cloud SQL read pool.
    ReadPoolInstance,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `sqladmin.v1.DatabaseInstance.state`.
///
/// **GCP API**: `sqladmin.v1.DatabaseInstance.state`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstanceState {
    /// The state of the instance is unknown.
    SqlInstanceStateUnspecified,

    /// The instance is running, or has been stopped by owner.
    Runnable,

    /// The instance is not available, for example due to problems with billing.
    Suspended,

    /// The instance is being deleted.
    PendingDelete,

    /// The instance is being created.
    PendingCreate,

    /// The instance is down for maintenance.
    Maintenance,

    /// The creation of the instance failed or a fatal error occurred during maintenance.
    Failed,

    /// Deprecated
    OnlineMaintenance,

    /// (Applicable to read pool nodes only.) The read pool node needs to be repaired. The
    /// database might be unavailable.
    Repairing,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// A Cloud SQL instance resource.
///
/// **GCP API**: `sqladmin.v1.DatabaseInstance`
/// **Reference**: <https://cloud.google.com/sql/docs/DatabaseInstance>
///
/// ## Coverage
/// 29 of 51 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInstance {
    /// The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database
    /// server that is not managed by Google. This property is read-only; use the `tier`
    /// property in the `settings` object to determine the database type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_type: Option<SqlBackendType>,

    /// Connection name of the Cloud SQL instance used in connection strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,

    /// Output only. The time when the instance was created in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// The current disk usage of the instance in bytes. This property has been deprecated. Use
    /// the "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud Monitoring API
    /// instead. Please see [this announcement](https://groups.google.com/d/msg/google-cloud-
    /// sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_disk_size: Option<String>,

    /// The database engine type and version. The `databaseVersion` field cannot be changed
    /// after instance creation.
    ///
    /// **Possible values**:
    /// - `SQL_DATABASE_VERSION_UNSPECIFIED` — This is an unknown database version.
    /// - `MYSQL_5_1` — The database version is MySQL 5.1.
    /// - `MYSQL_5_5` — The database version is MySQL 5.5.
    /// - `MYSQL_5_6` — The database version is MySQL 5.6.
    /// - `MYSQL_5_7` — The database version is MySQL 5.7.
    /// - `MYSQL_8_0` — The database version is MySQL 8.
    /// - `MYSQL_8_0_18` — The database major version is MySQL 8.0 and the minor version is 18.
    /// - `MYSQL_8_0_26` — The database major version is MySQL 8.0 and the minor version is 26.
    /// - `MYSQL_8_0_27` — The database major version is MySQL 8.0 and the minor version is 27.
    /// - `MYSQL_8_0_28` — The database major version is MySQL 8.0 and the minor version is 28.
    /// - `MYSQL_8_0_29` — The database major version is MySQL 8.0 and the minor version is 29.
    /// - `MYSQL_8_0_30` — The database major version is MySQL 8.0 and the minor version is 30.
    /// - `MYSQL_8_0_31` — The database major version is MySQL 8.0 and the minor version is 31.
    /// - `MYSQL_8_0_32` — The database major version is MySQL 8.0 and the minor version is 32.
    /// - `MYSQL_8_0_33` — The database major version is MySQL 8.0 and the minor version is 33.
    /// - `MYSQL_8_0_34` — The database major version is MySQL 8.0 and the minor version is 34.
    /// - `MYSQL_8_0_35` — The database major version is MySQL 8.0 and the minor version is 35.
    /// - `MYSQL_8_0_36` — The database major version is MySQL 8.0 and the minor version is 36.
    /// - `MYSQL_8_0_37` — The database major version is MySQL 8.0 and the minor version is 37.
    /// - `MYSQL_8_0_39` — The database major version is MySQL 8.0 and the minor version is 39.
    /// - `MYSQL_8_0_40` — The database major version is MySQL 8.0 and the minor version is 40.
    /// - `MYSQL_8_0_41` — The database major version is MySQL 8.0 and the minor version is 41.
    /// - `MYSQL_8_0_42` — The database major version is MySQL 8.0 and the minor version is 42.
    /// - `MYSQL_8_0_43` — The database major version is MySQL 8.0 and the minor version is 43.
    /// - `MYSQL_8_0_44` — The database major version is MySQL 8.0 and the minor version is 44.
    /// - `MYSQL_8_0_45` — The database major version is MySQL 8.0 and the minor version is 45.
    /// - `MYSQL_8_0_46` — The database major version is MySQL 8.0 and the minor version is 46.
    /// - `MYSQL_8_4` — The database version is MySQL 8.4.
    /// - `MYSQL_9_7` — The database version is MySQL 9.7.
    /// - `SQLSERVER_2017_STANDARD` — The database version is SQL Server 2017 Standard.
    /// - `SQLSERVER_2017_ENTERPRISE` — The database version is SQL Server 2017 Enterprise.
    /// - `SQLSERVER_2017_EXPRESS` — The database version is SQL Server 2017 Express.
    /// - `SQLSERVER_2017_WEB` — The database version is SQL Server 2017 Web.
    /// - `POSTGRES_9_6` — The database version is PostgreSQL 9.6.
    /// - `POSTGRES_10` — The database version is PostgreSQL 10.
    /// - `POSTGRES_11` — The database version is PostgreSQL 11.
    /// - `POSTGRES_12` — The database version is PostgreSQL 12.
    /// - `POSTGRES_13` — The database version is PostgreSQL 13.
    /// - `POSTGRES_14` — The database version is PostgreSQL 14.
    /// - `POSTGRES_15` — The database version is PostgreSQL 15.
    /// - `POSTGRES_16` — The database version is PostgreSQL 16.
    /// - `POSTGRES_17` — The database version is PostgreSQL 17.
    /// - `POSTGRES_18` — The database version is PostgreSQL 18.
    /// - `SQLSERVER_2019_STANDARD` — The database version is SQL Server 2019 Standard.
    /// - `SQLSERVER_2019_ENTERPRISE` — The database version is SQL Server 2019 Enterprise.
    /// - `SQLSERVER_2019_EXPRESS` — The database version is SQL Server 2019 Express.
    /// - `SQLSERVER_2019_WEB` — The database version is SQL Server 2019 Web.
    /// - `SQLSERVER_2022_STANDARD` — The database version is SQL Server 2022 Standard.
    /// - `SQLSERVER_2022_ENTERPRISE` — The database version is SQL Server 2022 Enterprise.
    /// - `SQLSERVER_2022_EXPRESS` — The database version is SQL Server 2022 Express.
    /// - `SQLSERVER_2022_WEB` — The database version is SQL Server 2022 Web.
    /// - `SQLSERVER_2025_STANDARD` — The database version is SQL Server 2025 Standard.
    /// - `SQLSERVER_2025_ENTERPRISE` — The database version is SQL Server 2025 Enterprise.
    /// - `SQLSERVER_2025_EXPRESS` — The database version is SQL Server 2025 Express.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_version: Option<String>,

    /// Disk encryption configuration specific to an instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_configuration: Option<DiskEncryptionConfiguration>,

    /// Disk encryption status specific to an instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_status: Option<DiskEncryptionStatus>,

    /// This field is deprecated and will be removed from a future version of the API. Use the
    /// `settings.settingsVersion` field instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The name and status of the failover replica.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_replica: Option<serde_json::Value>,

    /// The Compute Engine zone that the instance is currently serving from. This value could be
    /// different from the zone that was specified when the instance was created if the instance
    /// has failed over to its secondary zone. WARNING: Changing this might restart the
    /// instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_zone: Option<String>,

    /// The instance type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<SqlInstanceType>,

    /// The assigned IP addresses for the instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<IpMapping>,

    /// The IPv6 address assigned to the instance. (Deprecated) This property was applicable
    /// only to First Generation instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,

    /// This is always `sql#instance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of the instance which will act as primary in the replication setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_name: Option<String>,

    /// The maximum disk size of the instance in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disk_size: Option<String>,

    /// Name of the Cloud SQL instance. This does not include the project ID.
    pub name: String,

    /// The project ID of the project containing the Cloud SQL instance. The Google apps domain
    /// is prefixed if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// The geographical region of the Cloud SQL instance. It can be one of the
    /// [regions](https://cloud.google.com/sql/docs/mysql/locations#location-r) where Cloud SQL
    /// operates: For example, `asia-east1`, `europe-west1`, and `us-central1`. The default
    /// value is `us-central1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Configuration specific to failover replicas and read replicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_configuration: Option<ReplicaConfiguration>,

    /// The replicas of the instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub replica_names: Vec<String>,

    /// Initial root password. Use only on creation. You must set root passwords before you can
    /// connect to PostgreSQL instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,

    /// The Compute Engine zone that the failover instance is currently serving from for a
    /// regional instance. This value could be different from the zone that was specified when
    /// the instance was created if the instance has failed over to its secondary/failover zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gce_zone: Option<String>,

    /// The URI of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// SSL configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ca_cert: Option<SslCert>,

    /// The service account email address assigned to the instance.\This property is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_email_address: Option<String>,

    /// The user settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,

    /// The current serving state of the Cloud SQL instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,

    /// If the instance state is SUSPENDED, the reason for the suspension.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suspension_reason: Vec<String>,
}

impl DatabaseInstance {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            backend_type: Some(SqlBackendType::SqlBackendTypeUnspecified),
            connection_name: Some("test-connection_name".into()),
            create_time: Some("test-create_time".into()),
            current_disk_size: Some("test-current_disk_size".into()),
            database_version: Some("test-database_version".into()),
            disk_encryption_configuration: Some(DiskEncryptionConfiguration::fixture()),
            disk_encryption_status: Some(DiskEncryptionStatus::fixture()),
            etag: Some("test-etag".into()),
            failover_replica: Default::default(),
            gce_zone: Some("test-gce_zone".into()),
            instance_type: Some(SqlInstanceType::SqlInstanceTypeUnspecified),
            ip_addresses: vec![],
            ipv6_address: Some("test-ipv6_address".into()),
            kind: Some("test-kind".into()),
            master_instance_name: Some("test-master_instance_name".into()),
            max_disk_size: Some("test-max_disk_size".into()),
            name: "test-database_instance".into(),
            project: Some("test-project".into()),
            region: Some("test-region".into()),
            replica_configuration: Some(ReplicaConfiguration::fixture()),
            replica_names: vec![],
            root_password: Some("test-root_password".into()),
            secondary_gce_zone: Some("test-secondary_gce_zone".into()),
            self_link: Some("test-self_link".into()),
            server_ca_cert: Some(SslCert::fixture()),
            service_account_email_address: Some("test-service_account_email_address".into()),
            settings: Some(Settings::fixture()),
            state: Some(InstanceState::SqlInstanceStateUnspecified),
            suspension_reason: vec![],
        }
    }
}

/// Database instance settings.
///
/// **GCP API**: `sqladmin.v1.Settings`
/// **Reference**: <https://cloud.google.com/sql/docs/Settings>
///
/// ## Coverage
/// 28 of 45 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The activation policy specifies when the instance is activated; it is applicable only
    /// when the instance state is RUNNABLE. Valid values:
    /// * `ALWAYS`: The instance is on, and remains so even in the absence of connection
    ///   requests.
    /// * `NEVER`: The instance is off; it is not activated, even if a connection request
    ///   arrives.
    ///
    /// **Possible values**:
    /// - `SQL_ACTIVATION_POLICY_UNSPECIFIED` — Unknown activation plan.
    /// - `ALWAYS` — The instance is always up and running.
    /// - `NEVER` — The instance never starts.
    /// - `ON_DEMAND` — The instance starts upon receiving requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_policy: Option<String>,

    /// The App Engine app IDs that can access this instance. (Deprecated) Applied to First
    /// Generation instances only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authorized_gae_applications: Vec<String>,

    /// Availability type. Potential values:
    /// * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data
    ///   accessibility.
    /// * `REGIONAL`: The instance can serve data from more than one zone in a region (it is
    ///   highly available)./ For more information, see [Overview of the High Availability
    ///   Configuration](https://cloud.google.com/sql/docs/mysql/high-availability).
    ///
    /// **Possible values**:
    /// - `SQL_AVAILABILITY_TYPE_UNSPECIFIED` — This is an unknown Availability type.
    /// - `ZONAL` — Zonal available instance.
    /// - `REGIONAL` — Regional available instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_type: Option<String>,

    /// The daily backup configuration for the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_configuration: Option<BackupConfiguration>,

    /// The name of server Instance collation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,

    /// Specifies if connections must use Cloud SQL connectors. Option values include the
    /// following: `NOT_REQUIRED` (Cloud SQL instances can be connected without Cloud SQL
    /// Connectors) and `REQUIRED` (Only allow connections that use Cloud SQL Connectors). Note
    /// that using REQUIRED disables all existing authorized networks. If this field is not
    /// specified when creating a new instance, NOT_REQUIRED is used. If this field is not
    /// specified when patching or updating an existing instance, it is left unchanged in the
    /// instance.
    ///
    /// **Possible values**:
    /// - `CONNECTOR_ENFORCEMENT_UNSPECIFIED` — The requirement for Cloud SQL connectors is unknown.
    /// - `NOT_REQUIRED` — Do not require Cloud SQL connectors.
    /// - `REQUIRED` — Require all connections to use Cloud SQL connectors, including the Cloud SQL Aut...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_enforcement: Option<String>,

    /// Configuration specific to read replica instances. Indicates whether database flags for
    /// crash-safe replication are enabled. This property was only applicable to First
    /// Generation instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crash_safe_replication_enabled: Option<bool>,

    /// The size of data disk, in GB. The data disk size minimum is 10GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disk_size_gb: Option<String>,

    /// The type of data disk: `PD_SSD` (default) or `PD_HDD`. Not used for First Generation
    /// instances.
    ///
    /// **Possible values**:
    /// - `SQL_DATA_DISK_TYPE_UNSPECIFIED` — This is an unknown data disk type.
    /// - `PD_SSD` — An SSD data disk.
    /// - `PD_HDD` — An HDD data disk.
    /// - `OBSOLETE_LOCAL_SSD` — This field is deprecated and will be removed from a future version of the API.
    /// - `HYPERDISK_BALANCED` — A Hyperdisk Balanced data disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disk_type: Option<String>,

    /// The database flags passed to the instance at startup.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub database_flags: Vec<DatabaseFlags>,

    /// Configuration specific to read replica instances. Indicates whether replication is
    /// enabled or not. WARNING: Changing this restarts the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_replication_enabled: Option<bool>,

    /// Configuration to protect against accidental instance deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,

    /// Deny maintenance periods
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deny_maintenance_periods: Vec<DenyMaintenancePeriod>,

    /// Optional. The edition of the instance.
    ///
    /// **Possible values**:
    /// - `EDITION_UNSPECIFIED` — The instance did not specify the edition.
    /// - `ENTERPRISE` — The instance is an enterprise edition.
    /// - `ENTERPRISE_PLUS` — The instance is an Enterprise Plus edition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,

    /// Insights configuration, for now relevant only for Postgres.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_config: Option<InsightsConfig>,

    /// The settings for IP Management. This allows to enable or disable the instance IP and
    /// manage which external networks can connect to the instance. The IPv4 address cannot be
    /// disabled for Second Generation instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_configuration: Option<IpConfiguration>,

    /// This is always `sql#settings`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The location preference settings. This allows the instance to be located as near as
    /// possible to either an App Engine app or Compute Engine zone for better performance. App
    /// Engine co-location was only applicable to First Generation instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_preference: Option<LocationPreference>,

    /// The maintenance window for this instance. This specifies when the instance can be
    /// restarted for maintenance purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,

    /// The local user password validation policy of the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_validation_policy: Option<PasswordValidationPolicy>,

    /// The pricing plan for this instance. This can be either `PER_USE` or `PACKAGE`. Only
    /// `PER_USE` is supported for Second Generation instances.
    ///
    /// **Possible values**:
    /// - `SQL_PRICING_PLAN_UNSPECIFIED` — This is an unknown pricing plan for this instance.
    /// - `PACKAGE` — The instance is billed at a monthly flat rate.
    /// - `PER_USE` — The instance is billed per usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,

    /// The type of replication this instance uses. This can be either `ASYNCHRONOUS` or
    /// `SYNCHRONOUS`. (Deprecated) This property was only applicable to First Generation
    /// instances.
    ///
    /// **Possible values**:
    /// - `SQL_REPLICATION_TYPE_UNSPECIFIED` — This is an unknown replication type for a Cloud SQL instance.
    /// - `SYNCHRONOUS` — The synchronous replication mode for First Generation instances. It is the defau...
    /// - `ASYNCHRONOUS` — The asynchronous replication mode for First Generation instances. It provides a ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_type: Option<String>,

    /// The version of instance settings. This is a required field for update method to make
    /// sure concurrent updates are handled properly. During update, use the most recent
    /// settingsVersion value for this instance and do not try to update this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_version: Option<String>,

    /// SQL Server specific audit configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_server_audit_config: Option<SqlServerAuditConfig>,

    /// Configuration to increase storage size automatically. The default value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_auto_resize: Option<bool>,

    /// The maximum size to which storage capacity can be automatically increased. The default
    /// value is 0, which specifies that there is no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_auto_resize_limit: Option<String>,

    /// The tier (or machine type) for this instance, for example `db-custom-1-3840`. WARNING:
    /// Changing this restarts the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,

    /// User-provided labels, represented as a dictionary where each label is a single key value
    /// pair.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub user_labels: HashMap<String, String>,
}

impl Settings {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            activation_policy: Some("test-activation_policy".into()),
            authorized_gae_applications: vec![],
            availability_type: Some("test-availability_type".into()),
            backup_configuration: Some(BackupConfiguration::fixture()),
            collation: Some("test-collation".into()),
            connector_enforcement: Some("test-connector_enforcement".into()),
            crash_safe_replication_enabled: Some(false),
            data_disk_size_gb: Some("test-data_disk_size_gb".into()),
            data_disk_type: Some("test-data_disk_type".into()),
            database_flags: vec![],
            database_replication_enabled: Some(false),
            deletion_protection_enabled: Some(false),
            deny_maintenance_periods: vec![],
            edition: Some("test-edition".into()),
            insights_config: Some(InsightsConfig::fixture()),
            ip_configuration: Some(IpConfiguration::fixture()),
            kind: Some("test-kind".into()),
            location_preference: Some(LocationPreference::fixture()),
            maintenance_window: Some(MaintenanceWindow::fixture()),
            password_validation_policy: Some(PasswordValidationPolicy::fixture()),
            pricing_plan: Some("test-pricing_plan".into()),
            replication_type: Some("test-replication_type".into()),
            settings_version: Some("test-settings_version".into()),
            sql_server_audit_config: Some(SqlServerAuditConfig::fixture()),
            storage_auto_resize: Some(false),
            storage_auto_resize_limit: Some("test-storage_auto_resize_limit".into()),
            tier: Some("test-tier".into()),
            user_labels: Default::default(),
        }
    }
}

/// IP Management configuration.
///
/// **GCP API**: `sqladmin.v1.IpConfiguration`
/// **Reference**: <https://cloud.google.com/sql/docs/IpConfiguration>
///
/// ## Coverage
/// 7 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpConfiguration {
    /// The name of the allocated ip range for the private ip Cloud SQL instance. For example:
    /// "google-managed-services-default". If set, the instance ip will be created in the
    /// allocated range. The range name must comply with [RFC
    /// 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_ip_range: Option<String>,

    /// The list of external networks that are allowed to connect to the instance using the IP.
    /// In 'CIDR' notation, also known as 'slash' notation (for example: `157.197.200.0/24`).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authorized_networks: Vec<AclEntry>,

    /// Controls connectivity to private IP instances from Google services, such as BigQuery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_private_path_for_google_cloud_services: Option<bool>,

    /// Whether the instance is assigned a public IP address or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_enabled: Option<bool>,

    /// The resource link for the VPC network from which the Cloud SQL instance is accessible
    /// for private IP. For example, `/projects/myProject/global/networks/default`. This setting
    /// can be updated, but it cannot be removed after it is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_network: Option<String>,

    /// Use `ssl_mode` instead. Whether SSL/TLS connections over IP are enforced. If set to
    /// false, then allow both non-SSL/non-TLS and SSL/TLS connections. For SSL/TLS connections,
    /// the client certificate won't be verified. If set to true, then only allow connections
    /// encrypted with SSL/TLS and with valid client certificates. If you want to enforce
    /// SSL/TLS without enforcing the requirement for valid client certificates, then use the
    /// `ssl_mode` flag instead of the `require_ssl` flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_ssl: Option<bool>,

    /// Specify how SSL/TLS is enforced in database connections. If you must use the
    /// `require_ssl` flag for backward compatibility, then only the following value pairs are
    /// valid: For PostgreSQL and MySQL:
    /// * `ssl_mode=ALLOW_UNENCRYPTED_AND_ENCRYPTED` and `require_ssl=false`
    /// * `ssl_mode=ENCRYPTED_ONLY` and `require_ssl=false`
    /// * `ssl_mode=TRUSTED_CLIENT_CERTIFICATE_REQUIRED` and `require_ssl=true` For SQL Server:
    /// * `ssl_mode=ALLOW_UNENCRYPTED_AND_ENCRYPTED` and `require_ssl=false`
    /// * `ssl_mode=ENCRYPTED_ONLY` and `require_ssl=true` The value of `ssl_mode` has priority
    ///   over the value of `require_ssl`. For example, for the pair `ssl_mode=ENCRYPTED_ONLY`
    ///   and `require_ssl=false`, `ssl_mode=ENCRYPTED_ONLY` means accept only SSL connections,
    ///   while `require_ssl=false` means accept both non-SSL and SSL connections. In this case,
    ///   MySQL and PostgreSQL databases respect `ssl_mode` and accepts only SSL connections.
    ///
    /// **Possible values**:
    /// - `SSL_MODE_UNSPECIFIED` — The SSL mode is unknown.
    /// - `ALLOW_UNENCRYPTED_AND_ENCRYPTED` — Allow non-SSL/non-TLS and SSL/TLS connections. For SSL connections to MySQL and ...
    /// - `ENCRYPTED_ONLY` — Only allow connections encrypted with SSL/TLS. For SSL connections to MySQL and ...
    /// - `TRUSTED_CLIENT_CERTIFICATE_REQUIRED` — Only allow connections encrypted with SSL/TLS and with valid client certificates...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
}

impl IpConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allocated_ip_range: Some("test-allocated_ip_range".into()),
            authorized_networks: vec![],
            enable_private_path_for_google_cloud_services: Some(false),
            ipv4_enabled: Some(false),
            private_network: Some("test-private_network".into()),
            require_ssl: Some(false),
            ssl_mode: Some("test-ssl_mode".into()),
        }
    }
}

/// An entry for an Access Control list.
///
/// **GCP API**: `sqladmin.v1.AclEntry`
/// **Reference**: <https://cloud.google.com/sql/docs/AclEntry>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclEntry {
    /// The time when this access control entry expires in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,

    /// This is always `sql#aclEntry`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Optional. A label to identify this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The allowlisted value for the access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AclEntry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expiration_time: Some("test-expiration_time".into()),
            kind: Some("test-kind".into()),
            name: Some("test-acl_entry".into()),
            value: Some("test-value".into()),
        }
    }
}

/// Preferred location. This specifies where a Cloud SQL instance is located. Note that if the
/// preferred location is not available, the instance will be located as close as possible
/// within the region. Only one location may be specified.
///
/// **GCP API**: `sqladmin.v1.LocationPreference`
/// **Reference**: <https://cloud.google.com/sql/docs/LocationPreference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationPreference {
    /// The App Engine application to follow, it must be in the same region as the Cloud SQL
    /// instance. WARNING: Changing this might restart the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_gae_application: Option<String>,

    /// This is always `sql#locationPreference`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The preferred Compute Engine zone for the secondary/failover (for example: us-
    /// central1-a, us-central1-b, etc.). To disable this field, set it to 'no_secondary_zone'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone: Option<String>,

    /// The preferred Compute Engine zone (for example: us-central1-a, us-central1-b, etc.).
    /// WARNING: Changing this might restart the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl LocationPreference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            follow_gae_application: Some("test-follow_gae_application".into()),
            kind: Some("test-kind".into()),
            secondary_zone: Some("test-secondary_zone".into()),
            zone: Some("test-zone".into()),
        }
    }
}

/// Maintenance window. This specifies when a Cloud SQL instance is restarted for system
/// maintenance purposes.
///
/// **GCP API**: `sqladmin.v1.MaintenanceWindow`
/// **Reference**: <https://cloud.google.com/sql/docs/MaintenanceWindow>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceWindow {
    /// Day of week
    /// - `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, or `SUNDAY`.
    ///   Specify in the UTC time zone. Returned in output as an integer, 1 to 7, where `1`
    ///   equals Monday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,

    /// Hour of day - 0 to 23. Specify in the UTC time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,

    /// This is always `sql#maintenanceWindow`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Maintenance timing settings: `canary`, `stable`, or `week5`. For more information, see
    /// [About maintenance on Cloud SQL
    /// instances](https://cloud.google.com/sql/docs/mysql/maintenance).
    ///
    /// **Possible values**:
    /// - `SQL_UPDATE_TRACK_UNSPECIFIED` — This is an unknown maintenance timing preference.
    /// - `canary` — For an instance with a scheduled maintenance window, this maintenance timing ind...
    /// - `stable` — For an instance with a scheduled maintenance window, this maintenance timing ind...
    /// - `week5` — For instance with a scheduled maintenance window, this maintenance timing indica...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_track: Option<String>,
}

impl MaintenanceWindow {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            day: Some(100),
            hour: Some(100),
            kind: Some("test-kind".into()),
            update_track: Some("test-update_track".into()),
        }
    }
}

/// Database instance backup configuration.
///
/// **GCP API**: `sqladmin.v1.BackupConfiguration`
/// **Reference**: <https://cloud.google.com/sql/docs/BackupConfiguration>
///
/// ## Coverage
/// 9 of 11 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupConfiguration {
    /// Backup retention settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_settings: Option<BackupRetentionSettings>,

    /// (MySQL only) Whether binary log is enabled. If backup configuration is disabled,
    /// binarylog must be disabled as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_log_enabled: Option<bool>,

    /// Whether this configuration is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// This is always `sql#backupConfiguration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Location of the backup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Whether point in time recovery is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_enabled: Option<bool>,

    /// Reserved for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_log_archiving_enabled: Option<bool>,

    /// Start time for the daily backup configuration in UTC timezone in the 24 hour format
    /// - `HH:MM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The number of days of transaction logs we retain for point in time restore, from 1-7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_log_retention_days: Option<i32>,
}

impl BackupConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            backup_retention_settings: Some(BackupRetentionSettings::fixture()),
            binary_log_enabled: Some(false),
            enabled: Some(false),
            kind: Some("test-kind".into()),
            location: Some("test-location".into()),
            point_in_time_recovery_enabled: Some(false),
            replication_log_archiving_enabled: Some(false),
            start_time: Some("test-start_time".into()),
            transaction_log_retention_days: Some(100),
        }
    }
}

/// We currently only support backup retention by specifying the number of backups we will
/// retain.
///
/// **GCP API**: `sqladmin.v1.BackupRetentionSettings`
/// **Reference**: <https://cloud.google.com/sql/docs/BackupRetentionSettings>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRetentionSettings {
    /// Depending on the value of retention_unit, this is used to determine if a backup needs to
    /// be deleted. If retention_unit is 'COUNT', we will retain this many backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retained_backups: Option<i32>,

    /// The unit that 'retained_backups' represents.
    ///
    /// **Possible values**:
    /// - `RETENTION_UNIT_UNSPECIFIED` — Backup retention unit is unspecified, will be treated as COUNT.
    /// - `COUNT` — Retention will be by count, eg. "retain the most recent 7 backups".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_unit: Option<String>,
}

impl BackupRetentionSettings {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            retained_backups: Some(100),
            retention_unit: Some("test-retention_unit".into()),
        }
    }
}

/// Disk encryption configuration for an instance.
///
/// **GCP API**: `sqladmin.v1.DiskEncryptionConfiguration`
/// **Reference**: <https://cloud.google.com/sql/docs/DiskEncryptionConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskEncryptionConfiguration {
    /// This is always `sql#diskEncryptionConfiguration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Resource name of KMS key for disk encryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,
}

impl DiskEncryptionConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            kms_key_name: Some("test-kms_key_name".into()),
        }
    }
}

/// Disk encryption status for an instance.
///
/// **GCP API**: `sqladmin.v1.DiskEncryptionStatus`
/// **Reference**: <https://cloud.google.com/sql/docs/DiskEncryptionStatus>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskEncryptionStatus {
    /// This is always `sql#diskEncryptionStatus`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// KMS key version used to encrypt the Cloud SQL instance resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_version_name: Option<String>,
}

impl DiskEncryptionStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            kms_key_version_name: Some("test-kms_key_version_name".into()),
        }
    }
}

/// Database instance IP mapping
///
/// **GCP API**: `sqladmin.v1.IpMapping`
/// **Reference**: <https://cloud.google.com/sql/docs/IpMapping>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpMapping {
    /// The IP address assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The due time for this IP to be retired in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`. This field is only available when the IP is scheduled to be
    /// retired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_retire: Option<String>,

    /// The type of this IP address. A `PRIMARY` address is a public address that can accept
    /// incoming connections. A `PRIVATE` address is a private address that can accept incoming
    /// connections. An `OUTGOING` address is the source address of connections originating from
    /// the instance, if supported.
    ///
    /// **Possible values**:
    /// - `SQL_IP_ADDRESS_TYPE_UNSPECIFIED` — This is an unknown IP address type.
    /// - `PRIMARY` — IP address the customer is supposed to connect to. Usually this is the load bala...
    /// - `OUTGOING` — Source IP address of the connection a read replica establishes to its external p...
    /// - `PRIVATE` — Private IP used when using private IPs and network peering.
    /// - `MIGRATED_1ST_GEN` — V1 IP of a migrated instance. We want the user to decommission this IP as soon a...
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
}

impl IpMapping {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ip_address: Some("test-ip_address".into()),
            time_to_retire: Some("test-time_to_retire".into()),
            type_value: Some("test-type".into()),
        }
    }
}

/// Read-replica configuration for connecting to the primary instance.
///
/// **GCP API**: `sqladmin.v1.ReplicaConfiguration`
/// **Reference**: <https://cloud.google.com/sql/docs/ReplicaConfiguration>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicaConfiguration {
    /// Specifies if the replica is the failover target. If the field is set to `true`, the
    /// replica will be designated as a failover replica. In case the primary instance fails,
    /// the replica instance will be promoted as the new primary instance. Only one replica can
    /// be specified as failover target, and the replica has to be in different zone with the
    /// primary instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_target: Option<bool>,

    /// This is always `sql#replicaConfiguration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// MySQL specific configuration when replicating from a MySQL on-premises primary instance.
    /// Replication configuration information such as the username, password, certificates, and
    /// keys are not stored in the instance metadata. The configuration information is used only
    /// to set up the replication connection and is stored by MySQL in a file named
    /// `master.info` in the data directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mysql_replica_configuration: Option<MySqlReplicaConfiguration>,
}

impl ReplicaConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            failover_target: Some(false),
            kind: Some("test-kind".into()),
            mysql_replica_configuration: Some(MySqlReplicaConfiguration::fixture()),
        }
    }
}

/// Read-replica configuration specific to MySQL databases.
///
/// **GCP API**: `sqladmin.v1.MySqlReplicaConfiguration`
/// **Reference**: <https://cloud.google.com/sql/docs/MySqlReplicaConfiguration>
///
/// ## Coverage
/// 10 of 11 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MySqlReplicaConfiguration {
    /// PEM representation of the trusted CA's x509 certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate: Option<String>,

    /// PEM representation of the replica's x509 certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,

    /// PEM representation of the replica's private key. The corresponding public key is encoded
    /// in the client's certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,

    /// Path to a SQL dump file in Google Cloud Storage from which the replica instance is to be
    /// created. The URI is in the form gs://bucketName/fileName. Compressed gzip files (.gz)
    /// are also supported. Dumps have the binlog co-ordinates from which replication begins.
    /// This can be accomplished by setting --master-data to 1 when using mysqldump.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dump_file_path: Option<String>,

    /// This is always `sql#mysqlReplicaConfiguration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Interval in milliseconds between replication heartbeats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_heartbeat_period: Option<String>,

    /// The password for the replication connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// A list of permissible ciphers to use for SSL encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_cipher: Option<String>,

    /// The username for the replication connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Whether or not to check the primary instance's Common Name value in the certificate that
    /// it sends during the SSL handshake.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_server_certificate: Option<bool>,
}

impl MySqlReplicaConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ca_certificate: Some("test-ca_certificate".into()),
            client_certificate: Some("test-client_certificate".into()),
            client_key: Some("test-client_key".into()),
            dump_file_path: Some("test-dump_file_path".into()),
            kind: Some("test-kind".into()),
            master_heartbeat_period: Some("test-master_heartbeat_period".into()),
            password: Some("test-password".into()),
            ssl_cipher: Some("test-ssl_cipher".into()),
            username: Some("test-username".into()),
            verify_server_certificate: Some(false),
        }
    }
}

/// Database flags for Cloud SQL instances.
///
/// **GCP API**: `sqladmin.v1.DatabaseFlags`
/// **Reference**: <https://cloud.google.com/sql/docs/DatabaseFlags>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseFlags {
    /// The name of the flag. These flags are passed at instance startup, so include both server
    /// options and system variables. Flags are specified with underscores, not hyphens. For
    /// more information, see [Configuring Database
    /// Flags](https://cloud.google.com/sql/docs/mysql/flags) in the Cloud SQL documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The value of the flag. Boolean flags are set to `on` for true and `off` for false. This
    /// field must be omitted if the flag doesn't take a value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DatabaseFlags {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-database_flags".into()),
            value: Some("test-value".into()),
        }
    }
}

/// Database instances list response.
///
/// **GCP API**: `sqladmin.v1.InstancesListResponse`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesListResponse>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesListResponse {
    /// List of database instance resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DatabaseInstance>,

    /// This is always `sql#instancesList`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The continuation token, used to page through large result sets. Provide this value in a
    /// subsequent request to return the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl InstancesListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Represents a SQL database on the Cloud SQL instance.
///
/// **GCP API**: `sqladmin.v1.Database`
/// **Reference**: <https://cloud.google.com/sql/docs/Database>
///
/// ## Coverage
/// 8 of 9 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    /// The Cloud SQL charset value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,

    /// The Cloud SQL collation value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,

    /// This field is deprecated and will be removed from a future version of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The name of the Cloud SQL instance. This does not include the project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// This is always `sql#database`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of the database in the Cloud SQL instance. This does not include the project ID
    /// or instance name.
    pub name: String,

    /// The project ID of the project containing the Cloud SQL database. The Google apps domain
    /// is prefixed if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// The URI of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl Database {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            charset: Some("test-charset".into()),
            collation: Some("test-collation".into()),
            etag: Some("test-etag".into()),
            instance: Some("test-instance".into()),
            kind: Some("test-kind".into()),
            name: "test-database".into(),
            project: Some("test-project".into()),
            self_link: Some("test-self_link".into()),
        }
    }
}

/// Database list response.
///
/// **GCP API**: `sqladmin.v1.DatabasesListResponse`
/// **Reference**: <https://cloud.google.com/sql/docs/DatabasesListResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabasesListResponse {
    /// List of database resources in the instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Database>,

    /// This is always `sql#databasesList`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DatabasesListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
        }
    }
}

/// A Cloud SQL user resource.
///
/// **GCP API**: `sqladmin.v1.User`
/// **Reference**: <https://cloud.google.com/sql/docs/User>
///
/// ## Coverage
/// 9 of 14 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// This field is deprecated and will be removed from a future version of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Optional. The host from which the user can connect. For `insert` operations, host
    /// defaults to an empty string. For `update` operations, host is specified as part of the
    /// request URL. The host name cannot be updated after insertion. For a MySQL instance, it's
    /// required; for a PostgreSQL or SQL Server instance, it's optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// The name of the Cloud SQL instance. This does not include the project ID. Can be omitted
    /// for `update` because it is already specified on the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// This is always `sql#user`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of the user in the Cloud SQL instance. Can be omitted for `update` because it
    /// is already specified in the URL.
    pub name: String,

    /// The password for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// The project ID of the project containing the Cloud SQL database. The Google apps domain
    /// is prefixed if applicable. Can be omitted for `update` because it is already specified
    /// on the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// The user type. It determines the method to authenticate the user during login. The
    /// default is the database's built-in user type.
    ///
    /// **Possible values**:
    /// - `BUILT_IN` — The database's built-in user type.
    /// - `CLOUD_IAM_USER` — Cloud IAM user.
    /// - `CLOUD_IAM_SERVICE_ACCOUNT` — Cloud IAM service account.
    /// - `CLOUD_IAM_GROUP` — Cloud IAM group. Not used for login.
    /// - `CLOUD_IAM_GROUP_USER` — Read-only. Login for a user that belongs to the Cloud IAM group.
    /// - `CLOUD_IAM_GROUP_SERVICE_ACCOUNT` — Read-only. Login for a service account that belongs to the Cloud IAM group.
    /// - `ENTRAID_USER` — Microsoft Entra ID user.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,

    /// Dual password status for the user.
    ///
    /// **Possible values**:
    /// - `DUAL_PASSWORD_TYPE_UNSPECIFIED` — The default value.
    /// - `NO_MODIFY_DUAL_PASSWORD` — Do not update the user's dual password status.
    /// - `NO_DUAL_PASSWORD` — No dual password usable for connecting using this user.
    /// - `DUAL_PASSWORD` — Dual password usable for connecting using this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_password_type: Option<String>,
}

impl User {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            etag: Some("test-etag".into()),
            host: Some("test-host".into()),
            instance: Some("test-instance".into()),
            kind: Some("test-kind".into()),
            name: "test-user".into(),
            password: Some("test-password".into()),
            project: Some("test-project".into()),
            type_value: Some("test-type".into()),
            dual_password_type: Some("test-dual_password_type".into()),
        }
    }
}

/// User list response.
///
/// **GCP API**: `sqladmin.v1.UsersListResponse`
/// **Reference**: <https://cloud.google.com/sql/docs/UsersListResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListResponse {
    /// List of user resources in the instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<User>,

    /// This is always `sql#usersList`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Unused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl UsersListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// An Operation resource. For successful operations that return an Operation resource, only the
/// fields relevant to the operation are populated in the resource.
///
/// **GCP API**: `sqladmin.v1.Operation`
/// **Reference**: <https://cloud.google.com/sql/docs/Operation>
///
/// ## Coverage
/// 9 of 20 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResponse {
    /// An identifier that uniquely identifies the operation. You can use this identifier to
    /// retrieve the Operations resource that has information about the operation.
    pub name: String,

    /// The type of the operation. Valid values are:
    /// * `CREATE`
    /// * `DELETE`
    /// * `UPDATE`
    /// * `RESTART`
    /// * `IMPORT`
    /// * `EXPORT`
    /// * `BACKUP_VOLUME`
    /// * `RESTORE_VOLUME`
    /// * `CREATE_USER`
    /// * `DELETE_USER`
    /// * `CREATE_DATABASE`
    /// * `DELETE_DATABASE`
    ///
    /// **Possible values**:
    /// - `SQL_OPERATION_TYPE_UNSPECIFIED` — Unknown operation type.
    /// - `IMPORT` — Imports data into a Cloud SQL instance.
    /// - `EXPORT` — Exports data from a Cloud SQL instance to a Cloud Storage bucket.
    /// - `CREATE` — Creates a new Cloud SQL instance.
    /// - `UPDATE` — Updates the settings of a Cloud SQL instance.
    /// - `DELETE` — Deletes a Cloud SQL instance.
    /// - `RESTART` — Restarts the Cloud SQL instance.
    /// - `BACKUP`
    /// - `SNAPSHOT`
    /// - `BACKUP_VOLUME` — Performs instance backup.
    /// - `DELETE_VOLUME` — Deletes an instance backup.
    /// - `RESTORE_VOLUME` — Restores an instance backup.
    /// - `INJECT_USER` — Injects a privileged user in mysql for MOB instances.
    /// - `CLONE` — Clones a Cloud SQL instance.
    /// - `STOP_REPLICA` — Stops replication on a Cloud SQL read replica instance.
    /// - `START_REPLICA` — Starts replication on a Cloud SQL read replica instance.
    /// - `PROMOTE_REPLICA` — Promotes a Cloud SQL replica instance.
    /// - `CREATE_REPLICA` — Creates a Cloud SQL replica instance.
    /// - `CREATE_USER` — Creates a new user in a Cloud SQL instance.
    /// - `DELETE_USER` — Deletes a user from a Cloud SQL instance.
    /// - `UPDATE_USER` — Updates an existing user in a Cloud SQL instance. If a user with the specified u...
    /// - `CREATE_DATABASE` — Creates a database in the Cloud SQL instance.
    /// - `DELETE_DATABASE` — Deletes a database in the Cloud SQL instance.
    /// - `UPDATE_DATABASE` — Updates a database in the Cloud SQL instance.
    /// - `FAILOVER` — Performs failover of an HA-enabled Cloud SQL failover replica.
    /// - `DELETE_BACKUP` — Deletes the backup taken by a backup run.
    /// - `RECREATE_REPLICA`
    /// - `TRUNCATE_LOG` — Truncates a general or slow log table in MySQL.
    /// - `DEMOTE_MASTER` — Demotes the stand-alone instance to be a Cloud SQL read replica for an external ...
    /// - `MAINTENANCE` — Indicates that the instance is currently in maintenance. Maintenance typically c...
    /// - `ENABLE_PRIVATE_IP` — This field is deprecated, and will be removed in future version of API.
    /// - `DEFER_MAINTENANCE`
    /// - `CREATE_CLONE` — Creates clone instance.
    /// - `RESCHEDULE_MAINTENANCE` — Reschedule maintenance to another time.
    /// - `START_EXTERNAL_SYNC` — Starts external sync of a Cloud SQL EM replica to an external primary instance.
    /// - `LOG_CLEANUP` — Recovers logs from an instance's old data disk.
    /// - `AUTO_RESTART` — Performs auto-restart of an HA-enabled Cloud SQL database for auto recovery.
    /// - `REENCRYPT` — Re-encrypts CMEK instances with latest key version.
    /// - `SWITCHOVER` — Switches the roles of the primary and replica pair. The target instance should b...
    /// - `UPDATE_BACKUP` — Update a backup.
    /// - `ACQUIRE_SSRS_LEASE` — Acquire a lease for the setup of SQL Server Reporting Services (SSRS).
    /// - `RELEASE_SSRS_LEASE` — Release a lease for the setup of SQL Server Reporting Services (SSRS).
    /// - `RECONFIGURE_OLD_PRIMARY` — Reconfigures old primary after a promote replica operation. Effect of a promote ...
    /// - `CLUSTER_MAINTENANCE` — Indicates that the instance, its read replicas, and its cascading replicas are i...
    /// - `SELF_SERVICE_MAINTENANCE` — Indicates that the instance (and any of its replicas) are currently in maintenan...
    /// - `SWITCHOVER_TO_REPLICA` — Switches a primary instance to a replica. This operation runs as part of a switc...
    /// - `MAJOR_VERSION_UPGRADE` — Updates the major version of a Cloud SQL instance.
    /// - `ADVANCED_BACKUP` — Deprecated: ADVANCED_BACKUP is deprecated. Use ENHANCED_BACKUP instead.
    /// - `MANAGE_BACKUP` — Changes the BackupTier of a Cloud SQL instance.
    /// - `ENHANCED_BACKUP` — Creates a backup for an Enhanced BackupTier Cloud SQL instance.
    /// - `REPAIR_READ_POOL` — Repairs entire read pool or specified read pool nodes in the read pool.
    /// - `CREATE_READ_POOL` — Creates a Cloud SQL read pool instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,

    /// The status of an operation.
    ///
    /// **Possible values**:
    /// - `SQL_OPERATION_STATUS_UNSPECIFIED` — The state of the operation is unknown.
    /// - `PENDING` — The operation has been queued, but has not started yet.
    /// - `RUNNING` — The operation is running.
    /// - `DONE` — The operation completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The `targetLink` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_link: Option<String>,

    /// The URI of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// If errors occurred during processing of this operation, this field will be populated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationErrors>,

    /// The time this operation actually started in UTC timezone in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The time this operation finished in UTC timezone in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// The email address of the user who initiated this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl OperationResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a completed operation fixture for testing.
    pub fn fixture() -> Self {
        Self::fixture_done()
    }

    #[cfg(any(test, feature = "test-support"))]
    /// Create a pending operation fixture for testing.
    pub fn fixture_pending() -> Self {
        Self {
            name: "operation-pending".into(),
            status: Some("PENDING".into()),
            self_link: Some(
                "https://compute.googleapis.com/compute/v1/projects/test-project/operations/op-123"
                    .into(),
            ),
            ..Default::default()
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    /// Create a completed operation fixture for testing.
    pub fn fixture_done() -> Self {
        Self {
            name: "operation-done".into(),
            status: Some("DONE".into()),
            self_link: Some(
                "https://compute.googleapis.com/compute/v1/projects/test-project/operations/op-123"
                    .into(),
            ),
            ..Default::default()
        }
    }
}

/// Database instance operation errors list wrapper.
///
/// **GCP API**: `sqladmin.v1.OperationErrors`
/// **Reference**: <https://cloud.google.com/sql/docs/OperationErrors>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationErrors {
    /// The list of errors encountered while processing this operation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<OperationError>,

    /// This is always `sql#operationErrors`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl OperationErrors {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            errors: vec![],
            kind: Some("test-kind".into()),
        }
    }
}

/// Database instance operation error.
///
/// **GCP API**: `sqladmin.v1.OperationError`
/// **Reference**: <https://cloud.google.com/sql/docs/OperationError>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationError {
    /// Identifies the specific error that occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// This is always `sql#operationError`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Additional information about the error encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl OperationError {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            kind: Some("test-kind".into()),
            message: Some("test-message".into()),
        }
    }
}

/// Operations list response.
///
/// **GCP API**: `sqladmin.v1.OperationsListResponse`
/// **Reference**: <https://cloud.google.com/sql/docs/OperationsListResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationsListResponse {
    /// List of operation resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OperationResponse>,

    /// This is always `sql#operationsList`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The continuation token, used to page through large result sets. Provide this value in a
    /// subsequent request to return the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl OperationsListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            kind: Some("test-kind".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Rotate server CA request.
///
/// **GCP API**: `sqladmin.v1.InstancesRotateServerCaRequest`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesRotateServerCaRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesRotateServerCaRequest {
    /// Contains details about the rotate server CA operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_server_ca_context: Option<RotateServerCaContext>,
}

impl InstancesRotateServerCaRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            rotate_server_ca_context: Some(RotateServerCaContext::fixture()),
        }
    }
}

/// Instance rotate server CA context.
///
/// **GCP API**: `sqladmin.v1.RotateServerCaContext`
/// **Reference**: <https://cloud.google.com/sql/docs/RotateServerCaContext>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotateServerCaContext {
    /// This is always `sql#rotateServerCaContext`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The fingerprint of the next version to be rotated to. If left unspecified, will be
    /// rotated to the most recently added server CA version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
}

impl RotateServerCaContext {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            next_version: Some("test-next_version".into()),
        }
    }
}

/// Database instance clone request.
///
/// **GCP API**: `sqladmin.v1.InstancesCloneRequest`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesCloneRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesCloneRequest {
    /// Required. Contains details about the clone operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_context: Option<CloneContext>,
}

impl InstancesCloneRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clone_context: Some(CloneContext::fixture()),
        }
    }
}

/// Database instance clone context.
///
/// **GCP API**: `sqladmin.v1.CloneContext`
/// **Reference**: <https://cloud.google.com/sql/docs/CloneContext>
///
/// ## Coverage
/// 6 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloneContext {
    /// Binary log coordinates, if specified, identify the position up to which the source
    /// instance is cloned. If not specified, the source instance is cloned up to the most
    /// recent binary log coordinates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_log_coordinates: Option<BinLogCoordinates>,

    /// (SQL Server only) Clone only the specified databases from the source instance. Clone all
    /// databases if empty.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub database_names: Vec<String>,

    /// Required. Name of the Cloud SQL instance to be created as a clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_instance_name: Option<String>,

    /// This is always `sql#cloneContext`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Reserved for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitr_timestamp_ms: Option<String>,

    /// Timestamp, if specified, identifies the time to which the source instance is cloned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time: Option<String>,
}

impl CloneContext {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bin_log_coordinates: Some(BinLogCoordinates::fixture()),
            database_names: vec![],
            destination_instance_name: Some("test-destination_instance_name".into()),
            kind: Some("test-kind".into()),
            pitr_timestamp_ms: Some("test-pitr_timestamp_ms".into()),
            point_in_time: Some("test-point_in_time".into()),
        }
    }
}

/// Binary log coordinates.
///
/// **GCP API**: `sqladmin.v1.BinLogCoordinates`
/// **Reference**: <https://cloud.google.com/sql/docs/BinLogCoordinates>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinLogCoordinates {
    /// Name of the binary log file for a Cloud SQL instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_log_file_name: Option<String>,

    /// Position (offset) within the binary log file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_log_position: Option<String>,

    /// This is always `sql#binLogCoordinates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl BinLogCoordinates {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bin_log_file_name: Some("test-bin_log_file_name".into()),
            bin_log_position: Some("test-bin_log_position".into()),
            kind: Some("test-kind".into()),
        }
    }
}

/// Instance failover request.
///
/// **GCP API**: `sqladmin.v1.InstancesFailoverRequest`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesFailoverRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesFailoverRequest {
    /// Failover Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_context: Option<FailoverContext>,
}

impl InstancesFailoverRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            failover_context: Some(FailoverContext::fixture()),
        }
    }
}

/// Database instance failover context.
///
/// **GCP API**: `sqladmin.v1.FailoverContext`
/// **Reference**: <https://cloud.google.com/sql/docs/FailoverContext>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailoverContext {
    /// This is always `sql#failoverContext`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The current settings version of this instance. Request will be rejected if this version
    /// doesn't match the current settings version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_version: Option<String>,
}

impl FailoverContext {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            settings_version: Some("test-settings_version".into()),
        }
    }
}

/// Database instance export request.
///
/// **GCP API**: `sqladmin.v1.InstancesExportRequest`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesExportRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesExportRequest {
    /// Contains details about the export operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_context: Option<ExportContext>,
}

impl InstancesExportRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            export_context: Some(ExportContext::fixture()),
        }
    }
}

/// Database instance export context.
///
/// **GCP API**: `sqladmin.v1.ExportContext`
/// **Reference**: <https://cloud.google.com/sql/docs/ExportContext>
///
/// ## Coverage
/// 7 of 9 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportContext {
    /// Options for exporting data as CSV. `MySQL` and `PostgreSQL` instances only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_export_options: Option<serde_json::Value>,

    /// Databases to be exported. `MySQL instances:` If `fileType` is `SQL` and no database is
    /// specified, all databases are exported, except for the `mysql` system database. If
    /// `fileType` is `CSV`, you can specify one database, either by using this property or by
    /// using the `csvExportOptions.selectQuery` property, which takes precedence over this
    /// property. `PostgreSQL instances:` If you don't specify a database by name, all user
    /// databases in the instance are exported. This excludes system databases and Cloud SQL
    /// databases used to manage internal operations. Exporting all user databases is only
    /// available for directory-formatted parallel export. If `fileType` is `CSV`, this database
    /// must match the one specified in the `csvExportOptions.selectQuery` property. `SQL Server
    /// instances:` You must specify one database to be exported, and the `fileType` must be
    /// `BAK`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub databases: Vec<String>,

    /// The file type for the specified uri.
    ///
    /// **Possible values**:
    /// - `SQL_FILE_TYPE_UNSPECIFIED` — Unknown file type.
    /// - `SQL` — File containing SQL statements.
    /// - `CSV` — File in CSV format.
    /// - `BAK`
    /// - `TDE` — TDE certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// This is always `sql#exportContext`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Whether to perform a serverless export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offload: Option<bool>,

    /// Options for exporting data as SQL statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_export_options: Option<serde_json::Value>,

    /// The path to the file in Google Cloud Storage where the export will be stored. The URI is
    /// in the form `gs://bucketName/fileName`. If the file already exists, the request
    /// succeeds, but the operation fails. If `fileType` is `SQL` and the filename ends with
    /// .gz, the contents are compressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ExportContext {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            csv_export_options: Default::default(),
            databases: vec![],
            file_type: Some("test-file_type".into()),
            kind: Some("test-kind".into()),
            offload: Some(false),
            sql_export_options: Default::default(),
            uri: Some("test-uri".into()),
        }
    }
}

/// Database instance import request.
///
/// **GCP API**: `sqladmin.v1.InstancesImportRequest`
/// **Reference**: <https://cloud.google.com/sql/docs/InstancesImportRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesImportRequest {
    /// Contains details about the import operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_context: Option<ImportContext>,
}

impl InstancesImportRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            import_context: Some(ImportContext::fixture()),
        }
    }
}

/// Database instance import context.
///
/// **GCP API**: `sqladmin.v1.ImportContext`
/// **Reference**: <https://cloud.google.com/sql/docs/ImportContext>
///
/// ## Coverage
/// 7 of 9 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportContext {
    /// Import parameters specific to SQL Server .BAK files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bak_import_options: Option<serde_json::Value>,

    /// Options for importing data as CSV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_import_options: Option<serde_json::Value>,

    /// The target database for the import. If `fileType` is `SQL`, this field is required only
    /// if the import file does not specify a database, and is overridden by any database
    /// specification in the import file. For entire instance parallel import operations, the
    /// database is overridden by the database name stored in subdirectory name. If `fileType`
    /// is `CSV`, one database must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    /// The file type for the specified uri.\`SQL`: The file contains SQL statements. \`CSV`:
    /// The file contains CSV data.
    ///
    /// **Possible values**:
    /// - `SQL_FILE_TYPE_UNSPECIFIED` — Unknown file type.
    /// - `SQL` — File containing SQL statements.
    /// - `CSV` — File in CSV format.
    /// - `BAK`
    /// - `TDE` — TDE certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_user: Option<String>,

    /// This is always `sql#importContext`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Path to the import file in Cloud Storage, in the form `gs://bucketName/fileName`.
    /// Compressed gzip files (.gz) are supported when `fileType` is `SQL`. The instance must
    /// have write permissions to the bucket and read access to the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ImportContext {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bak_import_options: Default::default(),
            csv_import_options: Default::default(),
            database: Some("test-database".into()),
            file_type: Some("test-file_type".into()),
            import_user: Some("test-import_user".into()),
            kind: Some("test-kind".into()),
            uri: Some("test-uri".into()),
        }
    }
}

/// A BackupRun resource.
///
/// **GCP API**: `sqladmin.v1.BackupRun`
/// **Reference**: <https://cloud.google.com/sql/docs/BackupRun>
///
/// ## Coverage
/// 14 of 19 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRun {
    /// Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT.
    ///
    /// **Possible values**:
    /// - `SQL_BACKUP_KIND_UNSPECIFIED` — This is an unknown BackupKind.
    /// - `SNAPSHOT` — Snapshot-based backups.
    /// - `PHYSICAL` — Physical backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_kind: Option<String>,

    /// The description of this run, only applicable to on-demand backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The time the backup operation completed in UTC timezone in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// The time the run was enqueued in UTC timezone in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enqueued_time: Option<String>,

    /// Information about why the backup operation failed. This is only present if the run has
    /// the FAILED status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,

    /// The identifier for this backup run. Unique only for a specific Cloud SQL instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the database instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// This is always `sql#backupRun`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Location of the backups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// The URI of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// The time the backup operation actually started in UTC timezone in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// The status of this run.
    ///
    /// **Possible values**:
    /// - `SQL_BACKUP_RUN_STATUS_UNSPECIFIED` — The status of the run is unknown.
    /// - `ENQUEUED` — The backup operation was enqueued.
    /// - `OVERDUE` — The backup is overdue across a given backup window. Indicates a problem. Example...
    /// - `RUNNING` — The backup is in progress.
    /// - `FAILED` — The backup failed.
    /// - `SUCCESSFUL` — The backup was successful.
    /// - `SKIPPED` — The backup was skipped (without problems) for a given backup window. Example: In...
    /// - `DELETION_PENDING` — The backup is about to be deleted.
    /// - `DELETION_FAILED` — The backup deletion failed.
    /// - `DELETED` — The backup has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field
    /// defaults to "ON_DEMAND" and is ignored, when specified for insert requests.
    ///
    /// **Possible values**:
    /// - `SQL_BACKUP_RUN_TYPE_UNSPECIFIED` — This is an unknown BackupRun type.
    /// - `AUTOMATED` — The backup schedule automatically triggers a backup.
    /// - `ON_DEMAND` — The user manually triggers a backup.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,

    /// The start time of the backup window during which this the backup was attempted in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start_time: Option<String>,
}

impl BackupRun {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            backup_kind: Some("test-backup_kind".into()),
            description: Some("test-description".into()),
            end_time: Some("test-end_time".into()),
            enqueued_time: Some("test-enqueued_time".into()),
            error: Some(OperationError::fixture()),
            id: Some("test-id".into()),
            instance: Some("test-instance".into()),
            kind: Some("test-kind".into()),
            location: Some("test-location".into()),
            self_link: Some("test-self_link".into()),
            start_time: Some("test-start_time".into()),
            status: Some("test-status".into()),
            type_value: Some("test-type".into()),
            window_start_time: Some("test-window_start_time".into()),
        }
    }
}

// ======================================================================
// Auto-generated dependency types (referenced via $ref)
// ======================================================================

/// Deny maintenance Periods. This specifies a date range during when all CSA rollout will be
/// denied.
///
/// **GCP API**: `sqladmin.v1.DenyMaintenancePeriod`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenyMaintenancePeriod {
    /// "deny maintenance period" end date. If the year of the end date is empty, the year of
    /// the start date also must be empty. In this case, it means the no maintenance interval
    /// recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e.,
    /// 11-01
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// "deny maintenance period" start date. If the year of the start date is empty, the year
    /// of the end date also must be empty. In this case, it means the deny maintenance period
    /// recurs every year. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e.,
    /// 11-01
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// Time in UTC when the "deny maintenance period" starts on start_date and ends on
    /// end_date. The time is in format: HH:mm:SS, i.e., 00:00:00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl DenyMaintenancePeriod {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            end_date: Some("test-end_date".into()),
            start_date: Some("test-start_date".into()),
            time: Some("test-time".into()),
        }
    }
}

/// Insights configuration. This specifies when Cloud SQL Insights feature is enabled and
/// optional configuration.
///
/// **GCP API**: `sqladmin.v1.InsightsConfig`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsightsConfig {
    /// Optional. Whether enhanced query insights feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_query_insights_enabled: Option<bool>,

    /// Whether Query Insights feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_insights_enabled: Option<bool>,

    /// Whether Query Insights will record client address when enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_client_address: Option<bool>,

    /// Whether Query Insights will record application tags from query when enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_application_tags: Option<bool>,

    /// Number of query execution plans captured by Insights per minute for all queries
    /// combined. Default is 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_plans_per_minute: Option<i32>,

    /// Maximum query length stored in bytes. Default value: 1024 bytes. Range: 256-4500 bytes.
    /// Query lengths greater than this field value will be truncated to this value. When unset,
    /// query length will be the default value. Changing query length will restart the database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_length: Option<i32>,
}

impl InsightsConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enhanced_query_insights_enabled: Some(false),
            query_insights_enabled: Some(false),
            record_client_address: Some(false),
            record_application_tags: Some(false),
            query_plans_per_minute: Some(100),
            query_string_length: Some(100),
        }
    }
}

/// Database instance local user password validation policy. This message defines the password
/// policy for local database users. When enabled, it enforces constraints on password
/// complexity, length, and reuse. Keep this policy enabled to help prevent unauthorized access.
///
/// **GCP API**: `sqladmin.v1.PasswordValidationPolicy`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordValidationPolicy {
    /// The complexity of the password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<String>,

    /// Number of previous passwords that cannot be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_interval: Option<i32>,

    /// This field is deprecated and will be removed in a future version of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallow_compromised_credentials: Option<bool>,

    /// Minimum interval after which the password can be changed. This flag is only supported
    /// for PostgreSQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_change_interval: Option<String>,

    /// Minimum number of characters allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,

    /// Whether to enable the password policy or not. When enabled, passwords must meet
    /// complexity requirements. Keep this policy enabled to help prevent unauthorized access.
    /// Disabling this policy allows weak passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_password_policy: Option<bool>,

    /// Disallow username as a part of the password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallow_username_substring: Option<bool>,
}

impl PasswordValidationPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            complexity: Some("test-complexity".into()),
            reuse_interval: Some(100),
            disallow_compromised_credentials: Some(false),
            password_change_interval: Some("test-password_change_interval".into()),
            min_length: Some(100),
            enable_password_policy: Some(false),
            disallow_username_substring: Some(false),
        }
    }
}

/// SQL Server specific audit configuration.
///
/// **GCP API**: `sqladmin.v1.SqlServerAuditConfig`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlServerAuditConfig {
    /// How long to keep generated audit files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_interval: Option<String>,

    /// How often to upload generated audit files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_interval: Option<String>,

    /// This is always sql#sqlServerAuditConfig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of the destination bucket (e.g., gs://mybucket).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
}

impl SqlServerAuditConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            retention_interval: Some("test-retention_interval".into()),
            upload_interval: Some("test-upload_interval".into()),
            kind: Some("test-kind".into()),
            bucket: Some("test-bucket".into()),
        }
    }
}

/// SslCerts Resource
///
/// **GCP API**: `sqladmin.v1.SslCert`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SslCert {
    /// User supplied name. Constrained to [a-zA-Z.-_ ]+.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,

    /// The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339)
    /// format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,

    /// The URI of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Sha1 Fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1_fingerprint: Option<String>,

    /// The time when the certificate was created in [RFC
    /// 3339](https://tools.ietf.org/html/rfc3339) format, for example
    /// `2012-11-15T16:19:00.094Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Serial number, as extracted from the certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_serial_number: Option<String>,

    /// Name of the database instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// PEM representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,

    /// This is always `sql#sslCert`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl SslCert {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            common_name: Some("test-common_name".into()),
            expiration_time: Some("test-expiration_time".into()),
            self_link: Some("test-self_link".into()),
            sha1_fingerprint: Some("test-sha1_fingerprint".into()),
            create_time: Some("test-create_time".into()),
            cert_serial_number: Some("test-cert_serial_number".into()),
            instance: Some("test-instance".into()),
            cert: Some("test-cert".into()),
            kind: Some("test-kind".into()),
        }
    }
}
