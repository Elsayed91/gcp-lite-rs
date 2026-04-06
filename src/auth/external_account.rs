//! External account credential file parsing.
//!
//! Supports parsing GCP's external account JSON format for Workload Identity Federation.

use serde::Deserialize;

/// External account credential configuration.
#[derive(Debug, Deserialize)]
pub struct ExternalAccountConfig {
    /// Must be "external_account".
    #[serde(rename = "type")]
    pub credential_type: String,

    /// Workload Identity Pool audience.
    pub audience: String,

    /// Subject token type (usually JWT).
    pub subject_token_type: String,

    /// STS token URL.
    pub token_url: String,

    /// Credential source configuration.
    pub credential_source: CredentialSource,

    /// Service account impersonation URL (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_impersonation_url: Option<String>,
}

/// Credential source configuration.
#[derive(Debug, Deserialize)]
pub struct CredentialSource {
    /// Token file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    /// Token URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Environment ID (github, aws1, azure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,

    /// Token format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CredentialFormat>,
}

/// Token format configuration.
#[derive(Debug, Deserialize)]
pub struct CredentialFormat {
    /// Format type (text, json).
    #[serde(rename = "type")]
    pub format_type: String,
}

/// External account parsing errors.
#[derive(Debug, thiserror::Error)]
pub enum ExternalAccountError {
    /// Invalid JSON.
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    /// Unsupported credential type.
    #[error("Unsupported credential type: {0}")]
    UnsupportedType(String),

    /// Unsupported environment ID.
    #[error("Unsupported environment ID: {0}")]
    UnsupportedEnvironment(String),

    /// Missing credential source.
    #[error("Missing credential_source")]
    MissingCredentialSource,

    /// Missing service account impersonation URL.
    #[error("Missing service_account_impersonation_url")]
    MissingImpersonationUrl,

    /// Invalid impersonation URL format.
    #[error("Invalid impersonation URL format: {0}")]
    InvalidImpersonationUrl(String),

    /// OIDC provider error.
    #[error("OIDC error: {0}")]
    OidcError(#[from] crate::auth::oidc_providers::OidcError),
}

/// Extract service account email from impersonation URL.
///
/// # Errors
///
/// Returns an error if the URL format is invalid.
pub fn extract_sa_email(url: &str) -> Result<String, ExternalAccountError> {
    // URL format: https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{email}:generateAccessToken
    let parts: Vec<&str> = url.split('/').collect();
    if let Some(last) = parts.last()
        && let Some(email) = last.strip_suffix(":generateAccessToken")
    {
        return Ok(email.to_string());
    }

    Err(ExternalAccountError::InvalidImpersonationUrl(
        url.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_external_account_config() {
        let json = r#"{
            "type": "external_account",
            "audience": "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider",
            "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
            "token_url": "https://sts.googleapis.com/v1/token",
            "credential_source": {
                "environment_id": "github"
            },
            "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/sa@project.iam.gserviceaccount.com:generateAccessToken"
        }"#;

        let config: ExternalAccountConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.credential_type, "external_account");
        assert_eq!(
            config.audience,
            "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/pool/providers/provider"
        );
        assert_eq!(
            config.credential_source.environment_id,
            Some("github".into())
        );
    }

    #[test]
    fn test_extract_sa_email_from_url() {
        let url = "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/sa@project.iam.gserviceaccount.com:generateAccessToken";
        let email = extract_sa_email(url).unwrap();
        assert_eq!(email, "sa@project.iam.gserviceaccount.com");
    }
}
