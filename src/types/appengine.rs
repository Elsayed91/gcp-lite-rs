//! Types for the App Engine Admin API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/appengine/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An Application resource contains the top-level configuration of an App Engine application.
///
/// **GCP API**: `appengine.v1.Application`
/// **Reference**: <https://cloud.google.com/appengine/docs/admin-api//Application>
///
/// ## Coverage
/// 7 of 17 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    /// Google Apps authentication domain that controls which users can access this
    /// application.Defaults to open access for any Google Account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_domain: Option<String>,

    /// Output only. Hostname used to reach this application, as resolved by App
    /// Engine.@OutputOnly
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hostname: Option<String>,

    /// Identifier of the Application resource. This identifier is equivalent to the project ID
    /// of the Google Cloud Platform project where you want to deploy your application. Example:
    /// myapp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Location from which this application runs. Application instances run out of the data
    /// centers in the specified location, which is also where all of the application's end user
    /// content is stored.Defaults to us-central.View the list of supported locations
    /// (https://cloud.google.com/appengine/docs/locations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,

    /// The `name` field.
    pub name: String,

    /// Serving status of this application.
    ///
    /// **Possible values**:
    /// - `UNSPECIFIED` — Serving status is unspecified.
    /// - `SERVING` — Application is serving.
    /// - `USER_DISABLED` — Application has been disabled by the user.
    /// - `SYSTEM_DISABLED` — Application has been disabled by the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_status: Option<String>,

    /// The SSL policy that will be applied to the application. If set to Modern it will
    /// restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite
    ///
    /// **Possible values**:
    /// - `SSL_POLICY_UNSPECIFIED` — Required by linter. Will work same as DEFAULT
    /// - `DEFAULT` — DEFAULT is to allow all TLS versions and cipher suites supported by App Engine
    /// - `MODERN` — MODERN is to allow only TLS 1.2 and TLS 1.3 along with Modern cipher suites only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policy: Option<String>,
}

impl Application {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            auth_domain: Some("test-auth_domain".into()),
            default_hostname: Some("test-default_hostname".into()),
            id: Some("test-id".into()),
            location_id: Some("test-location_id".into()),
            name: "test-application".into(),
            serving_status: Some("test-serving_status".into()),
            ssl_policy: Some("test-ssl_policy".into()),
        }
    }
}

/// A NetworkSettings resource is a container for ingress settings for a version or service.
///
/// **GCP API**: `appengine.v1.NetworkSettings`
/// **Reference**: <https://cloud.google.com/appengine/docs/admin-api//NetworkSettings>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    /// The ingress settings for version or service.
    ///
    /// **Possible values**:
    /// - `INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED` — Unspecified
    /// - `INGRESS_TRAFFIC_ALLOWED_ALL` — Allow HTTP traffic from public and private sources.
    /// - `INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY` — Allow HTTP traffic from only private VPC sources.
    /// - `INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB` — Allow HTTP traffic from private VPC sources and through load balancers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_traffic_allowed: Option<String>,
}

impl NetworkSettings {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ingress_traffic_allowed: Some("test-ingress_traffic_allowed".into()),
        }
    }
}

/// A Service resource is a logical component of an application that can share state and
/// communicate in a secure fashion with other services. For example, an application that
/// handles customer requests might include separate services to handle tasks such as backend
/// data analysis or API requests from mobile devices. Each service has a collection of versions
/// that define a specific set of code used to implement the functionality of that service.
///
/// **GCP API**: `appengine.v1.Service`
/// **Reference**: <https://cloud.google.com/appengine/docs/admin-api//Service>
///
/// ## Coverage
/// 4 of 6 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    /// Output only. Relative name of the service within the application. Example:
    /// default.@OutputOnly
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A set of labels to apply to this service. Labels are key/value pairs that describe the
    /// service and all resources that belong to it (e.g., versions). The labels can be used to
    /// search and group resources, and are propagated to the usage and billing reports,
    /// enabling fine-grain analysis of costs. An example of using labels is to tag resources
    /// belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values
    /// can be no longer than 63 characters and can only contain lowercase letters, numeric
    /// characters, underscores, dashes, and international characters. Label keys must start
    /// with a lowercase letter or an international character. Each service can have at most 32
    /// labels.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,

    /// Output only. Full path to the Service resource in the API. Example:
    /// apps/myapp/services/default.@OutputOnly
    ///
    /// *Output-only field.*
    pub name: String,

    /// Ingress settings for this service. Will apply to all versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<NetworkSettings>,
}

impl Service {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            labels: Default::default(),
            name: "test-service".into(),
            network_settings: Some(NetworkSettings::fixture()),
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing Service resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServicesResponse {
    /// A list of Service resources.
    #[serde(default)]
    pub services: Vec<Service>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListServicesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            services: vec![],
            next_page_token: None,
        }
    }
}
