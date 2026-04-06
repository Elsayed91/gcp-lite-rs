//! Types for the Service Usage API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://serviceusage.googleapis.com/$discovery/rest?version=v1`

use serde::{Deserialize, Serialize};

/// Possible values for `service_usage.v1.GoogleApiServiceusageV1Service.state`.
///
/// **GCP API**: `service_usage.v1.GoogleApiServiceusageV1Service.state`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceStateEnum {
    /// The default value, which indicates that the enabled state of the service is unspecified
    /// or not meaningful. Currently, all consumers other than projects (such as folders and
    /// organizations) are always in this state.
    StateUnspecified,

    /// The service cannot be used by this consumer. It has either been explicitly disabled, or
    /// has never been enabled.
    Disabled,

    /// The service has been explicitly enabled for use by this consumer.
    Enabled,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `service_usage.v1.DisableServiceRequest.checkIfServiceHasUsage`.
///
/// **GCP API**: `service_usage.v1.DisableServiceRequest.checkIfServiceHasUsage`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckIfServiceHasUsage {
    /// When unset, the default behavior is used, which is SKIP.
    CheckIfServiceHasUsageUnspecified,

    /// If set, skip checking service usage when disabling a service.
    Skip,

    /// If set, service usage is checked when disabling the service. If a service, or its
    /// dependents, has usage in the last 30 days, the request returns a FAILED_PRECONDITION
    /// error.
    Check,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// A service that is available for use by the consumer.
///
/// **GCP API**: `service_usage.v1.GoogleApiServiceusageV1Service`
/// **Reference**: <https://cloud.google.com/service-usage/docs/reference/rest/v1/GoogleApiServiceusageV1Service>
///
/// ## Coverage
/// 3 of 4 fields included.
/// Omitted fields:
/// - `config` — Triggers ~15 auto-dependency types (Api, Documentation, Monitoring, etc.) that nobody uses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceState {
    /// The resource name of the consumer and service. A valid name would be:
    /// - projects/123/services/serviceusage.googleapis.com
    pub name: String,

    /// The resource name of the consumer. A valid name would be:
    /// - projects/123
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Whether or not the service has been enabled for use by the consumer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceStateEnum>,
}

impl ServiceState {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-google_api_serviceusage_v1_service".into(),
            parent: Some("test-parent".into()),
            state: Some(ServiceStateEnum::StateUnspecified),
        }
    }
}

/// Request message for the `DisableService` method.
///
/// **GCP API**: `service_usage.v1.DisableServiceRequest`
/// **Reference**: <https://cloud.google.com/service-usage/docs/reference/rest/v1/DisableServiceRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableServiceRequest {
    /// Indicates if services that are enabled and which depend on this service should also be
    /// disabled. If not set, an error will be generated if any enabled services depend on the
    /// service to be disabled. When set, the service, and any enabled services that depend on
    /// it, will be disabled together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_dependent_services: Option<bool>,

    /// Defines the behavior for checking service usage when disabling a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_if_service_has_usage: Option<CheckIfServiceHasUsage>,
}

impl DisableServiceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            disable_dependent_services: Some(false),
            check_if_service_has_usage: Some(
                CheckIfServiceHasUsage::CheckIfServiceHasUsageUnspecified,
            ),
        }
    }
}

/// Request message for the `EnableService` method.
///
/// **GCP API**: `service_usage.v1.EnableServiceRequest`
/// **Reference**: <https://cloud.google.com/service-usage/docs/reference/rest/v1/EnableServiceRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableServiceRequest {}

impl EnableServiceRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {}
    }
}

/// This resource represents a long-running operation that is the result of a network API call.
///
/// **GCP API**: `service_usage.v1.Operation`
/// **Reference**: <https://cloud.google.com/service-usage/docs/reference/rest/v1/Operation>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUsageLro {
    /// The server-assigned name, which is only unique within the same service that originally
    /// returns it. If you use the default HTTP mapping, the `name` should be a resource name
    /// ending with `operations/{unique_id}`.
    pub name: String,

    /// If the value is `false`, it means the operation is still in progress. If `true`, the
    /// operation is completed, and either `error` or `response` is available.
    #[serde(default)]
    pub done: bool,

    /// The error result of the operation in case of failure or cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,

    /// The normal, successful response of the operation. If the original method returns no data
    /// on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original
    /// method is standard `Get`/`Create`/`Update`, the response should be the resource. For
    /// other methods, the response should have the type `XxxResponse`, where `Xxx` is the
    /// original method name. For example, if the original method name is `TakeSnapshot()`, the
    /// inferred response type is `TakeSnapshotResponse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,

    /// Service-specific metadata associated with the operation. It typically contains progress
    /// information and common metadata such as create time. Some services might not provide
    /// such metadata. Any method that returns a long-running operation should document the
    /// metadata type, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl ServiceUsageLro {
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
            done: false,
            ..Default::default()
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    /// Create a completed operation fixture for testing.
    pub fn fixture_done() -> Self {
        Self {
            name: "operation-done".into(),
            done: true,
            ..Default::default()
        }
    }
}

/// Request message for the `BatchEnableServices` method.
///
/// **GCP API**: `service_usage.v1.BatchEnableServicesRequest`
/// **Reference**: <https://cloud.google.com/service-usage/docs/reference/rest/v1/BatchEnableServicesRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEnableServicesRequest {
    /// The identifiers of the services to enable on the project. A valid identifier would be:
    /// serviceusage.googleapis.com Enabling services requires that each service is public or is
    /// shared with the user enabling the service. A single request can enable a maximum of 20
    /// services at a time. If more than 20 services are specified, the request will fail, and
    /// no state changes will occur.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_ids: Vec<String>,
}

impl BatchEnableServicesRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            service_ids: vec![],
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing ServiceState resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListServicesResponse {
    /// A list of ServiceState resources.
    #[serde(default)]
    pub services: Vec<ServiceState>,

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
