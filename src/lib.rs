//! Lightweight HTTP-based client for Google Cloud Platform APIs.
//!
//! `gcp-http-lite` provides clean REST API access without the overhead of gRPC.
//! It includes automatic retry, backoff, structured errors, and multi-tenant support.
//!
//! # Quick Start
//!
//! ```no_run
//! use gcp_lite::{GcpHttpClient, token::StaticTokenProvider};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = StaticTokenProvider::new("your-token");
//! let client = GcpHttpClient::builder()
//!     .token_provider(provider)
//!     .build()?;
//!
//! let compute = client.compute();
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod operation;
pub(crate) mod ops;
pub mod rate_limit;
pub mod retry;
pub(crate) mod serde_base64;
pub mod token;
pub mod types;

#[cfg(feature = "test-support")]
pub mod mock_client;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

#[cfg(test)]
mod fixture_tests;

#[cfg(test)]
mod test_support_demo;

pub use client::{BuilderError, GcpHttpClient, GcpHttpClientBuilder};
pub use error::{GcpError, Result};
pub use operation::{
    ContainerOperation, GkeBackupOperation, Operation, PollConfig, ServiceUsageOperation,
    SqlOperation,
};
pub use rate_limit::{RateLimitConfig, RateLimitStats};
pub use retry::RetryConfig;

/// Build a URL query string from key-value pairs, omitting any with empty values.
pub(crate) fn append_query_params(url: String, params: &[(&str, &str)]) -> String {
    let qs: Vec<String> = params
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect();
    if qs.is_empty() {
        url
    } else {
        format!("{}?{}", url, qs.join("&"))
    }
}

#[cfg(feature = "test-support")]
pub use mock_client::MockClient;
