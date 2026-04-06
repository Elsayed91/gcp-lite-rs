//! Test support utilities and MockClient helpers.
//!
//! This module provides extension traits for `MockClient` that make test setup more ergonomic.
//! Each API has its own helper trait with `expect_*` methods for ergonomic test setup.
//!
//! # Example
//!
//! ```no_run
//! use gcp_lite::MockClient;
//! use gcp_lite::test_support::AccessapprovalMockHelpers;
//!
//! let mut mock = MockClient::new();
//! ```

#[cfg(any(test, feature = "test-support"))]
pub mod accessapproval_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod apikeys_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod appengine_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod bigquery_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudasset_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudbilling_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudkms_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudresourcemanager_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cloudscheduler_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod compute_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod container_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod dlp_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod dns_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod essentialcontacts_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod gkebackup_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod iam_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod logging_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod monitoring_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod osconfig_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod recommender_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod secretmanager_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod service_usage_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod sqladmin_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod storage_mock_helpers;

// Re-export traits for convenience
#[cfg(any(test, feature = "test-support"))]
pub use accessapproval_mock_helpers::AccessapprovalMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use apikeys_mock_helpers::ApikeysMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use appengine_mock_helpers::AppengineMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use bigquery_mock_helpers::BigqueryMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudasset_mock_helpers::CloudassetMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudbilling_mock_helpers::CloudbillingMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudkms_mock_helpers::CloudkmsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudresourcemanager_mock_helpers::CloudresourcemanagerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cloudscheduler_mock_helpers::CloudschedulerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use compute_mock_helpers::ComputeMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use container_mock_helpers::ContainerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use dlp_mock_helpers::DlpMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use dns_mock_helpers::DnsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use essentialcontacts_mock_helpers::EssentialcontactsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use gkebackup_mock_helpers::GkebackupMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use iam_mock_helpers::IamMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use logging_mock_helpers::LoggingMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use monitoring_mock_helpers::MonitoringMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use osconfig_mock_helpers::OsconfigMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use recommender_mock_helpers::RecommenderMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use secretmanager_mock_helpers::SecretmanagerMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use service_usage_mock_helpers::ServiceUsageMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use sqladmin_mock_helpers::SqladminMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use storage_mock_helpers::StorageMockHelpers;
