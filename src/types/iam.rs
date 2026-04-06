//! Types for the IAM API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/iam/v1/rest`

use serde::{Deserialize, Serialize};

/// An IAM service account. A service account is an account for an application or a virtual
/// machine (VM) instance, not a person. You can use a service account to call Google APIs. To
/// learn more, read the [overview of service
/// accounts](https://cloud.google.com/iam/help/service-accounts/overview). When you create a
/// service account, you specify the project ID that owns the service account, as well as a name
/// that must be unique within the project. IAM uses these values to create an email address
/// that identifies the service account. //
///
/// **GCP API**: `iam.v1.ServiceAccount`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/ServiceAccount>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    /// The resource name of the service account. Use one of the following formats:
    /// * `projects/{PROJECT_ID}/serviceAccounts/{EMAIL_ADDRESS}`
    /// * `projects/{PROJECT_ID}/serviceAccounts/{UNIQUE_ID}` As an alternative, you can use the
    ///   `-` wildcard character instead of the project ID:
    /// * `projects/-/serviceAccounts/{EMAIL_ADDRESS}`
    /// * `projects/-/serviceAccounts/{UNIQUE_ID}` When possible, avoid using the `-` wildcard
    ///   character, because it can cause response messages to contain misleading error codes.
    ///   For example, if you try to access the service account
    ///   `projects/-/serviceAccounts/fake@example.com`, which does not exist, the response
    ///   contains an HTTP `403 Forbidden` error instead of a `404 Not Found` error.
    pub name: String,

    /// Output only. The ID of the project that owns the service account.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// Output only. The unique, stable numeric ID for the service account. Each service account
    /// retains its unique ID even if you delete the service account. For example, if you delete
    /// a service account, then create a new service account with the same name, the new service
    /// account has a different unique ID than the deleted service account.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Output only. The email address of the service account.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Optional. A user-specified, human-readable name for the service account. The maximum
    /// length is 100 UTF-8 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional. A user-specified, human-readable description of the service account. The
    /// maximum length is 256 UTF-8 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output only. Whether the service account is disabled.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Output only. The OAuth 2.0 client ID for the service account.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_client_id: Option<String>,

    /// Deprecated. Do not use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

impl ServiceAccount {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-service_account".into(),
            project_id: Some("test-project_id".into()),
            unique_id: Some("test-unique_id".into()),
            email: Some("test-email".into()),
            display_name: Some("test-display_name".into()),
            description: Some("test-description".into()),
            disabled: Some(false),
            oauth2_client_id: Some("test-oauth2_client_id".into()),
            etag: Some("test-etag".into()),
        }
    }
}

/// The service account create request.
///
/// **GCP API**: `iam.v1.CreateServiceAccountRequest`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/CreateServiceAccountRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountRequest {
    /// Required. The account id that is used to generate the service account email address and
    /// a stable unique id. It is unique within a project, must be 6-30 characters long, and
    /// match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])` to comply with RFC1035.
    pub account_id: String,

    /// The ServiceAccount resource to create. Currently, only the following values are user
    /// assignable: `display_name` and `description`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<ServiceAccount>,
}

impl CreateServiceAccountRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            account_id: "test-account_id".into(),
            service_account: Some(ServiceAccount::fixture()),
        }
    }
}

/// Represents a service account key. A service account has two sets of key-pairs: user-managed,
/// and system-managed. User-managed key-pairs can be created and deleted by users. Users are
/// responsible for rotating these keys periodically to ensure security of their service
/// accounts. Users retain the private key of these key-pairs, and Google retains ONLY the
/// public key. System-managed keys are automatically rotated by Google, and are used for
/// signing for a maximum of two weeks. The rotation process is probabilistic, and usage of the
/// new key will gradually ramp up and down over the key's lifetime. If you cache the public key
/// set for a service account, we recommend that you update the cache every 15 minutes. User-
/// managed keys can be added and removed at any time, so it is important to update the cache
/// frequently. For Google-managed keys, Google will publish a key at least 6 hours before it is
/// first used for signing and will keep publishing it for at least 6 hours after it was last
/// used for signing. Public keys for all service accounts are also published at the OAuth2
/// Service Account API.
///
/// **GCP API**: `iam.v1.ServiceAccountKey`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/ServiceAccountKey>
///
/// ## Coverage
/// 10 of 12 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountKey {
    /// The resource name of the service account key in the following format
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    pub name: String,

    /// Specifies the algorithm (and possibly key size) for the key.
    ///
    /// **Possible values**:
    /// - `KEY_ALG_UNSPECIFIED` — An unspecified key algorithm.
    /// - `KEY_ALG_RSA_1024` — 1k RSA Key.
    /// - `KEY_ALG_RSA_2048` — 2k RSA Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,

    /// The key origin.
    ///
    /// **Possible values**:
    /// - `ORIGIN_UNSPECIFIED` — Unspecified key origin.
    /// - `USER_PROVIDED` — Key is provided by user.
    /// - `GOOGLE_PROVIDED` — Key is provided by Google.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_origin: Option<String>,

    /// The key type.
    ///
    /// **Possible values**:
    /// - `KEY_TYPE_UNSPECIFIED` — Unspecified key type. The presence of this in the message will immediately resul...
    /// - `USER_MANAGED` — User-managed keys (managed and rotated by the user).
    /// - `SYSTEM_MANAGED` — System-managed keys (managed and rotated by Google).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,

    /// The private key data. Only provided in `CreateServiceAccountKey` responses. Make sure to
    /// keep the private key data secure because it allows for the assertion of the service
    /// account identity. When base64 decoded, the private key data can be used to authenticate
    /// with Google API client libraries and with gcloud auth activate-service-account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_data: Option<String>,

    /// The output format for the private key. Only provided in `CreateServiceAccountKey`
    /// responses, not in `GetServiceAccountKey` or `ListServiceAccountKey` responses. Google
    /// never exposes system-managed private keys, and never retains user-managed private keys.
    ///
    /// **Possible values**:
    /// - `TYPE_UNSPECIFIED` — Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`.
    /// - `TYPE_PKCS12_FILE` — PKCS12 format. The password for the PKCS12 file is `notasecret`. For more inform...
    /// - `TYPE_GOOGLE_CREDENTIALS_FILE` — Google Credentials File format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_type: Option<String>,

    /// The public key data. Only provided in `GetServiceAccountKey` responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_data: Option<String>,

    /// The key can be used after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_after_time: Option<String>,

    /// The key can be used before this timestamp. For system-managed key pairs, this timestamp
    /// is the end time for the private key signing operation. The public key could still be
    /// used for verification for a few hours after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_before_time: Option<String>,

    /// The key status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl ServiceAccountKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-service_account_key".into(),
            key_algorithm: Some("test-key_algorithm".into()),
            key_origin: Some("test-key_origin".into()),
            key_type: Some("test-key_type".into()),
            private_key_data: Some("test-private_key_data".into()),
            private_key_type: Some("test-private_key_type".into()),
            public_key_data: Some("test-public_key_data".into()),
            valid_after_time: Some("test-valid_after_time".into()),
            valid_before_time: Some("test-valid_before_time".into()),
            disabled: Some(false),
        }
    }
}

/// The service account key create request.
///
/// **GCP API**: `iam.v1.CreateServiceAccountKeyRequest`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/CreateServiceAccountKeyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountKeyRequest {
    /// Which type of key and algorithm to use for the key. The default is currently a 2K RSA
    /// key. However this may change in the future.
    ///
    /// **Possible values**:
    /// - `KEY_ALG_UNSPECIFIED` — An unspecified key algorithm.
    /// - `KEY_ALG_RSA_1024` — 1k RSA Key.
    /// - `KEY_ALG_RSA_2048` — 2k RSA Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,

    /// The output format of the private key. The default value is
    /// `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File format.
    ///
    /// **Possible values**:
    /// - `TYPE_UNSPECIFIED` — Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`.
    /// - `TYPE_PKCS12_FILE` — PKCS12 format. The password for the PKCS12 file is `notasecret`. For more inform...
    /// - `TYPE_GOOGLE_CREDENTIALS_FILE` — Google Credentials File format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_type: Option<String>,
}

impl CreateServiceAccountKeyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_algorithm: Some("test-key_algorithm".into()),
            private_key_type: Some("test-private_key_type".into()),
        }
    }
}

/// The service account keys list response.
///
/// **GCP API**: `iam.v1.ListServiceAccountKeysResponse`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/ListServiceAccountKeysResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServiceAccountKeysResponse {
    /// The public keys for the service account.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<ServiceAccountKey>,
}

impl ListServiceAccountKeysResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { keys: vec![] }
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in
/// your APIs. A typical example is to use it as the request or the response type of an API
/// method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns
/// (google.protobuf.Empty); }
///
/// **GCP API**: `iam.v1.Empty`
/// **Reference**: <https://cloud.google.com/iam/docs/reference/rest/v1/Empty>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamEmpty {}

impl IamEmpty {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing ServiceAccount resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServiceAccountsResponse {
    /// A list of ServiceAccount resources.
    #[serde(default)]
    pub accounts: Vec<ServiceAccount>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListServiceAccountsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            accounts: vec![],
            next_page_token: None,
        }
    }
}
