pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::auth_service::error::AuthServiceError;
use crate::models::users::{User, UserAuthType};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthService: Send + Sync {
    async fn import_user(
        &self,
        comhairle_user: &User,
    ) -> Result<serde_json::Value, AuthServiceError>;

    async fn get_user(&self, token: &str) -> Result<GetUserResponse, AuthServiceError>;

    async fn get_authorization_tokens(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError>;

    async fn refresh_session(
        &self,
        refresh_token: &str,
    ) -> Result<GetAuthorizationTokensResponse, AuthServiceError>;
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct GetUserResponse {
    pub sub: Uuid,
    pub email_verified: bool,
    pub preferred_username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub comhairle_auth_type: UserAuthType,
    pub avatar_url: Option<String>,
    pub organization_id: Option<Uuid>,
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct GetAuthorizationTokensResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub id_token: String,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    pub session_state: String,
    pub token_type: String,
}

#[cfg(test)]
impl MockAuthService {
    pub fn base() -> MockAuthService {
        let mut auth_service = MockAuthService::new();

        auth_service
            .expect_import_user()
            .returning(|_| Box::pin(async move { Ok(serde_json::json!({})) }));
        auth_service
            .expect_get_user()
            .returning(|_| Box::pin(async move { Ok(GetUserResponse::default()) }));
        auth_service
            .expect_get_authorization_tokens()
            .returning(|_, _| {
                Box::pin(async move { Ok(GetAuthorizationTokensResponse::default()) })
            });
        auth_service
            .expect_refresh_session()
            .returning(|_| Box::pin(async move { Ok(GetAuthorizationTokensResponse::default()) }));

        auth_service
    }
}
