//! Types for the Kubernetes Engine API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/container/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `container.v1.Cluster.status`.
///
/// **GCP API**: `container.v1.Cluster.status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClusterStatus {
    /// Not set.
    StatusUnspecified,

    /// The PROVISIONING state indicates the cluster is being created.
    Provisioning,

    /// The RUNNING state indicates the cluster has been created and is fully usable.
    Running,

    /// The RECONCILING state indicates that some work is actively being done on the cluster,
    /// such as upgrading the master or node software. Details can be found in the
    /// `statusMessage` field.
    Reconciling,

    /// The STOPPING state indicates the cluster is being deleted.
    Stopping,

    /// The ERROR state indicates the cluster is unusable. It will be automatically deleted.
    /// Details can be found in the `statusMessage` field.
    Error,

    /// The DEGRADED state indicates the cluster requires user action to restore full
    /// functionality. Details can be found in the `statusMessage` field.
    Degraded,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// A Google Kubernetes Engine cluster.
///
/// **GCP API**: `container.v1.Cluster`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs//Cluster>
///
/// ## Coverage
/// 15 of 80 fields included.
/// Omitted fields:
/// - `addonsConfig` — Complex nested config — not needed for basic cluster management
/// - `nodePools` — Complex nested type — use get for node pool details
/// - `nodeConfig` — Deprecated — use nodePools[].config instead
/// - `ipAllocationPolicy` — Advanced networking config
/// - `networkConfig` — Advanced networking config
/// - `privateClusterConfig` — Advanced networking config
/// - `masterAuthorizedNetworksConfig` — Advanced networking config
/// - `masterAuth` — Auth config — rarely needed at this level
/// - `loggingConfig` — Monitoring/logging config
/// - `monitoringConfig` — Monitoring/logging config
/// - `autoscaling` — Cluster-level autoscaling config
/// - `autopilot` — Autopilot mode config
/// - `workloadIdentityConfig` — Workload identity config
/// - `binaryAuthorization` — Security config
/// - `shieldedNodes` — Security config
/// - `securityPostureConfig` — Security config
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    /// The name of this cluster. The name must be unique within this project and location (e.g.
    /// zone or region), and can be up to 40 characters with the following restrictions:
    /// * Lowercase letters, numbers, and hyphens only.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    pub name: String,

    /// Output only. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or
    /// [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in
    /// which the cluster resides.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Output only. The current status of this cluster.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatus>,

    /// Output only. Deprecated. Use conditions instead. Additional information about the
    /// current status of this cluster, if available.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,

    /// Output only. The IP address of this cluster's master endpoint. The endpoint can be
    /// accessed from the internet at `https://username:password@endpoint/`. See the
    /// `masterAuth` property of this resource for username and password information.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// Output only. The current software version of the master endpoint.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_master_version: Option<String>,

    /// Output only. The number of nodes currently in the cluster. Deprecated. Call Kubernetes
    /// API directly to retrieve node information.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_node_count: Option<i32>,

    /// Output only. The time the cluster was created, in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// The name of the Google Compute Engine
    /// [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to
    /// which the cluster is connected. If left unspecified, the `default` network will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// The name of the Google Compute Engine
    /// [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is
    /// connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,

    /// Output only. Server-defined URL for the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// An optional description of this cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The resource labels for the cluster to use to annotate any related Google Compute Engine
    /// resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub resource_labels: HashMap<String, String>,

    /// The initial Kubernetes version for this cluster. Valid versions are those found in
    /// validMasterVersions returned by getServerConfig. The version can be upgraded over time;
    /// such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may
    /// specify either explicit versions offered by Kubernetes Engine or version aliases, which
    /// have the following behavior: - "latest": picks the highest valid Kubernetes version -
    /// "1.X": picks the highest valid patch+gke.N patch in the 1.X version - "1.X.Y": picks the
    /// highest valid gke.N patch in the 1.X.Y version - "1.X.Y-gke.N": picks an explicit
    /// Kubernetes version - "","-": picks the default Kubernetes version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_cluster_version: Option<String>,

    /// Release channel configuration. If left unspecified on cluster creation and a version is
    /// specified, the cluster is enrolled in the most mature release channel where the version
    /// is available (first checking STABLE, then REGULAR, and finally RAPID). Otherwise, if no
    /// release channel configuration and no version is specified, the cluster is enrolled in
    /// the REGULAR channel with its default version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_channel: Option<ReleaseChannel>,
}

impl Cluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-cluster".into(),
            location: Some("test-location".into()),
            status: Some(ClusterStatus::StatusUnspecified),
            status_message: Some("test-status_message".into()),
            endpoint: Some("test-endpoint".into()),
            current_master_version: Some("test-current_master_version".into()),
            current_node_count: Some(100),
            create_time: Some("test-create_time".into()),
            network: Some("test-network".into()),
            subnetwork: Some("test-subnetwork".into()),
            self_link: Some("test-self_link".into()),
            description: Some("test-description".into()),
            resource_labels: Default::default(),
            initial_cluster_version: Some("test-initial_cluster_version".into()),
            release_channel: Some(ReleaseChannel::fixture()),
        }
    }
}

/// ListClustersResponse is the result of ListClustersRequest.
///
/// **GCP API**: `container.v1.ListClustersResponse`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs//ListClustersResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClustersResponse {
    /// A list of clusters in the project in the specified zone, or across all ones.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clusters: Vec<Cluster>,

    /// If any zones are listed here, the list of clusters returned may be missing those zones.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub missing_zones: Vec<String>,
}

impl ListClustersResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            clusters: vec![],
            missing_zones: vec![],
        }
    }
}

/// This operation resource represents operations that may have happened or are happening on the
/// cluster. All fields are output only.
///
/// **GCP API**: `container.v1.Operation`
/// **Reference**: <https://cloud.google.com/kubernetes-engine/docs//Operation>
///
/// ## Coverage
/// 7 of 15 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerLro {
    /// Output only. The server-assigned ID for the operation.
    ///
    /// *Output-only field.*
    pub name: String,

    /// Output only. The current status of the operation.
    ///
    /// **Possible values**:
    /// - `STATUS_UNSPECIFIED` — Not set.
    /// - `PENDING` — The operation has been created.
    /// - `RUNNING` — The operation is currently running.
    /// - `DONE` — The operation is done, either cancelled or completed.
    /// - `ABORTING` — The operation is aborting.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Output only. If an error has occurred, a textual description of the error. Deprecated.
    /// Use the field error instead.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,

    /// The error result of the operation in case of failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,

    /// Output only. Server-defined URI for the operation. Example:
    /// `https://container.googleapis.com/v1alpha1/projects/123/locations/us-
    /// central1/operations/operation-123`.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. Server-defined URI for the target of the operation. The format of this is a
    /// URI to the resource being modified (such as a cluster, node pool, or node). For node
    /// pool repairs, there may be multiple nodes being repaired, but only one will be the
    /// target. Examples: - ## `https://container.googleapis.com/v1/projects/123/locations/us-
    /// central1/clusters/my-cluster` ##
    /// `https://container.googleapis.com/v1/projects/123/zones/us-central1-c/clusters/my-
    /// cluster/nodePools/my-np` `https://container.googleapis.com/v1/projects/123/zones/us-
    /// central1-c/clusters/my-cluster/nodePools/my-np/node/my-node`
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_link: Option<String>,

    /// Output only. The operation type.
    ///
    /// **Possible values**:
    /// - `TYPE_UNSPECIFIED` — Not set.
    /// - `CREATE_CLUSTER` — The cluster is being created. The cluster should be assumed to be unusable until...
    /// - `DELETE_CLUSTER` — The cluster is being deleted. The cluster should be assumed to be unusable as so...
    /// - `UPGRADE_MASTER` — The cluster version is being updated. Note that this includes "upgrades" to the ...
    /// - `UPGRADE_NODES` — A node pool is being updated. Despite calling this an "upgrade", this includes m...
    /// - `REPAIR_CLUSTER` — A problem has been detected with the control plane and is being repaired. This o...
    /// - `UPDATE_CLUSTER` — The cluster is being updated. This is a broad category of operations and include...
    /// - `CREATE_NODE_POOL` — A node pool is being created. The node pool should be assumed to be unusable unt...
    /// - `DELETE_NODE_POOL` — The node pool is being deleted. The node pool should be assumed to be unusable a...
    /// - `SET_NODE_POOL_MANAGEMENT` — The node pool's manamagent field is being updated. These operations only update ...
    /// - `AUTO_REPAIR_NODES` — A problem has been detected with nodes and [they are being repaired](https://clo...
    /// - `AUTO_UPGRADE_NODES` — Unused. Automatic node upgrade uses UPGRADE_NODES.
    /// - `SET_LABELS` — Unused. Updating labels uses UPDATE_CLUSTER.
    /// - `SET_MASTER_AUTH` — Unused. Updating master auth uses UPDATE_CLUSTER.
    /// - `SET_NODE_POOL_SIZE` — The node pool is being resized. With the exception of resizing to or from size z...
    /// - `SET_NETWORK_POLICY` — Unused. Updating network policy uses UPDATE_CLUSTER.
    /// - `SET_MAINTENANCE_POLICY` — Unused. Updating maintenance policy uses UPDATE_CLUSTER.
    /// - `RESIZE_CLUSTER` — The control plane is being resized. This operation type is initiated by GKE. The...
    /// - `FLEET_FEATURE_UPGRADE` — Fleet features of GKE Enterprise are being upgraded. The cluster should be assum...
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
}

impl ContainerLro {
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

// ======================================================================
// Auto-generated dependency types (referenced via $ref)
// ======================================================================

/// ReleaseChannel indicates which release channel a cluster is subscribed to. Release channels
/// are arranged in order of risk. When a cluster is subscribed to a release channel, Google
/// maintains both the master version and the node version. Node auto-upgrade defaults to true
/// and cannot be disabled.
///
/// **GCP API**: `container.v1.ReleaseChannel`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseChannel {
    /// channel specifies which release channel the cluster is subscribed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl ReleaseChannel {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            channel: Some("test-channel".into()),
        }
    }
}
