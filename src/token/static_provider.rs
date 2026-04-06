//! Static token provider for testing and pre-authenticated scenarios.

use super::{TokenError, TokenProvider};
use async_trait::async_trait;

/// Token provider that returns a fixed token.
///
/// Useful for testing and scenarios where the token is obtained externally.
pub struct StaticTokenProvider {
    token: String,
}

impl StaticTokenProvider {
    /// Create a new static token provider.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token to return
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait]
impl TokenProvider for StaticTokenProvider {
    async fn get_token(&self, _scopes: &[&str]) -> Result<String, TokenError> {
        Ok(self.token.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_configured_token() {
        let provider = StaticTokenProvider::new("my-token");
        let token = provider.get_token(&[]).await.unwrap();
        assert_eq!(token, "my-token");
    }
}
