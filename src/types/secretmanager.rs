//! Types for the Secret Manager API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/secretmanager/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Secret is a logical secret whose value and versions can be accessed. A Secret is made up
/// of zero or more SecretVersions that represent the secret data.
///
/// **GCP API**: `secretmanager.v1.Secret`
/// **Reference**: <https://cloud.google.com/secret-manager//Secret>
///
/// ## Coverage
/// 8 of 14 fields included.
/// Omitted fields:
/// - `customerManagedEncryption` — CMEK support - advanced feature
/// - `topics` — Pub/Sub integration - uncommon use case
/// - `ttl` — TTL input field - rarely used directly
/// - `versionAliases` — Version aliasing - requires version operations
/// - `versionDestroyTtl` — Delayed destruction - advanced feature
/// - `tags` — Resource tags - uncommon use case
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    /// Output only. The resource name of the Secret in the format `projects/*/secrets/*`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// The labels assigned to this Secret. Label keys must be between 1 and 63 characters long,
    /// have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63
    /// characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the
    /// following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels
    /// can be assigned to a given resource.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Optional. Custom metadata about the secret. Annotations are distinct from various forms
    /// of labels. Annotations exist to allow client tools to store their own state information
    /// without requiring a database. Annotation keys must be between 1 and 63 characters long,
    /// have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character
    /// ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and alphanumerics in
    /// between these symbols. The total size of annotation keys and values must be less than
    /// 16KiB.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,

    /// Optional. Etag of the currently stored Secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Optional. Immutable. The replication policy of the secret data attached to the Secret.
    /// The replication policy cannot be changed after the Secret has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<Replication>,

    /// Output only. The time at which the Secret was created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Optional. Timestamp in UTC when the Secret is scheduled to expire. This is always
    /// provided on output, regardless of what was sent on input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,

    /// Optional. Rotation policy attached to the Secret. May be excluded if there is no
    /// rotation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Rotation>,
}

impl Secret {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-secret".into(),
            labels: Default::default(),
            annotations: Default::default(),
            etag: Some("test-etag".into()),
            replication: Some(Replication::fixture()),
            create_time: Some("test-create_time".into()),
            expire_time: Some("test-expire_time".into()),
            rotation: Some(Rotation::fixture()),
        }
    }
}

/// A policy that defines the replication and encryption configuration of data.
///
/// **GCP API**: `secretmanager.v1.Replication`
/// **Reference**: <https://cloud.google.com/secret-manager//Replication>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replication {
    /// The Secret will only be replicated into the locations specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_managed: Option<UserManaged>,

    /// The Secret will automatically be replicated without any restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<Automatic>,
}

impl Replication {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            user_managed: Some(UserManaged::fixture()),
            automatic: Some(Automatic::fixture()),
        }
    }
}

/// A replication policy that replicates the Secret payload without any restrictions.
///
/// **GCP API**: `secretmanager.v1.Automatic`
/// **Reference**: <https://cloud.google.com/secret-manager//Automatic>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Automatic {
    /// Optional. The customer-managed encryption configuration of the Secret. If no
    /// configuration is provided, Google-managed default encryption is used. Updates to the
    /// Secret encryption configuration only apply to SecretVersions added afterwards. They do
    /// not apply retroactively to existing SecretVersions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_encryption: Option<CustomerManagedEncryption>,
}

impl Automatic {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            customer_managed_encryption: Some(CustomerManagedEncryption::fixture()),
        }
    }
}

/// A replication policy that replicates the Secret payload into the locations specified in
/// Replication.UserManaged.replicas
///
/// **GCP API**: `secretmanager.v1.UserManaged`
/// **Reference**: <https://cloud.google.com/secret-manager//UserManaged>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserManaged {
    /// Required. The list of Replicas for this Secret. Cannot be empty.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub replicas: Vec<Replica>,
}

impl UserManaged {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { replicas: vec![] }
    }
}

/// Represents a Replica for this Secret.
///
/// **GCP API**: `secretmanager.v1.Replica`
/// **Reference**: <https://cloud.google.com/secret-manager//Replica>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replica {
    /// The canonical IDs of the location to replicate data. For example: `"us-east1"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Optional. The customer-managed encryption configuration of the User-Managed Replica. If
    /// no configuration is provided, Google-managed default encryption is used. Updates to the
    /// Secret encryption configuration only apply to SecretVersions added afterwards. They do
    /// not apply retroactively to existing SecretVersions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_encryption: Option<CustomerManagedEncryption>,
}

impl Replica {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: Some("test-location".into()),
            customer_managed_encryption: Some(CustomerManagedEncryption::fixture()),
        }
    }
}

/// Configuration for encrypting secret payloads using customer-managed encryption keys (CMEK).
///
/// **GCP API**: `secretmanager.v1.CustomerManagedEncryption`
/// **Reference**: <https://cloud.google.com/secret-manager//CustomerManagedEncryption>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerManagedEncryption {
    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt secret payloads.
    /// For secrets using the UserManaged replication policy type, Cloud KMS CryptoKeys must
    /// reside in the same location as the replica location. For secrets using the Automatic
    /// replication policy type, Cloud KMS CryptoKeys must reside in `global`. The expected
    /// format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_name: Option<String>,
}

impl CustomerManagedEncryption {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kms_key_name: Some("test-kms_key_name".into()),
        }
    }
}

/// The rotation time and period for a Secret. At next_rotation_time, Secret Manager will send a
/// Pub/Sub notification to the topics configured on the Secret. Secret.topics must be set to
/// configure rotation.
///
/// **GCP API**: `secretmanager.v1.Rotation`
/// **Reference**: <https://cloud.google.com/secret-manager//Rotation>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    /// Optional. Timestamp in UTC at which the Secret is scheduled to rotate. Cannot be set to
    /// less than 300s (5 min) in the future and at most 3153600000s (100 years).
    /// next_rotation_time MUST be set if rotation_period is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_time: Option<String>,

    /// Input only. The Duration between rotation notifications. Must be in seconds and at least
    /// 3600s (1h) and at most 3153600000s (100 years). If rotation_period is set,
    /// next_rotation_time must be set. next_rotation_time will be advanced by this period when
    /// the service automatically sends rotation notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period: Option<String>,
}

impl Rotation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            next_rotation_time: Some("test-next_rotation_time".into()),
            rotation_period: Some("test-rotation_period".into()),
        }
    }
}

/// Response message for SecretManagerService.ListSecrets.
///
/// **GCP API**: `secretmanager.v1.ListSecretsResponse`
/// **Reference**: <https://cloud.google.com/secret-manager//ListSecretsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsResponse {
    /// The total number of Secrets but 0 when the ListSecretsRequest.filter field is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,

    /// A token to retrieve the next page of results. Pass this value in
    /// ListSecretsRequest.page_token to retrieve the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,

    /// The list of Secrets sorted in reverse by create_time (newest first).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<Secret>,
}

impl ListSecretsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            total_size: Some(100),
            next_page_token: Some("test-next_page_token".into()),
            secrets: vec![],
        }
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in
/// your APIs. A typical example is to use it as the request or the response type of an API
/// method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns
/// (google.protobuf.Empty); }
///
/// **GCP API**: `secretmanager.v1.Empty`
/// **Reference**: <https://cloud.google.com/secret-manager//Empty>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Empty {}

impl Empty {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}
