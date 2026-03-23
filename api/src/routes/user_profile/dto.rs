use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user_profile::UserProfile;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub consented: bool,
    pub ethnicity: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub political_party: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserProfile> for UserProfileDto {
    fn from(profile: UserProfile) -> Self {
        UserProfileDto {
            id: profile.id,
            user_id: profile.user_id,
            consented: profile.consented,
            ethnicity: profile.ethnicity,
            age: profile.age,
            gender: profile.gender,
            zipcode: profile.zipcode,
            political_party: profile.political_party,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpsertUserProfileRequest {
    pub consented: Option<bool>,
    pub ethnicity: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub political_party: Option<String>,
}
