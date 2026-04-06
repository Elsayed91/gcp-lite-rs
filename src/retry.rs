//! Retry and backoff configuration for HTTP requests.
//!
//! Re-exports [`cloud_lite_core::retry`] unchanged. GCP uses the same defaults.

pub use cloud_lite_core::retry::RetryConfig;
