//! Authentication types for GCP credential handling.
//!
//! This module provides types for parsing and working with GCP credentials,
//! including service account keys and authorized user credentials.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// An OAuth2 access token with its expiration time.
#[derive(Debug, Clone)]
pub struct AccessToken {
    /// The bearer token string.
    pub token: String,
    /// Unix timestamp (seconds since epoch) when the token expires.
    pub expires_at: u64,
}

impl AccessToken {
    /// Create a new access token.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token string
    /// * `expires_at` - Unix timestamp when the token expires
    pub fn new(token: impl Into<String>, expires_at: u64) -> Self {
        Self {
            token: token.into(),
            expires_at,
        }
    }

    /// Check if the token has expired.
    ///
    /// Returns true if the current time is past the expiration time.
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now >= self.expires_at
    }

    /// Check if the token is expired or will expire within the given margin.
    ///
    /// # Arguments
    ///
    /// * `margin_secs` - Number of seconds before actual expiry to consider expired
    pub fn is_expired_with_margin(&self, margin_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now + margin_secs >= self.expires_at
    }
}

/// Thread-safe wrapper for caching access tokens.
///
/// Uses `tokio::sync::RwLock` for concurrent access.
#[derive(Debug)]
pub struct CachedToken {
    /// The cached token, protected by a read-write lock.
    token: Arc<RwLock<Option<AccessToken>>>,
}

impl Default for CachedToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CachedToken {
    /// Create a new empty cached token.
    pub fn new() -> Self {
        Self {
            token: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the cached token if it exists and is not expired.
    ///
    /// # Arguments
    ///
    /// * `margin_secs` - Number of seconds before actual expiry to consider expired
    pub async fn get(&self, margin_secs: u64) -> Option<String> {
        let guard = self.token.read().await;
        if let Some(ref token) = *guard
            && !token.is_expired_with_margin(margin_secs)
        {
            return Some(token.token.clone());
        }
        None
    }

    /// Store a new token in the cache.
    pub async fn set(&self, token: AccessToken) {
        let mut guard = self.token.write().await;
        *guard = Some(token);
    }

    /// Clear the cached token.
    pub async fn clear(&self) {
        let mut guard = self.token.write().await;
        *guard = None;
    }

    /// Clear the cached token synchronously (non-blocking).
    ///
    /// This is used by `on_token_rejected()` which is a sync method.
    /// Spawns an async task to acquire the write lock properly,
    /// ensuring the cache is always cleared even under contention.
    pub fn clear_sync(&self) {
        let token = Arc::clone(&self.token);
        tokio::spawn(async move {
            let mut guard = token.write().await;
            *guard = None;
        });
    }
}

impl Clone for CachedToken {
    fn clone(&self) -> Self {
        Self {
            token: Arc::clone(&self.token),
        }
    }
}

/// Service account key file contents.
///
/// This struct represents the JSON format used by GCP service account key files.
/// These files are typically downloaded from the GCP Console or created via
/// the `gcloud` CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccountKey {
    /// The credential type (should be "service_account").
    #[serde(rename = "type")]
    pub key_type: String,

    /// The GCP project ID.
    pub project_id: String,

    /// The private key ID.
    pub private_key_id: String,

    /// The PEM-encoded private key.
    pub private_key: String,

    /// The service account email address.
    pub client_email: String,

    /// The client ID.
    pub client_id: String,

    /// The authentication URI.
    pub auth_uri: String,

    /// The token URI (endpoint for obtaining tokens).
    pub token_uri: String,

    /// The auth provider X.509 certificate URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider_x509_cert_url: Option<String>,

    /// The client X.509 certificate URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_x509_cert_url: Option<String>,

    /// Universe domain (usually "googleapis.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_domain: Option<String>,
}

/// Authorized user credentials from OAuth2 flow.
///
/// This struct represents the JSON format used by `gcloud auth application-default login`
/// or other OAuth2 flows that produce refresh tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedUserCreds {
    /// The credential type (should be "authorized_user").
    #[serde(rename = "type")]
    pub cred_type: String,

    /// The OAuth2 client ID.
    pub client_id: String,

    /// The OAuth2 client secret.
    pub client_secret: String,

    /// The refresh token for obtaining new access tokens.
    pub refresh_token: String,

    /// Optional quota project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_project_id: Option<String>,
}

/// Credential file types that can be parsed from JSON.
///
/// GCP uses different JSON formats for different credential types.
/// This enum allows parsing a credential file without knowing the type ahead of time.
#[derive(Debug, Clone)]
pub enum CredentialFile {
    /// A service account key file.
    ServiceAccount(ServiceAccountKey),
    /// Authorized user credentials (from OAuth2 flow).
    AuthorizedUser(AuthorizedUserCreds),
}

impl CredentialFile {
    /// Parse a credential file from JSON bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid or the credential type is unknown.
    pub fn from_json(json: &[u8]) -> Result<Self, CredentialFileError> {
        // First, determine the type
        let value: serde_json::Value =
            serde_json::from_slice(json).map_err(|e| CredentialFileError::InvalidJson {
                message: e.to_string(),
            })?;

        let cred_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or(CredentialFileError::MissingType)?;

        match cred_type {
            "service_account" => {
                let key: ServiceAccountKey = serde_json::from_value(value).map_err(|e| {
                    CredentialFileError::InvalidJson {
                        message: e.to_string(),
                    }
                })?;
                Ok(CredentialFile::ServiceAccount(key))
            }
            "authorized_user" => {
                let creds: AuthorizedUserCreds = serde_json::from_value(value).map_err(|e| {
                    CredentialFileError::InvalidJson {
                        message: e.to_string(),
                    }
                })?;
                Ok(CredentialFile::AuthorizedUser(creds))
            }
            other => Err(CredentialFileError::UnknownType {
                cred_type: other.to_string(),
            }),
        }
    }
}

/// Errors that can occur when parsing credential files.
#[derive(Debug, thiserror::Error)]
pub enum CredentialFileError {
    /// Invalid JSON format.
    #[error("Invalid JSON: {message}")]
    InvalidJson {
        /// Error description
        message: String,
    },

    /// Missing "type" field in credential file.
    #[error("Credential file missing 'type' field")]
    MissingType,

    /// Unknown credential type.
    #[error("Unknown credential type: {cred_type}")]
    UnknownType {
        /// The unknown type string
        cred_type: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_creation() {
        let token = AccessToken::new("test-token", 1234567890);
        assert_eq!(token.token, "test-token");
        assert_eq!(token.expires_at, 1234567890);
    }

    #[test]
    fn test_access_token_expired() {
        // Token that expired in the past
        let token = AccessToken::new("test-token", 0);
        assert!(token.is_expired());

        // Token that expires far in the future
        let token = AccessToken::new("test-token", u64::MAX);
        assert!(!token.is_expired());
    }

    #[test]
    fn test_access_token_expired_with_margin() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Token expires in 60 seconds
        let token = AccessToken::new("test-token", now + 60);

        // Not expired with 30 second margin
        assert!(!token.is_expired_with_margin(30));

        // Expired with 120 second margin
        assert!(token.is_expired_with_margin(120));
    }

    #[tokio::test]
    async fn test_cached_token_empty() {
        let cache = CachedToken::new();
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_set_get() {
        let cache = CachedToken::new();
        let token = AccessToken::new("cached-token", u64::MAX);
        cache.set(token).await;

        let retrieved = cache.get(0).await;
        assert_eq!(retrieved, Some("cached-token".to_string()));
    }

    #[tokio::test]
    async fn test_cached_token_expired() {
        let cache = CachedToken::new();
        let token = AccessToken::new("expired-token", 0);
        cache.set(token).await;

        // Should return None because token is expired
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_clear() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        cache.clear().await;
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_clone() {
        let cache1 = CachedToken::new();
        let cache2 = cache1.clone();

        let token = AccessToken::new("shared-token", u64::MAX);
        cache1.set(token).await;

        // Both caches should see the same token (shared state)
        assert_eq!(cache2.get(0).await, Some("shared-token".to_string()));
    }

    #[tokio::test]
    async fn test_cached_token_clear_sync() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        // clear_sync should clear the token
        cache.clear_sync();
        // Give the spawned task time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_concurrent_clear_sync() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        // Simulate concurrent clear_sync calls (token rejection from multiple threads)
        let cache_clone1 = cache.clone();
        let cache_clone2 = cache.clone();
        let cache_clone3 = cache.clone();

        let handle1 = tokio::spawn(async move {
            cache_clone1.clear_sync();
        });
        let handle2 = tokio::spawn(async move {
            cache_clone2.clear_sync();
        });
        let handle3 = tokio::spawn(async move {
            cache_clone3.clear_sync();
        });

        // Wait for all tasks to complete
        let _ = tokio::join!(handle1, handle2, handle3);

        // Give spawned clear tasks time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Cache should be cleared even with concurrent calls
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_clear_sync_under_read_contention() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        // Simulate read contention: hold a read lock while calling clear_sync
        let cache_clone = cache.clone();
        let read_guard = cache.token.read().await;

        // Call clear_sync while read lock is held
        // The old implementation would silently fail here
        cache_clone.clear_sync();

        // Drop the read lock
        drop(read_guard);

        // Give the spawned clear task time to acquire the write lock and clear
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Cache should be cleared even though clear_sync was called during read contention
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_clear_sync_under_write_contention() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        // Simulate write contention: acquire write lock and hold it briefly
        let cache_clone = cache.clone();
        let write_task = tokio::spawn(async move {
            let mut guard = cache_clone.token.write().await;
            // Hold write lock briefly
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
            *guard = Some(AccessToken::new("contended-token", u64::MAX));
        });

        // Call clear_sync while write lock is held in another task
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        cache.clear_sync();

        // Wait for write task to complete
        write_task.await.unwrap();

        // Give the spawned clear task time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Cache should be cleared by clear_sync even though it was called during write contention
        assert!(cache.get(0).await.is_none());
    }

    #[tokio::test]
    async fn test_cached_token_concurrent_reads_and_clear_sync() {
        let cache = CachedToken::new();
        let token = AccessToken::new("test-token", u64::MAX);
        cache.set(token).await;

        // Simulate concurrent reads and clear_sync
        let mut handles = vec![];

        // Spawn multiple readers
        for _ in 0..10 {
            let cache_clone = cache.clone();
            handles.push(tokio::spawn(async move {
                for _ in 0..5 {
                    let _ = cache_clone.get(0).await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                }
            }));
        }

        // Spawn clear_sync in the middle of reads
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        cache.clear_sync();

        // Wait for all readers to complete
        for handle in handles {
            handle.await.unwrap();
        }

        // Give clear task time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Cache should be cleared
        assert!(cache.get(0).await.is_none());
    }

    #[test]
    fn test_service_account_key_deserialization() {
        let json = r#"{
            "type": "service_account",
            "project_id": "my-project",
            "private_key_id": "key123",
            "private_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n-----END RSA PRIVATE KEY-----\n",
            "client_email": "sa@my-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token",
            "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
            "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/sa%40my-project.iam.gserviceaccount.com"
        }"#;

        let key: ServiceAccountKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.key_type, "service_account");
        assert_eq!(key.project_id, "my-project");
        assert_eq!(key.private_key_id, "key123");
        assert!(key.private_key.contains("BEGIN RSA PRIVATE KEY"));
        assert_eq!(key.client_email, "sa@my-project.iam.gserviceaccount.com");
        assert_eq!(key.client_id, "123456789");
        assert_eq!(key.auth_uri, "https://accounts.google.com/o/oauth2/auth");
        assert_eq!(key.token_uri, "https://oauth2.googleapis.com/token");
        assert!(key.auth_provider_x509_cert_url.is_some());
        assert!(key.client_x509_cert_url.is_some());
    }

    #[test]
    fn test_service_account_key_minimal() {
        // Test with only required fields
        let json = r#"{
            "type": "service_account",
            "project_id": "my-project",
            "private_key_id": "key123",
            "private_key": "-----BEGIN RSA PRIVATE KEY-----\ntest\n-----END RSA PRIVATE KEY-----\n",
            "client_email": "sa@my-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }"#;

        let key: ServiceAccountKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.key_type, "service_account");
        assert!(key.auth_provider_x509_cert_url.is_none());
        assert!(key.client_x509_cert_url.is_none());
        assert!(key.universe_domain.is_none());
    }

    #[test]
    fn test_service_account_key_serialization() {
        let key = ServiceAccountKey {
            key_type: "service_account".to_string(),
            project_id: "my-project".to_string(),
            private_key_id: "key123".to_string(),
            private_key: "-----BEGIN RSA PRIVATE KEY-----\ntest\n-----END RSA PRIVATE KEY-----\n"
                .to_string(),
            client_email: "sa@my-project.iam.gserviceaccount.com".to_string(),
            client_id: "123456789".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            auth_provider_x509_cert_url: None,
            client_x509_cert_url: None,
            universe_domain: None,
        };

        let json = serde_json::to_string(&key).unwrap();
        let parsed: ServiceAccountKey = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.project_id, key.project_id);
    }

    #[test]
    fn test_authorized_user_creds_deserialization() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "client123.apps.googleusercontent.com",
            "client_secret": "secret456",
            "refresh_token": "1//refresh-token-here"
        }"#;

        let creds: AuthorizedUserCreds = serde_json::from_str(json).unwrap();
        assert_eq!(creds.cred_type, "authorized_user");
        assert_eq!(creds.client_id, "client123.apps.googleusercontent.com");
        assert_eq!(creds.client_secret, "secret456");
        assert_eq!(creds.refresh_token, "1//refresh-token-here");
        assert!(creds.quota_project_id.is_none());
    }

    #[test]
    fn test_authorized_user_creds_with_quota_project() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "client123.apps.googleusercontent.com",
            "client_secret": "secret456",
            "refresh_token": "1//refresh-token-here",
            "quota_project_id": "my-quota-project"
        }"#;

        let creds: AuthorizedUserCreds = serde_json::from_str(json).unwrap();
        assert_eq!(creds.quota_project_id, Some("my-quota-project".to_string()));
    }

    #[test]
    fn test_authorized_user_creds_serialization() {
        let creds = AuthorizedUserCreds {
            cred_type: "authorized_user".to_string(),
            client_id: "client123".to_string(),
            client_secret: "secret456".to_string(),
            refresh_token: "refresh789".to_string(),
            quota_project_id: None,
        };

        let json = serde_json::to_string(&creds).unwrap();
        let parsed: AuthorizedUserCreds = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.client_id, creds.client_id);
    }

    #[test]
    fn test_credential_file_parse_service_account() {
        let json = br#"{
            "type": "service_account",
            "project_id": "my-project",
            "private_key_id": "key123",
            "private_key": "-----BEGIN RSA PRIVATE KEY-----\ntest\n-----END RSA PRIVATE KEY-----\n",
            "client_email": "sa@my-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }"#;

        let cred = CredentialFile::from_json(json).unwrap();
        match cred {
            CredentialFile::ServiceAccount(key) => {
                assert_eq!(key.project_id, "my-project");
            }
            _ => panic!("Expected ServiceAccount"),
        }
    }

    #[test]
    fn test_credential_file_parse_authorized_user() {
        let json = br#"{
            "type": "authorized_user",
            "client_id": "client123.apps.googleusercontent.com",
            "client_secret": "secret456",
            "refresh_token": "1//refresh-token-here"
        }"#;

        let cred = CredentialFile::from_json(json).unwrap();
        match cred {
            CredentialFile::AuthorizedUser(creds) => {
                assert_eq!(creds.client_id, "client123.apps.googleusercontent.com");
            }
            _ => panic!("Expected AuthorizedUser"),
        }
    }

    #[test]
    fn test_credential_file_unknown_type() {
        let json = br#"{"type": "unknown_type"}"#;

        let result = CredentialFile::from_json(json);
        assert!(matches!(
            result,
            Err(CredentialFileError::UnknownType { .. })
        ));
    }

    #[test]
    fn test_credential_file_missing_type() {
        let json = br#"{"project_id": "test"}"#;

        let result = CredentialFile::from_json(json);
        assert!(matches!(result, Err(CredentialFileError::MissingType)));
    }

    #[test]
    fn test_credential_file_invalid_json() {
        let json = b"not valid json";

        let result = CredentialFile::from_json(json);
        assert!(matches!(
            result,
            Err(CredentialFileError::InvalidJson { .. })
        ));
    }
}
