//! Error types for GCP HTTP client operations.

use std::time::Duration;
use thiserror::Error;

/// Result type alias using GcpError
pub type Result<T> = std::result::Result<T, GcpError>;

/// Errors that can occur during GCP API operations
#[derive(Debug, Error, Clone)]
pub enum GcpError {
    /// Authentication failed
    #[error("Authentication failed: {message}")]
    Auth {
        /// Error message from GCP
        message: String,
    },

    /// Permission denied for the requested resource
    #[error("Permission denied: {message}")]
    PermissionDenied {
        /// Error message from GCP
        message: String,
        /// Resource that was accessed
        resource: String,
        /// HTTP method used
        method: String,
    },

    /// Resource not found
    #[error("Resource not found: {resource}")]
    NotFound {
        /// Resource that was not found
        resource: String,
        /// HTTP method used
        method: String,
    },

    /// Rate limit exceeded
    #[error("Rate limited (retry after {retry_after:?}s)")]
    RateLimited {
        /// Seconds to wait before retry (from Retry-After header)
        retry_after: Option<u64>,
        /// Error message from GCP
        message: String,
        /// Resource that was accessed
        resource: String,
    },

    /// GCP API is not enabled
    #[error("API not enabled: {api}")]
    ApiNotEnabled {
        /// API that needs to be enabled
        api: String,
        /// Error message from GCP
        message: String,
    },

    /// Quota exceeded
    #[error("Quota exceeded: {message}")]
    QuotaExceeded {
        /// Error message from GCP
        message: String,
        /// Resource that was accessed
        resource: String,
    },

    /// Invalid argument provided
    #[error("Invalid argument: {message}")]
    InvalidArgument {
        /// Error message from GCP
        message: String,
        /// Field that was invalid (if available)
        field: Option<String>,
    },

    /// Server error from GCP
    #[error("Server error ({status}): {message}")]
    ServerError {
        /// HTTP status code
        status: u16,
        /// Error message from GCP
        message: String,
        /// Resource that was accessed
        resource: String,
        /// Whether this error is retryable
        retryable: bool,
    },

    /// Operation timed out
    #[error("Operation timeout after {timeout:?}")]
    OperationTimeout {
        /// Operation name
        operation: String,
        /// Timeout duration
        timeout: Duration,
    },

    /// Operation failed
    #[error("Operation failed: {message}")]
    OperationFailed {
        /// Operation name
        operation: String,
        /// Error message
        message: String,
        /// GCP error code (if available)
        code: Option<String>,
    },

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Invalid response from GCP
    #[error("Invalid response: {message}")]
    InvalidResponse {
        /// Error description
        message: String,
        /// Raw response body (for debugging)
        body: Option<String>,
    },
}

impl From<reqwest::Error> for GcpError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
}

impl GcpError {
    /// Returns true if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited { .. }
                | Self::ServerError {
                    retryable: true,
                    ..
                }
                | Self::Network(_)
        )
    }

    /// Returns true if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::Auth { .. })
    }

    /// Extract retry-after duration if present
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited {
                retry_after: Some(secs),
                ..
            } => Some(Duration::from_secs(*secs)),
            _ => None,
        }
    }

    /// Get HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Auth { .. } => Some(401),
            Self::PermissionDenied { .. } => Some(403),
            Self::NotFound { .. } => Some(404),
            Self::RateLimited { .. } => Some(429),
            Self::ServerError { status, .. } => Some(*status),
            _ => None,
        }
    }
}
