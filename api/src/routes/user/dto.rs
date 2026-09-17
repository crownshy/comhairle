use axum_keycloak_auth::decode::KeycloakToken;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth_service;
use crate::error::ComhairleError;
use crate::models::users::{User, UserAuthType};
use crate::routes::auth::extract::ComhairleExtAttrs;

/// Data transfer object (public API representation) for a User.
///
/// This DTO is returned by user and auth related endpoints and is safe to expose
/// to clients. It intentionally omits sensitive and internal-only fields such
/// as:
///
/// * `password`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub guest_code: Option<String>,
    pub auth_type: UserAuthType,
    pub email_verified: bool,
    pub organization_id: Option<Uuid>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
            email: user.email,
            guest_code: user.guest_code,
            auth_type: user.auth_type,
            email_verified: user.email_verified,
            organization_id: user.organization_id,
        }
    }
}

impl From<auth_service::GetUserResponse> for UserDto {
    fn from(value: auth_service::GetUserResponse) -> Self {
        Self {
            id: value.sub,
            username: Some(value.preferred_username),
            avatar_url: None,
            email: value.email,
            guest_code: None,
            auth_type: value.comhairle_auth_type,
            email_verified: value.email_verified,
            organization_id: None,
        }
    }
}

impl TryFrom<KeycloakToken<String, ComhairleExtAttrs>> for UserDto {
    type Error = ComhairleError;

    fn try_from(token: KeycloakToken<String, ComhairleExtAttrs>) -> Result<Self, Self::Error> {
        let user_id = Uuid::parse_str(&token.subject)?;

        Ok(Self {
            id: user_id,
            username: Some(token.extra.profile.profile.preferred_username),
            // TODO: this assumes email always
            // present? Problematic for guest users
            email: Some(token.extra.profile.email.email),
            avatar_url: token.extra.avatar_url,
            guest_code: token.extra.guest_code,
            auth_type: token.extra.comhairle_auth_type,
            organization_id: token.extra.organization_id,
            email_verified: token.extra.profile.email.email_verified,
        })
    }
}
