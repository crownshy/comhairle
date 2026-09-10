pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use crate::auth_service::error::AuthServiceError;
use crate::models::users::User;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthService: Send + Sync {
    async fn create_user(
        &self,
        comhairle_user: &User,
    ) -> Result<serde_json::Value, AuthServiceError>;

    // TODO: service error
    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<serde_json::Value, AuthServiceError>;
}
