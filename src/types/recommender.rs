//! Types for the Recommender API API (v1).
//!
//! Auto-generated from the GCP Discovery Document.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! Discovery document: `https://www.googleapis.com/discovery/v1/apis/recommender/v1/rest`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Possible values for `recommender.v1.GoogleCloudRecommenderV1Recommendation.priority`.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1Recommendation.priority`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationPriority {
    /// Recommendation has unspecified priority.
    PriorityUnspecified,

    /// Recommendation has P4 priority (lowest priority).
    P4,

    /// Recommendation has P3 priority (second lowest priority).
    P3,

    /// Recommendation has P2 priority (second highest priority).
    P2,

    /// Recommendation has P1 priority (highest priority).
    P1,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `recommender.v1.GoogleCloudRecommenderV1RecommendationStateInfo.state`.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1RecommendationStateInfo.state`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationState {
    /// Default state. Don't use directly.
    StateUnspecified,

    /// Recommendation is active and can be applied. Recommendations content can be updated by
    /// Google. ACTIVE recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
    Active,

    /// Recommendation is in claimed state. Recommendations content is immutable and cannot be
    /// updated by Google. CLAIMED recommendations can be marked as CLAIMED, SUCCEEDED, or
    /// FAILED.
    Claimed,

    /// Recommendation is in succeeded state. Recommendations content is immutable and cannot be
    /// updated by Google. SUCCEEDED recommendations can be marked as SUCCEEDED, or FAILED.
    Succeeded,

    /// Recommendation is in failed state. Recommendations content is immutable and cannot be
    /// updated by Google. FAILED recommendations can be marked as SUCCEEDED, or FAILED.
    Failed,

    /// Recommendation is in dismissed state. Recommendation content can be updated by Google.
    /// DISMISSED recommendations can be marked as ACTIVE.
    Dismissed,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// Possible values for `recommender.v1.GoogleCloudRecommenderV1Impact.category`.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1Impact.category`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImpactCategory {
    /// Default unspecified category. Don't use directly.
    CategoryUnspecified,

    /// Indicates a potential increase or decrease in cost.
    Cost,

    /// Indicates a potential increase or decrease in security.
    Security,

    /// Indicates a potential increase or decrease in performance.
    Performance,

    /// Indicates a potential increase or decrease in manageability.
    Manageability,

    /// Indicates a potential increase or decrease in sustainability.
    Sustainability,

    /// Indicates a potential increase or decrease in reliability.
    Reliability,

    /// Unknown or future value not yet represented.
    #[serde(other)]
    Unknown,
}

/// A recommendation along with a suggested action. E.g., a rightsizing recommendation for an
/// underutilized VM, IAM role recommendations, etc
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1Recommendation`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1Recommendation>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    /// Identifier. Name of recommendation.
    pub name: String,

    /// Free-form human readable summary in English. The maximum length is 500 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Information for state. Contains state and metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_info: Option<RecommendationStateInfo>,

    /// Contains an identifier for a subtype of recommendations produced for the same
    /// recommender. Subtype is a function of content and impact, meaning a new subtype might be
    /// added when significant changes to `content` or `primary_impact.category` are introduced.
    /// See the Recommenders section to see a list of subtypes for a given Recommender.
    /// Examples: For recommender = "google.iam.policy.Recommender", recommender_subtype can be
    /// one of "REMOVE_ROLE"/"REPLACE_ROLE"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_subtype: Option<String>,

    /// Fully qualified resource names that this recommendation is targeting.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_resources: Vec<String>,

    /// Content of the recommendation describing recommended changes to resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<RecommendationContent>,

    /// Insights that led to this recommendation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_insights: Vec<RecommendationInsightReference>,

    /// The primary impact that this recommendation can have while trying to optimize for one
    /// category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_impact: Option<Impact>,

    /// Optional set of additional impact that this recommendation may have when trying to
    /// optimize for the primary category. These may be positive or negative.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_impact: Vec<Impact>,

    /// Last time this recommendation was refreshed by the system that created it in the first
    /// place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,

    /// Recommendation's priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<RecommendationPriority>,

    /// Fingerprint of the Recommendation. Provides optimistic locking when updating states.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Corresponds to a mutually exclusive group ID within a recommender. A non-empty ID
    /// indicates that the recommendation belongs to a mutually exclusive group. This means that
    /// only one recommendation within the group is suggested to be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xor_group_id: Option<String>,
}

impl Recommendation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-google_cloud_recommender_v1_recommendation".into(),
            description: Some("test-description".into()),
            state_info: Some(RecommendationStateInfo::fixture()),
            recommender_subtype: Some("test-recommender_subtype".into()),
            target_resources: vec![],
            content: Some(RecommendationContent::fixture()),
            associated_insights: vec![],
            primary_impact: Some(Impact::fixture()),
            additional_impact: vec![],
            last_refresh_time: Some("test-last_refresh_time".into()),
            priority: Some(RecommendationPriority::PriorityUnspecified),
            etag: Some("test-etag".into()),
            xor_group_id: Some("test-xor_group_id".into()),
        }
    }
}

/// Response to the `ListRecommendations` method.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1ListRecommendationsResponse`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1ListRecommendationsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRecommendationsResponse {
    /// The set of recommendations for the `parent` resource.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<Recommendation>,

    /// A token that can be used to request the next page of results. This field is empty if
    /// there are no additional results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListRecommendationsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            recommendations: vec![],
            next_page_token: Some("test-next_page_token".into()),
        }
    }
}

/// Information for state. Contains state and metadata.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1RecommendationStateInfo`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1RecommendationStateInfo>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationStateInfo {
    /// The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<RecommendationState>,

    /// A map of metadata for the state, provided by user or automations systems.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub state_metadata: HashMap<String, String>,
}

impl RecommendationStateInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some(RecommendationState::StateUnspecified),
            state_metadata: Default::default(),
        }
    }
}

/// Contains what resources are changing and how they are changing.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1RecommendationContent`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1RecommendationContent>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationContent {
    /// Condensed overview information about the recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<serde_json::Value>,

    /// Operations to one or more Google Cloud resources grouped in such a way that, all
    /// operations within one group are expected to be performed atomically and in an order.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub operation_groups: Vec<OperationGroup>,
}

impl RecommendationContent {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            overview: Default::default(),
            operation_groups: vec![],
        }
    }
}

/// Reference to an associated insight.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1RecommendationInsightReference`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1RecommendationInsightReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationInsightReference {
    /// Insight resource name, e.g. projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/
    /// [INSIGHT_TYPE_ID]/insights/[INSIGHT_ID]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<String>,
}

impl RecommendationInsightReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            insight: Some("test-insight".into()),
        }
    }
}

/// Contains the impact a recommendation can have for a given category.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1Impact`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1Impact>
///
/// ## Coverage
/// 6 of 7 fields included.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Impact {
    /// Category that is being targeted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ImpactCategory>,

    /// The service that this impact is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Use with CategoryType.COST
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_projection: Option<CostProjection>,

    /// Use with CategoryType.SECURITY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_projection: Option<SecurityProjection>,

    /// Use with CategoryType.SUSTAINABILITY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sustainability_projection: Option<SustainabilityProjection>,

    /// Use with CategoryType.RELIABILITY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reliability_projection: Option<ReliabilityProjection>,
}

impl Impact {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            category: Some(ImpactCategory::CategoryUnspecified),
            service: Some("test-service".into()),
            cost_projection: Some(CostProjection::fixture()),
            security_projection: Some(SecurityProjection::fixture()),
            sustainability_projection: Some(SustainabilityProjection::fixture()),
            reliability_projection: Some(ReliabilityProjection::fixture()),
        }
    }
}

/// Contains metadata about how much money a recommendation can save or incur.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1CostProjection`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1CostProjection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostProjection {
    /// An approximate projection on amount saved or amount incurred. Negative cost units
    /// indicate cost savings and positive cost units indicate increase. See google.type.Money
    /// documentation for positive/negative units. A user's permissions may affect whether the
    /// cost is computed using list prices or custom contract prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Money>,

    /// The approximate cost savings in the billing account's local currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_in_local_currency: Option<Money>,

    /// Duration for which this cost applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl CostProjection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            cost: Some(Money::fixture()),
            cost_in_local_currency: Some(Money::fixture()),
            duration: Some("test-duration".into()),
        }
    }
}

/// Contains various ways of describing the impact on Security.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1SecurityProjection`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1SecurityProjection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityProjection {
    /// Additional security impact details that is provided by the recommender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl SecurityProjection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            details: Default::default(),
        }
    }
}

/// Contains metadata about how much sustainability a recommendation can save or incur.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1SustainabilityProjection`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1SustainabilityProjection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SustainabilityProjection {
    /// Carbon Footprint generated in kg of CO2 equivalent. Chose kg_c_o2e so that the name
    /// renders correctly in camelCase (kgCO2e).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg_co2e: Option<f64>,

    /// Duration for which this sustainability applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl SustainabilityProjection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            duration: Some("test-duration".into()),
            ..Default::default()
        }
    }
}

/// Contains information on the impact of a reliability recommendation.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1ReliabilityProjection`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1ReliabilityProjection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReliabilityProjection {
    /// Reliability risks mitigated by this recommendation.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub risks: Vec<String>,

    /// Per-recommender projection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ReliabilityProjection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            risks: vec![],
            details: Default::default(),
        }
    }
}

/// Group of operations that need to be performed atomically.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1OperationGroup`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1OperationGroup>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationGroup {
    /// List of operations across one or more resources that belong to this group. Loosely based
    /// on RFC6902 and should be performed in the order they appear.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<RecommendationOperation>,
}

impl OperationGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { operations: vec![] }
    }
}

/// Contains an operation for a resource loosely based on the JSON-PATCH format with support
/// for:
/// * Custom filters for describing partial array patch.
/// * Extended path values for describing nested arrays.
/// * Custom fields for describing the resource for which the operation is being described.
/// * Allows extension to custom operations not natively supported by RFC6902. See
///   https://tools.ietf.org/html/rfc6902 for details on the original RFC.
///
/// **GCP API**: `recommender.v1.GoogleCloudRecommenderV1Operation`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleCloudRecommenderV1Operation>
///
/// ## Coverage
/// 7 of 10 fields included.
/// Omitted fields:
/// - `pathFilters` — Advanced filtering — rarely used
/// - `pathValueMatchers` — Advanced matching — rarely used
/// - `valueMatcher` — Advanced matching — rarely used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationOperation {
    /// Type of this operation. Contains one of 'add', 'remove', 'replace', 'move', 'copy',
    /// 'test' and custom operations. This field is case-insensitive and always populated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Contains the fully qualified resource name. This field is always populated. ex:
    /// //cloudresourcemanager.googleapis.com/projects/foo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    /// Type of GCP resource being modified/tested. This field is always populated. Example:
    /// cloudresourcemanager.googleapis.com/Project, compute.googleapis.com/Instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Path to the target field being operated on. If the operation is at the resource level,
    /// then path should be "/". This field is always populated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Value for the `path` field. Will be set for actions:'add'/'replace'. Maybe set for
    /// action: 'test'. Either this or `value_matcher` will be set for 'test' operation. An
    /// exact match must be performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,

    /// Can be set with action 'copy' or 'move' to indicate the source field within resource or
    /// source_resource, ignored if provided for other operation types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,

    /// Can be set with action 'copy' to copy resource configuration across different resources
    /// of the same type. Example: A resource clone can be done via action = 'copy', path = "/",
    /// from = "/", source_resource = and resource_name = . This field is empty for all other
    /// values of `action`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource: Option<String>,
}

impl RecommendationOperation {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            action: Some("test-action".into()),
            resource: Some("test-resource".into()),
            resource_type: Some("test-resource_type".into()),
            path: Some("test-path".into()),
            source_path: Some("test-source_path".into()),
            source_resource: Some("test-source_resource".into()),
            ..Default::default()
        }
    }
}

/// Represents an amount of money with its currency type.
///
/// **GCP API**: `recommender.v1.GoogleTypeMoney`
/// **Reference**: <https://cloud.google.com/recommender/docs//GoogleTypeMoney>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is
    /// one US dollar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,

    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and
    /// +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If
    /// `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative,
    /// `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and
    /// `nanos`=-750,000,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanos: Option<i32>,
}

impl Money {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            currency_code: Some("test-currency_code".into()),
            units: Some("test-units".into()),
            nanos: Some(100),
        }
    }
}
