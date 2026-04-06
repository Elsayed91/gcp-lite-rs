//! Types for the Cloud Resource Manager API (v3).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://cloudresourcemanager.googleapis.com/$discovery/rest?version=v3`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `cloudresourcemanager.v3.Project.state`.
///
/// **GCP API**: `cloudresourcemanager.v3.Project.state`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectState {
    /// Unspecified state. This is only used/useful for distinguishing unset values.
    StateUnspecified,

    /// The normal and active state.
    Active,

    /// The project has been marked for deletion by the user (by invoking DeleteProject) or by
    /// the system (Google Cloud Platform). This can generally be reversed by invoking
    /// UndeleteProject.
    DeleteRequested,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// A project is a high-level Google Cloud entity. It is a container for ACLs, APIs, App Engine
/// Apps, VMs, and other Google Cloud Platform resources.
///
/// **GCP API**: `cloudresourcemanager.v3.Project`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/Project>
///
/// ## Coverage
/// 9 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Output only. The unique resource name of the project. It is an int64 generated number
    /// prefixed by "projects/". Example: `projects/415104041262`
    ///
    /// *Output-only field.*
    pub name: String,

    /// Immutable. The unique, user-assigned id of the project. It must be 6 to 30 lowercase
    /// ASCII letters, digits, or hyphens. It must start with a letter. Trailing hyphens are
    /// prohibited. Example: `tokyo-rain-123`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// Optional. A user-assigned display name of the project. When present it must be between 4
    /// to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, single-quote, double-quote, space, and exclamation point. Example: `My Project`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Output only. The project lifecycle state.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ProjectState>,

    /// Optional. A reference to a parent Resource. eg., `organizations/123` or `folders/876`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Optional. The labels associated with this project. Label keys must be between 1 and 63
    /// characters long and must conform to the following regular expression:
    /// \[a-z\](\[-a-z0-9\]*\[a-z0-9\])?. Label values must be between 0 and 63 characters long
    /// and must conform to the regular expression (\[a-z\](\[-a-z0-9\]*\[a-z0-9\])?)?. No more
    /// than 64 labels can be associated with a given resource. Clients should store labels in a
    /// representation such as JSON that does not depend on specific characters being
    /// disallowed. Example: `"myBusinessDimension" : "businessValue"`
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. Creation time.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. A checksum computed by the server based on the current value of the Project
    /// resource. This may be sent on update and delete requests to ensure the client has an up-
    /// to-date value before proceeding.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Output only. The time at which this resource was requested for deletion.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
}

impl Project {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-project".into(),
            project_id: Some("test-project_id".into()),
            display_name: Some("test-display_name".into()),
            state: Some(ProjectState::StateUnspecified),
            parent: Some("test-parent".into()),
            labels: Default::default(),
            create_time: Some("test-create_time".into()),
            etag: Some("test-etag".into()),
            delete_time: Some("test-delete_time".into()),
        }
    }
}

/// An Identity and Access Management (IAM) policy, which specifies access controls for Google
/// Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more
/// `members`, or principals, to a single `role`. Principals can be user accounts, service
/// accounts, Google groups, and domains (such as G Suite). A `role` is a named list of
/// permissions; each `role` can be an IAM predefined role or a user-created custom role. For
/// some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a
/// logical expression that allows access to a resource only if the expression evaluates to
/// `true`. A condition can add constraints based on attributes of the request, the resource, or
/// both. To learn which resources support conditions in their IAM policies, see the [IAM
/// documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON
/// example:** ``` { "bindings": [ { "role": "roles/resourcemanager.organizationAdmin",
/// "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com",
/// "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role":
/// "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ],
/// "condition": { "title": "expirable access", "description": "Does not grant access after Sep
/// 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag":
/// "BwWWja0YfJA=", "version": 3 } ``` **YAML example:** ``` bindings:
/// - members:
/// - user:mike@example.com
/// - group:admins@example.com
/// - domain:google.com
/// - serviceAccount:my-project-id@appspot.gserviceaccount.com role:
///   roles/resourcemanager.organizationAdmin
/// - members:
/// - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title:
///   expirable access description: Does not grant access after Sep 2020 expression:
///   request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3 ``` For
///   a description of IAM and its features, see the [IAM
///   documentation](https://cloud.google.com/iam/docs/).
///
/// **GCP API**: `cloudresourcemanager.v3.Policy`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/Policy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamPolicy {
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that
    /// specify an invalid value are rejected. Any operation that affects conditional role
    /// bindings must specify version `3`. This requirement applies to the following operations:
    /// * Getting a policy that includes a conditional role binding
    /// * Adding a conditional role binding to a policy
    /// * Changing a conditional role binding in a policy
    /// * Removing any role binding, with or without a condition, from a policy that includes
    ///   conditions **Important:** If you use IAM Conditions, you must include the `etag` field
    ///   whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to
    ///   overwrite a version `3` policy with a version `1` policy, and all of the conditions in
    ///   the version `3` policy are lost. If a policy does not include any conditions,
    ///   operations on that policy may specify any valid version or leave the field unset. To
    ///   learn which resources support conditions in their IAM policies, see the [IAM
    ///   documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,

    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a
    /// `condition` that determines how and when the `bindings` are applied. Each of the
    /// `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer
    /// to up to 1,500 principals; up to 250 of these principals can be Google groups. Each
    /// occurrence of a principal counts towards these limits. For example, if the `bindings`
    /// grant 50 different roles to `user:alice@example.com`, and not to any other principal,
    /// then you can add another 1,450 principals to the `bindings` in the `Policy`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<IamBinding>,

    /// Specifies cloud audit logging configuration for this policy.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audit_configs: Vec<AuditConfig>,

    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous
    /// updates of a policy from overwriting each other. It is strongly suggested that systems
    /// make use of the `etag` in the read-modify-write cycle to perform policy updates in order
    /// to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and
    /// systems are expected to put that etag in the request to `setIamPolicy` to ensure that
    /// their change will be applied to the same version of the policy. **Important:** If you
    /// use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`.
    /// If you omit this field, then IAM allows you to overwrite a version `3` policy with a
    /// version `1` policy, and all of the conditions in the version `3` policy are lost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

impl IamPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            version: Some(100),
            bindings: vec![],
            audit_configs: vec![],
            etag: Some("test-etag".into()),
        }
    }
}

/// Associates `members`, or principals, with a `role`.
///
/// **GCP API**: `cloudresourcemanager.v3.Binding`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/Binding>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamBinding {
    /// Role that is assigned to the list of `members`, or principals. For example,
    /// `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and
    /// permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-
    /// overview). For a list of the available pre-defined roles, see
    /// [here](https://cloud.google.com/iam/docs/understanding-roles).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Specifies the principals requesting access for a Google Cloud resource. `members` can
    /// have the following values:
    /// * `allUsers`: A special identifier that represents anyone who is on the internet; with
    ///   or without a Google account.
    /// * `allAuthenticatedUsers`: A special identifier that represents anyone who is
    ///   authenticated with a Google account or a service account. Does not include identities
    ///   that come from external identity providers (IdPs) through identity federation.
    /// * `user:{emailid}`: An email address that represents a specific Google account. For
    ///   example, `alice@example.com` .
    /// * `serviceAccount:{emailid}`: An email address that represents a Google service account.
    ///   For example, `my-other-app@appspot.gserviceaccount.com`.
    /// * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier
    ///   for a [Kubernetes service account](https://cloud.google.com/kubernetes-
    ///   engine/docs/how-to/kubernetes-service-accounts). For example, `my-
    ///   project.svc.id.goog[my-namespace/my-kubernetes-sa]`.
    /// * `group:{emailid}`: An email address that represents a Google group. For example,
    ///   `admins@example.com`.
    /// * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that
    ///   domain. For example, `google.com` or `example.com`.
    /// * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{sub
    ///   ject_attribute_value}`: A single identity in a workforce identity pool.
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{gr
    ///   oup_id}`: All workforce identities in a group.
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute
    ///   .{attribute_name}/{attribute_value}`: All workforce identities with a specific
    ///   attribute value.
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All
    ///   identities in a workforce identity pool.
    /// * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIde
    ///   ntityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a
    ///   workload identity pool.
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workload
    ///   IdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group.
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workload
    ///   IdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities
    ///   in a workload identity pool with a certain attribute.
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workload
    ///   IdentityPools/{pool_id}/*`: All identities in a workload identity pool.
    /// * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier)
    ///   representing a user that has been recently deleted. For example,
    ///   `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value
    ///   reverts to `user:{emailid}` and the recovered user retains the role in the binding.
    /// * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique
    ///   identifier) representing a service account that has been recently deleted. For
    ///   example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the
    ///   service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the
    ///   undeleted service account retains the role in the binding.
    /// * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier)
    ///   representing a Google group that has been recently deleted. For example,
    ///   `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value
    ///   reverts to `group:{emailid}` and the recovered group retains the role in the binding.
    /// * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subj
    ///   ect/{subject_attribute_value}`: Deleted single identity in a workforce identity pool.
    ///   For example,
    ///   `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-
    ///   id/subject/my-subject-attribute-value`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,

    /// The condition that is associated with this binding. If the condition evaluates to
    /// `true`, then this binding applies to the current request. If the condition evaluates to
    /// `false`, then this binding does not apply to the current request. However, a different
    /// role binding might grant the same role to one or more of the principals in this binding.
    /// To learn which resources support conditions in their IAM policies, see the [IAM
    /// documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<IamCondition>,
}

impl IamBinding {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role: Some("test-role".into()),
            members: vec![],
            condition: Some(IamCondition::fixture()),
        }
    }
}

/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a
/// C-like expression language. The syntax and semantics of CEL are documented at
/// https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit"
/// description: "Determines if a summary is less than 100 chars" expression:
/// "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description:
/// "Determines if requestor is the document owner" expression: "document.owner ==
/// request.auth.claims.email" Example (Logic): title: "Public documents" description:
/// "Determine whether the document should be publicly visible" expression: "document.type !=
/// 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification
/// string" description: "Create a notification string with a timestamp." expression: "'New
/// message received at ' + string(document.create_time)" The exact variables and functions that
/// may be referenced within an expression are determined by the service that evaluates it. See
/// the service documentation for additional information.
///
/// **GCP API**: `cloudresourcemanager.v3.Expr`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/Expr>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamCondition {
    /// Textual representation of an expression in Common Expression Language syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can
    /// be used e.g. in UIs which allow to enter the expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Optional. Description of the expression. This is a longer text which describes the
    /// expression, e.g. when hovered over it in a UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl IamCondition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expression: Some("test-expression".into()),
            title: Some("test-title".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Specifies the audit configuration for a service. The configuration determines which
/// permission types are logged, and what identities, if any, are exempted from logging. An
/// AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both
/// `allServices` and a specific service, the union of the two AuditConfigs is used for that
/// service: the log_types specified in each AuditConfig are enabled, and the exempted_members
/// in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: {
/// "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type":
/// "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE"
/// }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com",
/// "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE",
/// "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy
/// enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com`
/// from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
///
/// **GCP API**: `cloudresourcemanager.v3.AuditConfig`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/AuditConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditConfig {
    /// Specifies a service that will be enabled for audit logging. For example,
    /// `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value
    /// that covers all services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// The configuration for logging of each type of permission.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audit_log_configs: Vec<AuditLogConfig>,
}

impl AuditConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service: Some("test-service".into()),
            audit_log_configs: vec![],
        }
    }
}

/// Provides the configuration for logging a type of permissions. Example: {
/// "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [
/// "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and
/// 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
///
/// **GCP API**: `cloudresourcemanager.v3.AuditLogConfig`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/AuditLogConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogConfig {
    /// The log type that this config enables.
    ///
    /// **Possible values**:
    /// - `LOG_TYPE_UNSPECIFIED` — Default case. Should never be this.
    /// - `ADMIN_READ` — Admin reads. Example: CloudIAM getIamPolicy
    /// - `DATA_WRITE` — Data writes. Example: CloudSQL Users create
    /// - `DATA_READ` — Data reads. Example: CloudSQL Users list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,

    /// Specifies the identities that do not cause logging for this type of permission. Follows
    /// the same format of Binding.members.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exempted_members: Vec<String>,
}

impl AuditLogConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            log_type: Some("test-log_type".into()),
            exempted_members: vec![],
        }
    }
}

/// This resource represents a long-running operation that is the result of a network API call.
///
/// **GCP API**: `cloudresourcemanager.v3.Operation`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/Operation>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsLro {
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

impl ProjectsLro {
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

/// Request message for `GetIamPolicy` method.
///
/// **GCP API**: `cloudresourcemanager.v3.GetIamPolicyRequest`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/GetIamPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GetPolicyOptions>,
}

impl GetIamPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            options: Some(GetPolicyOptions::fixture()),
        }
    }
}

/// Encapsulates settings provided to GetIamPolicy.
///
/// **GCP API**: `cloudresourcemanager.v3.GetPolicyOptions`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/GetPolicyOptions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid
    /// values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests
    /// for policies with any conditional role bindings must specify version 3. Policies with no
    /// conditional role bindings may specify any valid value or leave the field unset. The
    /// policy in the response might use the policy version that you specified, or it might use
    /// a lower policy version. For example, if you specify version 3, but the policy has no
    /// conditional role bindings, the response uses version 1. To learn which resources support
    /// conditions in their IAM policies, see the [IAM
    /// documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_policy_version: Option<i32>,
}

impl GetPolicyOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            requested_policy_version: Some(100),
        }
    }
}

/// Request message for `SetIamPolicy` method.
///
/// **GCP API**: `cloudresourcemanager.v3.SetIamPolicyRequest`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/SetIamPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is
    /// limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud
    /// services (such as Projects) might reject them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<IamPolicy>,

    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields
    /// in the mask will be modified. If no mask is provided, the following default mask is
    /// used: `paths: "bindings, etag"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<String>,
}

impl SetIamPolicyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy: Some(IamPolicy::fixture()),
            update_mask: Some("test-update_mask".into()),
        }
    }
}

/// Request message for `TestIamPermissions` method.
///
/// **GCP API**: `cloudresourcemanager.v3.TestIamPermissionsRequest`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/TestIamPermissionsRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as
    /// `*` or `storage.*`) are not allowed. For more information see [IAM
    /// Overview](https://cloud.google.com/iam/docs/overview#permissions).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
}

impl TestIamPermissionsRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            permissions: vec![],
        }
    }
}

/// Response message for `TestIamPermissions` method.
///
/// **GCP API**: `cloudresourcemanager.v3.TestIamPermissionsResponse`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/TestIamPermissionsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
}

impl TestIamPermissionsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            permissions: vec![],
        }
    }
}

/// The request sent to MoveProject method.
///
/// **GCP API**: `cloudresourcemanager.v3.MoveProjectRequest`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/MoveProjectRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveProjectRequest {
    /// Required. The new parent to move the Project under.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_parent: Option<String>,
}

impl MoveProjectRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            destination_parent: Some("test-destination_parent".into()),
        }
    }
}

/// The request sent to the UndeleteProject method.
///
/// **GCP API**: `cloudresourcemanager.v3.UndeleteProjectRequest`
/// **Reference**: <https://cloud.google.com/resource-manager/reference/rest/v3/UndeleteProjectRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndeleteProjectRequest {}

impl UndeleteProjectRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing Project resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsResponse {
    /// A list of Project resources.
    #[serde(default)]
    pub projects: Vec<Project>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListProjectsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            projects: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Project resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchProjectsResponse {
    /// A list of Project resources.
    #[serde(default)]
    pub projects: Vec<Project>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SearchProjectsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            projects: vec![],
            next_page_token: None,
        }
    }
}
