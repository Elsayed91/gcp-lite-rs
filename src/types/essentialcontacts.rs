//! Types for the Essential Contacts API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://essentialcontacts.googleapis.com/$discovery/rest?version=v1`

use serde::{Deserialize, Serialize};

/// A contact that will receive notifications from Google Cloud.
///
/// **GCP API**: `essentialcontacts.v1.GoogleCloudEssentialcontactsV1Contact`
/// **Reference**: <https://cloud.google.com/resource-manager/docs/managing-notification-contacts/GoogleCloudEssentialcontactsV1Contact>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EssentialContact {
    /// Output only. The identifier for the contact. Format:
    /// {resource_type}/{resource_id}/contacts/{contact_id}
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Required. The email address to send notifications to. The email address does not need to
    /// be a Google Account.
    pub email: String,

    /// Required. The categories of notifications that the contact will receive communications
    /// for.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notification_category_subscriptions: Vec<String>,

    /// Required. The preferred language for notifications, as a ISO 639-1 language code. See
    /// [Supported languages](https://cloud.google.com/resource-manager/docs/managing-
    /// notification-contacts#supported-languages) for a list of supported languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_tag: Option<String>,

    /// Output only. The validity of the contact. A contact is considered valid if it is the
    /// correct recipient for notifications for a particular resource.
    ///
    /// **Possible values**:
    /// - `VALIDATION_STATE_UNSPECIFIED` — The validation state is unknown or unspecified.
    /// - `VALID` — The contact is marked as valid. This is usually done manually by the contact adm...
    /// - `INVALID` — The contact is considered invalid. This may become the state if the contact's em...
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,

    /// Output only. The last time the validation_state was updated, either manually or
    /// automatically. A contact is considered stale if its validation state was updated more
    /// than 1 year ago.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_time: Option<String>,
}

impl EssentialContact {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-google_cloud_essentialcontacts_v1_contact".into()),
            email: "test-email".into(),
            notification_category_subscriptions: vec![],
            language_tag: Some("test-language_tag".into()),
            validation_state: Some("test-validation_state".into()),
            validate_time: Some("test-validate_time".into()),
        }
    }
}

/// Response message for the ListContacts method.
///
/// **GCP API**: `essentialcontacts.v1.GoogleCloudEssentialcontactsV1ListContactsResponse`
/// **Reference**: <https://cloud.google.com/resource-manager/docs/managing-notification-contacts/GoogleCloudEssentialcontactsV1ListContactsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListContactsResponse {
    /// The contacts for the specified resource.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contacts: Vec<EssentialContact>,

    /// If there are more results than those appearing in this response, then `next_page_token`
    /// is included. To get the next set of results, call this method again using the value of
    /// `next_page_token` as `page_token` and the rest of the parameters the same as the
    /// original request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListContactsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            contacts: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in
/// your APIs. A typical example is to use it as the request or the response type of an API
/// method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns
/// (google.protobuf.Empty); }
///
/// **GCP API**: `essentialcontacts.v1.GoogleProtobufEmpty`
/// **Reference**: <https://cloud.google.com/resource-manager/docs/managing-notification-contacts/GoogleProtobufEmpty>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EssentialContactsEmpty {}

impl EssentialContactsEmpty {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}
