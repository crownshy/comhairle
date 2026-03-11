use std::sync::Arc;

use aide::axum::{
    routing::{get_with, put_with},
    ApiRouter,
};
use axum::{extract::State, http::StatusCode, Json};

use crate::{
    error::ComhairleError,
    models::{self, user_profile::CreateUserProfile},
    routes::{auth::RequiredUser, user_profile::dto::UserProfileDto},
    ComhairleState,
};

pub mod dto;

use dto::UpsertUserProfileRequest;

/// Get the current user's profile
pub async fn get_profile(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<UserProfileDto>), ComhairleError> {
    let profile = models::user_profile::get_by_user_id(&state.db, &user.id).await?;
    Ok((StatusCode::OK, Json(profile.into())))
}

/// Create or update the current user's profile
pub async fn upsert_profile(
    State(state): State<Arc<ComhairleState>>,
    RequiredUser(user): RequiredUser,
    Json(request): Json<UpsertUserProfileRequest>,
) -> Result<(StatusCode, Json<UserProfileDto>), ComhairleError> {
    // Try to get the existing profile
    let existing_profile = models::user_profile::get_by_user_id(&state.db, &user.id).await;

    let profile = match existing_profile {
        Ok(existing) => {
            // Profile exists, update it
            models::user_profile::update(
                &state.db,
                &existing.id,
                request.consented,
                request.ethnicity,
                request.age,
                request.gender,
                request.zipcode,
            )
            .await?
        }
        Err(ComhairleError::ResourceNotFound(_)) => {
            // Profile doesn't exist, create it
            let create_profile = CreateUserProfile {
                user_id: user.id,
                consented: request.consented.unwrap_or(false),
                ethnicity: request.ethnicity,
                age: request.age,
                gender: request.gender,
                zipcode: request.zipcode,
            };
            models::user_profile::create(&state.db, &create_profile).await?
        }
        Err(e) => {
            // Some other error occurred, propagate it
            return Err(e);
        }
    };

    Ok((StatusCode::OK, Json(profile.into())))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(get_profile, |op| {
                op.id("GetUserProfile")
                    .tag("User Profile")
                    .description("Get the current user's profile")
                    .security_requirement("JWT")
                    .response::<200, Json<UserProfileDto>>()
            }),
        )
        .api_route(
            "/",
            put_with(upsert_profile, |op| {
                op.id("UpsertUserProfile")
                    .tag("User Profile")
                    .description("Create or update the current user's profile")
                    .security_requirement("JWT")
                    .response::<200, Json<UserProfileDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        test_helpers::UserSession,
    };
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test]
    async fn should_create_user_profile_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let profile_data = json!({
            "consented": true,
            "ethnicity": "Asian",
            "age": 28,
            "gender": "Female",
            "zipcode": "12345"
        });

        let body = serde_json::to_vec(&profile_data)?;
        let (status, response, _) = session.put(&app, "/user/profile", body.into()).await?;

        assert!(status.is_success(), "error response status");

        let profile: UserProfileDto = serde_json::from_value(response)?;
        assert_eq!(profile.consented, true, "incorrect consented");
        assert_eq!(
            profile.ethnicity,
            Some("Asian".to_string()),
            "incorrect ethnicity"
        );
        assert_eq!(profile.age, Some(28), "incorrect age");
        assert_eq!(
            profile.gender,
            Some("Female".to_string()),
            "incorrect gender"
        );
        assert_eq!(
            profile.zipcode,
            Some("12345".to_string()),
            "incorrect zipcode"
        );
        assert_eq!(
            profile.user_id,
            session.id.unwrap(),
            "incorrect user_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_user_profile_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        // Create a profile first
        let profile_data = json!({
            "consented": true,
            "ethnicity": "Hispanic",
            "age": 30,
            "gender": "Male",
            "zipcode": "67890"
        });

        let body = serde_json::to_vec(&profile_data)?;
        let _ = session.put(&app, "/user/profile", body.into()).await?;

        // Now get the profile
        let (status, response, _) = session.get(&app, "/user/profile").await?;

        assert!(status.is_success(), "error response status");

        let profile: UserProfileDto = serde_json::from_value(response)?;
        assert_eq!(profile.consented, true, "incorrect consented");
        assert_eq!(
            profile.ethnicity,
            Some("Hispanic".to_string()),
            "incorrect ethnicity"
        );
        assert_eq!(profile.age, Some(30), "incorrect age");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_user_profile_via_api(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        // Create a profile first
        let profile_data = json!({
            "consented": false,
            "ethnicity": "White",
            "age": 25,
            "gender": "Male",
            "zipcode": "11111"
        });

        let body = serde_json::to_vec(&profile_data)?;
        let (_, response, _) = session.put(&app, "/user/profile", body.into()).await?;
        let original_profile: UserProfileDto = serde_json::from_value(response)?;

        // Update the profile
        let updated_data = json!({
            "consented": true,
            "ethnicity": "Black",
            "age": 35,
            "gender": "Non-binary",
            "zipcode": "54321"
        });

        let body = serde_json::to_vec(&updated_data)?;
        let (status, response, _) = session.put(&app, "/user/profile", body.into()).await?;

        assert!(status.is_success(), "error response status");

        let updated_profile: UserProfileDto = serde_json::from_value(response)?;
        assert_eq!(
            updated_profile.id, original_profile.id,
            "should be same profile"
        );
        assert_eq!(updated_profile.consented, true, "consented not updated");
        assert_eq!(
            updated_profile.ethnicity,
            Some("Black".to_string()),
            "ethnicity not updated"
        );
        assert_eq!(updated_profile.age, Some(35), "age not updated");
        assert_eq!(
            updated_profile.gender,
            Some("Non-binary".to_string()),
            "gender not updated"
        );
        assert_eq!(
            updated_profile.zipcode,
            Some("54321".to_string()),
            "zipcode not updated"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_not_allow_user_to_access_another_users_profile(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session1) = setup_default_app_and_session(&pool).await?;

        // Create a profile for user 1
        let profile_data = json!({
            "consented": true,
            "ethnicity": "Asian",
            "age": 28,
            "gender": "Female",
            "zipcode": "12345"
        });

        let body = serde_json::to_vec(&profile_data)?;
        let _ = session1.put(&app, "/user/profile", body.into()).await?;

        // Create a second user
        let mut session2 = UserSession::new("user2", "password2", "user2@test.com");
        session2.signup(&app).await?;

        // User 2 tries to get their own profile (should fail because they don't have one yet)
        let (status, response, _) = session2.get(&app, "/user/profile").await?;

        assert!(!status.is_success(), "should fail to get non-existent profile");
        assert!(
            response.get("err").is_some(),
            "should return error when profile not found"
        );

        // Create profile for user 2
        let profile_data2 = json!({
            "consented": true,
            "ethnicity": "Hispanic",
            "age": 30,
            "gender": "Male",
            "zipcode": "67890"
        });

        let body = serde_json::to_vec(&profile_data2)?;
        let (_, response, _) = session2.put(&app, "/user/profile", body.into()).await?;
        let profile2: UserProfileDto = serde_json::from_value(response)?;

        // Verify user 2's profile has different data than user 1's
        assert_eq!(
            profile2.ethnicity,
            Some("Hispanic".to_string()),
            "user 2 should have their own profile data"
        );
        assert_eq!(profile2.age, Some(30), "user 2 should have their own age");

        // Get user 1's profile again to ensure it's unchanged
        let (_, response, _) = session1.get(&app, "/user/profile").await?;
        let profile1: UserProfileDto = serde_json::from_value(response)?;
        assert_eq!(
            profile1.ethnicity,
            Some("Asian".to_string()),
            "user 1 profile should be unchanged"
        );
        assert_eq!(
            profile1.user_id,
            session1.id.unwrap(),
            "user 1 profile should belong to user 1"
        );
        assert_eq!(
            profile2.user_id,
            session2.id.unwrap(),
            "user 2 profile should belong to user 2"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_require_authentication(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, _) = setup_default_app_and_session(&pool).await?;

        // Create an unauthenticated session
        let mut unauth_session = UserSession::new_anon();

        // Try to get profile without authentication
        let (status, _, _) = unauth_session.get(&app, "/user/profile").await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "should return 401 for unauthenticated request"
        );

        // Try to create/update profile without authentication
        let profile_data = json!({
            "consented": true,
            "ethnicity": "Asian",
            "age": 28,
            "gender": "Female",
            "zipcode": "12345"
        });

        let body = serde_json::to_vec(&profile_data)?;
        let (status, _, _) = unauth_session
            .put(&app, "/user/profile", body.into())
            .await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "should return 401 for unauthenticated request"
        );

        Ok(())
    }
}
