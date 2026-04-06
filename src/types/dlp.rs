//! Types for the Cloud DLP API API (v2).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/dlp/v2/rest`

use serde::{Deserialize, Serialize};

/// Score is calculated from of all elements in the data profile. A higher level means the data
/// is more sensitive.
///
/// **GCP API**: `dlp.v2.GooglePrivacyDlpV2SensitivityScore`
/// **Reference**: <https://cloud.google.com/sensitive-data-protection/docs//GooglePrivacyDlpV2SensitivityScore>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DlpSensitivityScore {
    /// The sensitivity score applied to the resource.
    ///
    /// **Possible values**:
    /// - `SENSITIVITY_SCORE_UNSPECIFIED` — Unused.
    /// - `SENSITIVITY_LOW` — No sensitive information detected. The resource isn't publicly accessible.
    /// - `SENSITIVITY_UNKNOWN` — Unable to determine sensitivity.
    /// - `SENSITIVITY_MODERATE` — Medium risk. Contains personally identifiable information (PII), potentially sen...
    /// - `SENSITIVITY_HIGH` — High risk. Sensitive personally identifiable information (SPII) can be present. ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
}

impl DlpSensitivityScore {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            score: Some("test-score".into()),
        }
    }
}

/// Score is a summary of all elements in the data profile. A higher number means more risk.
///
/// **GCP API**: `dlp.v2.GooglePrivacyDlpV2DataRiskLevel`
/// **Reference**: <https://cloud.google.com/sensitive-data-protection/docs//GooglePrivacyDlpV2DataRiskLevel>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DlpDataRiskLevel {
    /// The score applied to the resource.
    ///
    /// **Possible values**:
    /// - `RISK_SCORE_UNSPECIFIED` — Unused.
    /// - `RISK_LOW` — Low risk - Lower indication of sensitive data that appears to have additional ac...
    /// - `RISK_UNKNOWN` — Unable to determine risk.
    /// - `RISK_MODERATE` — Medium risk - Sensitive data may be present but additional access or fine grain ...
    /// - `RISK_HIGH` — High risk – SPII may be present. Access controls may include public ACLs. Exfilt...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
}

impl DlpDataRiskLevel {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            score: Some("test-score".into()),
        }
    }
}

/// Configuration for discovery to scan resources for profile generation. Only one discovery
/// configuration may exist per organization, folder, or project. The generated data profiles
/// are retained according to the [data retention policy] (https://cloud.google.com/sensitive-
/// data-protection/docs/data-profiles#retention).
///
/// **GCP API**: `dlp.v2.GooglePrivacyDlpV2DiscoveryConfig`
/// **Reference**: <https://cloud.google.com/sensitive-data-protection/docs//GooglePrivacyDlpV2DiscoveryConfig>
///
/// ## Coverage
/// 6 of 13 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryConfig {
    /// Unique resource name for the DiscoveryConfig, assigned by the service when the
    /// DiscoveryConfig is created, for example `projects/dlp-test-
    /// project/locations/global/discoveryConfigs/53234423`.
    pub name: String,

    /// Display name (max 100 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Required. A status for this configuration.
    ///
    /// **Possible values**:
    /// - `STATUS_UNSPECIFIED` — Unused
    /// - `RUNNING` — The discovery config is currently active.
    /// - `PAUSED` — The discovery config is paused temporarily.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Output only. The creation timestamp of a DiscoveryConfig.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,

    /// Output only. The last update timestamp of a DiscoveryConfig.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,

    /// Output only. The timestamp of the last time this config was executed.
    ///
    /// *Output-only field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<String>,
}

impl DiscoveryConfig {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-google_privacy_dlp_v2_discovery_config".into(),
            display_name: Some("test-display_name".into()),
            status: Some("test-status".into()),
            create_time: Some("test-create_time".into()),
            update_time: Some("test-update_time".into()),
            last_run_time: Some("test-last_run_time".into()),
        }
    }
}

/// An aggregated profile for this project, based on the resources profiled within it.
///
/// **GCP API**: `dlp.v2.GooglePrivacyDlpV2ProjectDataProfile`
/// **Reference**: <https://cloud.google.com/sensitive-data-protection/docs//GooglePrivacyDlpV2ProjectDataProfile>
///
/// ## Coverage
/// 7 of 8 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDataProfile {
    /// The resource name of the profile.
    pub name: String,

    /// Project ID or account that was profiled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    /// The last time the profile was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_last_generated: Option<String>,

    /// The sensitivity score of this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score: Option<DlpSensitivityScore>,

    /// The data risk level of this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_risk_level: Option<DlpDataRiskLevel>,

    /// The number of table data profiles generated for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_data_profile_count: Option<String>,

    /// The number of file store data profiles generated for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_store_data_profile_count: Option<String>,
}

impl ProjectDataProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-google_privacy_dlp_v2_project_data_profile".into(),
            project_id: Some("test-project_id".into()),
            profile_last_generated: Some("test-profile_last_generated".into()),
            sensitivity_score: Some(DlpSensitivityScore::fixture()),
            data_risk_level: Some(DlpDataRiskLevel::fixture()),
            table_data_profile_count: Some("test-table_data_profile_count".into()),
            file_store_data_profile_count: Some("test-file_store_data_profile_count".into()),
        }
    }
}

// ======================================================================
// List response types (generated from operation list_response)
// ======================================================================

/// Response for listing DiscoveryConfig resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GooglePrivacyDlpV2ListDiscoveryConfigsResponse {
    /// A list of DiscoveryConfig resources.
    #[serde(default)]
    pub discovery_configs: Vec<DiscoveryConfig>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GooglePrivacyDlpV2ListDiscoveryConfigsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            discovery_configs: vec![],
            next_page_token: None,
        }
    }
}

/// Response for listing ProjectDataProfile resources.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GooglePrivacyDlpV2ListProjectDataProfilesResponse {
    /// A list of ProjectDataProfile resources.
    #[serde(default)]
    pub project_data_profiles: Vec<ProjectDataProfile>,

    /// Token for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GooglePrivacyDlpV2ListProjectDataProfilesResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            project_data_profiles: vec![],
            next_page_token: None,
        }
    }
}
