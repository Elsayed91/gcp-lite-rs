//! Rate limiting configuration and runtime for GCP API requests.
//!
//! Re-exports [`cloud_lite_core::rate_limit`] types with a GCP-specific [`Default`]
//! impl that ships conservative per-API limits derived from GCP quotas.

pub use cloud_lite_core::rate_limit::{RateLimitStats, RateLimiter};

use std::collections::HashMap;

/// Configuration for per-API concurrency limiting.
///
/// Ships with conservative defaults derived from GCP API quotas.
/// Enabled by default on all client construction paths.
///
/// # Example
///
/// ```rust
/// use gcp_lite::RateLimitConfig;
///
/// // Use defaults (recommended)
/// let config = RateLimitConfig::default();
///
/// // Tune specific APIs
/// let config = RateLimitConfig::default()
///     .with_default_limit(30)
///     .with_api_limit("cloudasset.googleapis.com", 5);
///
/// // Disable entirely
/// let config = RateLimitConfig::disabled();
/// ```
#[derive(Debug, Clone)]
pub struct RateLimitConfig(pub(crate) cloud_lite_core::rate_limit::RateLimitConfig);

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut api_limits = HashMap::new();
        api_limits.insert("cloudasset.googleapis.com".into(), 10);
        api_limits.insert("recommender.googleapis.com".into(), 10);
        api_limits.insert("serviceusage.googleapis.com".into(), 25);
        api_limits.insert("compute.googleapis.com".into(), 40);
        api_limits.insert("bigquery.googleapis.com".into(), 20);
        api_limits.insert("sqladmin.googleapis.com".into(), 10);
        api_limits.insert("storage.googleapis.com".into(), 50);
        api_limits.insert("run.googleapis.com".into(), 20);
        api_limits.insert("cloudbilling.googleapis.com".into(), 10);
        api_limits.insert("cloudresourcemanager.googleapis.com".into(), 20);
        Self(cloud_lite_core::rate_limit::RateLimitConfig {
            default_limit: 20,
            api_limits,
        })
    }
}

impl RateLimitConfig {
    /// Create a config that effectively disables rate limiting.
    pub fn disabled() -> Self {
        Self(cloud_lite_core::rate_limit::RateLimitConfig::disabled())
    }

    /// Override the default concurrency limit for unknown APIs.
    pub fn with_default_limit(mut self, limit: usize) -> Self {
        self.0 = self.0.with_default_limit(limit);
        self
    }

    /// Set or override the concurrency limit for a specific API host.
    pub fn with_api_limit(mut self, host: &str, limit: usize) -> Self {
        self.0 = self.0.with_api_limit(host, limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_known_api_limits() {
        let config = RateLimitConfig::default();
        assert_eq!(config.0.default_limit, 20);
        assert_eq!(
            config.0.api_limits.get("cloudasset.googleapis.com"),
            Some(&10)
        );
        assert_eq!(config.0.api_limits.get("compute.googleapis.com"), Some(&40));
        assert_eq!(config.0.api_limits.get("storage.googleapis.com"), Some(&50));
    }

    #[test]
    fn disabled_config_uses_usize_max() {
        let config = RateLimitConfig::disabled();
        assert_eq!(config.0.default_limit, usize::MAX);
        assert!(config.0.api_limits.is_empty());
    }

    #[test]
    fn with_default_limit_overrides() {
        let config = RateLimitConfig::default().with_default_limit(30);
        assert_eq!(config.0.default_limit, 30);
        // API-specific limits unchanged
        assert_eq!(
            config.0.api_limits.get("cloudasset.googleapis.com"),
            Some(&10)
        );
    }

    #[test]
    fn with_api_limit_overrides_specific_api() {
        let config = RateLimitConfig::default().with_api_limit("cloudasset.googleapis.com", 5);
        assert_eq!(
            config.0.api_limits.get("cloudasset.googleapis.com"),
            Some(&5)
        );
        // Default unchanged
        assert_eq!(config.0.default_limit, 20);
    }

    #[test]
    fn with_api_limit_adds_new_api() {
        let config = RateLimitConfig::default().with_api_limit("custom.googleapis.com", 15);
        assert_eq!(config.0.api_limits.get("custom.googleapis.com"), Some(&15));
    }

    #[test]
    fn rate_limiter_uses_api_specific_semaphore() {
        let config = RateLimitConfig::default().with_api_limit("test.googleapis.com", 5);
        let limiter = RateLimiter::new(config.0);
        let stats = limiter.stats();
        let test_api = stats
            .iter()
            .find(|s| s.api == "test.googleapis.com")
            .unwrap();
        assert_eq!(test_api.limit, 5);
        assert_eq!(test_api.available, 5);
        assert_eq!(test_api.in_flight, 0);
    }

    #[test]
    fn rate_limiter_default_semaphore_in_stats() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config.0);
        let stats = limiter.stats();
        let default = stats.iter().find(|s| s.api == "default").unwrap();
        assert_eq!(default.limit, 20);
        assert_eq!(default.available, 20);
    }

    #[tokio::test]
    async fn acquire_uses_correct_semaphore() {
        let config = RateLimitConfig::default()
            .with_default_limit(100)
            .with_api_limit("compute.googleapis.com", 2);
        let limiter = RateLimiter::new(config.0);

        let _p1 = limiter
            .acquire("https://compute.googleapis.com/v1/foo")
            .await;
        let _p2 = limiter
            .acquire("https://compute.googleapis.com/v1/bar")
            .await;

        let stats = limiter.stats();
        let compute = stats
            .iter()
            .find(|s| s.api == "compute.googleapis.com")
            .unwrap();
        assert_eq!(compute.in_flight, 2);
        assert_eq!(compute.available, 0);

        let default = stats.iter().find(|s| s.api == "default").unwrap();
        assert_eq!(default.in_flight, 0);
    }

    #[tokio::test]
    async fn acquire_falls_back_to_default() {
        let config = RateLimitConfig::default().with_default_limit(3);
        let limiter = RateLimiter::new(config.0);

        let _p = limiter
            .acquire("https://unknown.googleapis.com/v1/foo")
            .await;

        let stats = limiter.stats();
        let default = stats.iter().find(|s| s.api == "default").unwrap();
        assert_eq!(default.in_flight, 1);
    }

    #[tokio::test]
    async fn permit_released_on_drop() {
        let config = RateLimitConfig::default().with_api_limit("test.googleapis.com", 1);
        let limiter = RateLimiter::new(config.0);

        {
            let _permit = limiter.acquire("https://test.googleapis.com/v1/foo").await;
            let stats = limiter.stats();
            let test_api = stats
                .iter()
                .find(|s| s.api == "test.googleapis.com")
                .unwrap();
            assert_eq!(test_api.in_flight, 1);
        }
        // Permit dropped

        let stats = limiter.stats();
        let test_api = stats
            .iter()
            .find(|s| s.api == "test.googleapis.com")
            .unwrap();
        assert_eq!(test_api.in_flight, 0);
    }
}
