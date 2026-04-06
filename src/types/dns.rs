//! Types for the Cloud DNS API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/dns/v1/rest`

use serde::{Deserialize, Serialize};

///
/// **GCP API**: `dns.v1.ManagedZoneDnsSecConfig`
/// **Reference**: <https://cloud.google.com/dns/docs/ManagedZoneDnsSecConfig>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneDnsSecConfig {
    /// Specifies whether DNSSEC is enabled, and what mode it is in.
    ///
    /// **Possible values**:
    /// - `off` — DNSSEC is disabled; the zone is not signed.
    /// - `on` — DNSSEC is enabled; the zone is signed and fully managed.
    /// - `transfer` — DNSSEC is enabled, but in a "transfer" mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Specifies the mechanism for authenticated denial-of-existence responses. Can only be
    /// changed while the state is OFF.
    ///
    /// **Possible values**:
    /// - `nsec` — Indicates that Cloud DNS will sign records in the managed zone according to RFC ...
    /// - `nsec3` — Indicates that Cloud DNS will sign records in the managed zone according to RFC ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_existence: Option<String>,

    /// Specifies parameters for generating initial DnsKeys for this ManagedZone. Can only be
    /// changed while the state is OFF.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_key_specs: Vec<DnsKeySpec>,
}

impl ManagedZoneDnsSecConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("test-state".into()),
            non_existence: Some("test-non_existence".into()),
            default_key_specs: vec![],
        }
    }
}

/// Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone
/// and as default when adding a new DnsKey.
///
/// **GCP API**: `dns.v1.DnsKeySpec`
/// **Reference**: <https://cloud.google.com/dns/docs/DnsKeySpec>
///
/// ## Coverage
/// 3 of 4 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsKeySpec {
    /// Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key
    /// signing keys have the Secure Entry Point flag set and, when active, are only used to
    /// sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry
    /// Point flag set and are used to sign all other types of resource record sets.
    ///
    /// **Possible values**:
    /// - `keySigning`
    /// - `zoneSigning`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,

    /// String mnemonic specifying the DNSSEC algorithm of this key.
    ///
    /// **Possible values**:
    /// - `rsasha1`
    /// - `rsasha256`
    /// - `rsasha512`
    /// - `ecdsap256sha256`
    /// - `ecdsap384sha384`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Length of the keys in bits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_length: Option<u32>,
}

impl DnsKeySpec {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_type: Some("test-key_type".into()),
            algorithm: Some("test-algorithm".into()),
            key_length: Some(100),
        }
    }
}

/// Cloud Logging configurations for publicly visible zones.
///
/// **GCP API**: `dns.v1.ManagedZoneCloudLoggingConfig`
/// **Reference**: <https://cloud.google.com/dns/docs/ManagedZoneCloudLoggingConfig>
///
/// ## Coverage
/// 1 of 2 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZoneCloudLoggingConfig {
    /// If set, enable query logging for this ManagedZone. False by default, making logging opt-
    /// in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,
}

impl ManagedZoneCloudLoggingConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enable_logging: Some(false),
        }
    }
}

/// A zone is a subtree of the DNS namespace under one administrative responsibility. A
/// ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service.
///
/// **GCP API**: `dns.v1.ManagedZone`
/// **Reference**: <https://cloud.google.com/dns/docs/ManagedZone>
///
/// ## Coverage
/// 8 of 17 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZone {
    /// User assigned name for this resource. Must be unique within the project. The name must
    /// be 1-63 characters long, must begin with a letter, end with a letter or digit, and only
    /// contain lowercase letters, digits or dashes.
    pub name: String,

    /// The DNS name of this managed zone, for instance "example.com.".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,

    /// A mutable string of at most 1024 characters associated with this resource for the user's
    /// convenience. Has no effect on the managed zone's function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Unique identifier for the resource; defined by the server (output only)
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// DNSSEC configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_config: Option<ManagedZoneDnsSecConfig>,

    /// The `cloudLoggingConfig` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_logging_config: Option<ManagedZoneCloudLoggingConfig>,

    /// The zone's visibility: public zones are exposed to the Internet, while private zones are
    /// visible only to Virtual Private Cloud resources.
    ///
    /// **Possible values**:
    /// - `public` — Indicates that records in this zone can be queried from the public internet.
    /// - `private` — Indicates that records in this zone cannot be queried from the public internet. ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,

    /// Delegate your managed_zone to these virtual name servers; defined by the server (output
    /// only)
    ///
    /// *Output-only field.*
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name_servers: Vec<String>,
}

impl ManagedZone {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-managed_zone".into(),
            dns_name: Some("test-dns_name".into()),
            description: Some("test-description".into()),
            id: Some("test-id".into()),
            dnssec_config: Some(ManagedZoneDnsSecConfig::fixture()),
            cloud_logging_config: Some(ManagedZoneCloudLoggingConfig::fixture()),
            visibility: Some("test-visibility".into()),
            name_servers: vec![],
        }
    }
}

///
/// **GCP API**: `dns.v1.ManagedZonesListResponse`
/// **Reference**: <https://cloud.google.com/dns/docs/ManagedZonesListResponse>
///
/// ## Coverage
/// 2 of 3 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedZonesListResponse {
    /// The managed zone resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub managed_zones: Vec<ManagedZone>,

    /// This field indicates that more results are available beyond the last page displayed. To
    /// fetch the results, make another list request and use this value as your page token. This
    /// lets you retrieve the complete contents of a very large collection one page at a time.
    /// However, if the contents of the collection change between the first and last paginated
    /// list request, the set of all elements returned are an inconsistent view of the
    /// collection. You can't retrieve a consistent snapshot of a collection larger than the
    /// maximum page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ManagedZonesListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            managed_zones: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

///
/// **GCP API**: `dns.v1.PolicyNetwork`
/// **Reference**: <https://cloud.google.com/dns/docs/PolicyNetwork>
///
/// ## Coverage
/// 1 of 2 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyNetwork {
    /// The fully qualified URL of the VPC network to bind to. This should be formatted like
    /// https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_url: Option<String>,
}

impl PolicyNetwork {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_url: Some("test-network_url".into()),
        }
    }
}

/// A policy is a collection of DNS rules applied to one or more Virtual Private Cloud
/// resources.
///
/// **GCP API**: `dns.v1.Policy`
/// **Reference**: <https://cloud.google.com/dns/docs/Policy>
///
/// ## Coverage
/// 6 of 9 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    /// User-assigned name for this policy.
    pub name: String,

    /// Unique identifier for the resource; defined by the server (output only).
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A mutable string of at most 1024 characters associated with this resource for the user's
    /// convenience. Has no effect on the policy's function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Allows networks bound to this policy to receive DNS queries sent by VMs or applications
    /// over VPN connections. When enabled, a virtual IP address is allocated from each of the
    /// subnetworks that are bound to this policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inbound_forwarding: Option<bool>,

    /// Controls whether logging is enabled for the networks bound to this policy. Defaults to
    /// no logging if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,

    /// List of network names specifying networks to which this policy is applied.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub networks: Vec<PolicyNetwork>,
}

impl Policy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-policy".into(),
            id: Some("test-id".into()),
            description: Some("test-description".into()),
            enable_inbound_forwarding: Some(false),
            enable_logging: Some(false),
            networks: vec![],
        }
    }
}

///
/// **GCP API**: `dns.v1.PoliciesListResponse`
/// **Reference**: <https://cloud.google.com/dns/docs/PoliciesListResponse>
///
/// ## Coverage
/// 2 of 3 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesListResponse {
    /// The policy resources.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<Policy>,

    /// This field indicates that more results are available beyond the last page displayed. To
    /// fetch the results, make another list request and use this value as your page token. This
    /// lets you retrieve the complete contents of a very large collection one page at a time.
    /// However, if the contents of the collection change between the first and last paginated
    /// list request, the set of all elements returned are an inconsistent view of the
    /// collection. You can't retrieve a consistent snapshot of a collection larger than the
    /// maximum page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl PoliciesListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policies: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

///
/// **GCP API**: `dns.v1.PoliciesPatchResponse`
/// **Reference**: <https://cloud.google.com/dns/docs/PoliciesPatchResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesPatchResponse {
    /// The `policy` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

impl PoliciesPatchResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            policy: Some(Policy::fixture()),
        }
    }
}

/// An operation represents a successful mutation performed on a Cloud DNS resource. Operations
/// provide:
/// - An audit log of server resource mutations.
/// - A way to recover/retry API calls in the case where the response is never received by the
///   caller. Use the caller specified client_operation_id.
///
/// **GCP API**: `dns.v1.Operation`
/// **Reference**: <https://cloud.google.com/dns/docs/Operation>
///
/// ## Coverage
/// 3 of 8 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsOperation {
    /// Unique identifier for the resource. This is the client_operation_id if the client
    /// specified it when the mutation was initiated, otherwise, it is generated by the server.
    /// The name must be 1-63 characters long and match the regular expression [-a-z0-9]?
    /// (output only)
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only).
    /// A status of "DONE" means that the request to update the authoritative servers has been
    /// sent, but the servers might not be updated yet.
    ///
    /// **Possible values**:
    /// - `pending`
    /// - `done`
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The `kind` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DnsOperation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            status: Some("test-status".into()),
            kind: Some("test-kind".into()),
        }
    }
}
