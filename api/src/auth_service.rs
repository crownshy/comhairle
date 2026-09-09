pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use crate::error::ComhairleError;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthService: Send + Sync {
    // TODO:
    async fn get_users(&self) -> ();

    // TODO: service error
    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<serde_json::Value, ComhairleError>;
}
