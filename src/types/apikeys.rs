//! Types for the API Keys API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/apikeys/v2/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A restriction for a specific service and optionally one or multiple specific methods. Both
/// fields are case insensitive.
///
/// **GCP API**: `apikeys.v2.V2ApiTarget`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2ApiTarget>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2ApiTarget {
    /// The service for this restriction. It should be the canonical service name, for example:
    /// `translate.googleapis.com`. You can use [`gcloud services
    /// list`](https://cloud.google.com/sdk/gcloud/reference/services/list) to get a list of
    /// services that are enabled in the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Optional. List of one or more methods that can be called. If empty, all methods for the
    /// service are allowed. A wildcard (*) can be used as the last symbol. Valid examples:
    /// `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*`
    /// `translate.googleapis.com.Get*`
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<String>,
}

impl V2ApiTarget {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service: Some("test-service".into()),
            methods: vec![],
        }
    }
}

/// The HTTP referrers (websites) that are allowed to use the key.
///
/// **GCP API**: `apikeys.v2.V2BrowserKeyRestrictions`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2BrowserKeyRestrictions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2BrowserKeyRestrictions {
    /// A list of regular expressions for the referrer URLs that are allowed to make API calls
    /// with this key.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_referrers: Vec<String>,
}

impl V2BrowserKeyRestrictions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allowed_referrers: vec![],
        }
    }
}

/// The IP addresses of callers that are allowed to use the key.
///
/// **GCP API**: `apikeys.v2.V2ServerKeyRestrictions`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2ServerKeyRestrictions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2ServerKeyRestrictions {
    /// A list of the caller IP addresses that are allowed to make API calls with this key.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_ips: Vec<String>,
}

impl V2ServerKeyRestrictions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allowed_ips: vec![],
        }
    }
}

/// Identifier of an Android application for key use.
///
/// **GCP API**: `apikeys.v2.V2AndroidApplication`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2AndroidApplication>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2AndroidApplication {
    /// The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable :
    /// DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or
    /// DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1_fingerprint: Option<String>,

    /// The package name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

impl V2AndroidApplication {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            sha1_fingerprint: Some("test-sha1_fingerprint".into()),
            package_name: Some("test-package_name".into()),
        }
    }
}

/// The Android apps that are allowed to use the key.
///
/// **GCP API**: `apikeys.v2.V2AndroidKeyRestrictions`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2AndroidKeyRestrictions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2AndroidKeyRestrictions {
    /// A list of Android applications that are allowed to make API calls with this key.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_applications: Vec<V2AndroidApplication>,
}

impl V2AndroidKeyRestrictions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allowed_applications: vec![],
        }
    }
}

/// The iOS apps that are allowed to use the key.
///
/// **GCP API**: `apikeys.v2.V2IosKeyRestrictions`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2IosKeyRestrictions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2IosKeyRestrictions {
    /// A list of bundle IDs that are allowed when making API calls with this key.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_bundle_ids: Vec<String>,
}

impl V2IosKeyRestrictions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allowed_bundle_ids: vec![],
        }
    }
}

/// Describes the restrictions on the key.
///
/// **GCP API**: `apikeys.v2.V2Restrictions`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2Restrictions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2Restrictions {
    /// The HTTP referrers (websites) that are allowed to use the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_key_restrictions: Option<V2BrowserKeyRestrictions>,

    /// The IP addresses of callers that are allowed to use the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_key_restrictions: Option<V2ServerKeyRestrictions>,

    /// The Android apps that are allowed to use the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_key_restrictions: Option<V2AndroidKeyRestrictions>,

    /// The iOS apps that are allowed to use the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_key_restrictions: Option<V2IosKeyRestrictions>,

    /// A restriction for a specific service and optionally one or more specific methods.
    /// Requests are allowed if they match any of these restrictions. If no restrictions are
    /// specified, all targets are allowed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub api_targets: Vec<V2ApiTarget>,
}

impl V2Restrictions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            browser_key_restrictions: Some(V2BrowserKeyRestrictions::fixture()),
            server_key_restrictions: Some(V2ServerKeyRestrictions::fixture()),
            android_key_restrictions: Some(V2AndroidKeyRestrictions::fixture()),
            ios_key_restrictions: Some(V2IosKeyRestrictions::fixture()),
            api_targets: vec![],
        }
    }
}

/// The representation of a key managed by the API Keys API.
///
/// **GCP API**: `apikeys.v2.V2Key`
/// **Reference**: <https://cloud.google.com/api-keys/docs/V2Key>
///
/// ## Coverage
/// 9 of 11 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2Key {
    /// Output only. The resource name of the key. The `name` has the form:
    /// `projects//locations/global/keys/`. For example:
    /// `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2` NOTE:
    /// Key is a global resource; hence the only supported value for location is `global`.
    ///
    /// *Output-only field.*
    pub name: String,

    /// Output only. Unique id in UUID4 format.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Human-readable display name of this key that you can modify. The maximum length is 63
    /// characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Output only. A timestamp identifying the time this key was originally created.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. A timestamp identifying the time this key was last updated.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// Annotations is an unstructured key-value map stored with a policy that may be set by
    /// external tools to store and retrieve arbitrary metadata. They are not queryable and
    /// should be preserved when modifying objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, String>,

    /// Key restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<V2Restrictions>,

    /// A checksum computed by the server based on the current value of the Key resource. This
    /// may be sent on update and delete requests to ensure the client has an up-to-date value
    /// before proceeding. See https://google.aip.dev/154.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Optional. The email address of [the service
    /// account](https://cloud.google.com/iam/docs/service-accounts) the key is bound to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_email: Option<String>,
}

impl V2Key {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-v2_key".into(),
            uid: Some("test-uid".into()),
            display_name: Some("test-display_name".into()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
            annotations: Default::default(),
            restrictions: Some(V2Restrictions::fixture()),
            etag: Some("test-etag".into()),
            service_account_email: Some("test-service_account_email".into()),
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing V2Key resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2ListKeysResponse {
    /// A list of V2Key resources.
    #[serde(default)]
    pub keys: Vec<V2Key>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl V2ListKeysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            keys: vec![],
            next_page_token: None,
        }
    }
}
