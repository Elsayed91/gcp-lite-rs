//! Types for the Cloud Asset API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/cloudasset/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An asset in Google Cloud. An asset can be any resource in the Google Cloud [resource
/// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-
/// hierarchy), a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. IAM policy), or a relationship
/// (e.g. an INSTANCE_TO_INSTANCEGROUP relationship). See [Supported asset
/// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more
/// information.
///
/// **GCP API**: `cloudasset.v1.Asset`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/Asset>
///
/// ## Coverage
/// 6 of 14 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See
    /// [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more
    /// information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,

    /// The last update timestamp of an asset. update_time is updated when create/update/delete
    /// operation is performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-
    /// hierarchy), represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset is a project,
    /// folder, or organization, the ancestry path starts from the asset itself. Example:
    /// `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ancestors: Vec<String>,

    /// A representation of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,

    /// A representation of the IAM policy set on a Google Cloud resource. There can be a
    /// maximum of one IAM policy set on any given resource. In addition, IAM policies inherit
    /// their granted access scope from any policies set on parent resources in the resource
    /// hierarchy. Therefore, the effectively policy is the union of both the policy set on this
    /// resource and each policy set on all of the resource's ancestry resource levels in the
    /// hierarchy. See [this topic](https://cloud.google.com/iam/help/allow-
    /// policies/inheritance) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_policy: Option<Policy>,
}

impl Asset {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-asset".into()),
            asset_type: Some("test-asset_type".into()),
            update_time: Some("test-update_time".into()),
            ancestors: vec![],
            resource: Some(Resource::fixture()),
            iam_policy: Some(Policy::fixture()),
        }
    }
}

/// A representation of a Google Cloud resource.
///
/// **GCP API**: `cloudasset.v1.Resource`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/Resource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// The full name of the immediate parent of this resource. See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more
    /// information. For Google Cloud assets, this value is the parent resource defined in the
    /// [IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
    /// Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// The content of the resource, in which some sensitive fields are removed and may not be
    /// present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// The location of the resource in Google Cloud, such as its zone and region. For more
    /// information, see https://cloud.google.com/about/locations/.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// The API version. Example: `v1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The JSON schema name listed in the discovery document. Example: `Project` This value is
    /// unspecified for resources that do not have an API based on a discovery document, such as
    /// Cloud Bigtable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_name: Option<String>,

    /// The URL of the discovery document containing the resource's JSON schema. Example:
    /// `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified
    /// for resources that do not have an API based on a discovery document, such as Cloud
    /// Bigtable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_document_uri: Option<String>,

    /// The REST URL for accessing the resource. An HTTP `GET` request using this URL returns
    /// the resource itself. Example:
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is
    /// unspecified for resources without a REST API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_url: Option<String>,
}

impl Resource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            parent: Some("test-parent".into()),
            data: Default::default(),
            location: Some("test-location".into()),
            version: Some("test-version".into()),
            discovery_name: Some("test-discovery_name".into()),
            discovery_document_uri: Some("test-discovery_document_uri".into()),
            resource_url: Some("test-resource_url".into()),
        }
    }
}

/// A result of Resource Search, containing information of a cloud resource.
///
/// **GCP API**: `cloudasset.v1.ResourceSearchResult`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/ResourceSearchResult>
///
/// ## Coverage
/// 15 of 28 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSearchResult {
    /// The full resource name of this resource. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See
    /// [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-
    /// inventory/docs/resource-name-format) for more information. To search against the `name`:
    /// * Use a field query. Example: `name:instance1`
    /// * Use a free text query. Example: `instance1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of this resource. Example: `compute.googleapis.com/Disk`. To search against the
    /// `asset_type`:
    /// * Specify the `asset_type` field in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,

    /// The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}.
    /// This field is available when the resource belongs to a project. To search against
    /// `project`:
    /// * Use a field query. Example: `project:12345`
    /// * Use a free text query. Example: `12345`
    /// * Specify the `scope` field as this project in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}.
    /// This field is available when the resource belongs to one or more folders. To search
    /// against `folders`:
    /// * Use a field query. Example: `folders:(123 OR 456)`
    /// * Use a free text query. Example: `123`
    /// * Specify the `scope` field as this folder in your search request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,

    /// The organization that this resource belongs to, in the form of
    /// organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs
    /// to an organization. To search against `organization`:
    /// * Use a field query. Example: `organization:123`
    /// * Use a free text query. Example: `123`
    /// * Specify the `scope` field as this organization in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// The display name of this resource. This field is available only when the resource's
    /// Protobuf contains it. To search against the `display_name`:
    /// * Use a field query. Example: `displayName:"My Instance"`
    /// * Use a free text query. Example: `"My Instance"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// One or more paragraphs of text description of this resource. Maximum length could be up
    /// to 1M bytes. This field is available only when the resource's Protobuf contains it. To
    /// search against the `description`:
    /// * Use a field query. Example: `description:"important instance"`
    /// * Use a free text query. Example: `"important instance"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`. This
    /// field is available only when the resource's Protobuf contains it. To search against the
    /// `location`:
    /// * Use a field query. Example: `location:us-west*`
    /// * Use a free text query. Example: `us-west*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// User labels associated with this resource. See [Labelling and grouping Google Cloud
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-
    /// google-cloud-platform-resources) for more information. This field is available only when
    /// the resource's Protobuf contains it. To search against the `labels`:
    /// * Use a field query:
    /// - query on any label's key or value. Example: `labels:prod`
    /// - query by a given label. Example: `labels.env:prod`
    /// - query by a given label's existence. Example: `labels.env:*`
    /// * Use a free text query. Example: `prod`
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Network tags associated with this resource. Like labels, network tags are a type of
    /// annotations used to group Google Cloud resources. See [Labelling Google Cloud
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-
    /// google-cloud-platform-resources) for more information. This field is available only when
    /// the resource's Protobuf contains it. To search against the `network_tags`:
    /// * Use a field query. Example: `networkTags:internal`
    /// * Use a free text query. Example: `internal`
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_tags: Vec<String>,

    /// The state of this resource. Different resources types have different state definitions
    /// that are mapped from various fields of different resource types. This field is available
    /// only when the resource's Protobuf contains it. Example: If the resource is an instance
    /// provided by Compute Engine, its state will include PROVISIONING, STAGING, RUNNING,
    /// STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. See `status` definition in
    /// [API Reference](https://cloud.google.com/compute/docs/reference/rest/v1/instances). If
    /// the resource is a project provided by Resource Manager, its state will include
    /// LIFECYCLE_STATE_UNSPECIFIED, ACTIVE, DELETE_REQUESTED and DELETE_IN_PROGRESS. See
    /// `lifecycleState` definition in [API Reference](https://cloud.google.com/resource-
    /// manager/reference/rest/v1/projects). To search against the `state`:
    /// * Use a field query. Example: `state:RUNNING`
    /// * Use a free text query. Example: `RUNNING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The create timestamp of this resource, at which the resource was created. The
    /// granularity is in seconds. Timestamp.nanos will always be 0. This field is available
    /// only when the resource's Protobuf contains it. To search against `create_time`:
    /// * Use a field query.
    /// - value in seconds since unix epoch. Example: `createTime > 1609459200`
    /// - value in date string. Example: `createTime > 2021-01-01`
    /// - value in date-time string (must be quoted). Example: `createTime >
    ///   "2021-01-01T00:00:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// The last update timestamp of this resource, at which the resource was last modified or
    /// deleted. The granularity is in seconds. Timestamp.nanos will always be 0. This field is
    /// available only when the resource's Protobuf contains it. To search against
    /// `update_time`:
    /// * Use a field query.
    /// - value in seconds since unix epoch. Example: `updateTime < 1609459200`
    /// - value in date string. Example: `updateTime < 2021-01-01`
    /// - value in date-time string (must be quoted). Example: `updateTime <
    ///   "2021-01-01T00:00:00"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// The full resource name of this resource's parent, if it has one. To search against the
    /// `parent_full_resource_name`:
    /// * Use a field query. Example: `parentFullResourceName:"project-name"`
    /// * Use a free text query. Example: `project-name`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_full_resource_name: Option<String>,

    /// The type of this resource's immediate parent, if there is one. To search against the
    /// `parent_asset_type`:
    /// * Use a field query. Example:
    ///   `parentAssetType:"cloudresourcemanager.googleapis.com/Project"`
    /// * Use a free text query. Example: `cloudresourcemanager.googleapis.com/Project`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_asset_type: Option<String>,
}

impl ResourceSearchResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-resource_search_result".into()),
            asset_type: Some("test-asset_type".into()),
            project: Some("test-project".into()),
            folders: vec![],
            organization: Some("test-organization".into()),
            display_name: Some("test-display_name".into()),
            description: Some("test-description".into()),
            location: Some("test-location".into()),
            labels: Default::default(),
            network_tags: vec![],
            state: Some("test-state".into()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
            parent_full_resource_name: Some("test-parent_full_resource_name".into()),
            parent_asset_type: Some("test-parent_asset_type".into()),
        }
    }
}

/// ListAssets response.
///
/// **GCP API**: `cloudasset.v1.ListAssetsResponse`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/ListAssetsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetsResponse {
    /// Assets.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<Asset>,

    /// Time the snapshot was taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_time: Option<String>,

    /// Token to retrieve the next page of results. It expires 72 hours after the page token for
    /// the first page is generated. Set to empty if there are no remaining results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListAssetsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            assets: vec![],
            read_time: Some("test-read_time".into()),
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Search all resources response.
///
/// **GCP API**: `cloudasset.v1.SearchAllResourcesResponse`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/SearchAllResourcesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAllResourcesResponse {
    /// A list of Resources that match the search query. It contains the resource standard
    /// metadata information.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<ResourceSearchResult>,

    /// If there are more results than those appearing in this response, then `next_page_token`
    /// is included. To get the next set of results, call this method again using the value of
    /// `next_page_token` as `page_token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SearchAllResourcesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            results: vec![],
            next_page_token: Some("test-next_page_token".into()),
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
/// **GCP API**: `cloudasset.v1.Policy`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/Policy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a
    /// `condition` that determines how and when the `bindings` are applied. Each of the
    /// `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer
    /// to up to 1,500 principals; up to 250 of these principals can be Google groups. Each
    /// occurrence of a principal counts towards these limits. For example, if the `bindings`
    /// grant 50 different roles to `user:alice@example.com`, and not to any other principal,
    /// then you can add another 1,450 principals to the `bindings` in the `Policy`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<Binding>,

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

    /// Specifies cloud audit logging configuration for this policy.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audit_configs: Vec<AuditConfig>,
}

impl Policy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bindings: vec![],
            etag: Some("test-etag".into()),
            version: Some(100),
            audit_configs: vec![],
        }
    }
}

/// Associates `members`, or principals, with a `role`.
///
/// **GCP API**: `cloudasset.v1.Binding`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/Binding>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
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
    pub condition: Option<Expr>,
}

impl Binding {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role: Some("test-role".into()),
            members: vec![],
            condition: Some(Expr::fixture()),
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
/// **GCP API**: `cloudasset.v1.Expr`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/Expr>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expr {
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

impl Expr {
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
/// **GCP API**: `cloudasset.v1.AuditConfig`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/AuditConfig>
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
/// **GCP API**: `cloudasset.v1.AuditLogConfig`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/AuditLogConfig>
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

/// A result of IAM Policy search, containing information of an IAM policy.
///
/// **GCP API**: `cloudasset.v1.IamPolicySearchResult`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/IamPolicySearchResult>
///
/// ## Coverage
/// 6 of 7 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamPolicySearchResult {
    /// The full resource name of the resource associated with this IAM policy. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See
    /// [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-
    /// inventory/docs/resource-name-format) for more information. To search against the
    /// `resource`:
    /// * use a field query. Example: `resource:organizations/123`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    /// The type of the resource associated with this IAM policy. Example:
    /// `compute.googleapis.com/Disk`. To search against the `asset_type`:
    /// * specify the `asset_types` field in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,

    /// The IAM policy directly set on the given resource. Note that the original IAM policy can
    /// contain multiple bindings. This only contains the bindings that match the given query.
    /// For queries that don't contain a constrain on policies (e.g., an empty query), this
    /// contains all the bindings. To search against the `policy` bindings:
    /// * use a field query:
    /// - query by the policy contained members. Example: `policy:amy@gmail.com`
    /// - query by the policy contained roles. Example: `policy:roles/compute.admin`
    /// - query by the policy contained roles' included permissions. Example:
    ///   `policy.role.permissions:compute.instances.create`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,

    /// The project that the associated Google Cloud resource belongs to, in the form of
    /// projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM instance,
    /// Cloud Storage bucket), the project field will indicate the project that contains the
    /// resource. If an IAM policy is set on a folder or organization, this field will be empty.
    /// To search against the `project`:
    /// * specify the `scope` field as this project in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    /// The folder(s) that the IAM policy belongs to, in the form of folders/{FOLDER_NUMBER}.
    /// This field is available when the IAM policy belongs to one or more folders. To search
    /// against `folders`:
    /// * use a field query. Example: `folders:(123 OR 456)`
    /// * use a free text query. Example: `123`
    /// * specify the `scope` field as this folder in your search request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,

    /// The organization that the IAM policy belongs to, in the form of
    /// organizations/{ORGANIZATION_NUMBER}. This field is available when the IAM policy belongs
    /// to an organization. To search against `organization`:
    /// * use a field query. Example: `organization:123`
    /// * use a free text query. Example: `123`
    /// * specify the `scope` field as this organization in your search request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

impl IamPolicySearchResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource: Some("test-resource".into()),
            asset_type: Some("test-asset_type".into()),
            policy: Some(Policy::fixture()),
            project: Some("test-project".into()),
            folders: vec![],
            organization: Some("test-organization".into()),
        }
    }
}

/// Search all IAM policies response.
///
/// **GCP API**: `cloudasset.v1.SearchAllIamPoliciesResponse`
/// **Reference**: <https://cloud.google.com/asset-inventory/docs/SearchAllIamPoliciesResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAllIamPoliciesResponse {
    /// A list of IAM policies that match the search query. Related information such as the
    /// associated resource is returned along with the policy.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<IamPolicySearchResult>,

    /// Set if there are more results than those appearing in this response; to get the next set
    /// of results, call this method again, using this value as the `page_token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SearchAllIamPoliciesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            results: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}
