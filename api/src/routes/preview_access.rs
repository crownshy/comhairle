use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower::util::option_layer;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::permissions::{PermissionRequirement, authorize as permission_middleware};
use crate::middleware::rate_limit::RateLimitLayers;
use crate::models::conversation::{self, IdOrSlug};
use crate::models::conversation_preview_password;
use crate::models::permissions::ConversationResource;
use crate::models::permissions::conversation::Action;
use crate::routes::auth::hash_pw;

const GRANT_AUDIENCE: &str = "preview-access";
const GRANT_DAYS: i64 = 7;
const MIN_PASSWORD_LENGTH: usize = 8;

/// One cookie per conversation, so unlocking a second preview does not log you out of the first.
pub fn grant_cookie_name(conversation_id: &Uuid) -> String {
    format!("preview-access-{conversation_id}")
}

/// `aud` keeps a grant from ever decoding as a session token, since both are signed with
/// the same secret. `password_set_at` ties the grant to the password it was issued for.
#[derive(Debug, Serialize, Deserialize)]
struct PreviewGrantClaims {
    aud: String,
    exp: usize,
    conversation_id: Uuid,
    password_set_at: i64,
}

fn encode_grant(
    secret: &str,
    conversation_id: Uuid,
    password_set_at: DateTime<Utc>,
) -> Result<String, ComhairleError> {
    let exp = (Utc::now() + chrono::Duration::days(GRANT_DAYS)).timestamp() as usize;
    let claims = PreviewGrantClaims {
        aud: GRANT_AUDIENCE.to_string(),
        exp,
        conversation_id,
        password_set_at: password_set_at.timestamp_micros(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| ComhairleError::AuthJWTError(e.to_string()))
}

fn decode_grant(secret: &str, token: &str) -> Option<PreviewGrantClaims> {
    let mut validation = Validation::default();
    validation.set_audience(&[GRANT_AUDIENCE]);
    decode::<PreviewGrantClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .ok()
    .map(|data| data.claims)
}

/// True when the request carries an unexpired grant for this conversation, issued against
/// its current password. Clearing or changing the password makes every older grant false.
#[instrument(err(Debug), skip(state, jar))]
pub async fn has_preview_grant(
    state: &Arc<ComhairleState>,
    jar: &CookieJar,
    conversation_id: Uuid,
) -> Result<bool, ComhairleError> {
    let Some(cookie) = jar.get(&grant_cookie_name(&conversation_id)) else {
        return Ok(false);
    };
    let Some(claims) = decode_grant(&state.config.jwt_secret, cookie.value()) else {
        return Ok(false);
    };
    if claims.conversation_id != conversation_id {
        return Ok(false);
    }
    let Some(password) = conversation_preview_password::get(&state.db, conversation_id).await?
    else {
        return Ok(false);
    };
    Ok(claims.password_set_at == password.updated_at.timestamp_micros())
}

/// The error for someone who can't read a draft conversation. When a preview password is set
/// they are told so, which is what makes the frontend show the password form.
pub async fn preview_denied_error(
    state: &Arc<ComhairleState>,
    conversation_id: Uuid,
    fallback: ComhairleError,
) -> Result<ComhairleError, ComhairleError> {
    let password = conversation_preview_password::get(&state.db, conversation_id).await?;
    Ok(if password.is_some() {
        ComhairleError::PreviewPasswordRequired
    } else {
        fallback
    })
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct PreviewPasswordRequest {
    pub password: String,
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviewPasswordStatus {
    pub enabled: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

#[instrument(err(Debug), skip(state, jar, payload))]
async fn unlock_preview(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_ident): Path<IdOrSlug>,
    jar: CookieJar,
    Json(payload): Json<PreviewPasswordRequest>,
) -> Result<(CookieJar, StatusCode), ComhairleError> {
    // Visitors only have the preview URL, which usually carries the slug rather than the id.
    let conversation_id = conversation::get_by_id_or_slug(&state.db, &conversation_ident)
        .await?
        .id;
    let Some(password) = conversation_preview_password::get(&state.db, conversation_id).await?
    else {
        return Err(ComhairleError::WrongPreviewPassword);
    };

    let hash =
        PasswordHash::new(&password.password_hash).map_err(|_| ComhairleError::PasswordHash)?;
    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &hash)
        .is_err()
    {
        return Err(ComhairleError::WrongPreviewPassword);
    }

    let token = encode_grant(
        &state.config.jwt_secret,
        conversation_id,
        password.updated_at,
    )?;
    let cookie = Cookie::build((grant_cookie_name(&conversation_id), token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .max_age(Duration::days(GRANT_DAYS))
        .build();

    Ok((jar.add(cookie), StatusCode::NO_CONTENT))
}

#[instrument(err(Debug), skip(state))]
async fn get_preview_password(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<PreviewPasswordStatus>), ComhairleError> {
    let password = conversation_preview_password::get(&state.db, conversation_id).await?;
    Ok((StatusCode::OK, Json(status_of(password))))
}

#[instrument(err(Debug), skip(state, payload))]
async fn set_preview_password(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<PreviewPasswordRequest>,
) -> Result<(StatusCode, Json<PreviewPasswordStatus>), ComhairleError> {
    if payload.password.chars().count() < MIN_PASSWORD_LENGTH {
        return Err(ComhairleError::BadRequest(format!(
            "Preview password must be at least {MIN_PASSWORD_LENGTH} characters"
        )));
    }
    let password = conversation_preview_password::upsert(
        &state.db,
        conversation_id,
        hash_pw(&payload.password)?,
    )
    .await?;
    Ok((StatusCode::OK, Json(status_of(Some(password)))))
}

#[instrument(err(Debug), skip(state))]
async fn clear_preview_password(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<PreviewPasswordStatus>), ComhairleError> {
    conversation_preview_password::delete(&state.db, conversation_id).await?;
    Ok((StatusCode::OK, Json(status_of(None))))
}

fn status_of(
    password: Option<conversation_preview_password::ConversationPreviewPassword>,
) -> PreviewPasswordStatus {
    PreviewPasswordStatus {
        enabled: password.is_some(),
        updated_at: password.map(|p| p.updated_at),
    }
}

pub fn router(rate_limiters: Option<&RateLimitLayers>) -> ApiRouter<Arc<ComhairleState>> {
    let requirement = PermissionRequirement::<ConversationResource>::new;
    let auth_limit = option_layer(rate_limiters.map(|limiters| limiters.auth_rate_limiter()));

    ApiRouter::new()
        .api_route(
            "/",
            post_with(unlock_preview, |op| {
                op.id("UnlockConversationPreview")
                    .tag("Conversation")
                    .summary("Unlock a draft conversation's preview with its password")
                    .description(
                        "Sets a cookie that lets this browser read the draft conversation \
                         for a week, or until the password is changed or cleared.",
                    )
                    .response::<204, ()>()
            })
            .layer(auth_limit),
        )
        .api_route(
            "/password",
            get_with(get_preview_password, |op| {
                op.id("GetConversationPreviewPassword")
                    .tag("Conversation")
                    .summary("Whether a preview password is set")
                    .response::<200, Json<PreviewPasswordStatus>>()
            })
            .route_layer(from_fn_with_state(
                requirement(Action::Update),
                permission_middleware::<ConversationResource>,
            )),
        )
        .api_route(
            "/password",
            put_with(set_preview_password, |op| {
                op.id("SetConversationPreviewPassword")
                    .tag("Conversation")
                    .summary("Set or change the preview password")
                    .description("Changing it signs out everyone who unlocked the old one.")
                    .response::<200, Json<PreviewPasswordStatus>>()
            })
            .route_layer(from_fn_with_state(
                requirement(Action::Update),
                permission_middleware::<ConversationResource>,
            )),
        )
        .api_route(
            "/password",
            delete_with(clear_preview_password, |op| {
                op.id("ClearConversationPreviewPassword")
                    .tag("Conversation")
                    .summary("Turn off password access to the preview")
                    .response::<200, Json<PreviewPasswordStatus>>()
            })
            .route_layer(from_fn_with_state(
                requirement(Action::Update),
                permission_middleware::<ConversationResource>,
            )),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::routes::conversations::dto::ConversationDto;
    use crate::test_helpers::{UserSession, test_state};
    use crate::{App, setup_server};

    const PASSWORD: &str = "shared-preview-pw";

    async fn draft_with_password(pool: PgPool) -> Result<(App, UserSession, Uuid), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut owner = UserSession::new_admin();
        owner.signup(&app).await?;
        let (_, value, _) = owner.create_random_unlaunched_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(value)?;
        let (status, _, _) = owner
            .put(
                &app,
                &format!("/conversation/{}/preview_access/password", conversation.id),
                json!({ "password": PASSWORD }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        Ok((app, owner, conversation.id))
    }

    async fn unlock(
        session: &mut UserSession,
        app: &App,
        conversation_id: Uuid,
        password: &str,
    ) -> Result<StatusCode, Box<dyn Error>> {
        let (status, _, _) = session
            .post(
                app,
                &format!("/conversation/{conversation_id}/preview_access"),
                json!({ "password": password }).to_string().into(),
            )
            .await?;
        Ok(status)
    }

    async fn read_conversation(
        session: &mut UserSession,
        app: &App,
        conversation_id: Uuid,
    ) -> Result<(StatusCode, serde_json::Value), Box<dyn Error>> {
        let (status, value, _) = session
            .get(app, &format!("/conversation/{conversation_id}"))
            .await?;
        Ok((status, value))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn guest_without_grant_is_asked_for_the_password(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, _owner, conversation_id) = draft_with_password(pool).await?;
        let mut guest = UserSession::new_guest();
        guest.signup_guest(&app).await?;

        let (status, value) = read_conversation(&mut guest, &app, conversation_id).await?;

        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(value["err"], "Preview password required");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn unlocks_by_slug_as_well_as_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut owner, conversation_id) = draft_with_password(pool).await?;
        owner
            .update_conversation(
                &app,
                &conversation_id.to_string(),
                json!({ "slug": "draft-for-review" }),
            )
            .await?;
        let mut visitor = UserSession::new_guest();

        let (status, _, _) = visitor
            .post(
                &app,
                "/conversation/draft-for-review/preview_access",
                json!({ "password": PASSWORD }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::NO_CONTENT);

        let (status, _) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::OK);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn wrong_password_does_not_unlock(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, _owner, conversation_id) = draft_with_password(pool).await?;
        let mut visitor = UserSession::new_guest();

        let status = unlock(&mut visitor, &app, conversation_id, "not-the-password").await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, _) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn right_password_lets_an_anonymous_visitor_read_the_draft(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, _owner, conversation_id) = draft_with_password(pool).await?;
        let mut visitor = UserSession::new_guest();

        assert_eq!(
            unlock(&mut visitor, &app, conversation_id, PASSWORD).await?,
            StatusCode::NO_CONTENT
        );
        let (status, _) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::OK, "unlocked before signing up");

        visitor.signup_guest(&app).await?;
        let (status, _) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::OK, "still unlocked as a guest account");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn changing_the_password_revokes_existing_grants(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut owner, conversation_id) = draft_with_password(pool).await?;
        let mut visitor = UserSession::new_guest();
        unlock(&mut visitor, &app, conversation_id, PASSWORD).await?;

        owner
            .put(
                &app,
                &format!("/conversation/{conversation_id}/preview_access/password"),
                json!({ "password": "a-brand-new-password" })
                    .to_string()
                    .into(),
            )
            .await?;

        let (status, _) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn clearing_the_password_revokes_access_and_hides_the_form(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut owner, conversation_id) = draft_with_password(pool).await?;
        let mut visitor = UserSession::new_guest();
        unlock(&mut visitor, &app, conversation_id, PASSWORD).await?;

        let (status, value, _) = owner
            .delete(
                &app,
                &format!("/conversation/{conversation_id}/preview_access/password"),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["enabled"], false);

        let (status, value) = read_conversation(&mut visitor, &app, conversation_id).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(value["err"], "No user logged in");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn a_grant_only_opens_its_own_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut owner, conversation_id) = draft_with_password(pool).await?;
        let (_, value, _) = owner.create_random_unlaunched_conversation(&app).await?;
        let other: ConversationDto = serde_json::from_value(value)?;

        let mut visitor = UserSession::new_guest();
        unlock(&mut visitor, &app, conversation_id, PASSWORD).await?;

        let (status, _) = read_conversation(&mut visitor, &app, other.id).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn only_the_team_can_manage_the_password(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, _owner, conversation_id) = draft_with_password(pool).await?;
        let mut guest = UserSession::new_guest();
        guest.signup_guest(&app).await?;

        let (status, _, _) = guest
            .put(
                &app,
                &format!("/conversation/{conversation_id}/preview_access/password"),
                json!({ "password": "guest-takes-over" }).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::FORBIDDEN);
        Ok(())
    }
}
