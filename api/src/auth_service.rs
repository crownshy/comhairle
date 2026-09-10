pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;

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
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError>;
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct GetAuthorizationTokensResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub id_token: String,
    pub refresh_expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
    pub session_state: String,
    pub token_type: String,
}
