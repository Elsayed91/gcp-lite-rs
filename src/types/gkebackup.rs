//! Types for the Backup for GKE API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://gkebackup.googleapis.com/$discovery/rest?version=v1`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `gkebackup.v1.BackupPlan.state`.
///
/// **GCP API**: `gkebackup.v1.BackupPlan.state`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupPlanState {
    /// Default first value for Enums.
    StateUnspecified,

    /// Waiting for cluster state to be RUNNING.
    ClusterPending,

    /// The BackupPlan is in the process of being created.
    Provisioning,

    /// The BackupPlan has successfully been created and is ready for Backups.
    Ready,

    /// BackupPlan creation has failed.
    Failed,

    /// The BackupPlan has been deactivated.
    Deactivated,

    /// The BackupPlan is in the process of being deleted.
    Deleting,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Defines the configuration and scheduling for a "line" of Backups.
///
/// **GCP API**: `gkebackup.v1.BackupPlan`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/BackupPlan>
///
/// ## Coverage
/// 15 of 19 fields included.
/// Omitted fields:
/// - `backupChannel` — Cross-project backup channel — advanced feature
/// - `lastSuccessfulBackupTime` — Read-only field, not needed for CRUD
/// - `rpoRiskLevel` — RPO risk assessment — informational only
/// - `rpoRiskReason` — RPO risk assessment — informational only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupPlan {
    /// Output only. Identifier. The full name of the BackupPlan resource. Format:
    /// `projects/*/locations/*/backupPlans/*`
    ///
    /// *Output-only field.*
    pub name: String,

    /// Required. Immutable. The source cluster from which Backups will be created via this
    /// BackupPlan. Valid formats:
    /// - `projects/*/locations/*/clusters/*`
    /// - `projects/*/zones/*/clusters/*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,

    /// Optional. User specified descriptive string for this BackupPlan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output only. The timestamp when this BackupPlan resource was created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. The timestamp when this BackupPlan resource was last updated.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// Output only. `etag` is used for optimistic concurrency control as a way to help prevent
    /// simultaneous updates of a backup plan from overwriting each other. It is strongly
    /// suggested that systems make use of the 'etag' in the read-modify-write cycle to perform
    /// BackupPlan updates in order to avoid race conditions: An `etag` is returned in the
    /// response to `GetBackupPlan`, and systems are expected to put that etag in the request to
    /// `UpdateBackupPlan` or `DeleteBackupPlan` to ensure that their change will be applied to
    /// the same version of the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Optional. A set of custom labels supplied by user.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. State of the BackupPlan. This State field reflects the various stages a
    /// BackupPlan can be in during the Create operation. It will be set to "DEACTIVATED" if the
    /// BackupPlan is deactivated on an Update
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<BackupPlanState>,

    /// Output only. Human-readable description of why BackupPlan is in the current `state`.
    /// This field is only meant for human readability and should not be used programmatically
    /// as this field is not guaranteed to be consistent.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,

    /// Output only. Server generated global unique identifier of
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier) format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Optional. Defines the configuration of Backups created via this BackupPlan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_config: Option<BackupConfig>,

    /// Optional. Defines a schedule for automatic Backup creation via this BackupPlan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_schedule: Option<BackupSchedule>,

    /// Optional. RetentionPolicy governs lifecycle of Backups created under this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,

    /// Output only. The number of Kubernetes Pods backed up in the last successful Backup
    /// created via this BackupPlan.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_pod_count: Option<i32>,

    /// Optional. This flag indicates whether this BackupPlan has been deactivated. Setting this
    /// field to True locks the BackupPlan such that no further updates will be allowed (except
    /// deletes), including the deactivated field itself. It also prevents any new Backups from
    /// being created via this BackupPlan (including scheduled Backups). Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
}

impl BackupPlan {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-backup_plan".into(),
            cluster: Some("test-cluster".into()),
            description: Some("test-description".into()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
            etag: Some("test-etag".into()),
            labels: Default::default(),
            state: Some(BackupPlanState::StateUnspecified),
            state_reason: Some("test-state_reason".into()),
            uid: Some("test-uid".into()),
            backup_config: Some(BackupConfig::fixture()),
            backup_schedule: Some(BackupSchedule::fixture()),
            retention_policy: Some(RetentionPolicy::fixture()),
            protected_pod_count: Some(100),
            deactivated: Some(false),
        }
    }
}

/// BackupConfig defines the configuration of Backups created via this BackupPlan.
///
/// **GCP API**: `gkebackup.v1.BackupConfig`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/BackupConfig>
///
/// ## Coverage
/// 4 of 8 fields included.
/// Omitted fields:
/// - `selectedApplications` — Complex nested type (NamespacedNames) — use allNamespaces for basic usage
/// - `selectedNamespaces` — Complex nested type (Namespaces) — use allNamespaces for basic usage
/// - `selectedNamespaceLabels` — Complex nested type (ResourceLabels)
/// - `encryptionKey` — Advanced CMEK encryption config
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupConfig {
    /// If True, include all namespaced resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_namespaces: Option<bool>,

    /// Optional. This flag specifies whether volume data should be backed up when PVCs are
    /// included in the scope of a Backup. Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_volume_data: Option<bool>,

    /// Optional. This flag specifies whether Kubernetes Secret resources should be included
    /// when they fall into the scope of Backups. Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_secrets: Option<bool>,

    /// Optional. If false, Backups will fail when Backup for GKE detects Kubernetes
    /// configuration that is non-standard or requires additional setup to restore. Default:
    /// False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissive_mode: Option<bool>,
}

impl BackupConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            all_namespaces: Some(false),
            include_volume_data: Some(false),
            include_secrets: Some(false),
            permissive_mode: Some(false),
        }
    }
}

/// Defines scheduling parameters for automatically creating Backups via this BackupPlan.
///
/// **GCP API**: `gkebackup.v1.Schedule`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/Schedule>
///
/// ## Coverage
/// 2 of 4 fields included.
/// Omitted fields:
/// - `rpoConfig` — Advanced RPO configuration
/// - `nextScheduledBackupTime` — Read-only output field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupSchedule {
    /// Optional. A standard [cron](https://wikipedia.com/wiki/cron) string that defines a
    /// repeating schedule for creating Backups via this BackupPlan. This is mutually exclusive
    /// with the rpo_config field since at most one schedule can be defined for a BackupPlan. If
    /// this is defined, then backup_retain_days must also be defined. Default (empty): no
    /// automatic backup creation will occur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_schedule: Option<String>,

    /// Optional. This flag denotes whether automatic Backup creation is paused for this
    /// BackupPlan. Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

impl BackupSchedule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cron_schedule: Some("test-cron_schedule".into()),
            paused: Some(false),
        }
    }
}

/// RetentionPolicy defines a Backup retention policy for a BackupPlan.
///
/// **GCP API**: `gkebackup.v1.RetentionPolicy`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/RetentionPolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPolicy {
    /// Optional. Minimum age for Backups created via this BackupPlan (in days). This field MUST
    /// be an integer value between 0-90 (inclusive). A Backup created under this BackupPlan
    /// will NOT be deletable until it reaches Backup's (create_time + backup_delete_lock_days).
    /// Updating this field of a BackupPlan does NOT affect existing Backups under it. Backups
    /// created AFTER a successful update will inherit the new value. Default: 0 (no delete
    /// blocking)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_delete_lock_days: Option<i32>,

    /// Optional. The default maximum age of a Backup created via this BackupPlan. This field
    /// MUST be an integer value >= 0 and <= 365. If specified, a Backup created under this
    /// BackupPlan will be automatically deleted after its age reaches (create_time +
    /// backup_retain_days). If not specified, Backups created under this BackupPlan will NOT be
    /// subject to automatic deletion. Updating this field does NOT affect existing Backups
    /// under it. Backups created AFTER a successful update will automatically pick up the new
    /// value. NOTE: backup_retain_days must be >= backup_delete_lock_days. If cron_schedule is
    /// defined, then this must be <= 360
    /// * the creation interval. If rpo_config is defined, then this must be <= 360
    /// * target_rpo_minutes / (1440minutes/day). Default: 0 (no automatic deletion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retain_days: Option<i32>,

    /// Optional. This flag denotes whether the retention policy of this BackupPlan is locked.
    /// If set to True, no further update is allowed on this policy, including the `locked`
    /// field itself. Default: False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl RetentionPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            backup_delete_lock_days: Some(100),
            backup_retain_days: Some(100),
            locked: Some(false),
        }
    }
}

/// Response message for ListBackupPlans.
///
/// **GCP API**: `gkebackup.v1.ListBackupPlansResponse`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/ListBackupPlansResponse>
///
/// ## Coverage
/// 2 of 3 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBackupPlansResponse {
    /// The list of BackupPlans matching the given criteria.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub backup_plans: Vec<BackupPlan>,

    /// A token which may be sent as page_token in a subsequent `ListBackupPlans` call to
    /// retrieve the next page of results. If this field is omitted or empty, then there are no
    /// more results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListBackupPlansResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            backup_plans: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// This resource represents a long-running operation that is the result of a network API call.
///
/// **GCP API**: `gkebackup.v1.GoogleLongrunningOperation`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/GoogleLongrunningOperation>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GkeBackupLro {
    /// The server-assigned name, which is only unique within the same service that originally
    /// returns it. If you use the default HTTP mapping, the `name` should be a resource name
    /// ending with `operations/{unique_id}`.
    pub name: String,

    /// If the value is `false`, it means the operation is still in progress. If `true`, the
    /// operation is completed, and either `error` or `response` is available.
    #[serde(default)]
    pub done: bool,

    /// The error result of the operation in case of failure or cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,

    /// The normal, successful response of the operation. If the original method returns no data
    /// on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original
    /// method is standard `Get`/`Create`/`Update`, the response should be the resource. For
    /// other methods, the response should have the type `XxxResponse`, where `Xxx` is the
    /// original method name. For example, if the original method name is `TakeSnapshot()`, the
    /// inferred response type is `TakeSnapshotResponse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,

    /// Service-specific metadata associated with the operation. It typically contains progress
    /// information and common metadata such as create time. Some services might not provide
    /// such metadata. Any method that returns a long-running operation should document the
    /// metadata type, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl GkeBackupLro {
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
            done: false,
            ..Default::default()
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    /// Create a completed operation fixture for testing.
    pub fn fixture_done() -> Self {
        Self {
            name: "operation-done".into(),
            done: true,
            ..Default::default()
        }
    }
}
