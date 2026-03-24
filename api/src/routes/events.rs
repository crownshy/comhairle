use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        event::{
            self, CreateEvent, EventFilterOptions, EventOrderOptions, EventWithTranslations,
            PartialEvent,
        },
        pagination::{PageOptions, PaginatedResults},
    },
    routes::{
        auth::{generate_jwt, is_user_admin, RequiredAdminUser, RequiredUser},
        events::dto::{EventDto, LocalizedEventDto},
        translations::LocaleExtractor,
    },
    ComhairleState,
};

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
        query.with_translations && is_user_admin(&user, &state.config);

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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateEventRequest {
    name: String,
    description: String,
    capacity: Option<i32>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    signup_mode: String,
}

#[instrument(err(Debug), skip(state))]
async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path(conversation_id): Path<Uuid>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let event = CreateEvent {
        name: payload.name,
        description: payload.description,
        capacity: payload.capacity,
        start_time: payload.start_time,
        end_time: payload.end_time,
        signup_mode: payload.signup_mode,
        conversation_id,
    };
    let event = event::create(&state.db, &event).await?.into();

    Ok((StatusCode::CREATED, Json(event)))
}

#[instrument(err(Debug), skip(state))]
async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(event): Json<PartialEvent>,
) -> Result<(StatusCode, Json<EventDto>), ComhairleError> {
    let event = event::update(&state.db, &event_id, &event).await?.into();

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
struct JwtResponse {
    jwt: String,
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
}

#[instrument(err(Debug), skip(state))]
async fn get_jwt(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<JwtResponse>), ComhairleError> {
    let event = event::get_by_id(&state.db, &event_id).await?;
    let video_meeting_id = event
        .video_meeting_id
        .ok_or(ComhairleError::NoVideoMeetingId)?;
    let video_call_config = &state
        .config
        .video_call_service
        .as_ref()
        .ok_or(ComhairleError::NoVideoServiceConfigured)?;

    let claims = VideoEventJwtClaims {
        iss: &video_call_config.jwt_app_id,
        aud: &video_call_config.jwt_app_id,
        room: &video_meeting_id.to_string(),
        context: VideoEventJwtContext {
            user: VideoEventJwtUser {
                name: user.username.as_deref(),
                id: &user.id.to_string(),
            },
        },
    };

    let jwt = generate_jwt()
        .user(&user)
        .secret(&video_call_config.jwt_app_secret)
        .custom_claims(claims)
        .duration(chrono::Duration::hours(1))
        .sub(video_call_config.jwt_sub.to_owned())
        .call();

    Ok((StatusCode::OK, Json(JwtResponse { jwt })))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(list, |op| {
            op.id("ListEvents")
                .tag("Events")
                .summary("List of events for a conversation")
                .description("Paginated list of events for a conversation with optional filtering and ordering")
                .security_requirement("JWT")
                .response::<200, Json<PaginatedResults<LocalizedEventDto>>>()
        }))
        .api_route("/{event_id}", 
            get_with(get, |op| {
                op.id("GetEvent")
                    .tag("Events")
                    .summary("Get an event by id")
                    .description("Event an event by id")
                    .security_requirement("JWT")
                    .response::<200, Json<EventResponse>>()

        }))
        .api_route("/", 
            post_with(create, |op| {
                op.id("CreateEvent")
                    .tag("Events")
                    .summary("Create a new event")
                    .description("Create a new event")
                    .security_requirement("JWT")
                    .response::<201, Json<EventDto>>()

        }))
        .api_route("/{event_id}", 
            put_with(update, |op| {
                op.id("UpdateEvent")
                    .tag("Events")
                    .summary("Update an event")
                    .description("Update an event")
                    .security_requirement("JWT")
                    .response::<200, Json<EventDto>>()

        }))
        .api_route("/{event_id}", 
            delete_with(delete, |op| {
                op.id("DeleteEvent")
                    .tag("Events")
                    .summary("Delete an event")
                    .description("Delete an event")
                    .security_requirement("JWT")
                    .response::<200, Json<EventDto>>()

        }))
        .api_route("/{event_id}/auth", 
            get_with(get_jwt, |op| {
                op.id("GetEventJWT")
                    .tag("Events")
                    .summary("Get a auth JWT for an event")
                    .description("Get a auth JWT for an event")
                    .security_requirement("JWT")
                    .response::<200, Json<JwtResponse>>()

        }))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    use super::*;

    #[sqlx::test]
    async fn should_create_an_event(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let new_event = CreateEventRequest {
            name: "test_event".to_string(),
            description: "test_desc".to_string(),
            capacity: Some(10),
            start_time: Utc::now(),
            end_time: Utc::now(),
            signup_mode: "invite".to_string(),
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
            "invite".to_string(),
            "incorrect signup_mode"
        );
        assert_eq!(event.capacity, Some(10), "incorrect signup_mode");

        Ok(())
    }

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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

    #[sqlx::test]
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
}
