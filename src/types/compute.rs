//! Types for the Compute Engine API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `compute.v1.Disk.status`.
///
/// **GCP API**: `compute.v1.Disk.status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiskStatus {
    /// Disk is provisioning
    Creating,

    /// Disk is deleting.
    Deleting,

    /// Disk creation failed.
    Failed,

    /// Disk is ready for use.
    Ready,

    /// Source data is being copied into the disk.
    Restoring,

    /// Disk is currently unavailable and cannot be accessed, attached or detached.
    Unavailable,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `compute.v1.Instance.status`.
///
/// **GCP API**: `compute.v1.Instance.status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstanceStatus {
    /// The instance is halted and we are performing tear down tasks like network deprogramming,
    /// releasing quota, IP, tearing down disks etc.
    Deprovisioning,

    /// For Flex Start provisioning instance is waiting for available capacity from Dynamic
    /// Workload Scheduler (DWS).
    Pending,

    /// Resources are being allocated for the instance.
    Provisioning,

    /// The instance is in repair.
    Repairing,

    /// The instance is running.
    Running,

    /// All required resources have been allocated and the instance is being started.
    Staging,

    /// The instance has stopped successfully.
    Stopped,

    /// The instance is currently stopping (either being deleted or killed).
    Stopping,

    /// The instance has suspended.
    Suspended,

    /// The instance is suspending.
    Suspending,

    /// The instance has stopped (either by explicit action or underlying failure).
    Terminated,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `compute.v1.Snapshot.status`.
///
/// **GCP API**: `compute.v1.Snapshot.status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SnapshotStatus {
    /// Snapshot creation is in progress.
    Creating,

    /// Snapshot is currently being deleted.
    Deleting,

    /// Snapshot creation failed.
    Failed,

    /// Snapshot has been created successfully.
    Ready,

    /// Snapshot is being uploaded.
    Uploading,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `compute.v1.Address.status`.
///
/// **GCP API**: `compute.v1.Address.status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressStatus {
    /// Address is being used by another resource and is not available.
    InUse,

    /// Address is reserved and available to use.
    Reserved,

    /// Address is being reserved.
    Reserving,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `compute.v1.Firewall.direction`.
///
/// **GCP API**: `compute.v1.Firewall.direction`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FirewallDirection {
    /// Indicates that firewall should apply to outgoing traffic.
    Egress,

    /// Indicates that firewall should apply to incoming traffic.
    Ingress,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Represents a Persistent Disk resource. Google Compute Engine has two Disk resources:
/// * [Zonal](/compute/docs/reference/rest/v1/disks)
/// * [Regional](/compute/docs/reference/rest/v1/regionDisks) Persistent disks are required for
///   running your VM instances. Create both boot and non-boot (data) persistent disks. For more
///   information, read Persistent Disks. For more storage options, read Storage options. The
///   disks resource represents a zonal persistent disk. For more information, readZonal
///   persistent disks. The regionDisks resource represents a regional persistent disk. For more
///   information, read Regional resources.
///
/// **GCP API**: `compute.v1.Disk`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Disk>
///
/// ## Coverage
/// 25 of 50 fields included.
/// Omitted fields:
/// - `sourceDisk` — Source disk cloning - uncommon operation
/// - `sourceImageEncryptionKey` — CMEK for source image - niche use case
/// - `sourceSnapshotEncryptionKey` — CMEK for source snapshot - niche use case
/// - `sourceStorageObject` — Import from Cloud Storage - niche use case
/// - `licenseCodes` — License tracking - read-only metadata
/// - `licenses` — License tracking - read-only metadata
/// - `satisfiesPzs` — Placement zone storage - enterprise feature
/// - `satisfiesPzi` — Placement zone isolation - enterprise feature
/// - `provisionedThroughput` — Hyperdisk throughput provisioning
/// - `resourceStatus` — Async replication status - niche feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    /// Name of the resource. Provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which
    /// means the first character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which cannot be a
    /// dash.
    pub name: String,

    /// Size, in GB, of the persistent disk. You can specify this field when creating a
    /// persistent disk using thesourceImage, sourceSnapshot, orsourceDisk parameter, or specify
    /// it alone to create an empty persistent disk. If you specify this field along with a
    /// source, the value ofsizeGb must not be less than the size of the source. Acceptable
    /// values are greater than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<String>,

    /// URL of the disk type resource describing which disk type to use to create the disk.
    /// Provide this when creating the disk. For
    /// example:projects/project/zones/zone/diskTypes/pd-ssd. See Persistent disk types.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,

    /// Output only. [Output Only] URL of the zone where the disk resides. You must specify this
    /// field as part of the HTTP request URL. It is not settable as a field in the request
    /// body.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,

    /// Output only. [Output Only] The status of disk creation.
    /// - CREATING: Disk is provisioning.
    /// - RESTORING: Source data is being copied into the disk.
    /// - FAILED: Disk creation failed.
    /// - READY: Disk is ready for use.
    /// - DELETING: Disk is deleting.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DiskStatus>,

    /// Output only. [Output Only] Server-defined fully-qualified URL for this resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// The source snapshot used to create this disk. You can provide this as a partial or full
    /// URL to the resource. For example, the following are valid values:
    /// - https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot
    /// - projects/project/global/snapshots/snapshot
    /// - global/snapshots/snapshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot: Option<String>,

    /// The source image used to create this disk. If the source image is deleted, this field
    /// will not be set. To create a disk with one of the public operating system images,
    /// specify the image by its family name. For example, specifyfamily/debian-9 to use the
    /// latest Debian 9 image: projects/debian-cloud/global/images/family/debian-9
    /// Alternatively, use a specific version of a public operating system image:
    /// projects/debian-cloud/global/images/debian-9-stretch-vYYYYMMDD To create a disk with a
    /// custom image that you created, specify the image name in the following format:
    /// global/images/my-custom-image You can also specify a custom image by its image family,
    /// which returns the latest version of the image in that family. Replace the image name
    /// with family/family-name: global/images/family/my-image-family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Labels to apply to this disk. These can be later modified by the setLabels method.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// A fingerprint for the labels being applied to this disk, which is essentially a hash of
    /// the labels set used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update labels. You must
    /// always provide an up-to-date fingerprint hash in order to update or change labels,
    /// otherwise the request will fail with error412 conditionNotMet. To see the latest
    /// fingerprint, make a get() request to retrieve a disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,

    /// Physical block size of the persistent disk, in bytes. If not present in a request, a
    /// default value is used. The currently supported size is 4096, other sizes may be added in
    /// the future. If an unsupported value is requested, the error message will list the
    /// supported values for the caller's project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_block_size_bytes: Option<String>,

    /// Output only. [Output Only] Links to the users of the disk (attached instances) in
    /// form:projects/project/zones/zone/instances/instance
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,

    /// Output only. [Output Only] Last attach timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attach_timestamp: Option<String>,

    /// Output only. [Output Only] Last detach timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_detach_timestamp: Option<String>,

    /// Whether this disk is using confidential compute mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_confidential_compute: Option<bool>,

    /// Indicates how many IOPS to provision for the disk. This sets the number of I/O
    /// operations per second that the disk can handle. Values must be between 10,000 and
    /// 120,000. For more details, see theExtreme persistent disk documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<String>,

    /// Encrypts the disk using a customer-supplied encryption key or a customer-managed
    /// encryption key. Encryption keys do not protect access to metadata of the disk. After you
    /// encrypt a disk with a customer-supplied key, you must provide the same key if you use
    /// the disk later. For example, to create a disk snapshot, to create a disk image, to
    /// create a machine image, or to attach the disk to a virtual machine. After you encrypt a
    /// disk with a customer-managed key, thediskEncryptionKey.kmsKeyName is set to a key
    /// *version* name once the disk is created. The disk is encrypted with this version of the
    /// key. In the response, diskEncryptionKey.kmsKeyName appears in the following format:
    /// "diskEncryptionKey.kmsKeyName": "projects/kms_project_id/locations/region/keyRings/
    /// key_region/cryptoKeys/key /cryptoKeysVersions/version If you do not provide an
    /// encryption key when creating the disk, then the disk is encrypted using an automatically
    /// generated key and you don't need to provide a key to use the disk later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<DiskEncryptionKey>,

    /// A list of features to enable on the guest operating system. Applicable only for bootable
    /// images. Read Enabling guest operating system features to see a list of available
    /// options.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guest_os_features: Vec<GuestOsFeature>,

    /// Resource policies applied to this disk for automatic snapshot creations.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_policies: Vec<String>,

    /// Output only. [Output Only] URL of the region where the disk resides. Only applicable for
    /// regional resources. You must specify this field as part of the HTTP request URL. It is
    /// not settable as a field in the request body.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// URLs of the zones where the disk should be replicated to. Only applicable for regional
    /// resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub replica_zones: Vec<String>,

    /// Output only. [Output Only] Type of the resource. Always compute#disk for disks.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl Disk {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-disk".into(),
            size_gb: Some("test-size_gb".into()),
            disk_type: Some("test-type".into()),
            zone: Some("test-zone".into()),
            status: Some(DiskStatus::Creating),
            self_link: Some("test-self_link".into()),
            source_snapshot: Some("test-source_snapshot".into()),
            source_image: Some("test-source_image".into()),
            id: Some("test-id".into()),
            creation_timestamp: Some("test-creation_timestamp".into()),
            description: Some("test-description".into()),
            labels: Default::default(),
            label_fingerprint: Some("test-label_fingerprint".into()),
            physical_block_size_bytes: Some("test-physical_block_size_bytes".into()),
            users: vec![],
            last_attach_timestamp: Some("test-last_attach_timestamp".into()),
            last_detach_timestamp: Some("test-last_detach_timestamp".into()),
            enable_confidential_compute: Some(false),
            provisioned_iops: Some("test-provisioned_iops".into()),
            disk_encryption_key: Some(DiskEncryptionKey::fixture()),
            guest_os_features: vec![],
            resource_policies: vec![],
            region: Some("test-region".into()),
            replica_zones: vec![],
            kind: Some("test-kind".into()),
        }
    }
}

///
/// **GCP API**: `compute.v1.CustomerEncryptionKey`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/CustomerEncryptionKey>
///
/// ## Coverage
/// 3 of 5 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskEncryptionKey {
    /// Specifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to
    /// either encrypt or decrypt this resource. You can provide either the rawKey or
    /// thersaEncryptedKey. For example: "rawKey":
    /// "SGVsbG8gZnJvbSBHb29nbGUgQ2xvdWQgUGxhdGZvcm0="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key: Option<String>,

    /// The name of the encryption key that is stored in Google Cloud KMS. For example:
    /// "kmsKeyName": "projects/kms_project_id/locations/region/keyRings/
    /// key_region/cryptoKeys/key The fully-qualifed key name may be returned for resource GET
    /// requests. For example: "kmsKeyName": "projects/kms_project_id/locations/region/keyRings/
    /// key_region/cryptoKeys/key /cryptoKeyVersions/1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,

    /// [Output only] TheRFC 4648 base64 encoded SHA-256 hash of the customer-supplied
    /// encryption key that protects this resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

impl DiskEncryptionKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            raw_key: Some("test-raw_key".into()),
            kms_key_name: Some("test-kms_key_name".into()),
            sha256: Some("test-sha256".into()),
        }
    }
}

/// Guest OS features.
///
/// **GCP API**: `compute.v1.GuestOsFeature`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/GuestOsFeature>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestOsFeature {
    /// The ID of a supported feature. To add multiple values, use commas to separate values.
    /// Set to one or more of the following values:
    /// - VIRTIO_SCSI_MULTIQUEUE
    /// - WINDOWS
    /// - MULTI_IP_SUBNET
    /// - UEFI_COMPATIBLE
    /// - GVNIC
    /// - SEV_CAPABLE
    /// - SUSPEND_RESUME_COMPATIBLE
    /// - SEV_LIVE_MIGRATABLE_V2
    /// - SEV_SNP_CAPABLE
    /// - TDX_CAPABLE
    /// - IDPF
    /// - SNP_SVSM_CAPABLE For more information, see Enabling guest operating system features.
    ///
    /// **Possible values**:
    /// - `BARE_METAL_LINUX_COMPATIBLE`
    /// - `FEATURE_TYPE_UNSPECIFIED`
    /// - `GVNIC`
    /// - `IDPF`
    /// - `MULTI_IP_SUBNET`
    /// - `SECURE_BOOT`
    /// - `SEV_CAPABLE`
    /// - `SEV_LIVE_MIGRATABLE`
    /// - `SEV_LIVE_MIGRATABLE_V2`
    /// - `SEV_SNP_CAPABLE`
    /// - `SNP_SVSM_CAPABLE`
    /// - `TDX_CAPABLE`
    /// - `UEFI_COMPATIBLE`
    /// - `VIRTIO_SCSI_MULTIQUEUE`
    /// - `WINDOWS`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
}

impl GuestOsFeature {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            feature_type: Some("test-type".into()),
        }
    }
}

/// Represents an Instance resource. An instance is a virtual machine that is hosted on Google
/// Cloud Platform. For more information, readVirtual Machine Instances.
///
/// **GCP API**: `compute.v1.Instance`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Instance>
///
/// ## Coverage
/// 32 of 46 fields included.
/// Omitted fields:
/// - `advancedMachineFeatures` — Advanced CPU/NUMA features
/// - `guestAccelerators` — GPU attachment - use dedicated GPU API
/// - `params` — Resource manager tags - niche
/// - `resourcePolicies` — Scheduled operations - niche
/// - `resourceStatus` — Scheduling/physical host status
/// - `sourceMachineImageEncryptionKey` — CMEK for machine images
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    /// The name of the resource, provided by the client when initially creating the resource.
    /// The resource name must be 1-63 characters long, and comply withRFC1035. Specifically,
    /// the name must be 1-63 characters long and match the regular expression
    /// `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter,
    /// and all following characters must be a dash, lowercase letter, or digit, except the last
    /// character, which cannot be a dash.
    pub name: String,

    /// Allows this instance to send and receive packets with non-matching destination or source
    /// IPs. This is required if you plan to use this instance to forward routes. For more
    /// information, seeEnabling IP Forwarding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_ip_forward: Option<bool>,

    /// The `confidentialInstanceConfig` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_instance_config: Option<ConfidentialInstanceConfig>,

    /// Output only. [Output Only] The CPU platform used by this instance.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_platform: Option<String>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// Whether the resource should be protected against deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Array of disks associated with this instance. Persistent disks must be created before
    /// you can assign them.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<AttachedDisk>,

    /// Enables display device for the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_device: Option<DisplayDevice>,

    /// Specifies a fingerprint for this resource, which is essentially a hash of the instance's
    /// contents and used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update the instance. You
    /// must always provide an up-to-date fingerprint hash in order to update the instance. To
    /// see the latest fingerprint, make get() request to the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// KeyRevocationActionType of the instance. Supported options are "STOP" and "NONE". The
    /// default value is "NONE" if it is not specified.
    ///
    /// **Possible values**:
    /// - `KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED` — Default value. This value is unused.
    /// - `NONE` — Indicates user chose no operation.
    /// - `STOP` — Indicates user chose to opt for VM shutdown on key revocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_revocation_action_type: Option<String>,

    /// Output only. [Output Only] Type of the resource. Always compute#instance for instances.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A fingerprint for this request, which is essentially a hash of the label's contents and
    /// used for optimistic locking. The fingerprint is initially generated by Compute Engine
    /// and changes after every request to modify or update labels. You must always provide an
    /// up-to-date fingerprint hash in order to update or change labels. To see the latest
    /// fingerprint, make get() request to the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,

    /// Labels to apply to this instance. These can be later modified by the setLabels method.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. [Output Only] Last start timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_timestamp: Option<String>,

    /// Output only. [Output Only] Last stop timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_timestamp: Option<String>,

    /// Full or partial URL of the machine type resource to use for this instance, in the
    /// format:zones/zone/machineTypes/machine-type. This is provided by the client when the
    /// instance is created. For example, the following is a valid partial url to a predefined
    /// machine type: zones/us-central1-f/machineTypes/n1-standard-1 To create acustom machine
    /// type, provide a URL to a machine type in the following format, where CPUS is 1 or an
    /// even number up to 32 (2, 4, 6, ... 24, etc), and MEMORY is the total memory for this
    /// instance. Memory must be a multiple of 256 MB and must be supplied in MB (e.g. 5 GB of
    /// memory is 5120 MB): zones/zone/machineTypes/custom-CPUS-MEMORY For example: zones/us-
    /// central1-f/machineTypes/custom-4-5120 For a full list of restrictions, read
    /// theSpecifications for custom machine types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,

    /// The metadata key/value pairs assigned to this instance. This includes metadata keys that
    /// were explicitly defined for the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// An array of network configurations for this instance. These specify how interfaces are
    /// configured to interact with other network services, such as connecting to the internet.
    /// Multiple interfaces are supported per instance.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,

    /// Specifies the reservations that this instance can consume from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_affinity: Option<ReservationAffinity>,

    /// Output only. [Output Only] Reserved for future use.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfies_pzi: Option<bool>,

    /// Sets the scheduling options for this instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,

    /// Output only. [Output Only] Server-defined URL for this resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// A list of service accounts, with their specified scopes, authorized for this instance.
    /// Only one service account per VM instance is supported. Service accounts generate access
    /// tokens that can be accessed through the metadata server and used to authenticate
    /// applications on the instance. SeeService Accounts for more information.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_accounts: Vec<ServiceAccount>,

    /// The `shieldedInstanceConfig` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,

    /// The `shieldedInstanceIntegrityPolicy` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shielded_instance_integrity_policy: Option<ShieldedInstanceIntegrityPolicy>,

    /// Source machine image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_machine_image: Option<String>,

    /// Output only. [Output Only] Whether a VM has been restricted for start because Compute
    /// Engine has detected suspicious activity.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_restricted: Option<bool>,

    /// Output only. [Output Only] The status of the instance. One of the following values:
    /// PROVISIONING, STAGING,RUNNING, STOPPING, SUSPENDING,SUSPENDED, REPAIRING, andTERMINATED.
    /// For more information about the status of the instance, see Instance life cycle.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceStatus>,

    /// Tags to apply to this instance. Tags are used to identify valid sources or targets for
    /// network firewalls and are specified by the client during instance creation. The tags can
    /// be later modified by the setTags method. Each tag within the list must comply
    /// withRFC1035. Multiple tags can be specified via the 'tags.items' field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    /// Output only. [Output Only] URL of the zone where the instance resides. You must specify
    /// this field as part of the HTTP request URL. It is not settable as a field in the request
    /// body.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl Instance {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-instance".into(),
            can_ip_forward: Some(false),
            confidential_instance_config: Some(ConfidentialInstanceConfig::fixture()),
            cpu_platform: Some("test-cpu_platform".into()),
            creation_timestamp: Some("test-creation_timestamp".into()),
            deletion_protection: Some(false),
            description: Some("test-description".into()),
            disks: vec![],
            display_device: Some(DisplayDevice::fixture()),
            fingerprint: Some("test-fingerprint".into()),
            id: Some("test-id".into()),
            key_revocation_action_type: Some("test-key_revocation_action_type".into()),
            kind: Some("test-kind".into()),
            label_fingerprint: Some("test-label_fingerprint".into()),
            labels: Default::default(),
            last_start_timestamp: Some("test-last_start_timestamp".into()),
            last_stop_timestamp: Some("test-last_stop_timestamp".into()),
            machine_type: Some("test-machine_type".into()),
            metadata: Some(Metadata::fixture()),
            network_interfaces: vec![],
            reservation_affinity: Some(ReservationAffinity::fixture()),
            satisfies_pzi: Some(false),
            scheduling: Some(Scheduling::fixture()),
            self_link: Some("test-self_link".into()),
            service_accounts: vec![],
            shielded_instance_config: Some(ShieldedInstanceConfig::fixture()),
            shielded_instance_integrity_policy: Some(ShieldedInstanceIntegrityPolicy::fixture()),
            source_machine_image: Some("test-source_machine_image".into()),
            start_restricted: Some(false),
            status: Some(InstanceStatus::Deprovisioning),
            tags: Some(Tags::fixture()),
            zone: Some("test-zone".into()),
        }
    }
}

/// An instance-attached disk resource.
///
/// **GCP API**: `compute.v1.AttachedDisk`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/AttachedDisk>
///
/// ## Coverage
/// 13 of 18 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedDisk {
    /// Output only. [Output Only] The architecture of the attached disk. Valid values are ARM64
    /// or X86_64.
    ///
    /// **Possible values**:
    /// - `ARCHITECTURE_UNSPECIFIED` — Default value indicating Architecture is not set.
    /// - `ARM64` — Machines with architecture ARM64
    /// - `X86_64` — Machines with architecture X86_64
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Specifies whether the disk will be auto-deleted when the instance is deleted (but not
    /// when the disk is detached from the instance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,

    /// Indicates that this is a boot disk. The virtual machine will use the first partition of
    /// the disk for its root filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,

    /// Specifies a unique device name of your choice that is reflected into the/dev/disk/by-
    /// id/google-* tree of a Linux operating system running within the instance. This name can
    /// be used to reference the device for mounting, resizing, and so on, from within the
    /// instance. If not specified, the server chooses a default device name to apply to this
    /// disk, in the form persistent-disk-x, where x is a number assigned by Google Compute
    /// Engine. This field is only applicable for persistent disks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// The size of the disk in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,

    /// A list of features to enable on the guest operating system. Applicable only for bootable
    /// images. Read Enabling guest operating system features to see a list of available
    /// options.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guest_os_features: Vec<GuestOsFeature>,

    /// Output only. [Output Only] A zero-based index to this disk, where 0 is reserved for the
    /// boot disk. If you have many disks attached to an instance, each disk would have a unique
    /// index number.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,

    /// Specifies the disk interface to use for attaching this disk, which is either SCSI or
    /// NVME. For most machine types, the default is SCSI. Local SSDs can use either NVME or
    /// SCSI. In certain configurations, persistent disks can use NVMe. For more information,
    /// seeAbout persistent disks.
    ///
    /// **Possible values**:
    /// - `NVME`
    /// - `SCSI`
    #[serde(rename = "interface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,

    /// Output only. [Output Only] Type of the resource. Alwayscompute#attachedDisk for attached
    /// disks.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Output only. [Output Only] Any valid publicly visible licenses.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub licenses: Vec<String>,

    /// The mode in which to attach this disk, either READ_WRITE orREAD_ONLY. If not specified,
    /// the default is to attach the disk in READ_WRITE mode.
    ///
    /// **Possible values**:
    /// - `READ_ONLY` — Attaches this disk in read-only mode. Multiple virtual machines can use a disk i...
    /// - `READ_WRITE` — *[Default]* Attaches this disk in read-write mode. Only one virtual machine at a...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Specifies a valid partial or full URL to an existing Persistent Disk resource. When
    /// creating a new instance boot disk, one ofinitializeParams.sourceImage
    /// orinitializeParams.sourceSnapshot or disks.source is required. If desired, you can also
    /// attach existing non-root persistent disks using this property. This field is only
    /// applicable for persistent disks. Note that for InstanceTemplate, specify the disk name
    /// for zonal disk, and the URL for regional disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Specifies the type of the disk, either SCRATCH orPERSISTENT. If not specified, the
    /// default isPERSISTENT.
    ///
    /// **Possible values**:
    /// - `PERSISTENT`
    /// - `SCRATCH`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}

impl AttachedDisk {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            architecture: Some("test-architecture".into()),
            auto_delete: Some(false),
            boot: Some(false),
            device_name: Some("test-device_name".into()),
            disk_size_gb: Some("test-disk_size_gb".into()),
            guest_os_features: vec![],
            index: Some(100),
            interface_type: Some("test-interface".into()),
            kind: Some("test-kind".into()),
            licenses: vec![],
            mode: Some("test-mode".into()),
            source: Some("test-source".into()),
            disk_type: Some("test-type".into()),
        }
    }
}

/// A network interface resource attached to an instance.
///
/// **GCP API**: `compute.v1.NetworkInterface`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/NetworkInterface>
///
/// ## Coverage
/// 8 of 19 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    /// An array of configurations for this interface. Currently, only one access config,
    /// ONE_TO_ONE_NAT, is supported. If there are noaccessConfigs specified, then this instance
    /// will have no external internet access.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_configs: Vec<AccessConfig>,

    /// Fingerprint hash of contents stored in this network interface. This field will be
    /// ignored when inserting an Instance or adding a NetworkInterface. An up-to-date
    /// fingerprint must be provided in order to update theNetworkInterface. The request will
    /// fail with error400 Bad Request if the fingerprint is not provided, or412 Precondition
    /// Failed if the fingerprint is out of date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Output only. [Output Only] Type of the resource. Alwayscompute#networkInterface for
    /// network interfaces.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// [Output Only] The name of the network interface, which is generated by the server. For a
    /// VM, the network interface uses the nicN naming format. Where N is a value between 0
    /// and7. The default interface value is nic0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// URL of the VPC network resource for this instance. When creating an instance, if neither
    /// the network nor the subnetwork is specified, the default network global/networks/default
    /// is used. If the selected project doesn't have the default network, you must specify a
    /// network or subnet. If the network is not specified but the subnetwork is specified, the
    /// network is inferred. If you specify this property, you can specify the network as a full
    /// or partial URL. For example, the following are all valid URLs:
    /// - https://www.googleapis.com/compute/v1/projects/project/global/networks/network
    /// - projects/project/global/networks/network
    /// - global/networks/default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// An IPv4 internal IP address to assign to the instance for this network interface. If not
    /// specified by the user, an unused internal IP is assigned by the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_ip: Option<String>,

    /// The stack type for this network interface. To assign only IPv4 addresses, use IPV4_ONLY.
    /// To assign both IPv4 and IPv6 addresses, useIPV4_IPV6. If not specified, IPV4_ONLY is
    /// used. This field can be both set at instance creation and update network interface
    /// operations.
    ///
    /// **Possible values**:
    /// - `IPV4_IPV6` — The network interface can have both IPv4 and IPv6 addresses.
    /// - `IPV4_ONLY` — The network interface will only be assigned IPv4 addresses.
    /// - `IPV6_ONLY` — The network interface will only be assigned IPv6 addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,

    /// The URL of the Subnetwork resource for this instance. If the network resource is
    /// inlegacy mode, do not specify this field. If the network is in auto subnet mode,
    /// specifying the subnetwork is optional. If the network is in custom subnet mode,
    /// specifying the subnetwork is required. If you specify this field, you can specify the
    /// subnetwork as a full or partial URL. For example, the following are all valid URLs:
    /// - https://www.googleapis.com/compute/v1/projects/project/regions/region/subnetworks/subn
    ///   etwork
    /// - regions/region/subnetworks/subnetwork
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,
}

impl NetworkInterface {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access_configs: vec![],
            fingerprint: Some("test-fingerprint".into()),
            kind: Some("test-kind".into()),
            name: Some("test-network_interface".into()),
            network: Some("test-network".into()),
            network_ip: Some("test-network_ip".into()),
            stack_type: Some("test-stack_type".into()),
            subnetwork: Some("test-subnetwork".into()),
        }
    }
}

/// An access configuration attached to an instance's network interface. Only one access config
/// per instance is supported.
///
/// **GCP API**: `compute.v1.AccessConfig`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/AccessConfig>
///
/// ## Coverage
/// 5 of 10 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessConfig {
    /// Output only. [Output Only] Type of the resource. Alwayscompute#accessConfig for access
    /// configs.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The name of this access configuration. In accessConfigs (IPv4), the default and
    /// recommended name is External NAT, but you can use any arbitrary string, such as My
    /// external IP orNetwork Access. In ipv6AccessConfigs, the recommend name is External IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Applies to accessConfigs (IPv4) only. Anexternal IP address associated with this
    /// instance. Specify an unused static external IP address available to the project or leave
    /// this field undefined to use an IP from a shared ephemeral IP address pool. If you
    /// specify a static external IP address, it must live in the same region as the zone of the
    /// instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_ip: Option<String>,

    /// This signifies the networking tier used for configuring this access configuration and
    /// can only take the following values: PREMIUM,STANDARD. If an AccessConfig is specified
    /// without a valid external IP address, an ephemeral IP will be created with this
    /// networkTier. If an AccessConfig with a valid external IP address is specified, it must
    /// match that of the networkTier associated with the Address resource owning that IP.
    ///
    /// **Possible values**:
    /// - `FIXED_STANDARD` — Public internet quality with fixed bandwidth.
    /// - `PREMIUM` — High quality, Google-grade network tier, support for all networking products.
    /// - `STANDARD` — Public internet quality, only limited support for other networking products.
    /// - `STANDARD_OVERRIDES_FIXED_STANDARD` — (Output only) Temporary tier for FIXED_STANDARD when fixed standard tier is expi...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_tier: Option<String>,

    /// The type of configuration. In accessConfigs (IPv4), the default and only option is
    /// ONE_TO_ONE_NAT. Inipv6AccessConfigs, the default and only option isDIRECT_IPV6.
    ///
    /// **Possible values**:
    /// - `DIRECT_IPV6`
    /// - `ONE_TO_ONE_NAT`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
}

impl AccessConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            name: Some("test-access_config".into()),
            nat_ip: Some("test-nat_ip".into()),
            network_tier: Some("test-network_tier".into()),
            access_type: Some("test-type".into()),
        }
    }
}

/// A metadata key/value entry.
///
/// **GCP API**: `compute.v1.Metadata`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Metadata>
///
/// ## Coverage
/// 2 of 3 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Specifies a fingerprint for this request, which is essentially a hash of the metadata's
    /// contents and used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update metadata. You must
    /// always provide an up-to-date fingerprint hash in order to update or change metadata,
    /// otherwise the request will fail with error412 conditionNotMet. To see the latest
    /// fingerprint, make a get() request to retrieve the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Array of key/value pairs. The total size of all keys and values must be less than 512
    /// KB.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetadataItem>,
}

impl Metadata {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            fingerprint: Some("test-fingerprint".into()),
            items: vec![],
        }
    }
}

/// A set of instance tags.
///
/// **GCP API**: `compute.v1.Tags`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Tags>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    /// An array of tags. Each tag must be 1-63 characters long, and comply with RFC1035.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,

    /// Specifies a fingerprint for this request, which is essentially a hash of the tags'
    /// contents and used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update tags. You must always
    /// provide an up-to-date fingerprint hash in order to update or change tags. To see the
    /// latest fingerprint, make get() request to the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

impl Tags {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            fingerprint: Some("test-fingerprint".into()),
        }
    }
}

/// A service account.
///
/// **GCP API**: `compute.v1.ServiceAccount`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/ServiceAccount>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    /// Email address of the service account.
    pub email: String,

    /// The list of scopes to be made available for this service account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

impl ServiceAccount {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            email: "test-email".into(),
            scopes: vec![],
        }
    }
}

/// Sets the scheduling options for an Instance.
///
/// **GCP API**: `compute.v1.Scheduling`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Scheduling>
///
/// ## Coverage
/// 5 of 15 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheduling {
    /// Specifies whether the instance should be automatically restarted if it is terminated by
    /// Compute Engine (not terminated by a user). You can only set the automatic restart option
    /// for standard instances.Preemptible instances cannot be automatically restarted. By
    /// default, this is set to true so an instance is automatically restarted if it is
    /// terminated by Compute Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_restart: Option<bool>,

    /// Specifies the termination action for the instance.
    ///
    /// **Possible values**:
    /// - `DELETE` — Delete the VM.
    /// - `INSTANCE_TERMINATION_ACTION_UNSPECIFIED` — Default value. This value is unused.
    /// - `STOP` — Stop the VM without storing in-memory content. default action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_termination_action: Option<String>,

    /// Defines the maintenance behavior for this instance. For standard instances, the default
    /// behavior is MIGRATE. Forpreemptible instances, the default and only possible behavior is
    /// TERMINATE. For more information, see Set VM host maintenance policy.
    ///
    /// **Possible values**:
    /// - `MIGRATE` — *[Default]* Allows Compute Engine to automatically migrate instances out of the ...
    /// - `TERMINATE` — Tells Compute Engine to terminate and (optionally) restart the instance away fro...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_host_maintenance: Option<String>,

    /// Defines whether the instance is preemptible. This can only be set during instance
    /// creation or while the instance isstopped and therefore, in a `TERMINATED` state.
    /// SeeInstance Life Cycle for more information on the possible instance states.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemptible: Option<bool>,

    /// Specifies the provisioning model of the instance.
    ///
    /// **Possible values**:
    /// - `FLEX_START` — Instance is provisioned using the Flex Start provisioning model and has a limite...
    /// - `RESERVATION_BOUND` — Bound to the lifecycle of the reservation in which it is provisioned.
    /// - `SPOT` — Heavily discounted, no guaranteed runtime.
    /// - `STANDARD` — Standard provisioning with user controlled runtime, no discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_model: Option<String>,
}

impl Scheduling {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            automatic_restart: Some(false),
            instance_termination_action: Some("test-instance_termination_action".into()),
            on_host_maintenance: Some("test-on_host_maintenance".into()),
            preemptible: Some(false),
            provisioning_model: Some("test-provisioning_model".into()),
        }
    }
}

/// Represents a Persistent Disk Snapshot resource. You can use snapshots to back up data on a
/// regular interval. For more information, read Creating persistent disk snapshots.
///
/// **GCP API**: `compute.v1.Snapshot`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Snapshot>
///
/// ## Coverage
/// 26 of 38 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    /// Name of the resource; provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which
    /// means the first character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which cannot be a
    /// dash.
    pub name: String,

    /// Output only. [Output Only] Set to true if snapshots are automatically created by
    /// applying resource policy on the target disk.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_created: Option<bool>,

    /// Creates the new snapshot in the snapshot chain labeled with the specified name. The
    /// chain name must be 1-63 characters long and comply with RFC1035. This is an uncommon
    /// option only for advanced service owners who needs to create separate snapshot chains,
    /// for example, for chargeback tracking. When you describe your snapshot resource, this
    /// field is visible only if it has a non-empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,

    /// Output only. [Output Only] Size in bytes of the snapshot at creation time.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_size_bytes: Option<String>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output only. [Output Only] Size of the source disk, specified in GB.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,

    /// Output only. [Output Only] Number of bytes downloaded to restore a snapshot to a disk.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_bytes: Option<String>,

    /// Output only. Whether this snapshot is created from a confidential compute mode disk.
    /// [Output Only]: This field is not set by user, but from source disk.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_confidential_compute: Option<bool>,

    /// Output only. [Output Only] A list of features to enable on the guest operating system.
    /// Applicable only for bootable images. Read Enabling guest operating system features to
    /// see a list of available options.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub guest_os_features: Vec<GuestOsFeature>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Type of the resource. Always compute#snapshot for Snapshot
    /// resources.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A fingerprint for the labels being applied to this snapshot, which is essentially a hash
    /// of the labels set used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update labels. You must
    /// always provide an up-to-date fingerprint hash in order to update or change labels,
    /// otherwise the request will fail with error412 conditionNotMet. To see the latest
    /// fingerprint, make a get() request to retrieve a snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,

    /// Labels to apply to this snapshot. These can be later modified by the setLabels method.
    /// Label values may be empty.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. [Output Only] Integer license codes indicating which licenses are attached
    /// to this snapshot.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub license_codes: Vec<String>,

    /// Output only. [Output Only] A list of public visible licenses that apply to this
    /// snapshot. This can be because the original image had licenses attached (such as a
    /// Windows image).
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub licenses: Vec<String>,

    /// Output only. [Output Only] Server-defined URL for the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Encrypts the snapshot using acustomer-supplied encryption key. After you encrypt a
    /// snapshot using a customer-supplied key, you must provide the same key if you use the
    /// snapshot later. For example, you must provide the encryption key when you create a disk
    /// from the encrypted snapshot in a future request. Customer-supplied encryption keys do
    /// not protect access to metadata of the snapshot. If you do not provide an encryption key
    /// when creating the snapshot, then the snapshot will be encrypted using an automatically
    /// generated key and you do not need to provide a key to use the snapshot later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_encryption_key: Option<DiskEncryptionKey>,

    /// Indicates the type of the snapshot.
    ///
    /// **Possible values**:
    /// - `ARCHIVE`
    /// - `STANDARD`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,

    /// The source disk used to create this snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_disk: Option<String>,

    /// The customer-supplied encryption key of the source disk. Required if the source disk is
    /// protected by a customer-supplied encryption key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_disk_encryption_key: Option<DiskEncryptionKey>,

    /// Output only. [Output Only] The ID value of the disk used to create this snapshot. This
    /// value may be used to determine whether the snapshot was taken from the current or a
    /// previous instance of a given disk name.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_disk_id: Option<String>,

    /// Output only. [Output Only] The status of the snapshot. This can beCREATING, DELETING,
    /// FAILED,READY, or UPLOADING.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatus>,

    /// Output only. [Output Only] A size of the storage used by the snapshot. As snapshots
    /// share storage, this number is expected to change with snapshot creation/deletion.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<String>,

    /// Output only. [Output Only] An indicator whether storageBytes is in a stable state or it
    /// is being adjusted as a result of shared storage reallocation. This status can either be
    /// UPDATING, meaning the size of the snapshot is being updated, or UP_TO_DATE, meaning the
    /// size of the snapshot is up-to-date.
    ///
    /// **Possible values**:
    /// - `UPDATING`
    /// - `UP_TO_DATE`
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_bytes_status: Option<String>,

    /// Cloud Storage bucket storage location of the snapshot (regional or multi-regional).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub storage_locations: Vec<String>,
}

impl Snapshot {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-snapshot".into(),
            auto_created: Some(false),
            chain_name: Some("test-chain_name".into()),
            creation_size_bytes: Some("test-creation_size_bytes".into()),
            creation_timestamp: Some("test-creation_timestamp".into()),
            description: Some("test-description".into()),
            disk_size_gb: Some("test-disk_size_gb".into()),
            download_bytes: Some("test-download_bytes".into()),
            enable_confidential_compute: Some(false),
            guest_os_features: vec![],
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            label_fingerprint: Some("test-label_fingerprint".into()),
            labels: Default::default(),
            license_codes: vec![],
            licenses: vec![],
            self_link: Some("test-self_link".into()),
            snapshot_encryption_key: Some(DiskEncryptionKey::fixture()),
            snapshot_type: Some("test-snapshot_type".into()),
            source_disk: Some("test-source_disk".into()),
            source_disk_encryption_key: Some(DiskEncryptionKey::fixture()),
            source_disk_id: Some("test-source_disk_id".into()),
            status: Some(SnapshotStatus::Creating),
            storage_bytes: Some("test-storage_bytes".into()),
            storage_bytes_status: Some("test-storage_bytes_status".into()),
            storage_locations: vec![],
        }
    }
}

/// Represents an Operation resource. Google Compute Engine has three Operation resources:
/// * [Global](/compute/docs/reference/rest/v1/globalOperations)
/// * [Regional](/compute/docs/reference/rest/v1/regionOperations)
/// * [Zonal](/compute/docs/reference/rest/v1/zoneOperations) You can use an operation resource
///   to manage asynchronous API requests. For more information, readHandling API responses.
///   Operations can be global, regional or zonal.
/// - For global operations, use the `globalOperations` resource.
/// - For regional operations, use the `regionOperations` resource.
/// - For zonal operations, use the `zoneOperations` resource. For more information, read
///   Global, Regional, and Zonal Resources. Note that completed Operation resources have a
///   limited retention period.
///
/// **GCP API**: `compute.v1.Operation`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Operation>
///
/// ## Coverage
/// 5 of 26 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResponse {
    /// [Output Only] Name of the operation.
    pub name: String,

    /// [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,

    /// [Output Only] The status of the operation, which can be one of the following: `PENDING`,
    /// `RUNNING`, or `DONE`.
    ///
    /// **Possible values**:
    /// - `DONE`
    /// - `PENDING`
    /// - `RUNNING`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// [Output Only] The URL of the resource that the operation modifies. For operations
    /// related to creating a snapshot, this points to the disk that the snapshot was created
    /// from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_link: Option<String>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
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

/// Represents an IP Address resource. Google Compute Engine has two IP Address resources:
/// * [Global (external and
///   internal)](https://cloud.google.com/compute/docs/reference/rest/v1/globalAddresses)
/// * [Regional (external and
///   internal)](https://cloud.google.com/compute/docs/reference/rest/v1/addresses) For more
///   information, see Reserving a static external IP address.
///
/// **GCP API**: `compute.v1.Address`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Address>
///
/// ## Coverage
/// 18 of 21 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Name of the resource. Provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?`. The first
    /// character must be a lowercase letter, and all following characters (except for the last
    /// character) must be a dash, lowercase letter, or digit. The last character must be a
    /// lowercase letter or digit.
    pub name: String,

    /// The static IP address represented by this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The type of address to reserve, either INTERNAL orEXTERNAL. If unspecified, defaults to
    /// EXTERNAL.
    ///
    /// **Possible values**:
    /// - `EXTERNAL` — A publicly visible external IP address.
    /// - `INTERNAL` — A private network IP address, for use with an Instance or Internal Load Balancer...
    /// - `UNSPECIFIED_TYPE`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// An optional description of this resource. Provide this field when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Type of the resource. Always compute#address for addresses.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Labels for this resource. These can only be added or modified by thesetLabels method.
    /// Each label key/value pair must comply withRFC1035. Label values may be empty.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// A fingerprint for the labels being applied to this Address, which is essentially a hash
    /// of the labels set used for optimistic locking. The fingerprint is initially generated by
    /// Compute Engine and changes after every request to modify or update labels. You must
    /// always provide an up-to-date fingerprint hash in order to update or change labels,
    /// otherwise the request will fail with error412 conditionNotMet. To see the latest
    /// fingerprint, make a get() request to retrieve an Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,

    /// The URL of the network in which to reserve the address. This field can only be used with
    /// INTERNAL type with theVPC_PEERING purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// This signifies the networking tier used for configuring this address and can only take
    /// the following values: PREMIUM orSTANDARD. Internal IP addresses are always Premium Tier;
    /// global external IP addresses are always Premium Tier; regional external IP addresses can
    /// be either Standard or Premium Tier. If this field is not specified, it is assumed to be
    /// PREMIUM.
    ///
    /// **Possible values**:
    /// - `FIXED_STANDARD` — Public internet quality with fixed bandwidth.
    /// - `PREMIUM` — High quality, Google-grade network tier, support for all networking products.
    /// - `STANDARD` — Public internet quality, only limited support for other networking products.
    /// - `STANDARD_OVERRIDES_FIXED_STANDARD` — (Output only) Temporary tier for FIXED_STANDARD when fixed standard tier is expi...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_tier: Option<String>,

    /// The prefix length if the resource represents an IP range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,

    /// The purpose of this resource, which can be one of the following values:
    /// - GCE_ENDPOINT for addresses that are used by VM instances, alias IP ranges, load
    ///   balancers, and similar resources.
    /// - DNS_RESOLVER for a DNS resolver address in a subnetwork for a Cloud DNS inbound
    ///   forwarder IP addresses (regional internal IP address in a subnet of a VPC network)
    /// - VPC_PEERING for global internal IP addresses used for private services access
    ///   allocated ranges.
    /// - NAT_AUTO for the regional external IP addresses used by Cloud NAT when allocating
    ///   addresses using automatic NAT IP address allocation.
    /// - IPSEC_INTERCONNECT for addresses created from a private IP range that are reserved for
    ///   a VLAN attachment in an *HA VPN over Cloud Interconnect* configuration. These
    ///   addresses are regional resources.
    /// - `SHARED_LOADBALANCER_VIP` for an internal IP address that is assigned to multiple
    ///   internal forwarding rules.
    /// - `PRIVATE_SERVICE_CONNECT` for a private network address that is used to configure
    ///   Private Service Connect. Only global internal addresses can use this purpose.
    ///
    /// **Possible values**:
    /// - `DNS_RESOLVER` — DNS resolver address in the subnetwork.
    /// - `GCE_ENDPOINT` — VM internal/alias IP, Internal LB service IP, etc.
    /// - `IPSEC_INTERCONNECT` — A regional internal IP address range reserved for the VLAN attachment that is us...
    /// - `NAT_AUTO` — External IP automatically reserved for Cloud NAT.
    /// - `PRIVATE_SERVICE_CONNECT` — A private network IP address that can be used to configure Private Service Conne...
    /// - `SERVERLESS` — A regional internal IP address range reserved for Serverless.
    /// - `SHARED_LOADBALANCER_VIP` — A private network IP address that can be shared by multiple Internal Load Balanc...
    /// - `VPC_PEERING` — IP range for peer networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,

    /// Output only. [Output Only] The URL of the region where a regional address resides. For
    /// regional addresses, you must specify the region as a path parameter in the HTTP request
    /// URL. *This field is not applicable to global addresses.*
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. [Output Only] The status of the address, which can be one ofRESERVING,
    /// RESERVED, or IN_USE. An address that is RESERVING is currently in the process of being
    /// reserved. A RESERVED address is currently reserved and available to use. An IN_USE
    /// address is currently being used by another resource and is not available.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AddressStatus>,

    /// The URL of the subnetwork in which to reserve the address. If an IP address is
    /// specified, it must be within the subnetwork's IP range. This field can only be used with
    /// INTERNAL type with aGCE_ENDPOINT or DNS_RESOLVER purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,

    /// [Output Only] The URLs of the resources that are using this address.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

impl Address {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-address".into(),
            address: Some("test-address".into()),
            address_type: Some("test-address_type".into()),
            creation_timestamp: Some("test-creation_timestamp".into()),
            description: Some("test-description".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            labels: Default::default(),
            label_fingerprint: Some("test-label_fingerprint".into()),
            network: Some("test-network".into()),
            network_tier: Some("test-network_tier".into()),
            prefix_length: Some(100),
            purpose: Some("test-purpose".into()),
            region: Some("test-region".into()),
            self_link: Some("test-self_link".into()),
            status: Some(AddressStatus::InUse),
            subnetwork: Some("test-subnetwork".into()),
            users: vec![],
        }
    }
}

/// Represents a Cloud Router resource. For more information about Cloud Router, read theCloud
/// Router overview.
///
/// **GCP API**: `compute.v1.Router`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Router>
///
/// ## Coverage
/// 9 of 15 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Router {
    /// Name of the resource. Provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which
    /// means the first character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which cannot be a
    /// dash.
    pub name: String,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// [Output Only] The unique identifier for the resource. This identifier is defined by the
    /// server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Type of resource. Always compute#router for routers.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// A list of NAT services created in this router.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nats: Vec<RouterNat>,

    /// URI of the network to which this router belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// [Output Only] URI of the region where the router resides. You must specify this field as
    /// part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl Router {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-router".into(),
            creation_timestamp: Some("test-creation_timestamp".into()),
            description: Some("test-description".into()),
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            nats: vec![],
            network: Some("test-network".into()),
            region: Some("test-region".into()),
            self_link: Some("test-self_link".into()),
        }
    }
}

/// Represents a Nat resource. It enables the VMs within the specified subnetworks to access
/// Internet without external IP addresses. It specifies a list of subnetworks (and the ranges
/// within) that want to use NAT. Customers can also provide the external IPs that would be used
/// for NAT. GCP would auto-allocate ephemeral IPs if no external IPs are provided.
///
/// **GCP API**: `compute.v1.RouterNat`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/RouterNat>
///
/// ## Coverage
/// 19 of 22 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterNat {
    /// Unique name of this Nat service. The name must be 1-63 characters long and comply
    /// withRFC1035.
    pub name: String,

    /// Specify the NatIpAllocateOption, which can take one of the following values:
    /// - MANUAL_ONLY: Uses only Nat IP addresses provided by customers. When there are not
    ///   enough specified Nat IPs, the Nat service fails for new VMs.
    /// - AUTO_ONLY: Nat IPs are allocated by Google Cloud Platform; customers can't specify any
    ///   Nat IPs. When choosing AUTO_ONLY, then nat_ip should be empty.
    ///
    /// **Possible values**:
    /// - `AUTO_ONLY` — Nat IPs are allocated by GCP; customers can not specify any Nat IPs.
    /// - `MANUAL_ONLY` — Only use Nat IPs provided by customers. When specified Nat IPs are not enough th...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_ip_allocate_option: Option<String>,

    /// Specify the Nat option, which can take one of the following values:
    /// - ALL_SUBNETWORKS_ALL_IP_RANGES: All of the IP ranges in every Subnetwork are allowed to
    ///   Nat.
    /// - ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES: All of the primary IP ranges in every
    ///   Subnetwork are allowed to Nat.
    /// - LIST_OF_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field
    ///   subnetwork below) The default is SUBNETWORK_IP_RANGE_TO_NAT_OPTION_UNSPECIFIED. Note
    ///   that if this field contains ALL_SUBNETWORKS_ALL_IP_RANGES then there should not be any
    ///   other Router.Nat section in any Router for this network in this region.
    ///
    /// **Possible values**:
    /// - `ALL_SUBNETWORKS_ALL_IP_RANGES` — All the IP ranges in every Subnetwork are allowed to Nat.
    /// - `ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES` — All the primary IP ranges in every Subnetwork are allowed to Nat.
    /// - `LIST_OF_SUBNETWORKS` — A list of Subnetworks are allowed to Nat (specified in the field subnetwork belo...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_subnetwork_ip_ranges_to_nat: Option<String>,

    /// A list of URLs of the IP resources used for this Nat service. These IP addresses must be
    /// valid static external IP addresses assigned to the project.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nat_ips: Vec<String>,

    /// A list of URLs of the IP resources to be drained. These IPs must be valid static
    /// external IPs that have been assigned to the NAT. These IPs should be used for
    /// updating/patching a NAT only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drain_nat_ips: Vec<String>,

    /// Minimum number of ports allocated to a VM from this NAT config. If not set, a default
    /// number of ports is allocated to a VM. This is rounded up to the nearest power of 2. For
    /// example, if the value of this field is 50, at least 64 ports are allocated to a VM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ports_per_vm: Option<i32>,

    /// Maximum number of ports allocated to a VM from this NAT config when Dynamic Port
    /// Allocation is enabled. If Dynamic Port Allocation is not enabled, this field has no
    /// effect. If Dynamic Port Allocation is enabled, and this field is set, it must be set to
    /// a power of two greater than minPortsPerVm, or 64 if minPortsPerVm is not set. If Dynamic
    /// Port Allocation is enabled and this field is not set, a maximum of 65536 ports will be
    /// allocated to a VM from this NAT config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ports_per_vm: Option<i32>,

    /// The `enableEndpointIndependentMapping` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_endpoint_independent_mapping: Option<bool>,

    /// Enable Dynamic Port Allocation. If not specified, it is disabled by default. If set to
    /// true,
    /// - Dynamic Port Allocation will be enabled on this NAT config.
    /// - enableEndpointIndependentMapping cannot be set to true.
    /// - If minPorts is set, minPortsPerVm must be set to a power of two greater than or equal
    ///   to 32. If minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM
    ///   from this NAT config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dynamic_port_allocation: Option<bool>,

    /// Configure logging on this NAT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<RouterNatLogConfig>,

    /// Indicates whether this NAT is used for public or private IP translation. If unspecified,
    /// it defaults to PUBLIC.
    ///
    /// **Possible values**:
    /// - `PRIVATE` — NAT used for private IP translation.
    /// - `PUBLIC` — NAT used for public IP translation. This is the default.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_type: Option<String>,

    /// The network tier to use when automatically reserving NAT IP addresses. Must be one of:
    /// PREMIUM, STANDARD. If not specified, then the current project-level default tier is
    /// used.
    ///
    /// **Possible values**:
    /// - `FIXED_STANDARD` — Public internet quality with fixed bandwidth.
    /// - `PREMIUM` — High quality, Google-grade network tier, support for all networking products.
    /// - `STANDARD` — Public internet quality, only limited support for other networking products.
    /// - `STANDARD_OVERRIDES_FIXED_STANDARD` — (Output only) Temporary tier for FIXED_STANDARD when fixed standard tier is expi...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_network_tier: Option<String>,

    /// A list of Subnetwork resources whose traffic should be translated by NAT Gateway. It is
    /// used only when LIST_OF_SUBNETWORKS is selected for the SubnetworkIpRangeToNatOption
    /// above.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnetworks: Vec<RouterNatSubnetworkToNat>,

    /// Timeout (in seconds) for UDP connections. Defaults to 30s if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_idle_timeout_sec: Option<i32>,

    /// Timeout (in seconds) for TCP established connections. Defaults to 1200s if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_established_idle_timeout_sec: Option<i32>,

    /// Timeout (in seconds) for TCP transitory connections. Defaults to 30s if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_transitory_idle_timeout_sec: Option<i32>,

    /// Timeout (in seconds) for TCP connections that are in TIME_WAIT state. Defaults to 120s
    /// if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_time_wait_timeout_sec: Option<i32>,

    /// Timeout (in seconds) for ICMP connections. Defaults to 30s if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_idle_timeout_sec: Option<i32>,

    /// List of NAT-ted endpoint types supported by the Nat Gateway. If the list is empty, then
    /// it will be equivalent to include ENDPOINT_TYPE_VM
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub endpoint_types: Vec<String>,
}

impl RouterNat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-router_nat".into(),
            nat_ip_allocate_option: Some("test-nat_ip_allocate_option".into()),
            source_subnetwork_ip_ranges_to_nat: Some(
                "test-source_subnetwork_ip_ranges_to_nat".into(),
            ),
            nat_ips: vec![],
            drain_nat_ips: vec![],
            min_ports_per_vm: Some(100),
            max_ports_per_vm: Some(100),
            enable_endpoint_independent_mapping: Some(false),
            enable_dynamic_port_allocation: Some(false),
            log_config: Some(RouterNatLogConfig::fixture()),
            nat_type: Some("test-type".into()),
            auto_network_tier: Some("test-auto_network_tier".into()),
            subnetworks: vec![],
            udp_idle_timeout_sec: Some(100),
            tcp_established_idle_timeout_sec: Some(100),
            tcp_transitory_idle_timeout_sec: Some(100),
            tcp_time_wait_timeout_sec: Some(100),
            icmp_idle_timeout_sec: Some(100),
            endpoint_types: vec![],
        }
    }
}

/// Configuration of logging on a NAT.
///
/// **GCP API**: `compute.v1.RouterNatLogConfig`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/RouterNatLogConfig>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterNatLogConfig {
    /// Indicates whether or not to export logs. This is false by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Specify the desired filtering of logs on this NAT. If unspecified, logs are exported for
    /// all connections handled by this NAT. This option can take one of the following values:
    /// - ERRORS_ONLY: Export logs only for connection failures.
    /// - TRANSLATIONS_ONLY: Export logs only for successful connections.
    /// - ALL: Export logs for all connections, successful and unsuccessful.
    ///
    /// **Possible values**:
    /// - `ALL` — Export logs for all (successful and unsuccessful) connections.
    /// - `ERRORS_ONLY` — Export logs for connection failures only.
    /// - `TRANSLATIONS_ONLY` — Export logs for successful connections only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

impl RouterNatLogConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable: Some(false),
            filter: Some("test-filter".into()),
        }
    }
}

/// Defines the IP ranges that want to use NAT for a subnetwork.
///
/// **GCP API**: `compute.v1.RouterNatSubnetworkToNat`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/RouterNatSubnetworkToNat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterNatSubnetworkToNat {
    /// URL for the subnetwork resource that will use NAT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specify the options for NAT ranges in the Subnetwork. All options of a single value are
    /// valid except NAT_IP_RANGE_OPTION_UNSPECIFIED. The only valid option with multiple values
    /// is: ["PRIMARY_IP_RANGE", "LIST_OF_SECONDARY_IP_RANGES"] Default: [ALL_IP_RANGES]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_ip_ranges_to_nat: Vec<String>,

    /// A list of the secondary ranges of the Subnetwork that are allowed to use NAT. This can
    /// be populated only if "LIST_OF_SECONDARY_IP_RANGES" is one of the values in
    /// source_ip_ranges_to_nat.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secondary_ip_range_names: Vec<String>,
}

impl RouterNatSubnetworkToNat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-router_nat_subnetwork_to_nat".into()),
            source_ip_ranges_to_nat: vec![],
            secondary_ip_range_names: vec![],
        }
    }
}

/// Represents a Backend Service resource. A backend service defines how Google Cloud load
/// balancers distribute traffic. The backend service configuration contains a set of values,
/// such as the protocol used to connect to backends, various distribution and session settings,
/// health checks, and timeouts. These settings provide fine-grained control over how your load
/// balancer behaves. Most of the settings have default values that allow for easy configuration
/// if you need to get started quickly. Backend services in Google Compute Engine can be either
/// regionally or globally scoped.
/// * [Global](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices)
/// * [Regional](https://cloud.google.com/compute/docs/reference/rest/v1/regionBackendServices)
///   For more information, seeBackend Services.
///
/// **GCP API**: `compute.v1.BackendService`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/BackendService>
///
/// ## Coverage
/// 19 of 51 fields included.
/// Omitted fields:
/// - `cdnPolicy` — CDN configuration - advanced feature
/// - `circuitBreakers` — Traffic management - advanced
/// - `consistentHash` — Session affinity - advanced
/// - `customRequestHeaders` — Custom headers - advanced
/// - `customResponseHeaders` — Custom headers - advanced
/// - `failoverPolicy` — Failover - enterprise feature
/// - `iap` — Identity-Aware Proxy - separate concern
/// - `localityLbPolicies` — Advanced load balancing
/// - `outlierDetection` — Traffic management - advanced
/// - `securityPolicy` — Security policy - separate concern
/// - `securitySettings` — mTLS - advanced
/// - `subsetting` — Subsetting - advanced
/// - `tlsSettings` — Backend TLS - advanced
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendService {
    /// Name of the resource. Provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which
    /// means the first character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which cannot be a
    /// dash.
    pub name: String,

    /// The list of backends that serve this BackendService.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub backends: Vec<Backend>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Fingerprint of this resource. A hash of the contents stored in this object. This field
    /// is used in optimistic locking. This field will be ignored when inserting a
    /// BackendService. An up-to-date fingerprint must be provided in order to update the
    /// BackendService, otherwise the request will fail with error 412 conditionNotMet. To see
    /// the latest fingerprint, make a get() request to retrieve a BackendService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// The list of URLs to the healthChecks, httpHealthChecks (legacy), or httpsHealthChecks
    /// (legacy) resource for health checking this backend service. Not all backend services
    /// support legacy health checks. See Load balancer guide. Currently, at most one health
    /// check can be specified for each backend service. Backend services with instance group or
    /// zonal NEG backends must have a health check unless haPolicy is specified. Backend
    /// services with internet or serverless NEG backends must not have a health check.
    /// healthChecks[] cannot be specified with haPolicy.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_checks: Vec<String>,

    /// [Output Only] The unique identifier for the resource. This identifier is defined by the
    /// server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Type of resource. Always compute#backendService for backend
    /// services.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Specifies the load balancer type. A backend service created for one type of load
    /// balancer cannot be used with another. For more information, refer toChoosing a load
    /// balancer.
    ///
    /// **Possible values**:
    /// - `EXTERNAL` — Signifies that this will be used for classic Application Load Balancers, global ...
    /// - `EXTERNAL_MANAGED` — Signifies that this will be used for global external Application Load Balancers,...
    /// - `INTERNAL` — Signifies that this will be used for internal passthrough Network Load Balancers...
    /// - `INTERNAL_MANAGED` — Signifies that this will be used for internal Application Load Balancers.
    /// - `INTERNAL_SELF_MANAGED` — Signifies that this will be used by Traffic Director.
    /// - `INVALID_LOAD_BALANCING_SCHEME`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancing_scheme: Option<String>,

    /// This field denotes the logging options for the load balancer traffic served by this
    /// backend service. If logging is enabled, logs will be exported to Stackdriver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<BackendServiceLogConfig>,

    /// The URL of the network to which this backend service belongs. This field must be set for
    /// Internal Passthrough Network Load Balancers when the haPolicy is enabled, and for
    /// External Passthrough Network Load Balancers when the haPolicy fastIpMove is enabled.
    /// This field can only be specified when the load balancing scheme is set toINTERNAL, or
    /// when the load balancing scheme is set toEXTERNAL and haPolicy fastIpMove is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// Deprecated in favor of portName. The TCP port to connect on the backend. The default
    /// value is 80. For internal passthrough Network Load Balancers and external passthrough
    /// Network Load Balancers, omit port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// A named port on a backend instance group representing the port for communication to the
    /// backend VMs in that group. The named port must be [defined on each backend instance
    /// group](https://cloud.google.com/load-balancing/docs/backend-service#named_ports). This
    /// parameter has no meaning if the backends are NEGs. For internal passthrough Network Load
    /// Balancers and external passthrough Network Load Balancers, omit port_name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,

    /// The protocol this BackendService uses to communicate with backends. Possible values are
    /// HTTP, HTTPS, HTTP2, H2C, TCP, SSL, UDP or GRPC. depending on the chosen load balancer or
    /// Traffic Director configuration. Refer to the documentation for the load balancers or for
    /// Traffic Director for more information. Must be set to GRPC when the backend service is
    /// referenced by a URL map that is bound to target gRPC proxy.
    ///
    /// **Possible values**:
    /// - `GRPC` — gRPC (available for Traffic Director).
    /// - `H2C` — HTTP2 over cleartext
    /// - `HTTP`
    /// - `HTTP2` — HTTP/2 with SSL.
    /// - `HTTPS`
    /// - `SSL` — TCP proxying with SSL.
    /// - `TCP` — TCP proxying or TCP pass-through.
    /// - `UDP` — UDP.
    /// - `UNSPECIFIED` — If a Backend Service has UNSPECIFIED as its protocol, it can be used with any L3...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Output only. [Output Only] URL of the region where the regional backend service resides.
    /// This field is not applicable to global backend services. You must specify this field as
    /// part of the HTTP request URL. It is not settable as a field in the request body.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Type of session affinity to use. The default is NONE. Only NONE and HEADER_FIELD are
    /// supported when the backend service is referenced by a URL map that is bound to target
    /// gRPC proxy that has validateForProxyless field set to true. For more details, see:
    /// [Session Affinity](https://cloud.google.com/load-balancing/docs/backend-
    /// service#session_affinity). sessionAffinity cannot be specified with haPolicy.
    ///
    /// **Possible values**:
    /// - `CLIENT_IP` — 2-tuple hash on packet's source and destination IP addresses. Connections from t...
    /// - `CLIENT_IP_NO_DESTINATION` — 1-tuple hash only on packet's source IP address. Connections from the same sourc...
    /// - `CLIENT_IP_PORT_PROTO` — 5-tuple hash on packet's source and destination IP addresses, IP protocol, and s...
    /// - `CLIENT_IP_PROTO` — 3-tuple hash on packet's source and destination IP addresses, and IP protocol. C...
    /// - `GENERATED_COOKIE` — Hash based on a cookie generated by the L7 loadbalancer. Only valid for HTTP(S) ...
    /// - `HEADER_FIELD` — The hash is based on a user specified header field.
    /// - `HTTP_COOKIE` — The hash is based on a user provided cookie.
    /// - `NONE` — No session affinity. Connections from the same client IP may go to any instance ...
    /// - `STRONG_COOKIE_AFFINITY` — Strong cookie-based affinity. Connections bearing the same cookie will be served...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_affinity: Option<String>,

    /// The backend service timeout has a different meaning depending on the type of load
    /// balancer. For more information see, Backend service settings. The default is 30 seconds.
    /// The full range of timeout values allowed goes from 1 through 2,147,483,647 seconds. This
    /// value can be overridden in the PathMatcher configuration of the UrlMap that references
    /// this backend service. Not supported when the backend service is referenced by a URL map
    /// that is bound to target gRPC proxy that has validateForProxyless field set to true.
    /// Instead, use maxStreamDuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_sec: Option<i32>,

    /// connectionDraining cannot be specified with haPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_draining: Option<ConnectionDraining>,
}

impl BackendService {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-backend_service".into(),
            backends: vec![],
            creation_timestamp: Some("test-creation_timestamp".into()),
            description: Some("test-description".into()),
            fingerprint: Some("test-fingerprint".into()),
            health_checks: vec![],
            id: Some("test-id".into()),
            kind: Some("test-kind".into()),
            load_balancing_scheme: Some("test-load_balancing_scheme".into()),
            log_config: Some(BackendServiceLogConfig::fixture()),
            network: Some("test-network".into()),
            port: Some(100),
            port_name: Some("test-port_name".into()),
            protocol: Some("test-protocol".into()),
            region: Some("test-region".into()),
            self_link: Some("test-self_link".into()),
            session_affinity: Some("test-session_affinity".into()),
            timeout_sec: Some(100),
            connection_draining: Some(ConnectionDraining::fixture()),
        }
    }
}

/// Message containing information of one individual backend.
///
/// **GCP API**: `compute.v1.Backend`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Backend>
///
/// ## Coverage
/// 12 of 14 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    /// Specifies how to determine whether the backend of a load balancer can handle additional
    /// traffic or is fully loaded. For usage guidelines, see Connection balancing mode.
    /// Backends must use compatible balancing modes. For more information, see Supported
    /// balancing modes and target capacity settings and Restrictions and guidance for instance
    /// groups. Note: Currently, if you use the API to configure incompatible balancing modes,
    /// the configuration might be accepted even though it has no impact and is ignored.
    /// Specifically, Backend.maxUtilization is ignored when Backend.balancingMode is RATE. In
    /// the future, this incompatible combination will be rejected.
    ///
    /// **Possible values**:
    /// - `CONNECTION` — Balance based on the number of simultaneous connections.
    /// - `CUSTOM_METRICS` — Based on custom defined and reported metrics.
    /// - `RATE` — Balance based on requests per second (RPS).
    /// - `UTILIZATION` — Balance based on the backend utilization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_mode: Option<String>,

    /// A multiplier applied to the backend's target capacity of its balancing mode. The default
    /// value is 1, which means the group serves up to 100% of its configured capacity
    /// (depending onbalancingMode). A setting of 0 means the group is completely drained,
    /// offering 0% of its available capacity. The valid ranges are 0.0 and [0.1,1.0]. You
    /// cannot configure a setting larger than 0 and smaller than0.1. You cannot configure a
    /// setting of 0 when there is only one backend attached to the backend service. Not
    /// available with backends that don't support using abalancingMode. This includes backends
    /// such as global internet NEGs, regional serverless NEGs, and PSC NEGs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_scaler: Option<f32>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// This field designates whether this is a failover backend. More than one failover backend
    /// can be configured for a given BackendService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<bool>,

    /// The fully-qualified URL of aninstance group or network endpoint group (NEG) resource. To
    /// determine what types of backends a load balancer supports, see the [Backend services
    /// overview](https://cloud.google.com/load-balancing/docs/backend-service#backends). You
    /// must use the *fully-qualified* URL (starting withhttps://www.googleapis.com/) to specify
    /// the instance group or NEG. Partial URLs are not supported. If haPolicy is specified,
    /// backends must refer to NEG resources of type GCE_VM_IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Defines a target maximum number of simultaneous connections. For usage guidelines,
    /// seeConnection balancing mode and Utilization balancing mode. Not available if the
    /// backend'sbalancingMode is RATE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,

    /// Defines a target maximum number of simultaneous connections. For usage guidelines,
    /// seeConnection balancing mode and Utilization balancing mode. Not available if the
    /// backend's balancingMode isRATE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections_per_endpoint: Option<i32>,

    /// Defines a target maximum number of simultaneous connections. For usage guidelines,
    /// seeConnection balancing mode and Utilization balancing mode. Not available if the
    /// backend's balancingMode isRATE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections_per_instance: Option<i32>,

    /// Defines a maximum number of HTTP requests per second (RPS). For usage guidelines,
    /// seeRate balancing mode and Utilization balancing mode. Not available if the backend's
    /// balancingMode isCONNECTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<i32>,

    /// Defines a maximum target for requests per second (RPS). For usage guidelines, seeRate
    /// balancing mode and Utilization balancing mode. Not available if the backend's
    /// balancingMode isCONNECTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate_per_endpoint: Option<f32>,

    /// Defines a maximum target for requests per second (RPS). For usage guidelines, seeRate
    /// balancing mode and Utilization balancing mode. Not available if the backend's
    /// balancingMode isCONNECTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rate_per_instance: Option<f32>,

    /// Optional parameter to define a target capacity for theUTILIZATION balancing mode. The
    /// valid range is[0.0, 1.0]. For usage guidelines, seeUtilization balancing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_utilization: Option<f32>,
}

impl Backend {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            balancing_mode: Some("test-balancing_mode".into()),
            description: Some("test-description".into()),
            failover: Some(false),
            group: Some("test-group".into()),
            max_connections: Some(100),
            max_connections_per_endpoint: Some(100),
            max_connections_per_instance: Some(100),
            max_rate: Some(100),
            ..Default::default()
        }
    }
}

/// The available logging options for the load balancer traffic served by this backend service.
///
/// **GCP API**: `compute.v1.BackendServiceLogConfig`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/BackendServiceLogConfig>
///
/// ## Coverage
/// 2 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendServiceLogConfig {
    /// Denotes whether to enable logging for the load balancer traffic served by this backend
    /// service. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// This field can only be specified if logging is enabled for this backend service. The
    /// value of the field must be in [0, 1]. This configures the sampling rate of requests to
    /// the load balancer where 1.0 means all logged requests are reported and 0.0 means no
    /// logged requests are reported. The default value is 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f32>,
}

impl BackendServiceLogConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable: Some(false),
            ..Default::default()
        }
    }
}

/// Message containing connection draining configuration.
///
/// **GCP API**: `compute.v1.ConnectionDraining`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/ConnectionDraining>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDraining {
    /// Configures a duration timeout for existing requests on a removed backend instance. For
    /// supported load balancers and protocols, as described inEnabling connection draining.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draining_timeout_sec: Option<i32>,
}

impl ConnectionDraining {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            draining_timeout_sec: Some(100),
        }
    }
}

/// Represents a Firewall Rule resource. Firewall rules allow or deny ingress traffic to, and
/// egress traffic from your instances. For more information, readFirewall rules.
///
/// **GCP API**: `compute.v1.Firewall`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Firewall>
///
/// ## Coverage
/// 18 of 20 fields included.
/// Omitted fields:
/// - `logConfig` — Firewall logging config - can add later
/// - `params` — Resource manager tags - niche
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Firewall {
    /// Name of the resource; provided by the client when the resource is created. The name must
    /// be 1-63 characters long, and comply withRFC1035. Specifically, the name must be 1-63
    /// characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?`. The first
    /// character must be a lowercase letter, and all following characters (except for the last
    /// character) must be a dash, lowercase letter, or digit. The last character must be a
    /// lowercase letter or digit.
    pub name: String,

    /// An optional description of this resource. Provide this field when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// URL of the network resource for this firewall rule. If not specified when creating a
    /// firewall rule, the default network is used: global/networks/default If you choose to
    /// specify this field, you can specify the network as a full or partial URL. For example,
    /// the following are all valid URLs:
    /// - https://www.googleapis.com/compute/v1/projects/myproject/global/networks/my-network
    /// - projects/myproject/global/networks/my-network
    /// - global/networks/default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// Direction of traffic to which this firewall applies, either `INGRESS` or `EGRESS`. The
    /// default is `INGRESS`. For `EGRESS` traffic, you cannot specify the sourceTags fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<FirewallDirection>,

    /// Priority for this rule. This is an integer between `0` and `65535`, both inclusive. The
    /// default value is `1000`. Relative priorities determine which rule takes effect if
    /// multiple rules apply. Lower values indicate higher priority. For example, a rule with
    /// priority `0` has higher precedence than a rule with priority `1`. DENY rules take
    /// precedence over ALLOW rules if they have equal priority. Note that VPC networks have
    /// implied rules with a priority of `65535`. To avoid conflicts with the implied rules, use
    /// a priority number less than `65535`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// The list of ALLOW rules specified by this firewall. Each rule specifies a protocol and
    /// port-range tuple that describes a permitted connection.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed: Vec<FirewallAllowed>,

    /// The list of DENY rules specified by this firewall. Each rule specifies a protocol and
    /// port-range tuple that describes a denied connection.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub denied: Vec<FirewallDenied>,

    /// If source ranges are specified, the firewall rule applies only to traffic that has a
    /// source IP address in these ranges. These ranges must be expressed inCIDR format. One or
    /// both of sourceRanges and sourceTags may be set. If both fields are set, the rule applies
    /// to traffic that has a source IP address within sourceRanges OR a source IP from a
    /// resource with a matching tag listed in thesourceTags field. The connection does not need
    /// to match both fields for the rule to apply. Both IPv4 and IPv6 are supported.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_ranges: Vec<String>,

    /// If destination ranges are specified, the firewall rule applies only to traffic that has
    /// destination IP address in these ranges. These ranges must be expressed inCIDR format.
    /// Both IPv4 and IPv6 are supported.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination_ranges: Vec<String>,

    /// If source tags are specified, the firewall rule applies only to traffic with source IPs
    /// that match the primary network interfaces of VM instances that have the tag and are in
    /// the same VPC network. Source tags cannot be used to control traffic to an instance's
    /// external IP address, it only applies to traffic between instances in the same virtual
    /// network. Because tags are associated with instances, not IP addresses. One or both of
    /// sourceRanges and sourceTags may be set. If both fields are set, the firewall applies to
    /// traffic that has a source IP address within sourceRanges OR a source IP from a resource
    /// with a matching tag listed in the sourceTags field. The connection does not need to
    /// match both fields for the firewall to apply.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_tags: Vec<String>,

    /// A list of tags that controls which instances the firewall rule applies to. If targetTags
    /// are specified, then the firewall rule applies only to instances in the VPC network that
    /// have one of those tags. If no targetTags are specified, the firewall rule applies to all
    /// instances on the specified network.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_tags: Vec<String>,

    /// If source service accounts are specified, the firewall rules apply only to traffic
    /// originating from an instance with a service account in this list. Source service
    /// accounts cannot be used to control traffic to an instance's external IP address because
    /// service accounts are associated with an instance, not an IP address.sourceRanges can be
    /// set at the same time assourceServiceAccounts. If both are set, the firewall applies to
    /// traffic that has a source IP address within the sourceRanges OR a source IP that belongs
    /// to an instance with service account listed insourceServiceAccount. The connection does
    /// not need to match both fields for the firewall to apply.sourceServiceAccounts cannot be
    /// used at the same time assourceTags or targetTags.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_service_accounts: Vec<String>,

    /// A list of service accounts indicating sets of instances located in the network that may
    /// make network connections as specified inallowed[].targetServiceAccounts cannot be used
    /// at the same time astargetTags or sourceTags. If neither targetServiceAccounts nor
    /// targetTags are specified, the firewall rule applies to all instances on the specified
    /// network.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_service_accounts: Vec<String>,

    /// Denotes whether the firewall rule is disabled. When set to true, the firewall rule is
    /// not enforced and the network behaves as if it did not exist. If this is unspecified, the
    /// firewall rule will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Creation timestamp inRFC3339 text format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,

    /// Output only. [Output Only] Type of the resource. Always compute#firewall for firewall
    /// rules.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl Firewall {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-firewall".into(),
            description: Some("test-description".into()),
            network: Some("test-network".into()),
            direction: Some(FirewallDirection::Egress),
            priority: Some(100),
            allowed: vec![],
            denied: vec![],
            source_ranges: vec![],
            destination_ranges: vec![],
            source_tags: vec![],
            target_tags: vec![],
            source_service_accounts: vec![],
            target_service_accounts: vec![],
            disabled: Some(false),
            self_link: Some("test-self_link".into()),
            id: Some("test-id".into()),
            creation_timestamp: Some("test-creation_timestamp".into()),
            kind: Some("test-kind".into()),
        }
    }
}

///
/// **GCP API**: `compute.v1.DisksResizeRequest`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/DisksResizeRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisksResizeRequest {
    /// The new size of the persistent disk, which is specified in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<String>,
}

impl DisksResizeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            size_gb: Some("test-size_gb".into()),
        }
    }
}

///
/// **GCP API**: `compute.v1.InstancesSetMachineTypeRequest`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/InstancesSetMachineTypeRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesSetMachineTypeRequest {
    /// Full or partial URL of the machine type resource. See Machine Types for a full list of
    /// machine types. For example:zones/us-central1-f/machineTypes/n1-standard-1
    pub machine_type: String,
}

impl InstancesSetMachineTypeRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            machine_type: "test-machine_type".into(),
        }
    }
}

///
/// **GCP API**: `compute.v1.InstancesSetServiceAccountRequest`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/InstancesSetServiceAccountRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstancesSetServiceAccountRequest {
    /// Email address of the service account.
    pub email: String,

    /// The list of scopes to be made available for this service account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

impl InstancesSetServiceAccountRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            email: "test-email".into(),
            scopes: vec![],
        }
    }
}

/// The available logging options for this subnetwork.
///
/// **GCP API**: `compute.v1.SubnetworkLogConfig`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/SubnetworkLogConfig>
///
/// ## Coverage
/// 4 of 6 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubnetworkLogConfig {
    /// Whether to enable flow logging for this subnetwork. If this field is not explicitly set,
    /// it will not appear in get listings. If not set the default behavior is determined by the
    /// org policy, if there is no org policy specified, then it will default to disabled. Flow
    /// logging isn't supported if the subnet purpose field is set to REGIONAL_MANAGED_PROXY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Can only be specified if VPC flow logging for this subnetwork is enabled. Toggles the
    /// aggregation interval for collecting flow logs. Increasing the interval time will reduce
    /// the amount of generated flow logs for long lasting connections. Default is an interval
    /// of 5 seconds per connection.
    ///
    /// **Possible values**:
    /// - `INTERVAL_10_MIN`
    /// - `INTERVAL_15_MIN`
    /// - `INTERVAL_1_MIN`
    /// - `INTERVAL_30_SEC`
    /// - `INTERVAL_5_MIN`
    /// - `INTERVAL_5_SEC`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_interval: Option<String>,

    /// Can only be specified if VPC flow logging for this subnetwork is enabled. The value of
    /// the field must be in [0, 1]. Set the sampling rate of VPC flow logs within the
    /// subnetwork where 1.0 means all collected logs are reported and 0.0 means no logs are
    /// reported. Default is 0.5 unless otherwise specified by the org policy, which means half
    /// of all collected logs are reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_sampling: Option<f32>,

    /// Can only be specified if VPC flow logs for this subnetwork is enabled. Configures
    /// whether all, none or a subset of metadata fields should be added to the reported VPC
    /// flow logs. Default isEXCLUDE_ALL_METADATA.
    ///
    /// **Possible values**:
    /// - `CUSTOM_METADATA`
    /// - `EXCLUDE_ALL_METADATA`
    /// - `INCLUDE_ALL_METADATA`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl SubnetworkLogConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable: Some(false),
            aggregation_interval: Some("test-aggregation_interval".into()),
            metadata: Some("test-metadata".into()),
            ..Default::default()
        }
    }
}

/// Represents a Subnetwork resource. A subnetwork (also known as a subnet) is a logical
/// partition of a Virtual Private Cloud network with one primary IP range and zero or more
/// secondary IP ranges. For more information, read Virtual Private Cloud (VPC) Network.
///
/// **GCP API**: `compute.v1.Subnetwork`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/Subnetwork>
///
/// ## Coverage
/// 10 of 33 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subnetwork {
    /// The name of the resource, provided by the client when initially creating the resource.
    /// The name must be 1-63 characters long, and comply withRFC1035. Specifically, the name
    /// must be 1-63 characters long and match the regular expression
    /// `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter,
    /// and all following characters must be a dash, lowercase letter, or digit, except the last
    /// character, which cannot be a dash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Fingerprint of this resource. A hash of the contents stored in this object. This field
    /// is used in optimistic locking. This field will be ignored when inserting a Subnetwork.
    /// An up-to-date fingerprint must be provided in order to update the Subnetwork, otherwise
    /// the request will fail with error 412 conditionNotMet. To see the latest fingerprint,
    /// make a get() request to retrieve a Subnetwork.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Whether to enable flow logging for this subnetwork. If this field is not explicitly set,
    /// it will not appear in get listings. If not set the default behavior is determined by the
    /// org policy, if there is no org policy specified, then it will default to disabled. This
    /// field isn't supported if the subnet purpose field is set toREGIONAL_MANAGED_PROXY. It is
    /// recommended to uselogConfig.enable field instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_flow_logs: Option<bool>,

    /// This field denotes the VPC flow logging options for this subnetwork. If logging is
    /// enabled, logs are exported to Cloud Logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<SubnetworkLogConfig>,

    /// Whether the VMs in this subnet can access Google services without assigned external IP
    /// addresses. This field can be both set at resource creation time and updated using
    /// setPrivateIpGoogleAccess.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_google_access: Option<bool>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource. This field can be set only at resource creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// URL of the region where the Subnetwork resides. This field can be set only at resource
    /// creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// The URL of the network to which this subnetwork belongs, provided by the client when
    /// initially creating the subnetwork. This field can be set only at resource creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// [Output Only] Server-defined URL for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Subnetwork {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-subnetwork".into()),
            fingerprint: Some("test-fingerprint".into()),
            enable_flow_logs: Some(false),
            log_config: Some(SubnetworkLogConfig::fixture()),
            private_ip_google_access: Some(false),
            description: Some("test-description".into()),
            region: Some("test-region".into()),
            network: Some("test-network".into()),
            self_link: Some("test-self_link".into()),
            id: Some("test-id".into()),
        }
    }
}

/// Represents an SSL Policy resource. Use SSL policies to control SSL features, such as
/// versions and cipher suites, that are offered by Application Load Balancers and proxy Network
/// Load Balancers. For more information, read SSL policies overview.
///
/// **GCP API**: `compute.v1.SslPolicy`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/SslPolicy>
///
/// ## Coverage
/// 9 of 13 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SslPolicy {
    /// Name of the resource. The name must be 1-63 characters long, and comply with RFC1035.
    /// Specifically, the name must be 1-63 characters long and match the regular expression
    /// `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter,
    /// and all following characters must be a dash, lowercase letter, or digit, except the last
    /// character, which cannot be a dash.
    pub name: String,

    /// Profile specifies the set of SSL features that can be used by the load balancer when
    /// negotiating SSL with clients. This can be one ofCOMPATIBLE, MODERN, RESTRICTED,
    /// orCUSTOM. If using CUSTOM, the set of SSL features to enable must be specified in the
    /// customFeatures field.
    ///
    /// **Possible values**:
    /// - `COMPATIBLE` — Compatible profile. Allows the broadset set of clients, even those which support...
    /// - `CUSTOM` — Custom profile. Allow only the set of allowed SSL features specified in the cust...
    /// - `FIPS_202205` — FIPS compatible profile. Supports a reduced set of SSL features, intended to mee...
    /// - `MODERN` — Modern profile. Supports a wide set of SSL features, allowing modern clients to ...
    /// - `RESTRICTED` — Restricted profile. Supports a reduced set of SSL features, intended to meet str...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    /// The minimum version of SSL protocol that can be used by the clients to establish a
    /// connection with the load balancer. This can be one ofTLS_1_0, TLS_1_1, TLS_1_2,TLS_1_3.
    /// When set to TLS_1_3, the profile field must be set to RESTRICTED.
    ///
    /// **Possible values**:
    /// - `TLS_1_0` — TLS 1.0
    /// - `TLS_1_1` — TLS 1.1
    /// - `TLS_1_2` — TLS 1.2
    /// - `TLS_1_3` — TLS 1.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<String>,

    /// A list of features enabled when the selected profile is CUSTOM. The method returns the
    /// set of features that can be specified in this list. This field must be empty if the
    /// profile is notCUSTOM.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub custom_features: Vec<String>,

    /// Fingerprint of this resource. A hash of the contents stored in this object. This field
    /// is used in optimistic locking. This field will be ignored when inserting a SslPolicy. An
    /// up-to-date fingerprint must be provided in order to update the SslPolicy, otherwise the
    /// request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a
    /// get() request to retrieve an SslPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Output only. [Output Only] The unique identifier for the resource. This identifier is
    /// defined by the server.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Output only. [Output Only] Server-defined URL for the resource.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Output only. [Output Only] The list of features enabled in the SSL policy.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enabled_features: Vec<String>,

    /// An optional description of this resource. Provide this property when you create the
    /// resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SslPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-ssl_policy".into(),
            profile: Some("test-profile".into()),
            min_tls_version: Some("test-min_tls_version".into()),
            custom_features: vec![],
            fingerprint: Some("test-fingerprint".into()),
            id: Some("test-id".into()),
            self_link: Some("test-self_link".into()),
            enabled_features: vec![],
            description: Some("test-description".into()),
        }
    }
}

///
/// **GCP API**: `compute.v1.SslPoliciesList`
/// **Reference**: <https://cloud.google.com/compute/docs/reference/rest/v1/SslPoliciesList>
///
/// ## Coverage
/// 2 of 6 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SslPoliciesList {
    /// Output only. A list of SslPolicy resources.
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SslPolicy>,

    /// Output only. [Output Only] This token allows you to get the next page of results for
    /// list requests. If the number of results is larger thanmaxResults, use the nextPageToken
    /// as a value for the query parameter pageToken in the next list request. Subsequent list
    /// requests will have their own nextPageToken to continue paging through the results.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SslPoliciesList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

// ======================================================================
// Auto-generated dependency types (referenced via $ref)
// ======================================================================

/// A set of Confidential Instance options.
///
/// **GCP API**: `compute.v1.ConfidentialInstanceConfig`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfidentialInstanceConfig {
    /// Defines whether the instance should have confidential compute enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_confidential_compute: Option<bool>,

    /// Defines the type of technology used by the confidential instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_instance_type: Option<String>,
}

impl ConfidentialInstanceConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable_confidential_compute: Some(false),
            confidential_instance_type: Some("test-confidential_instance_type".into()),
        }
    }
}

/// A set of Display Device options
///
/// **GCP API**: `compute.v1.DisplayDevice`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayDevice {
    /// Defines whether the instance has Display enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_display: Option<bool>,
}

impl DisplayDevice {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable_display: Some(false),
        }
    }
}

/// Specifies the reservations that this instance can consume from.
///
/// **GCP API**: `compute.v1.ReservationAffinity`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservationAffinity {
    /// Specifies the type of reservation from which this instance can consume resources:
    /// ANY_RESERVATION (default),SPECIFIC_RESERVATION, or NO_RESERVATION. See Consuming
    /// reserved instances for examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_reservation_type: Option<String>,

    /// Corresponds to the label key of a reservation resource. To target aSPECIFIC_RESERVATION
    /// by name, specifygoogleapis.com/reservation-name as the key and specify the name of your
    /// reservation as its value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Corresponds to the label values of a reservation resource. This can be either a name to
    /// a reservation in the same project or "projects/different-project/reservations/some-
    /// reservation-name" to target a shared reservation in the same zone but in a different
    /// project.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}

impl ReservationAffinity {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            consume_reservation_type: Some("test-consume_reservation_type".into()),
            key: Some("test-key".into()),
            values: vec![],
        }
    }
}

/// A set of Shielded Instance options.
///
/// **GCP API**: `compute.v1.ShieldedInstanceConfig`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled.Enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_integrity_monitoring: Option<bool>,

    /// Defines whether the instance has Secure Boot enabled.Disabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_secure_boot: Option<bool>,

    /// Defines whether the instance has the vTPM enabled.Enabled by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_vtpm: Option<bool>,
}

impl ShieldedInstanceConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable_integrity_monitoring: Some(false),
            enable_secure_boot: Some(false),
            enable_vtpm: Some(false),
        }
    }
}

/// The policy describes the baseline against which Instance boot integrity is measured.
///
/// **GCP API**: `compute.v1.ShieldedInstanceIntegrityPolicy`
///
/// *Auto-generated dependency — all fields included.*
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShieldedInstanceIntegrityPolicy {
    /// Updates the integrity policy baseline using the measurements from the VM instance's most
    /// recent boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_auto_learn_policy: Option<bool>,
}

impl ShieldedInstanceIntegrityPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            update_auto_learn_policy: Some(false),
        }
    }
}

// ======================================================================
// Inline struct types (from array/object fields)
// ======================================================================

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `compute.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataItem {
    /// Value for the metadata entry. These are free-form strings, and only have meaning as
    /// interpreted by the image running in the instance. The only restriction placed on values
    /// is that their size must be less than or equal to 262144 bytes (256 KiB).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Key for the metadata entry. Keys must conform to the following regexp: [a-zA-Z0-9-_]+,
    /// and be less than 128 bytes in length. This is reflected as part of a URL in the metadata
    /// server. Additionally, to avoid ambiguity, keys must not conflict with any other metadata
    /// keys for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `compute.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallAllowed {
    /// The IP protocol to which this rule applies. The protocol type is required when creating
    /// a firewall rule. This value can either be one of the following well known protocol
    /// strings (tcp, udp,icmp, esp, ah, ipip,sctp) or the IP protocol number.
    #[serde(rename = "IPProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,

    /// An optional list of ports to which this rule applies. This field is only applicable for
    /// the UDP or TCP protocol. Each entry must be either an integer or a range. If not
    /// specified, this rule applies to connections through any port. Example inputs include:
    /// ["22"], ["80","443"], and ["12345-12349"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
}

/// Inline type extracted from a parent schema's field.
///
/// **GCP API**: `compute.v1` (inline)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallDenied {
    /// The IP protocol to which this rule applies. The protocol type is required when creating
    /// a firewall rule. This value can either be one of the following well known protocol
    /// strings (tcp, udp,icmp, esp, ah, ipip,sctp) or the IP protocol number.
    #[serde(rename = "IPProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,

    /// An optional list of ports to which this rule applies. This field is only applicable for
    /// the UDP or TCP protocol. Each entry must be either an integer or a range. If not
    /// specified, this rule applies to connections through any port. Example inputs include:
    /// ["22"], ["80","443"], and ["12345-12349"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing Disk resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskList {
    /// A list of Disk resources.
    #[serde(default)]
    pub items: Vec<Disk>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl DiskList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Instance resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceList {
    /// A list of Instance resources.
    #[serde(default)]
    pub items: Vec<Instance>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl InstanceList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Snapshot resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotList {
    /// A list of Snapshot resources.
    #[serde(default)]
    pub items: Vec<Snapshot>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SnapshotList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Address resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressList {
    /// A list of Address resources.
    #[serde(default)]
    pub items: Vec<Address>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl AddressList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing BackendService resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendServiceList {
    /// A list of BackendService resources.
    #[serde(default)]
    pub items: Vec<BackendService>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl BackendServiceList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing Firewall resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallList {
    /// A list of Firewall resources.
    #[serde(default)]
    pub items: Vec<Firewall>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl FirewallList {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            items: vec![],
            next_page_token: None,
        }
    }
}
