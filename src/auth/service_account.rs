//! Service account credential implementation for GCP authentication.
//!
//! This module provides `ServiceAccountCredential`, which implements the `TokenProvider`
//! trait for authenticating with GCP APIs using service account key files.
//!
//! # Example
//!
//! ```no_run
//! use gcp_lite::auth::ServiceAccountCredential;
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // From a file
//! let cred = ServiceAccountCredential::from_file(Path::new("/path/to/service-account.json"))?;
//!
//! // Or from JSON string
//! let json = std::fs::read_to_string("/path/to/service-account.json")?;
//! let cred = ServiceAccountCredential::from_json(&json)?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::types::{AccessToken, CachedToken, ServiceAccountKey};
use crate::token::{TokenError, TokenProvider};

/// Default expiry buffer in seconds (60 seconds).
/// Tokens will be refreshed this many seconds before actual expiry.
const TOKEN_EXPIRY_BUFFER_SECS: u64 = 60;

/// Token endpoint for Google OAuth2.
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

/// JWT claims for GCP OAuth2 token exchange.
#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    /// Issuer (service account email).
    iss: String,
    /// Scopes requested (space-separated).
    scope: String,
    /// Audience (token endpoint URL).
    aud: String,
    /// Issued at timestamp (Unix seconds).
    iat: u64,
    /// Expiration timestamp (Unix seconds).
    exp: u64,
}

/// Response from the OAuth2 token endpoint.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    /// The access token.
    access_token: String,
    /// Token lifetime in seconds.
    expires_in: u64,
    /// Token type (usually "Bearer").
    #[allow(dead_code)]
    token_type: String,
}

/// A credential provider that uses a GCP service account key to obtain access tokens.
///
/// This credential type:
/// - Parses service account JSON key files
/// - Generates JWTs signed with RS256
/// - Exchanges JWTs for access tokens via Google's OAuth2 endpoint
/// - Caches tokens with a configurable expiry buffer
///
/// # Thread Safety
///
/// `ServiceAccountCredential` is `Send + Sync` and can be safely shared across threads.
/// Token caching is handled internally using `tokio::sync::RwLock`.
#[derive(Debug)]
pub struct ServiceAccountCredential {
    /// The parsed service account key.
    key: ServiceAccountKey,
    /// Cached access token.
    cache: CachedToken,
    /// HTTP client for token exchange.
    http_client: reqwest::Client,
}

impl ServiceAccountCredential {
    /// Create a new credential from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - The service account key JSON as a string
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid or missing required fields.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::ServiceAccountCredential;
    ///
    /// let json = r#"{
    ///     "type": "service_account",
    ///     "project_id": "my-project",
    ///     "private_key_id": "key123",
    ///     "private_key": "-----BEGIN RSA PRIVATE KEY-----\n...\n-----END RSA PRIVATE KEY-----\n",
    ///     "client_email": "sa@my-project.iam.gserviceaccount.com",
    ///     "client_id": "123456789",
    ///     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    ///     "token_uri": "https://oauth2.googleapis.com/token"
    /// }"#;
    ///
    /// let cred = ServiceAccountCredential::from_json(json).unwrap();
    /// ```
    pub fn from_json(json: &str) -> Result<Self, ServiceAccountError> {
        let key: ServiceAccountKey =
            serde_json::from_str(json).map_err(|e| ServiceAccountError::InvalidJson {
                message: e.to_string(),
            })?;

        Self::validate_key(&key)?;

        Ok(Self {
            key,
            cache: CachedToken::new(),
            http_client: reqwest::Client::new(),
        })
    }

    /// Create a new credential from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the service account key JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the JSON is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gcp_lite::auth::ServiceAccountCredential;
    /// use std::path::Path;
    ///
    /// let cred = ServiceAccountCredential::from_file(Path::new("/path/to/key.json")).unwrap();
    /// ```
    pub fn from_file(path: &Path) -> Result<Self, ServiceAccountError> {
        let json =
            std::fs::read_to_string(path).map_err(|e| ServiceAccountError::FileReadError {
                path: path.to_path_buf(),
                source: e,
            })?;

        Self::from_json(&json)
    }

    /// Create a new credential with a custom HTTP client.
    ///
    /// This is useful for testing or when you need to customize the HTTP client
    /// used for token exchange (e.g., proxy settings, timeouts).
    ///
    /// # Arguments
    ///
    /// * `key` - The parsed service account key
    /// * `http_client` - A custom reqwest::Client to use for token exchange
    pub fn with_http_client(key: ServiceAccountKey, http_client: reqwest::Client) -> Self {
        Self {
            key,
            cache: CachedToken::new(),
            http_client,
        }
    }

    /// Get the project ID from the service account key.
    pub fn project_id(&self) -> &str {
        &self.key.project_id
    }

    /// Get the service account email.
    pub fn client_email(&self) -> &str {
        &self.key.client_email
    }

    /// Validate that the key has all required fields.
    fn validate_key(key: &ServiceAccountKey) -> Result<(), ServiceAccountError> {
        if key.key_type != "service_account" {
            return Err(ServiceAccountError::InvalidKeyType {
                expected: "service_account".to_string(),
                actual: key.key_type.clone(),
            });
        }

        if key.private_key.is_empty() {
            return Err(ServiceAccountError::MissingField {
                field: "private_key".to_string(),
            });
        }

        if key.client_email.is_empty() {
            return Err(ServiceAccountError::MissingField {
                field: "client_email".to_string(),
            });
        }

        Ok(())
    }

    /// Create a JWT for the given scopes.
    fn create_jwt(&self, scopes: &[&str]) -> Result<String, ServiceAccountError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ServiceAccountError::JwtCreationFailed {
                message: format!("Failed to get current time: {}", e),
            })?
            .as_secs();

        let claims = JwtClaims {
            iss: self.key.client_email.clone(),
            scope: scopes.join(" "),
            aud: TOKEN_ENDPOINT.to_string(),
            iat: now,
            exp: now + 3600, // 1 hour
        };

        let header = Header::new(Algorithm::RS256);
        let encoding_key =
            EncodingKey::from_rsa_pem(self.key.private_key.as_bytes()).map_err(|e| {
                ServiceAccountError::InvalidPrivateKey {
                    message: e.to_string(),
                }
            })?;

        encode(&header, &claims, &encoding_key).map_err(|e| {
            ServiceAccountError::JwtCreationFailed {
                message: e.to_string(),
            }
        })
    }

    /// Exchange a JWT for an access token.
    async fn exchange_jwt(&self, jwt: &str) -> Result<TokenResponse, ServiceAccountError> {
        let body = format!(
            "grant_type={}&assertion={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
            urlencoding::encode(jwt)
        );

        let response = self
            .http_client
            .post(TOKEN_ENDPOINT)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .map_err(|e| ServiceAccountError::TokenExchangeFailed {
                message: format!("HTTP request failed: {}", e),
            })?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|e| ServiceAccountError::TokenExchangeFailed {
                    message: format!("Failed to read response body: {}", e),
                })?;

        if !status.is_success() {
            return Err(ServiceAccountError::TokenExchangeFailed {
                message: format!("Token endpoint returned {}: {}", status, response_text),
            });
        }

        serde_json::from_str(&response_text).map_err(|e| ServiceAccountError::TokenExchangeFailed {
            message: format!("Failed to parse token response: {}", e),
        })
    }

    /// Get a fresh access token for the given scopes.
    async fn fetch_token(&self, scopes: &[&str]) -> Result<AccessToken, ServiceAccountError> {
        let jwt = self.create_jwt(scopes)?;
        let response = self.exchange_jwt(&jwt).await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ServiceAccountError::TokenExchangeFailed {
                message: format!("Failed to get current time: {}", e),
            })?
            .as_secs();

        Ok(AccessToken::new(
            response.access_token,
            now + response.expires_in,
        ))
    }
}

#[async_trait]
impl TokenProvider for ServiceAccountCredential {
    async fn get_token(&self, scopes: &[&str]) -> Result<String, TokenError> {
        // Check cache first
        if let Some(token) = self.cache.get(TOKEN_EXPIRY_BUFFER_SECS).await {
            return Ok(token);
        }

        // Fetch a new token
        let token = self
            .fetch_token(scopes)
            .await
            .map_err(|e| TokenError::RefreshFailed {
                message: e.to_string(),
            })?;

        let token_string = token.token.clone();
        self.cache.set(token).await;

        Ok(token_string)
    }

    fn on_token_rejected(&self) {
        // Clear the cache synchronously when a token is rejected
        // Uses try_write() to avoid blocking - if contended, the next
        // get_token() call will refresh anyway
        self.cache.clear_sync();
    }

    fn quota_project_id(&self) -> Option<&str> {
        Some(&self.key.project_id)
    }
}

/// Errors that can occur when using service account credentials.
#[derive(Debug, thiserror::Error)]
pub enum ServiceAccountError {
    /// Failed to read the key file.
    #[error("Failed to read key file at {path}: {source}")]
    FileReadError {
        /// Path to the file that could not be read.
        path: std::path::PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Invalid JSON format.
    #[error("Invalid JSON: {message}")]
    InvalidJson {
        /// Error description.
        message: String,
    },

    /// Invalid key type (expected "service_account").
    #[error("Invalid key type: expected '{expected}', got '{actual}'")]
    InvalidKeyType {
        /// Expected key type.
        expected: String,
        /// Actual key type found.
        actual: String,
    },

    /// Missing required field.
    #[error("Missing required field: {field}")]
    MissingField {
        /// Name of the missing field.
        field: String,
    },

    /// Invalid private key format.
    #[error("Invalid private key: {message}")]
    InvalidPrivateKey {
        /// Error description.
        message: String,
    },

    /// Failed to create JWT.
    #[error("Failed to create JWT: {message}")]
    JwtCreationFailed {
        /// Error description.
        message: String,
    },

    /// Failed to exchange JWT for access token.
    #[error("Token exchange failed: {message}")]
    TokenExchangeFailed {
        /// Error description.
        message: String,
    },

    /// Failed to create HTTP client.
    #[error("Failed to create HTTP client: {source}")]
    HttpClientCreation {
        /// The underlying reqwest error.
        #[source]
        source: reqwest::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test RSA private key (DO NOT USE IN PRODUCTION - this is for testing only)
    // Generated with: openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 | openssl rsa -traditional
    const TEST_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEAv/ZgLU6ZAC6tVLSjrzeVeigCbmFVXFqeohH0wsveMsgUnktr
KoQUhclteEs7iGLwKPQdWyOMIaQFC320l1wqvJKg7XbWtYyC856yBtHBisXzjUIP
vppA7Ie6N0uKtqZ1HLXKbqEd5bNPEU61LJJgLXOdXRbb+9EhLusQpWQb3cPLI/za
qmSa6UYwEv8GCtIgNqTjKycHCi0MqKpjNZ6wFzvPruLqnkOhtA2LVsklp+9jYxca
yv9UBS5xVQ01WHHSR/J//G2v0yCUzmitdDJTQyOd4zlPkHpm6T69m7NaE1fTVSiP
sVfO3Hn7VfzAgbQkZH40Q+OSlTubBQcZ8JyJWQIDAQABAoIBAAJOMBqt3GO2lnbT
YjmJvPDAt8IXHFUVsoeG7diuuvOtraUMCf9dTY4gx0DgAxjwz+pnVM3s0p3vJW9d
T7SsqEe8/r6eBCyd1s8cYLjOaUO50Q0T1h1nfAWgiKw+1Zg6zg0YTX7VeQbdR9hm
SKyTx8tr8sp01T4EOuDgdQH40+aD0ivfbFdIyim82IGh7HHvJyMVxTsMHG1fRo/d
kYpT3g2jOpEYCe0EnmAp2bB1kLLonlW+Xp3OOYShLXUtwXf8q/fOMcYbm3BV2OvC
zhTaKmvStEpMhLHihsNJIf0uQZypY6lNu9IDhj2dacKjRNEpZn+ulwfdcx46VEAy
gnt9IKECgYEA/Y4fSVQ4TsGJvH4vjb0+3o4uckWEPTffUwJG6r4PYSZGA8qQpr1E
oyxXdZt0atcsXs1Qd1E2t2FruumS18CTQvJE0+q06kqa6djYH7svOzfjxC5F4Np4
EkMYEVecJ5qG1bU77BqJwY4rTRQ96e5PmgjTPV9hfxvNX51JTm06gfECgYEAwdA3
pabK+6x2hjxyNLldPdcctFZqA16OeL5VWk5/7L2kLFUYVjyz129vAtkJKnB+GAJs
uZcNwhBbqSiK0vv5WoQOIzrSzbzLZ7STjymMzZ6FOkdElbD6H811idKpI87fQD0O
Eo4L5wslqIVxR8ktoRJgRvI6sFm1ajSsarlrlekCgYAC7cJUwYFI/5lMsRRxia8R
OQk2TrFBV8Tfm5YgHgPldmC2qH9VPbhuPhPgiuQkW8nqamq0hh6graJl7U7B6TqK
OmwrGnnufuAdNWEBtNLN105tNK+f8kYSx+2ePanTF0jZbRd9Ga1fq/m6ETLJ4fPP
bqyp99ETe8m6ggGXw1E6sQKBgG950rf90pylWtrk44992qqaEtGLLpjXhzzdxPwX
UK8beNVi8IeRjKNqXcCWkxYM9AndQyoQPwKTJBWM0yR9d7PfZr5OtDdP0vLIQ2NB
s9IEzn5xxXoP/B3UsDlgqJaHA5PQSkrT1vbCS5u9fSWcChmuFyBXbPhH8PewakdM
dRwZAoGAexRCCrNBek6bxaCVI8JqRRIGYPAwr9sUjKwn8Tdhutx8lvcKOk6AHG1I
uQAVf8HQ3eHRgsCSodf1XeoLWX+0Nxt/KJ1KotVlchFlCLuSzpNkR7WbLC7QfCkJ
RLK9OKOIcBVVctVsUtrWLjTEHyKVhYwIW98X+LAal+i55n75SHU=
-----END RSA PRIVATE KEY-----";

    fn test_key_json() -> String {
        format!(
            r#"{{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "key123",
            "private_key": {:?},
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }}"#,
            TEST_PRIVATE_KEY
        )
    }

    #[test]
    fn test_from_json_valid() {
        let json = test_key_json();
        let cred = ServiceAccountCredential::from_json(&json).unwrap();

        assert_eq!(cred.project_id(), "test-project");
        assert_eq!(
            cred.client_email(),
            "test@test-project.iam.gserviceaccount.com"
        );
    }

    #[test]
    fn test_from_json_invalid_json() {
        let result = ServiceAccountCredential::from_json("not valid json");
        assert!(matches!(
            result,
            Err(ServiceAccountError::InvalidJson { .. })
        ));
    }

    #[test]
    fn test_from_json_wrong_type() {
        let json = r#"{
            "type": "authorized_user",
            "client_id": "123",
            "client_secret": "secret",
            "refresh_token": "token"
        }"#;
        // This will fail during deserialization because ServiceAccountKey requires different fields
        let result = ServiceAccountCredential::from_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_json_missing_private_key() {
        let json = r#"{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "key123",
            "private_key": "",
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }"#;

        let result = ServiceAccountCredential::from_json(json);
        assert!(matches!(
            result,
            Err(ServiceAccountError::MissingField { field }) if field == "private_key"
        ));
    }

    #[test]
    fn test_from_json_missing_client_email() {
        let json = format!(
            r#"{{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "key123",
            "private_key": {:?},
            "client_email": "",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }}"#,
            TEST_PRIVATE_KEY
        );

        let result = ServiceAccountCredential::from_json(&json);
        assert!(matches!(
            result,
            Err(ServiceAccountError::MissingField { field }) if field == "client_email"
        ));
    }

    #[test]
    fn test_from_file_not_found() {
        let result = ServiceAccountCredential::from_file(Path::new("/nonexistent/file.json"));
        assert!(matches!(
            result,
            Err(ServiceAccountError::FileReadError { .. })
        ));
    }

    #[test]
    fn test_create_jwt() {
        let json = test_key_json();
        let cred = ServiceAccountCredential::from_json(&json).unwrap();

        let jwt = cred
            .create_jwt(&["https://www.googleapis.com/auth/cloud-platform"])
            .unwrap();

        // JWT should have 3 parts separated by dots
        let parts: Vec<&str> = jwt.split('.').collect();
        assert_eq!(parts.len(), 3);

        // Decode and verify header
        let header_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[0])
                .unwrap();
        let header: serde_json::Value = serde_json::from_slice(&header_bytes).unwrap();
        assert_eq!(header["alg"], "RS256");
        assert_eq!(header["typ"], "JWT");

        // Decode and verify claims
        let claims_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[1])
                .unwrap();
        let claims: serde_json::Value = serde_json::from_slice(&claims_bytes).unwrap();
        assert_eq!(claims["iss"], "test@test-project.iam.gserviceaccount.com");
        assert_eq!(
            claims["scope"],
            "https://www.googleapis.com/auth/cloud-platform"
        );
        assert_eq!(claims["aud"], "https://oauth2.googleapis.com/token");

        // Verify exp is 1 hour after iat
        let iat = claims["iat"].as_u64().unwrap();
        let exp = claims["exp"].as_u64().unwrap();
        assert_eq!(exp - iat, 3600);
    }

    #[test]
    fn test_create_jwt_multiple_scopes() {
        let json = test_key_json();
        let cred = ServiceAccountCredential::from_json(&json).unwrap();

        let jwt = cred
            .create_jwt(&[
                "https://www.googleapis.com/auth/cloud-platform",
                "https://www.googleapis.com/auth/compute",
            ])
            .unwrap();

        let parts: Vec<&str> = jwt.split('.').collect();
        let claims_bytes =
            base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[1])
                .unwrap();
        let claims: serde_json::Value = serde_json::from_slice(&claims_bytes).unwrap();

        assert_eq!(
            claims["scope"],
            "https://www.googleapis.com/auth/cloud-platform https://www.googleapis.com/auth/compute"
        );
    }

    #[test]
    fn test_create_jwt_invalid_private_key() {
        let json = r#"{
            "type": "service_account",
            "project_id": "test-project",
            "private_key_id": "key123",
            "private_key": "not a valid key",
            "client_email": "test@test-project.iam.gserviceaccount.com",
            "client_id": "123456789",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token"
        }"#;

        // This should fail during key validation since we now check for empty private keys
        // But since "not a valid key" is not empty, it will pass validation but fail JWT creation
        let cred = ServiceAccountCredential::from_json(json).unwrap();
        let result = cred.create_jwt(&["scope"]);
        assert!(matches!(
            result,
            Err(ServiceAccountError::InvalidPrivateKey { .. })
        ));
    }

    #[test]
    fn test_jwt_claims_serialization() {
        let claims = JwtClaims {
            iss: "test@example.com".to_string(),
            scope: "scope1 scope2".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            iat: 1000,
            exp: 4600,
        };

        let json = serde_json::to_string(&claims).unwrap();
        let parsed: JwtClaims = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.iss, "test@example.com");
        assert_eq!(parsed.scope, "scope1 scope2");
        assert_eq!(parsed.aud, "https://oauth2.googleapis.com/token");
        assert_eq!(parsed.iat, 1000);
        assert_eq!(parsed.exp, 4600);
    }

    #[test]
    fn test_error_display() {
        let err = ServiceAccountError::InvalidJson {
            message: "test error".to_string(),
        };
        assert!(err.to_string().contains("Invalid JSON"));

        let err = ServiceAccountError::InvalidKeyType {
            expected: "service_account".to_string(),
            actual: "other".to_string(),
        };
        assert!(err.to_string().contains("service_account"));
        assert!(err.to_string().contains("other"));

        let err = ServiceAccountError::MissingField {
            field: "private_key".to_string(),
        };
        assert!(err.to_string().contains("private_key"));

        let err = ServiceAccountError::InvalidPrivateKey {
            message: "bad key".to_string(),
        };
        assert!(err.to_string().contains("bad key"));

        let err = ServiceAccountError::JwtCreationFailed {
            message: "jwt error".to_string(),
        };
        assert!(err.to_string().contains("jwt error"));

        let err = ServiceAccountError::TokenExchangeFailed {
            message: "exchange error".to_string(),
        };
        assert!(err.to_string().contains("exchange error"));
    }

    #[test]
    fn test_quota_project_id_returns_project_id() {
        let json = test_key_json();
        let cred = ServiceAccountCredential::from_json(&json).unwrap();

        // Service accounts use their project_id as quota project
        assert_eq!(cred.quota_project_id(), Some("test-project"));
    }
}
