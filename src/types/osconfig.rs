//! Types for the OS Config API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/osconfig/v1/rest`

use serde::{Deserialize, Serialize};

/// Operating system information for the VM.
///
/// **GCP API**: `osconfig.v1.InventoryOsInfo`
/// **Reference**: <https://cloud.google.com/compute/docs/osconfig/rest/InventoryOsInfo>
///
/// ## Coverage
/// 7 of 8 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryOsInfo {
    /// The VM hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// The operating system long name. For example 'Debian GNU/Linux 9' or 'Microsoft Window
    /// Server 2019 Datacenter'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,

    /// The operating system short name. For example, 'windows' or 'debian'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,

    /// The version of the operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The system architecture of the operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// The kernel version of the operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,

    /// The current version of the OS Config agent running on the VM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osconfig_agent_version: Option<String>,
}

impl InventoryOsInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            hostname: Some("test-hostname".into()),
            long_name: Some("test-long_name".into()),
            short_name: Some("test-short_name".into()),
            version: Some("test-version".into()),
            architecture: Some("test-architecture".into()),
            kernel_version: Some("test-kernel_version".into()),
            osconfig_agent_version: Some("test-osconfig_agent_version".into()),
        }
    }
}

/// This API resource represents the available inventory data for a Compute Engine virtual
/// machine (VM) instance at a given point in time. You can use this API resource to determine
/// the inventory data of your VM. For more information, see [Information provided by OS
/// inventory management](https://cloud.google.com/compute/docs/instances/os-inventory-
/// management#data-collected).
///
/// **GCP API**: `osconfig.v1.Inventory`
/// **Reference**: <https://cloud.google.com/compute/docs/osconfig/rest/Inventory>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    /// Output only. The `Inventory` API resource name. Format:
    /// `projects/{project_number}/locations/{location}/instances/{instance_id}/inventory`
    ///
    /// *Output-only field.*
    pub name: String,

    /// Base level operating system information for the VM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_info: Option<InventoryOsInfo>,

    /// Output only. Timestamp of the last reported inventory for the VM.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Inventory {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-inventory".into(),
            os_info: Some(InventoryOsInfo::fixture()),
            update_time: Some("test-update_time".into()),
        }
    }
}

/// Patch deployments are configurations that individual patch jobs use to complete a patch.
/// These configurations include instance filter, package repository settings, and a schedule.
/// For more information about creating and managing patch deployments, see [Scheduling patch
/// jobs](https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs).
///
/// **GCP API**: `osconfig.v1.PatchDeployment`
/// **Reference**: <https://cloud.google.com/compute/docs/osconfig/rest/PatchDeployment>
///
/// ## Coverage
/// 6 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchDeployment {
    /// Unique name for the patch deployment resource in a project. The patch deployment name is
    /// in the form: `projects/{project_id}/patchDeployments/{patch_deployment_id}`. This field
    /// is ignored when you create a new patch deployment.
    pub name: String,

    /// Optional. Description of the patch deployment. Length of the description is limited to
    /// 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output only. Current state of the patch deployment.
    ///
    /// **Possible values**:
    /// - `STATE_UNSPECIFIED` — The default value. This value is used if the state is omitted.
    /// - `ACTIVE` — Active value means that patch deployment generates Patch Jobs.
    /// - `PAUSED` — Paused value means that patch deployment does not generate Patch jobs. Requires ...
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Output only. Time the patch deployment was created. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// Output only. The last time a patch job was started by this deployment. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execute_time: Option<String>,
}

impl PatchDeployment {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-patch_deployment".into(),
            description: Some("test-description".into()),
            state: Some("test-state".into()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
            last_execute_time: Some("test-last_execute_time".into()),
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing PatchDeployment resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPatchDeploymentsResponse {
    /// A list of PatchDeployment resources.
    #[serde(default)]
    pub patch_deployments: Vec<PatchDeployment>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListPatchDeploymentsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            patch_deployments: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Inventory resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInventoriesResponse {
    /// A list of Inventory resources.
    #[serde(default)]
    pub inventories: Vec<Inventory>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListInventoriesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            inventories: vec![],
            next_page_token: None,
        }
    }
}
