pub mod config;
pub mod error;
pub mod keycloak;

use async_trait::async_trait;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::auth_service::error::AuthServiceError;
use crate::models::users::{UpdateUserRequest, User, UserAuthType};
use crate::routes::user::dto::UserDto;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthService: Send + Sync {
    async fn import_user(
        &self,
        comhairle_user: &User,
    ) -> Result<serde_json::Value, AuthServiceError>;

    async fn get_user_info(&self, token: &str) -> Result<GetUserInfoResponse, AuthServiceError>;

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserDto, AuthServiceError>;

    async fn get_user_by_email(&self, email: &str) -> Result<UserDto, AuthServiceError>;

    async fn update_user_details(
        &self,
        id: Uuid,
        payload: &UpdateUserRequest,
    ) -> Result<StatusCode, AuthServiceError>;

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
pub struct GetUserInfoResponse {
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
            .expect_get_user_info()
            .returning(|_| Box::pin(async move { Ok(GetUserInfoResponse::default()) }));
        auth_service.expect_get_user_by_id().returning(|_| {
            Box::pin(async move {
                Ok(UserDto {
                    id: Uuid::new_v4(),
                    username: Some("admin".to_string()),
                    email: Some("admin@crown-shy.com".to_string()),
                    auth_type: UserAuthType::EmailPassword,
                    guest_code: None,
                    avatar_url: None,
                    email_verified: false,
                    organization_id: None,
                })
            })
        });
        auth_service.expect_get_user_by_email().returning(|_| {
            Box::pin(async move {
                Ok(UserDto {
                    id: Uuid::new_v4(),
                    username: Some("admin".to_string()),
                    email: Some("admin@crown-shy.com".to_string()),
                    auth_type: UserAuthType::EmailPassword,
                    guest_code: None,
                    avatar_url: None,
                    email_verified: false,
                    organization_id: None,
                })
            })
        });
        auth_service.expect_update_user_details().returning(|_, _| {
            Box::pin(async move {
                Ok(UserDto {
                    id: Uuid::new_v4(),
                    username: Some("admin".to_string()),
                    email: Some("admin@crown-shy.com".to_string()),
                    auth_type: UserAuthType::EmailPassword,
                    guest_code: None,
                    avatar_url: None,
                    email_verified: false,
                    organization_id: None,
                })
            })
        });
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
