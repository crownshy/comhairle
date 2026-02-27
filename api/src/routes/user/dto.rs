use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::users::{User, UserAuthType};

/// Data transfer object (public API representation) for a User.
///
/// This DTO is returned by user and auth related endpoints and is safe to expose
/// to clients. It intentionally omits sensitive and internal-only fields such
/// as:
///
/// * `password`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub auth_type: UserAuthType,
    pub email_verified: bool,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
            email: user.email,
            auth_type: user.auth_type,
            email_verified: user.email_verified,
        }
    }
}
