//! Types for the Cloud KMS API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/cloudkms/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A resource that represents a Google Cloud location.
///
/// **GCP API**: `cloudkms.v1.Location`
/// **Reference**: <https://cloud.google.com/kms//Location>
///
/// ## Coverage
/// 4 of 5 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Resource name for the location, which may vary between implementations. For example:
    /// `"projects/example-project/locations/us-east1"`
    pub name: String,

    /// The canonical id for this location. For example: `"us-east1"`.
    pub location_id: String,

    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region":
    /// "us-east1"}
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl Location {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-location".into(),
            location_id: "test-location_id".into(),
            display_name: Some("test-display_name".into()),
            labels: Default::default(),
        }
    }
}

/// A KeyRing is a toplevel logical grouping of CryptoKeys.
///
/// **GCP API**: `cloudkms.v1.KeyRing`
/// **Reference**: <https://cloud.google.com/kms//KeyRing>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyRing {
    /// Output only. The resource name for the KeyRing in the format
    /// `projects/*/locations/*/keyRings/*`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// Output only. The time at which this KeyRing was created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl KeyRing {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-key_ring".into(),
            create_time: Some("test-create_time".into()),
        }
    }
}

/// A CryptoKeyVersionTemplate specifies the properties to use when creating a new
/// CryptoKeyVersion, either manually with CreateCryptoKeyVersion or automatically as a result
/// of auto-rotation.
///
/// **GCP API**: `cloudkms.v1.CryptoKeyVersionTemplate`
/// **Reference**: <https://cloud.google.com/kms//CryptoKeyVersionTemplate>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKeyVersionTemplate {
    /// ProtectionLevel to use when creating a CryptoKeyVersion based on this template.
    /// Immutable. Defaults to SOFTWARE.
    ///
    /// **Possible values**:
    /// - `PROTECTION_LEVEL_UNSPECIFIED` — Not specified.
    /// - `SOFTWARE` — Crypto operations are performed in software.
    /// - `HSM` — Crypto operations are performed in a Hardware Security Module.
    /// - `EXTERNAL` — Crypto operations are performed by an external key manager.
    /// - `EXTERNAL_VPC` — Crypto operations are performed in an EKM-over-VPC backend.
    /// - `HSM_SINGLE_TENANT` — Crypto operations are performed in a single-tenant HSM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<String>,

    /// Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For
    /// backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is
    /// omitted and CryptoKey.purpose is ENCRYPT_DECRYPT.
    ///
    /// **Possible values**:
    /// - `CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED` — Not specified.
    /// - `GOOGLE_SYMMETRIC_ENCRYPTION` — Creates symmetric encryption keys.
    /// - `AES_128_GCM` — AES-GCM (Galois Counter Mode) using 128-bit keys.
    /// - `AES_256_GCM` — AES-GCM (Galois Counter Mode) using 256-bit keys.
    /// - `AES_128_CBC` — AES-CBC (Cipher Block Chaining Mode) using 128-bit keys.
    /// - `AES_256_CBC` — AES-CBC (Cipher Block Chaining Mode) using 256-bit keys.
    /// - `AES_128_CTR` — AES-CTR (Counter Mode) using 128-bit keys.
    /// - `AES_256_CTR` — AES-CTR (Counter Mode) using 256-bit keys.
    /// - `RSA_SIGN_PSS_2048_SHA256` — RSASSA-PSS 2048 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_3072_SHA256` — RSASSA-PSS 3072 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_4096_SHA256` — RSASSA-PSS 4096 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_4096_SHA512` — RSASSA-PSS 4096 bit key with a SHA512 digest.
    /// - `RSA_SIGN_PKCS1_2048_SHA256` — RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_3072_SHA256` — RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_4096_SHA256` — RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_4096_SHA512` — RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest.
    /// - `RSA_SIGN_RAW_PKCS1_2048` — RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key.
    /// - `RSA_SIGN_RAW_PKCS1_3072` — RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key.
    /// - `RSA_SIGN_RAW_PKCS1_4096` — RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key.
    /// - `RSA_DECRYPT_OAEP_2048_SHA256` — RSAES-OAEP 2048 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_3072_SHA256` — RSAES-OAEP 3072 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA256` — RSAES-OAEP 4096 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA512` — RSAES-OAEP 4096 bit key with a SHA512 digest.
    /// - `RSA_DECRYPT_OAEP_2048_SHA1` — RSAES-OAEP 2048 bit key with a SHA1 digest.
    /// - `RSA_DECRYPT_OAEP_3072_SHA1` — RSAES-OAEP 3072 bit key with a SHA1 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA1` — RSAES-OAEP 4096 bit key with a SHA1 digest.
    /// - `EC_SIGN_P256_SHA256` — ECDSA on the NIST P-256 curve with a SHA256 digest. Other hash functions can als...
    /// - `EC_SIGN_P384_SHA384` — ECDSA on the NIST P-384 curve with a SHA384 digest. Other hash functions can als...
    /// - `EC_SIGN_SECP256K1_SHA256` — ECDSA on the non-NIST secp256k1 curve. This curve is only supported for HSM prot...
    /// - `EC_SIGN_ED25519` — EdDSA on the Curve25519 in pure mode (taking data as input).
    /// - `HMAC_SHA256` — HMAC-SHA256 signing with a 256 bit key.
    /// - `HMAC_SHA1` — HMAC-SHA1 signing with a 160 bit key.
    /// - `HMAC_SHA384` — HMAC-SHA384 signing with a 384 bit key.
    /// - `HMAC_SHA512` — HMAC-SHA512 signing with a 512 bit key.
    /// - `HMAC_SHA224` — HMAC-SHA224 signing with a 224 bit key.
    /// - `EXTERNAL_SYMMETRIC_ENCRYPTION` — Algorithm representing symmetric encryption by an external key manager.
    /// - `ML_KEM_768` — ML-KEM-768 (FIPS 203)
    /// - `ML_KEM_1024` — ML-KEM-1024 (FIPS 203)
    /// - `KEM_XWING` — X-Wing hybrid KEM combining ML-KEM-768 with X25519 following datatracker.ietf.or...
    /// - `PQ_SIGN_ML_DSA_44` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_65` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_87` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_SLH_DSA_SHA2_128S` — The post-quantum stateless hash-based digital signature algorithm, at security l...
    /// - `PQ_SIGN_HASH_SLH_DSA_SHA2_128S_SHA256` — The post-quantum stateless hash-based digital signature algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_44_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_65_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_87_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}

impl CryptoKeyVersionTemplate {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            protection_level: Some("test-protection_level".into()),
            algorithm: Some("test-algorithm".into()),
        }
    }
}

/// A CryptoKeyVersion represents an individual cryptographic key, and the associated key
/// material. An ENABLED version can be used for cryptographic operations. For security reasons,
/// the raw cryptographic key material represented by a CryptoKeyVersion can never be viewed or
/// exported. It can only be used to encrypt, decrypt, or sign data when an authorized user or
/// application invokes Cloud KMS.
///
/// **GCP API**: `cloudkms.v1.CryptoKeyVersion`
/// **Reference**: <https://cloud.google.com/kms//CryptoKeyVersion>
///
/// ## Coverage
/// 5 of 16 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKeyVersion {
    /// Output only. The resource name for this CryptoKeyVersion in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// The current state of the CryptoKeyVersion.
    ///
    /// **Possible values**:
    /// - `CRYPTO_KEY_VERSION_STATE_UNSPECIFIED` — Not specified.
    /// - `PENDING_GENERATION` — This version is still being generated. It may not be used, enabled, disabled, or...
    /// - `ENABLED` — This version may be used for cryptographic operations.
    /// - `DISABLED` — This version may not be used, but the key material is still available, and the v...
    /// - `DESTROYED` — The key material of this version is destroyed and no longer stored. This version...
    /// - `DESTROY_SCHEDULED` — This version is scheduled for destruction, and will be destroyed soon. Call Rest...
    /// - `PENDING_IMPORT` — This version is still being imported. It may not be used, enabled, disabled, or ...
    /// - `IMPORT_FAILED` — This version was not imported successfully. It may not be used, enabled, disable...
    /// - `GENERATION_FAILED` — This version was not generated successfully. It may not be used, enabled, disabl...
    /// - `PENDING_EXTERNAL_DESTRUCTION` — This version was destroyed, and it may not be used or enabled again. Cloud KMS i...
    /// - `EXTERNAL_DESTRUCTION_FAILED` — This version was destroyed, and it may not be used or enabled again. However, Cl...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
    ///
    /// **Possible values**:
    /// - `CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED` — Not specified.
    /// - `GOOGLE_SYMMETRIC_ENCRYPTION` — Creates symmetric encryption keys.
    /// - `AES_128_GCM` — AES-GCM (Galois Counter Mode) using 128-bit keys.
    /// - `AES_256_GCM` — AES-GCM (Galois Counter Mode) using 256-bit keys.
    /// - `AES_128_CBC` — AES-CBC (Cipher Block Chaining Mode) using 128-bit keys.
    /// - `AES_256_CBC` — AES-CBC (Cipher Block Chaining Mode) using 256-bit keys.
    /// - `AES_128_CTR` — AES-CTR (Counter Mode) using 128-bit keys.
    /// - `AES_256_CTR` — AES-CTR (Counter Mode) using 256-bit keys.
    /// - `RSA_SIGN_PSS_2048_SHA256` — RSASSA-PSS 2048 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_3072_SHA256` — RSASSA-PSS 3072 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_4096_SHA256` — RSASSA-PSS 4096 bit key with a SHA256 digest.
    /// - `RSA_SIGN_PSS_4096_SHA512` — RSASSA-PSS 4096 bit key with a SHA512 digest.
    /// - `RSA_SIGN_PKCS1_2048_SHA256` — RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_3072_SHA256` — RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_4096_SHA256` — RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest.
    /// - `RSA_SIGN_PKCS1_4096_SHA512` — RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest.
    /// - `RSA_SIGN_RAW_PKCS1_2048` — RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key.
    /// - `RSA_SIGN_RAW_PKCS1_3072` — RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key.
    /// - `RSA_SIGN_RAW_PKCS1_4096` — RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key.
    /// - `RSA_DECRYPT_OAEP_2048_SHA256` — RSAES-OAEP 2048 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_3072_SHA256` — RSAES-OAEP 3072 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA256` — RSAES-OAEP 4096 bit key with a SHA256 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA512` — RSAES-OAEP 4096 bit key with a SHA512 digest.
    /// - `RSA_DECRYPT_OAEP_2048_SHA1` — RSAES-OAEP 2048 bit key with a SHA1 digest.
    /// - `RSA_DECRYPT_OAEP_3072_SHA1` — RSAES-OAEP 3072 bit key with a SHA1 digest.
    /// - `RSA_DECRYPT_OAEP_4096_SHA1` — RSAES-OAEP 4096 bit key with a SHA1 digest.
    /// - `EC_SIGN_P256_SHA256` — ECDSA on the NIST P-256 curve with a SHA256 digest. Other hash functions can als...
    /// - `EC_SIGN_P384_SHA384` — ECDSA on the NIST P-384 curve with a SHA384 digest. Other hash functions can als...
    /// - `EC_SIGN_SECP256K1_SHA256` — ECDSA on the non-NIST secp256k1 curve. This curve is only supported for HSM prot...
    /// - `EC_SIGN_ED25519` — EdDSA on the Curve25519 in pure mode (taking data as input).
    /// - `HMAC_SHA256` — HMAC-SHA256 signing with a 256 bit key.
    /// - `HMAC_SHA1` — HMAC-SHA1 signing with a 160 bit key.
    /// - `HMAC_SHA384` — HMAC-SHA384 signing with a 384 bit key.
    /// - `HMAC_SHA512` — HMAC-SHA512 signing with a 512 bit key.
    /// - `HMAC_SHA224` — HMAC-SHA224 signing with a 224 bit key.
    /// - `EXTERNAL_SYMMETRIC_ENCRYPTION` — Algorithm representing symmetric encryption by an external key manager.
    /// - `ML_KEM_768` — ML-KEM-768 (FIPS 203)
    /// - `ML_KEM_1024` — ML-KEM-1024 (FIPS 203)
    /// - `KEM_XWING` — X-Wing hybrid KEM combining ML-KEM-768 with X25519 following datatracker.ietf.or...
    /// - `PQ_SIGN_ML_DSA_44` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_65` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_87` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_SLH_DSA_SHA2_128S` — The post-quantum stateless hash-based digital signature algorithm, at security l...
    /// - `PQ_SIGN_HASH_SLH_DSA_SHA2_128S_SHA256` — The post-quantum stateless hash-based digital signature algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_44_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_65_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    /// - `PQ_SIGN_ML_DSA_87_EXTERNAL_MU` — The post-quantum Module-Lattice-Based Digital Signature Algorithm, at security l...
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Output only. The ProtectionLevel describing how crypto operations are performed with
    /// this CryptoKeyVersion.
    ///
    /// **Possible values**:
    /// - `PROTECTION_LEVEL_UNSPECIFIED` — Not specified.
    /// - `SOFTWARE` — Crypto operations are performed in software.
    /// - `HSM` — Crypto operations are performed in a Hardware Security Module.
    /// - `EXTERNAL` — Crypto operations are performed by an external key manager.
    /// - `EXTERNAL_VPC` — Crypto operations are performed in an EKM-over-VPC backend.
    /// - `HSM_SINGLE_TENANT` — Crypto operations are performed in a single-tenant HSM.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_level: Option<String>,

    /// Output only. The time at which this CryptoKeyVersion was created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl CryptoKeyVersion {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-crypto_key_version".into(),
            state: Some("test-state".into()),
            algorithm: Some("test-algorithm".into()),
            protection_level: Some("test-protection_level".into()),
            create_time: Some("test-create_time".into()),
        }
    }
}

/// A CryptoKey represents a logical key that can be used for cryptographic operations. A
/// CryptoKey is made up of zero or more versions, which represent the actual key material used
/// in cryptographic operations.
///
/// **GCP API**: `cloudkms.v1.CryptoKey`
/// **Reference**: <https://cloud.google.com/kms//CryptoKey>
///
/// ## Coverage
/// 9 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoKey {
    /// Output only. The resource name for this CryptoKey in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// Immutable. The immutable purpose of this CryptoKey.
    ///
    /// **Possible values**:
    /// - `CRYPTO_KEY_PURPOSE_UNSPECIFIED` — Not specified.
    /// - `ENCRYPT_DECRYPT` — CryptoKeys with this purpose may be used with Encrypt and Decrypt.
    /// - `ASYMMETRIC_SIGN` — CryptoKeys with this purpose may be used with AsymmetricSign and GetPublicKey.
    /// - `ASYMMETRIC_DECRYPT` — CryptoKeys with this purpose may be used with AsymmetricDecrypt and GetPublicKey...
    /// - `RAW_ENCRYPT_DECRYPT` — CryptoKeys with this purpose may be used with RawEncrypt and RawDecrypt. This pu...
    /// - `MAC` — CryptoKeys with this purpose may be used with MacSign.
    /// - `KEY_ENCAPSULATION` — CryptoKeys with this purpose may be used with GetPublicKey and Decapsulate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,

    /// Output only. The time at which this CryptoKey was created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// At next_rotation_time, the Key Management Service will automatically: 1. Create a new
    /// version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed
    /// manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect
    /// next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For
    /// other keys, this field must be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_time: Option<String>,

    /// next_rotation_time will be advanced by this period when the service automatically
    /// rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period
    /// is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support
    /// automatic rotation. For other keys, this field must be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_period: Option<String>,

    /// Labels with user-defined metadata. For more information, see [Labeling
    /// Keys](https://cloud.google.com/kms/docs/labeling-keys).
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. A copy of the "primary" CryptoKeyVersion that will be used by Encrypt when
    /// this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be
    /// updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a
    /// primary. For other keys, this field will be omitted.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<CryptoKeyVersion>,

    /// A template describing settings for new CryptoKeyVersion instances. The properties of new
    /// CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are
    /// controlled by this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_template: Option<CryptoKeyVersionTemplate>,

    /// Immutable. Whether this key may contain imported versions only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_only: Option<bool>,
}

impl CryptoKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-crypto_key".into(),
            purpose: Some("test-purpose".into()),
            create_time: Some("test-create_time".into()),
            next_rotation_time: Some("test-next_rotation_time".into()),
            rotation_period: Some("test-rotation_period".into()),
            labels: Default::default(),
            primary: Some(CryptoKeyVersion::fixture()),
            version_template: Some(CryptoKeyVersionTemplate::fixture()),
            import_only: Some(false),
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
/// **GCP API**: `cloudkms.v1.Expr`
/// **Reference**: <https://cloud.google.com/kms//Expr>
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

    /// Optional. String indicating the location of the expression for error reporting, e.g. a
    /// file name and a position in the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl Expr {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            expression: Some("test-expression".into()),
            title: Some("test-title".into()),
            description: Some("test-description".into()),
            location: Some("test-location".into()),
        }
    }
}

/// Associates `members`, or principals, with a `role`.
///
/// **GCP API**: `cloudkms.v1.Binding`
/// **Reference**: <https://cloud.google.com/kms//Binding>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// Role that is assigned to the list of `members`, or principals. For example,
    /// `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and
    /// permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-
    /// overview). For a list of the available pre-defined roles, see
    /// [here](https://cloud.google.com/iam/docs/understanding-roles).
    pub role: String,

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
            role: "test-role".into(),
            members: vec![],
            condition: Some(Expr::fixture()),
        }
    }
}

/// Provides the configuration for logging a type of permissions. Example: {
/// "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [
/// "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and
/// 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
///
/// **GCP API**: `cloudkms.v1.AuditLogConfig`
/// **Reference**: <https://cloud.google.com/kms//AuditLogConfig>
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
/// **GCP API**: `cloudkms.v1.AuditConfig`
/// **Reference**: <https://cloud.google.com/kms//AuditConfig>
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
/// **GCP API**: `cloudkms.v1.Policy`
/// **Reference**: <https://cloud.google.com/kms//Policy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
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
    pub bindings: Vec<Binding>,

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

impl Policy {
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

/// Request message for `SetIamPolicy` method.
///
/// **GCP API**: `cloudkms.v1.SetIamPolicyRequest`
/// **Reference**: <https://cloud.google.com/kms//SetIamPolicyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is
    /// limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud
    /// services (such as Projects) might reject them.
    pub policy: Policy,

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
            policy: Policy::fixture(),
            update_mask: Some("test-update_mask".into()),
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing Location resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLocationsResponse {
    /// A list of Location resources.
    #[serde(default)]
    pub locations: Vec<Location>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListLocationsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            locations: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing KeyRing resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListKeyRingsResponse {
    /// A list of KeyRing resources.
    #[serde(default)]
    pub key_rings: Vec<KeyRing>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListKeyRingsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_rings: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing CryptoKey resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCryptoKeysResponse {
    /// A list of CryptoKey resources.
    #[serde(default)]
    pub crypto_keys: Vec<CryptoKey>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListCryptoKeysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            crypto_keys: vec![],
            next_page_token: None,
        }
    }
}
