use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::axum::routing::{delete_with, get_with, patch_with, post_with, put_with};
use axum::Extension;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum_extra::extract::cookie::CookieJar;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tower::util::option_layer;
use tracing::instrument;
use uuid::Uuid;

use crate::ComhairleState;
use crate::error::ComhairleError;
use crate::middleware::rate_limit::RateLimitLayers;
use crate::middleware::request_logging::{ClientIp, ClientUserAgent};
use crate::models::event::{
    self, CreateEvent, EventFilterOptions, EventOrderOptions, EventWithTranslations, PartialEvent,
};
use crate::models::pagination::{PageOptions, PaginatedResults};
use crate::models::{event_attendance, users};
use crate::routes::auth::{
    RequiredAdminUser, RequiredUser, create_session_cookie, decode_jwt, generate_jwt,
    is_user_admin, issue_refresh_token,
};
use crate::routes::events::dto::{EventDto, LocalizedEventDto};
use crate::routes::translations::LocaleExtractor;
use crate::routes::user::dto::UserDto;

pub mod breakout;
pub mod dto;

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<EventOrderOptions>,
    Query(filter_options): Query<EventFilterOptions>,
    Query(page_options): Query<PageOptions>,
    LocaleExtractor(locale): LocaleExtractor,
    RequiredUser(_user): RequiredUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<(StatusCode, Json<PaginatedResults<LocalizedEventDto>>), ComhairleError> {
    let events = event::list(
        &state.db,
        &conversation_id,
        page_options,
        filter_options,
        order_options,
        Some(locale),
    )
    .await?
    .into();

    Ok((StatusCode::OK, Json(events)))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct GetEventQuery {
    #[serde(rename = "withTranslations", default)]
    with_translations: bool,
}

#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
enum EventResponse {
    Localized(LocalizedEventDto),
    WithTranslations(EventWithTranslations),
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<GetEventQuery>,
    RequiredUser(user): RequiredUser,
    LocaleExtractor(locale): LocaleExtractor,
) -> Result<(StatusCode, Json<EventResponse>), ComhairleError> {
    let event = event::get_by_id(&state.db, &event_id).await?;

    let should_return_with_translations =
        query.with_translations && is_user_admin(&state, &user).await;

    if should_return_with_translations {
        let event_with_translations =
            EventWithTranslations::from_original(&state.db, event, &locale).await?;

        Ok((
            StatusCode::OK,
            Json(EventResponse::WithTranslations(event_with_translations)),
        ))
    } else {
        let event = event::get_localized_by_id(&state.db, &event_id, &locale)
            .await?
            .into();

        Ok((StatusCode::OK, Json(EventResponse::Localized(event))))
    }
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateEvent>,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let event = event::create(&state.db, &conversation_id, &payload)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(event)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(event): Json<PartialEvent>,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let agenda_updated = event.agenda.is_some();
    let event: EventDto = event::update(&state.db, &event_id, &event).await?.into();

    // If the agenda changed, push it to anyone currently on the call for this event.
    if agenda_updated && let Ok(agenda) = serde_json::to_value(&event.agenda) {
        let _ = state
            .video_call_handler
            .broadcast_agenda_update(&event_id, agenda, &state)
            .await;
    }

    Ok((StatusCode::OK, Json(event)))
}

///  Get an event's `metadata` jsonb column.
#[instrument(err(Debug), skip(state))]
async fn get_event_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Option<serde_json::Value>>), ComhairleError> {
    let metadata = event::get_metadata(&state.db, &event_id).await?;
    Ok((StatusCode::OK, Json(metadata)))
}

/// Shallow-merge the request body into the event's `metadata` jsonb column.
#[instrument(err(Debug), skip(state))]
async fn patch_event_metadata(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(patch): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let event = event::patch_metadata(&state.db, &event_id, patch).await?;
    let event: EventDto = event.into();

    Ok((StatusCode::OK, Json(event)))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let event = event::delete(&state.db, &event_id).await?.into();

    Ok((StatusCode::OK, Json(event)))
}

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
struct JwtResponse {
    jwt: String,
    is_moderator: bool,
}

#[derive(Serialize, Debug)]
struct VideoEventJwtClaims<'a> {
    iss: &'a str,
    aud: &'a str,
    room: &'a str,
    context: VideoEventJwtContext<'a>,
}

#[derive(Serialize, Debug)]
struct VideoEventJwtContext<'a> {
    user: VideoEventJwtUser<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VideoEventJwtUser<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    id: &'a str,
    moderator: bool,
}

#[instrument(err(Debug), skip(state))]
async fn get_jwt(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<JwtResponse>), ComhairleError> {
    let attendance =
        event_attendance::get_by_event_and_user(&state.db, &event_id, &user.id).await?;

    let event = event::get_by_id(&state.db, &event_id).await?;
    let _video_meeting_id = event
        .video_meeting_id
        .ok_or(ComhairleError::NoVideoMeetingId)?;
    let video_call_config = &state
        .config
        .video_call_service
        .as_ref()
        .ok_or(ComhairleError::NoVideoServiceConfigured)?;

    let is_moderator = attendance.role == "facilitator" || attendance.role == "moderator";

    let claims = VideoEventJwtClaims {
        iss: &video_call_config.jwt_app_id,
        aud: &video_call_config.jwt_app_id,
        room: "*",
        context: VideoEventJwtContext {
            user: VideoEventJwtUser {
                name: user.username.as_deref(),
                id: &user.id.to_string(),
                moderator: is_moderator,
            },
        },
    };

    let jwt = generate_jwt()
        .user(&user)
        .secret(&video_call_config.jwt_app_secret)
        .custom_claims(claims)
        .duration(chrono::Duration::hours(1))
        .sub(video_call_config.jwt_sub.to_owned())
        .call()?;

    Ok((StatusCode::OK, Json(JwtResponse { jwt, is_moderator })))
}

/// Claims in the access token on event email links. The token only grants
/// access to this event, for this user, until it expires.
#[derive(Serialize, Deserialize, Debug)]
pub struct EventAccessClaims {
    pub user_id: Uuid,
    pub event_id: Uuid,
}

#[derive(Deserialize, Debug, JsonSchema)]
struct EventAccessRequest {
    /// Access token taken from an event email link.
    token: String,
}

/// Logs a registered attendee in from the link in an event email.
///
/// Confirmation and reminder emails each carry a signed token for one user and
/// one event. Unlike single-use OTP links, these tokens can be reused until they
/// expire, so a link in any email the attendee received still gets them in.
/// The user must still be registered for the event when the link is used.
#[instrument(err(Debug), skip(state))]
async fn access(
    State(state): State<Arc<ComhairleState>>,
    Extension(client_ip): Extension<ClientIp>,
    Extension(user_agent): Extension<ClientUserAgent>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    jar: CookieJar,
    Json(payload): Json<EventAccessRequest>,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    let invalid = || ComhairleError::InvalidEventAccessToken;
    let not_found_as_invalid = |e| match e {
        ComhairleError::ResourceNotFound(_) | ComhairleError::NoUserFoundForId(_) => invalid(),
        other => other,
    };

    let token_data = decode_jwt::<EventAccessClaims>(&payload.token, &state.config.jwt_secret)
        .map_err(|_| invalid())?;
    let EventAccessClaims {
        user_id,
        event_id: token_event_id,
    } = token_data.claims.details;

    if event_id != token_event_id {
        return Err(invalid());
    }

    let event = event::get_by_id(&state.db, &event_id)
        .await
        .map_err(not_found_as_invalid)?;

    if !event.email_link_login {
        return Err(invalid());
    }

    event_attendance::get_by_event_and_user(&state.db, &event_id, &user_id)
        .await
        .map_err(not_found_as_invalid)?;

    let user = users::get_user_by_id(&user_id, &state.db).await?;

    let session_cookie = create_session_cookie(&user, &state)?;
    let refresh_cookie = issue_refresh_token(&state, &user, &client_ip, &user_agent).await;

    let mut jar = jar.add(session_cookie);
    if let Some(refresh_cookie) = refresh_cookie {
        jar = jar.add(refresh_cookie);
    }

    Ok((jar, (StatusCode::OK, Json(user.into()))))
}

pub fn router(rate_limiters: Option<&RateLimitLayers>) -> ApiRouter<Arc<ComhairleState>> {
    let auth_limit = option_layer(rate_limiters.map(|limiters| limiters.auth_rate_limiter()));

    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListEvents")
                    .tag("Events")
                    .summary("List of events for a conversation")
                    .description(
                        "Paginated list of events for a conversation with optional \
                    filtering and ordering",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<LocalizedEventDto>>>()
            }),
        )
        .api_route(
            "/{event_id}",
            get_with(get, |op| {
                op.id("GetEvent")
                    .tag("Events")
                    .summary("Get an event by id")
                    .description("Event an event by id")
                    .security_requirement("JWT")
                    .response::<200, Json<EventResponse>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateEvent")
                    .tag("Events")
                    .summary("Create a new event")
                    .description("Create a new event")
                    .security_requirement("JWT")
                    .response::<201, Json<EventDto>>()
            }),
        )
        .api_route(
            "/{event_id}",
            put_with(update, |op| {
                op.id("UpdateEvent")
                    .tag("Events")
                    .summary("Update an event")
                    .description("Update an event")
                    .security_requirement("JWT")
                    .response::<200, Json<EventDto>>()
            }),
        )
        .api_route(
            "/{event_id}/metadata",
            get_with(get_event_metadata, |op| {
                op.id("GetEventMetadata")
                    .tag("Events")
                    .summary("Get event metadata")
                    .description("Get event metadata")
                    .security_requirement("JWT")
                    .response::<200, Json<Option<serde_json::Value>>>()
            }),
        )
        .api_route(
            "/{event_id}/metadata",
            patch_with(patch_event_metadata, |op| {
                op.id("PatchEventMetadata")
                    .tag("Events")
                    .summary("Shallow-merge event metadata")
                    .description(
                        "Merge a JSON object into event.metadata at the top level \
                            using jsonb concatenation",
                    )
                    .security_requirement("JWT")
                    .response::<200, Json<EventDto>>()
            }),
        )
        .api_route(
            "/{event_id}",
            delete_with(delete, |op| {
                op.id("DeleteEvent")
                    .tag("Events")
                    .summary("Delete an event")
                    .description("Delete an event")
                    .security_requirement("JWT")
                    .response::<200, Json<EventDto>>()
            }),
        )
        .api_route(
            "/{event_id}/auth",
            get_with(get_jwt, |op| {
                op.id("GetEventJWT")
                    .tag("Events")
                    .summary("Get a auth JWT for an event")
                    .description("Get a auth JWT for an event")
                    .security_requirement("JWT")
                    .response::<200, Json<JwtResponse>>()
            }),
        )
        .api_route(
            "/{event_id}/access",
            post_with(access, |op| {
                op.id("AccessEvent")
                    .tag("Events")
                    .summary("Log a registered attendee in from an event email link")
                    .description(
                        "Exchanges the access token from an event email link for a session. \
                            The token is reusable until it expires and only works while the user \
                            is registered for the event",
                    )
                    .response::<200, Json<UserDto>>()
            })
            .layer(auth_limit),
        )
        .api_route(
            "/{event_id}/breakout",
            get_with(breakout::get_plan, |op| {
                op.id("GetEventBreakoutPlan")
                    .tag("Events")
                    .summary("Get the pre-assigned breakout plan for an event")
                    .security_requirement("JWT")
                    .response::<200, Json<breakout::BreakoutPlanDto>>()
            }),
        )
        .api_route(
            "/{event_id}/breakout",
            put_with(breakout::save_plan, |op| {
                op.id("SaveEventBreakoutPlan")
                    .tag("Events")
                    .summary("Save an edited pre-assigned breakout plan")
                    .security_requirement("JWT")
                    .response::<200, Json<breakout::BreakoutPlanDto>>()
            }),
        )
        .api_route(
            "/{event_id}/breakout/seed",
            post_with(breakout::seed_plan, |op| {
                op.id("SeedEventBreakoutPlan")
                    .tag("Events")
                    .summary("Randomly seed the breakout plan from attendees and invites")
                    .security_requirement("JWT")
                    .response::<200, Json<breakout::BreakoutPlanDto>>()
            }),
        )
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use axum::http::HeaderValue;
    use chrono::Utc;
    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::event::SignupMode;
    use crate::models::event_attendance::CreateEventAttendance;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_app_and_session_with_state, setup_default_app_and_session,
    };
    use crate::models::users::{self, User};
    use crate::routes::auth::OtpSignupRequest;
    use crate::test_helpers::{UserSession, test_state};

    struct RegisteredAttendee {
        app: crate::App,
        state: Arc<ComhairleState>,
        admin: UserSession,
        conversation_id: Uuid,
        event_id: Uuid,
        user: User,
    }

    async fn setup_registered_attendee(
        pool: &PgPool,
    ) -> Result<RegisteredAttendee, Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool.clone()).call()?);
        let (app, mut admin) = setup_app_and_session_with_state(state.clone()).await?;
        let conversation_id = get_random_conversation_id(&app, &mut admin).await?;
        let (_, response, _) = admin
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let user = register_otp_user(pool, "attendee@test.com", event.id).await?;

        Ok(RegisteredAttendee {
            app,
            state,
            admin,
            conversation_id,
            event_id: event.id,
            user,
        })
    }

    async fn register_otp_user(
        pool: &PgPool,
        email: &str,
        event_id: Uuid,
    ) -> Result<User, Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: email.to_string(),
                username: None,
            },
            pool,
        )
        .await?;

        event_attendance::create(
            pool,
            &CreateEventAttendance {
                user_id: user.id,
                event_id,
                role: "participant".to_string(),
            },
        )
        .await?;

        Ok(user)
    }

    fn access_token(
        secret: &str,
        user: &User,
        event_id: Uuid,
        duration: chrono::Duration,
    ) -> Result<String, ComhairleError> {
        generate_jwt()
            .user(user)
            .secret(secret)
            .custom_claims(EventAccessClaims {
                user_id: user.id,
                event_id,
            })
            .duration(duration)
            .call()
    }

    async fn redeem_access_token(
        app: &crate::App,
        conversation_id: Uuid,
        event_id: Uuid,
        token: &str,
    ) -> Result<(StatusCode, serde_json::Value, Vec<HeaderValue>), Box<dyn Error>> {
        UserSession::new_guest()
            .post(
                app,
                &format!("/conversation/{conversation_id}/events/{event_id}/access"),
                json!({ "token": token }).to_string().into(),
            )
            .await
    }

    fn sets_session_cookie(cookies: &[HeaderValue]) -> bool {
        cookies
            .iter()
            .any(|cookie| cookie.to_str().unwrap_or_default().contains("auth-token"))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEvent {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: SignupMode::Invite,
            agenda: None,
            ..Default::default()
        };

        let body = serde_json::to_vec(&new_event)?;
        let (status, response, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/events"),
                body.into(),
            )
            .await?;

        let event: EventDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            event.signup_mode,
            SignupMode::Invite,
            "incorrect signup_mode"
        );
        assert_eq!(event.capacity, Some(10), "incorrect signup_mode");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_an_event_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let (status, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events/{}", event.id),
            )
            .await?;
        let event: LocalizedEventDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(event.name, "test_event".to_string(), "incorrect event name");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_events(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let _ = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let _ = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let _ = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;

        let (status, response, _) = session
            .get(&app, &format!("/conversation/{conversation_id}/events"))
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(events.total, 3, "incorrect number of events");
        assert_eq!(
            events.records[0].name,
            "test_event".to_string(),
            "incorrect event json"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_ordered_list_of_events(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let _ = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "bar",
                "description": "1",
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;
        let _ = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "foo",
                "description": "2",
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;
        let _ = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "baz",
                "description": "3",
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events?created_at=desc"),
            )
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;
        assert_eq!(
            events.records[0].name,
            "baz".to_string(),
            "incorrect first event [created_at=desc]"
        );
        assert_eq!(
            events.records[2].name,
            "bar".to_string(),
            "incorrect last event [created_at=desc]"
        );

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events?name=asc"),
            )
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;
        assert_eq!(
            events.records[0].name,
            "bar".to_string(),
            "incorrect first event [name=asc]"
        );
        assert_eq!(
            events.records[2].name,
            "foo".to_string(),
            "incorrect last event [name=asc]"
        );

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events?name=desc"),
            )
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;
        assert_eq!(
            events.records[0].name,
            "foo".to_string(),
            "incorrect first event [name=desc]"
        );
        assert_eq!(
            events.records[2].name,
            "bar".to_string(),
            "incorrect last event [name=desc]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_filtered_list_of_events(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, event_1, _) = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "full_a",
                "description": "1",
                "capacity": 1,
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;
        let (_, event_2, _) = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "full_b",
                "capacity": 1,
                "description": "2",
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;
        let _ = session
            .create_event(
                &app,
                &conversation_id.to_string(),
                json!({
                "name": "available_a",
                "description": "3",
                "start_time": Utc::now(),
                "end_time": Utc::now(),
                "signup_mode": "invite"
                }),
            )
            .await?;

        let _ = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                event_1.get("id").and_then(|v| v.as_str()).unwrap(),
            )
            .await?;
        let _ = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                event_2.get("id").and_then(|v| v.as_str()).unwrap(),
            )
            .await?;

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events?capacity_status=full"),
            )
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;
        assert_eq!(
            events.total, 2,
            "incorrect number of events [capacity_state=full]"
        );
        assert_eq!(
            events.records[0].name,
            "full_a".to_string(),
            "incorrect first event [capacity_status=full]"
        );
        assert_eq!(
            events.records[1].name,
            "full_b".to_string(),
            "incorrect last event [capacity_status=full]"
        );

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events?capacity_status=available"),
            )
            .await?;
        let events: PaginatedResults<LocalizedEventDto> = serde_json::from_value(response)?;
        assert_eq!(
            events.total, 1,
            "incorrect number of events [capacity_state=available]"
        );
        assert_eq!(
            events.records[0].name,
            "available_a".to_string(),
            "incorrect first event [capacity_status=available]"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let update = PartialEvent {
            capacity: Some(1000),
            ..Default::default()
        };
        let body = serde_json::to_vec(&update)?;
        let (status, response, _) = session
            .put(
                &app,
                &format!("/conversation/{conversation_id}/events/{}", event.id),
                body.into(),
            )
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(event.capacity, Some(1000), "incorrect capacity");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let _ = session
            .delete(
                &app,
                &format!("/conversation/{conversation_id}/events/{}", event.id),
            )
            .await?;

        let (_, response, _) = session
            .get(
                &app,
                &format!("/conversation/{conversation_id}/events/{}", event.id),
            )
            .await?;

        assert_eq!(
            response.get("err").and_then(|v| v.as_str()).unwrap(),
            "Event not found",
            "incorrect error message"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_token_logs_registered_attendee_in(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let fixture = setup_registered_attendee(&pool).await?;
        let token = access_token(
            &fixture.state.config.jwt_secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;

        let (status, body, cookies) = redeem_access_token(
            &fixture.app,
            fixture.conversation_id,
            fixture.event_id,
            &token,
        )
        .await?;

        assert_eq!(status, StatusCode::OK, "body: {body}");
        assert!(sets_session_cookie(&cookies), "missing auth-token cookie");
        let user: UserDto = serde_json::from_value(body)?;
        assert_eq!(user.id, fixture.user.id, "logged in as the wrong user");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_token_can_be_reused(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let fixture = setup_registered_attendee(&pool).await?;
        let token = access_token(
            &fixture.state.config.jwt_secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;

        for attempt in 1..=2 {
            let (status, body, cookies) = redeem_access_token(
                &fixture.app,
                fixture.conversation_id,
                fixture.event_id,
                &token,
            )
            .await?;
            assert_eq!(status, StatusCode::OK, "attempt {attempt}: {body}");
            assert!(
                sets_session_cookie(&cookies),
                "attempt {attempt}: missing cookie"
            );
        }

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_tokens_from_separate_emails_all_work(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let fixture = setup_registered_attendee(&pool).await?;
        let secret = &fixture.state.config.jwt_secret;
        let confirmation = access_token(
            secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;
        let reminder = access_token(
            secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(2),
        )?;

        for token in [&reminder, &confirmation] {
            let (status, body, _) = redeem_access_token(
                &fixture.app,
                fixture.conversation_id,
                fixture.event_id,
                token,
            )
            .await?;
            assert_eq!(status, StatusCode::OK, "body: {body}");
        }

        Ok(())
    }

    fn assert_rejected(status: StatusCode, body: &serde_json::Value, cookies: &[HeaderValue]) {
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        assert_eq!(
            body["err"],
            ComhairleError::InvalidEventAccessToken.to_string(),
            "rejections should share one error message"
        );
        assert!(
            !sets_session_cookie(cookies),
            "rejection must not set a session"
        );
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_token_for_another_event_is_rejected(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut fixture = setup_registered_attendee(&pool).await?;
        let (_, response, _) = fixture
            .admin
            .create_random_event(&fixture.app, &fixture.conversation_id.to_string())
            .await?;
        let other_event: EventDto = serde_json::from_value(response)?;
        let token = access_token(
            &fixture.state.config.jwt_secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;

        let (status, body, cookies) = redeem_access_token(
            &fixture.app,
            fixture.conversation_id,
            other_event.id,
            &token,
        )
        .await?;

        assert_rejected(status, &body, &cookies);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_token_for_unregistered_user_is_rejected(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let fixture = setup_registered_attendee(&pool).await?;
        let stranger = users::create_otp_user(
            &OtpSignupRequest {
                email: "stranger@test.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;
        let token = access_token(
            &fixture.state.config.jwt_secret,
            &stranger,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;

        let (status, body, cookies) = redeem_access_token(
            &fixture.app,
            fixture.conversation_id,
            fixture.event_id,
            &token,
        )
        .await?;

        assert_rejected(status, &body, &cookies);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn access_token_is_rejected_when_email_link_login_disabled(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let fixture = setup_registered_attendee(&pool).await?;
        event::update(
            &pool,
            &fixture.event_id,
            &PartialEvent {
                email_link_login: Some(false),
                ..Default::default()
            },
        )
        .await?;
        let token = access_token(
            &fixture.state.config.jwt_secret,
            &fixture.user,
            fixture.event_id,
            chrono::Duration::hours(1),
        )?;

        let (status, body, cookies) = redeem_access_token(
            &fixture.app,
            fixture.conversation_id,
            fixture.event_id,
            &token,
        )
        .await?;

        assert_rejected(status, &body, &cookies);
        Ok(())
    }
}
