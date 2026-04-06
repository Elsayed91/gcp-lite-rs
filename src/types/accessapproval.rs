//! Types for the Access Approval API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/accessapproval/v1/rest`

use serde::{Deserialize, Serialize};

/// Represents the enrollment of a cloud resource into a specific service.
///
/// **GCP API**: `accessapproval.v1.EnrolledService`
/// **Reference**: <https://cloud.google.com/assured-workloads/access-approval/docs/EnrolledService>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are listed below
    /// (case-sensitive):
    /// * all
    /// * GA
    /// * Access Context Manager
    /// * Anthos Identity Service
    /// * AlloyDB for PostgreSQL
    /// * Apigee
    /// * Application Integration
    /// * App Hub
    /// * Artifact Registry
    /// * Anthos Service Mesh
    /// * Access Transparency
    /// * BigQuery
    /// * Certificate Authority Service
    /// * Cloud Bigtable
    /// * CCAI Assist and Knowledge
    /// * Cloud Dataflow
    /// * Cloud Dataproc
    /// * CEP Security Gateway
    /// * Compliance Evaluation Service
    /// * Cloud Firestore
    /// * Cloud Healthcare API
    /// * Chronicle
    /// * Cloud AI Companion Gateway
    /// - Titan
    /// * Google Cloud Armor
    /// * Cloud Asset Inventory
    /// * Cloud Asset Search
    /// * Cloud Deploy
    /// * Cloud DNS
    /// * Cloud Latency
    /// * Cloud Memorystore for Redis
    /// * CloudNet Control
    /// * Cloud Riptide
    /// * Cloud Tasks
    /// * Cloud Trace
    /// * Cloud Data Transfer
    /// * Cloud Composer
    /// * Integration Connectors
    /// * Contact Center AI Insights
    /// * Cloud Pub/Sub
    /// * Cloud Run
    /// * Resource Manager
    /// * Cloud Spanner
    /// * Database Center
    /// * Cloud Dataform
    /// * Cloud Data Fusion
    /// * Dataplex
    /// * Dialogflow Customer Experience Edition
    /// * Cloud DLP
    /// * Document AI
    /// * Edge Container
    /// * Edge Network
    /// * Cloud EKM
    /// * Eventarc
    /// * Firebase Data Connect
    /// * Firebase Rules
    /// * App Engine
    /// * Cloud Build
    /// * Compute Engine
    /// * Cloud Functions (2nd Gen)
    /// * Cloud Filestore
    /// * Cloud Interconnect
    /// * Cloud NetApp Volumes
    /// * Cloud Storage
    /// * Generative AI App Builder
    /// * Google Kubernetes Engine
    /// * Backup for GKE API
    /// * GKE Connect
    /// * GKE Hub
    /// * Hoverboard
    /// * Cloud HSM
    /// * Cloud Identity and Access Management
    /// * Cloud Identity-Aware Proxy
    /// * Infrastructure Manager
    /// * Identity Storage Service
    /// * Key Access Justifications
    /// * Cloud Key Management Service
    /// * Cloud Logging
    /// * Looker (Google Cloud core)
    /// * Looker Studio
    /// * Management Hub
    /// * Model Armor
    /// * Cloud Monitoring
    /// * Cloud NAT
    /// * Connectivity Hub
    /// * External passthrough Network Load Balancer
    /// * OIDC One
    /// * Organization Policy Service
    /// * Org Lifecycle
    /// * Persistent Disk
    /// * Parameter Manager
    /// * Private Services Access
    /// * Regional Internal Application Load Balancer
    /// * Storage Batch Operations
    /// * Cloud Security Command Center
    /// * Secure Source Manager
    /// * Seeker
    /// * Service Provisioning
    /// * Speaker ID
    /// * Secret Manager
    /// * Cloud SQL
    /// * Cloud Speech-to-Text
    /// * Traffic Director
    /// * Cloud Text-to-Speech
    /// * USPS Andromeda
    /// * Vertex AI
    /// * Virtual Private Cloud (VPC)
    /// * VPC Access
    /// * VPC Service Controls Troubleshooter
    /// * VPC virtnet
    /// * Cloud Workstations
    /// * Web Risk Note: These values are supported as input for legacy purposes, but will not
    ///   be returned from the API.
    /// * all
    /// * ga-only
    /// * appengine.googleapis.com
    /// * artifactregistry.googleapis.com
    /// * bigquery.googleapis.com
    /// * bigtable.googleapis.com
    /// * container.googleapis.com
    /// * cloudkms.googleapis.com
    /// * cloudresourcemanager.googleapis.com
    /// * cloudsql.googleapis.com
    /// * compute.googleapis.com
    /// * dataflow.googleapis.com
    /// * dataproc.googleapis.com
    /// * dlp.googleapis.com
    /// * iam.googleapis.com
    /// * logging.googleapis.com
    /// * orgpolicy.googleapis.com
    /// * pubsub.googleapis.com
    /// * spanner.googleapis.com
    /// * secretmanager.googleapis.com
    /// * speakerid.googleapis.com
    /// * storage.googleapis.com Calls to UpdateAccessApprovalSettings using 'all' or any of the
    ///   XXX.googleapis.com will be translated to the associated product name ('all', 'App
    ///   Engine', etc.). Note: 'all' will enroll the resource in all products supported at both
    ///   'GA' and 'Preview' levels. More information about levels of support is available at
    ///   https://cloud.google.com/access-approval/docs/supported-services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_product: Option<String>,

    /// The enrollment level of the service.
    ///
    /// **Possible values**:
    /// - `ENROLLMENT_LEVEL_UNSPECIFIED` — Default value if not set, defaults to "BLOCK_ALL". This value is not available t...
    /// - `BLOCK_ALL` — Service is enrolled in Access Approval for all requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_level: Option<String>,
}

impl EnrolledService {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cloud_product: Some("test-cloud_product".into()),
            enrollment_level: Some("test-enrollment_level".into()),
        }
    }
}

/// Settings on a Project/Folder/Organization related to Access Approval.
///
/// **GCP API**: `accessapproval.v1.AccessApprovalSettings`
/// **Reference**: <https://cloud.google.com/assured-workloads/access-approval/docs/AccessApprovalSettings>
///
/// ## Coverage
/// 13 of 15 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessApprovalSettings {
    /// The resource name of the settings. Format is one of: *
    /// "projects/{project}/accessApprovalSettings" * "folders/{folder}/accessApprovalSettings"
    /// * "organizations/{organization}/accessApprovalSettings"
    pub name: String,

    /// A list of email addresses to which notifications relating to approval requests should be
    /// sent. Notifications relating to a resource will be sent to all emails in the settings of
    /// ancestor resources of that resource. A maximum of 50 email addresses are allowed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notification_emails: Vec<String>,

    /// A list of Google Cloud Services for which the given resource has Access Approval
    /// enrolled. Access requests for the resource given by name against any of these services
    /// contained here will be required to have explicit approval. If name refers to an
    /// organization, enrollment can be done for individual services. If name refers to a folder
    /// or project, enrollment can only be done on an all or nothing basis. If a cloud_product
    /// is repeated in this list, the first entry will be honored and all following entries will
    /// be discarded.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enrolled_services: Vec<EnrolledService>,

    /// Output only. This field is read only (not settable via UpdateAccessApprovalSettings
    /// method). If the field is true, that indicates that at least one service is enrolled for
    /// Access Approval in one or more ancestors of the Project or Folder (this field will
    /// always be unset for the organization since organizations do not have ancestors).
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_ancestor: Option<bool>,

    /// Optional. A pubsub topic that notifications relating to access approval are published
    /// to. Notifications include pre-approved accesses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_pubsub_topic: Option<String>,

    /// The asymmetric crypto key version to use for signing approval requests. Empty
    /// active_key_version indicates that a Google-managed key should be used for signing. This
    /// property will be ignored if set by an ancestor of this resource, and new non-empty
    /// values may not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_key_version: Option<String>,

    /// Output only. This field is read only (not settable via UpdateAccessApprovalSettings
    /// method). If the field is true, that indicates that there is some configuration issue
    /// with the active_key_version configured at this level in the resource hierarchy (e.g. it
    /// doesn't exist or the Access Approval service account doesn't have the correct
    /// permissions on it, etc.) This key version is not necessarily the effective key version
    /// at this level, as key versions are inherited top-down.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_key_version: Option<bool>,

    /// Output only. This field is read only (not settable via UpdateAccessApprovalSettings
    /// method). If the field is true, that indicates that an ancestor of this Project or Folder
    /// has set active_key_version (this field will always be unset for the organization since
    /// organizations do not have ancestors).
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_has_active_key_version: Option<bool>,

    /// Output only. Field to differentiate ancestor enrolled services from locally enrolled
    /// services.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ancestors_enrolled_services: Vec<EnrolledService>,

    /// This field is used to set a preference for granularity of an access approval request. If
    /// true, Google personnel will be asked to send resource-level requests when possible. If
    /// false, Google personnel will be asked to send requests at the project level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_no_broad_approval_requests: Option<bool>,

    /// Set the default access approval request expiration time. This value is able to be set
    /// directly by the customer at the time of approval, overriding this suggested value. We
    /// recommend setting this value to 30 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_request_expiration_days: Option<i32>,

    /// Optional. A setting that indicates the maximum scope of an Access Approval request:
    /// either organization, folder, or project. Google administrators will be asked to send
    /// requests no broader than the configured scope.
    ///
    /// **Possible values**:
    /// - `REQUEST_SCOPE_MAX_WIDTH_PREFERENCE_UNSPECIFIED` — Default value, defaults to ORGANIZATION if not set. This value is not able to be...
    /// - `ORGANIZATION` — This is the widest scope possible. It means the customer has no scope restrictio...
    /// - `FOLDER` — Customer allows the scope of Access Approval requests as broad as the Folder lev...
    /// - `PROJECT` — Customer allows the scope of Access Approval requests as broad as the Project le...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_scope_max_width_preference: Option<String>,

    /// Optional. When enabled, Google will only be able to send approval requests for access
    /// reasons with a customer accessible case ID in the reason detail. Also known as "Require
    /// customer initiated support case justification"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_customer_visible_justification: Option<bool>,
}

impl AccessApprovalSettings {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-access_approval_settings".into(),
            notification_emails: vec![],
            enrolled_services: vec![],
            enrolled_ancestor: Some(false),
            notification_pubsub_topic: Some("test-notification_pubsub_topic".into()),
            active_key_version: Some("test-active_key_version".into()),
            invalid_key_version: Some(false),
            ancestor_has_active_key_version: Some(false),
            ancestors_enrolled_services: vec![],
            prefer_no_broad_approval_requests: Some(false),
            preferred_request_expiration_days: Some(100),
            request_scope_max_width_preference: Some(
                "test-request_scope_max_width_preference".into(),
            ),
            require_customer_visible_justification: Some(false),
        }
    }
}
