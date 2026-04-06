//! Authentication module for GCP credentials.
//!
//! This module provides types and implementations for handling GCP authentication,
//! including service account keys, authorized user credentials, Application
//! Default Credentials (ADC), and Workload Identity Federation (WIF).
//!
//! # Credential Types
//!
//! This module exports several credential providers, each implementing the
//! [`TokenProvider`](crate::token::TokenProvider) trait:
//!
//! - [`ServiceAccountCredential`] - For service account key files
//! - [`AuthorizedUserCredential`] - For OAuth2 user credentials
//! - [`MetadataServerCredential`] - For GCP-hosted environments
//! - [`AdcCredential`] - Application Default Credentials (auto-detection)
//! - [`WorkloadIdentityCredential`] - For Workload Identity Federation (multi-cloud)
//!
//! # Application Default Credentials (Recommended)
//!
//! The easiest way to authenticate is using ADC, which automatically resolves
//! credentials from the environment:
//!
//! ```no_run
//! use gcp_lite::auth::AdcCredential;
//! use gcp_lite::token::TokenProvider;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Automatically finds credentials from:
//! // 1. GOOGLE_APPLICATION_CREDENTIALS env var
//! // 2. ~/.config/gcloud/application_default_credentials.json
//! // 3. GCP metadata server (when running on GCP)
//! let cred = AdcCredential::new().await?;
//!
//! // Get a token for API calls
//! let token = cred.get_token(&["https://www.googleapis.com/auth/cloud-platform"]).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Service Account Authentication
//!
//! The most common way to authenticate in server environments is using a service account:
//!
//! ```no_run
//! use gcp_lite::auth::ServiceAccountCredential;
//! use std::path::Path;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let cred = ServiceAccountCredential::from_file(Path::new("/path/to/key.json"))?;
//! # Ok(())
//! # }
//! ```
//!
//! # Authorized User Authentication
//!
//! For user-based authentication (e.g., from `gcloud auth application-default login`):
//!
//! ```no_run
//! use gcp_lite::auth::AuthorizedUserCredential;
//! use std::path::Path;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let cred = AuthorizedUserCredential::from_file(Path::new("/path/to/credentials.json"))?;
//! # Ok(())
//! # }
//! ```
//!
//! # Metadata Server Authentication
//!
//! When running on GCP infrastructure (GCE, Cloud Run, GKE, etc.), use the metadata server:
//!
//! ```no_run
//! use gcp_lite::auth::MetadataServerCredential;
//!
//! // Uses the default service account
//! let cred = MetadataServerCredential::new();
//! ```
//!
//! # Workload Identity Federation
//!
//! For multi-cloud scenarios (GitHub Actions, AWS, Azure), use Workload Identity Federation
//! to authenticate without storing service account keys:
//!
//! ```no_run
//! use gcp_lite::auth::WorkloadIdentityCredential;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Auto-detect from environment
//! let cred = WorkloadIdentityCredential::from_environment()?;
//!
//! // Or from external account JSON
//! let cred = WorkloadIdentityCredential::from_json(r#"{
//!     "type": "external_account",
//!     ...
//! }"#)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Supporting Types
//!
//! This module also exports types for working with credentials directly:
//!
//! - [`AccessToken`] - An OAuth2 access token with expiration
//! - [`CachedToken`] - Thread-safe token caching
//! - [`ServiceAccountKey`] - Parsed service account key file
//! - [`AuthorizedUserCreds`] - Parsed authorized user credentials
//! - [`CredentialFile`] - Union type for parsing credential files
//! - [`CredentialFileError`] - Errors from parsing credential files

pub mod adc;
pub mod authorized_user;
pub mod external_account;
pub mod gcloud;
pub mod metadata;
pub mod oidc_providers;
pub mod service_account;
pub mod types;
pub mod workload_identity;

// Re-export credential providers and their errors
pub use adc::{AdcCredential, AdcError, AdcSource};
pub use authorized_user::{AuthorizedUserCredential, AuthorizedUserError};
pub use metadata::{MetadataServerCredential, MetadataServerError};
pub use service_account::{ServiceAccountCredential, ServiceAccountError};

// Re-export WIF types
pub use external_account::{ExternalAccountConfig, ExternalAccountError};
pub use oidc_providers::{
    AwsWebIdentityProvider, AzureWorkloadIdentityProvider, GitHubActionsProvider, OidcError,
    OidcTokenProvider, auto_detect_provider,
};
pub use workload_identity::{WifError, WorkloadIdentityCredential};

// Re-export supporting types
pub use types::{
    AccessToken, AuthorizedUserCreds, CachedToken, CredentialFile, CredentialFileError,
    ServiceAccountKey,
};
