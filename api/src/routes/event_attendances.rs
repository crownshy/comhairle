use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with, put_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::{
        event_attendance::{
            self, CreateEventAttendance, EventAttendanceEtx, EventAttendanceFilterOptions,
            EventAttendanceOrderOptions, UpdateEventAttendance,
        },
        pagination::{PageOptions, PaginatedResults},
        users,
    },
    routes::{
        auth::{RequiredAdminUser, RequiredUser},
        event_attendances::dto::EventAttendanceDto,
    },
    ComhairleState,
};

pub mod dto;

#[instrument(err(Debug), skip(state))]
pub async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<EventAttendanceOrderOptions>,
    Query(filter_options): Query<EventAttendanceFilterOptions>,
    Query(page_options): Query<PageOptions>,
    RequiredUser(_user): RequiredUser,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<PaginatedResults<EventAttendanceEtx>>), ComhairleError> {
    let event_attendances = event_attendance::list(
        &state.db,
        event_id,
        page_options,
        filter_options,
        order_options,
    )
    .await?;

    Ok((StatusCode::OK, Json(event_attendances)))
}

#[instrument(err(Debug), skip(state))]
pub async fn get(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id, event_attendance_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredUser(_user): RequiredUser,
) -> Result<(StatusCode, Json<EventAttendanceDto>), ComhairleError> {
    let event_attendance = event_attendance::get_by_id(&state.db, &event_attendance_id)
        .await?
        .into();

    Ok((StatusCode::OK, Json(event_attendance)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateEventAttendanceRequest {
    role: String,
}

#[instrument(err(Debug), skip(state))]
pub async fn create(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredUser(user): RequiredUser,
    Json(payload): Json<CreateEventAttendanceRequest>,
) -> Result<(StatusCode, Json<EventAttendanceDto>), ComhairleError> {
    let create_event_attendance = CreateEventAttendance {
        user_id: user.id,
        event_id,
        role: payload.role,
    };

    let event_attendance = event_attendance::create(&state.db, &create_event_attendance)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(event_attendance)))
}

#[derive(Deserialize, JsonSchema, Debug)]
struct CreateFacilitatorRequest {
    email: String,
}

#[instrument(err(Debug), skip(state))]
async fn create_facilitator(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(payload): Json<CreateFacilitatorRequest>,
) -> Result<(StatusCode, Json<EventAttendanceDto>), ComhairleError> {
    let user = users::get_user_by_email(&payload.email, &state.db).await?;

    let create_event_attendance = CreateEventAttendance {
        user_id: user.id,
        event_id,
        role: "facilitator".to_string(),
    };

    let event_attendance = event_attendance::create(&state.db, &create_event_attendance)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(event_attendance)))
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UpdateEventAttendanceRequest {
    role: Option<String>,
}

#[instrument(err(Debug), skip(state))]
pub async fn update(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id, event_attendance_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
    Json(payload): Json<UpdateEventAttendanceRequest>,
) -> Result<(StatusCode, Json<EventAttendanceDto>), ComhairleError> {
    let update_event_attendance = UpdateEventAttendance { role: payload.role };

    let event_attendance =
        event_attendance::update(&state.db, &event_attendance_id, &update_event_attendance)
            .await?
            .into();

    Ok((StatusCode::OK, Json(event_attendance)))
}

#[instrument(err(Debug), skip(state))]
pub async fn delete(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id, event_attendance_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<EventAttendanceDto>), ComhairleError> {
    let event_attendance = event_attendance::delete(&state.db, &event_attendance_id)
        .await?
        .into();

    Ok((StatusCode::OK, Json(event_attendance)))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListEventAttendances")
                    .summary("List attendances for an event")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description(
                        "List attendances for a conversation event with optional filtering
                        and ordering",
                    )
                    .response::<200, Json<PaginatedResults<EventAttendanceEtx>>>()
            }),
        )
        .api_route(
            "/{attendance_id}",
            get_with(get, |op| {
                op.id("GetEventAttendance")
                    .summary("Get an event attendance by id")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description("Get and event attendance by id")
                    .response::<200, Json<EventAttendanceDto>>()
            }),
        )
        .api_route(
            "/",
            post_with(create, |op| {
                op.id("CreateEventAttendance")
                    .summary("Create a new event attendance")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description("Create a new attendance for a conversation event")
                    .response::<201, Json<EventAttendanceDto>>()
            }),
        )
        .api_route(
            "/facilitator",
            post_with(create_facilitator, |op| {
                op.id("CreateFacilitatorEventAttendance")
                    .summary("Create a new event attendance with facilitator role")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description(
                        "Create a new attendance for a conversation event with facilitator role",
                    )
                    .response::<201, Json<EventAttendanceDto>>()
            }),
        )
        .api_route(
            "/{attendance_id}",
            put_with(update, |op| {
                op.id("UpdateEventAttendance")
                    .summary("Update an event attendance")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description("Update an event attendance by id")
                    .response::<201, Json<EventAttendanceDto>>()
            }),
        )
        .api_route(
            "/{attendance_id}",
            delete_with(delete, |op| {
                op.id("DeleteEventAttendance")
                    .summary("Delete an event attendance")
                    .tag("Event Attendances")
                    .security_requirement("JWT")
                    .description("Delete an event attendance by id")
                    .response::<201, Json<EventAttendanceDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use std::error::Error;

    use crate::{
        models::model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
        routes::events::dto::EventDto,
    };

    use super::*;

    #[sqlx::test]
    async fn should_create_an_event_attendance(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let new_attendance = CreateEventAttendanceRequest {
            role: "facilitator".to_string(),
        };

        let body = serde_json::to_vec(&new_attendance)?;
        let (status, response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{conversation_id}/events/{}/attendances",
                    event.id
                ),
                body.into(),
            )
            .await?;
        let event_attendance: EventAttendanceDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            event_attendance.role,
            "facilitator".to_string(),
            "incorrect role"
        );
        assert_eq!(event_attendance.event_id, event.id, "incorrect event_id");

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_an_event_attendance_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event_response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event_response)?;

        let (_, attendance_response, _) = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                &event.id.to_string(),
            )
            .await?;
        let attendance: EventAttendanceDto = serde_json::from_value(attendance_response)?;

        let (status, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances/{}",
                    conversation_id, event.id, attendance.id
                ),
            )
            .await?;
        let event_attendance: EventAttendanceDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(event_attendance.event_id, event.id, "incorrect event_id");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_event_attendances(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event_response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event_response)?;

        let _ = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                &event.id.to_string(),
            )
            .await?;

        let (status, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances",
                    conversation_id, event.id
                ),
            )
            .await?;
        let results: PaginatedResults<EventAttendanceDto> = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(results.total, 1, "incorrect total");
        assert_eq!(
            results.records[0].role,
            "participant".to_string(),
            "incorrect role"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_an_event_attendance_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event_response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event_response)?;

        let (_, attendance_response, _) = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                &event.id.to_string(),
            )
            .await?;
        let attendance: EventAttendanceDto = serde_json::from_value(attendance_response)?;

        let update = UpdateEventAttendanceRequest {
            role: Some("test_facilitator".to_string()),
        };
        let body = serde_json::to_vec(&update)?;
        let (status, response, _) = session
            .put(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances/{}",
                    conversation_id, event.id, attendance.id
                ),
                body.into(),
            )
            .await?;
        let event_attendance: EventAttendanceDto = serde_json::from_value(response)?;

        assert!(status.is_success(), "error response status");
        assert_eq!(
            event_attendance.role,
            "test_facilitator".to_string(),
            "incorrect event_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_an_event_attendance_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let (_, event_response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(event_response)?;

        let (_, attendance_response, _) = session
            .create_random_event_attendance(
                &app,
                &conversation_id.to_string(),
                &event.id.to_string(),
            )
            .await?;
        let attendance: EventAttendanceDto = serde_json::from_value(attendance_response)?;

        let _ = session
            .delete(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances/{}",
                    conversation_id, event.id, attendance.id
                ),
            )
            .await?;

        let (_, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/attendances/{}",
                    conversation_id, event.id, attendance.id
                ),
            )
            .await?;

        assert_eq!(
            response.get("err").and_then(|v| v.as_str()).unwrap(),
            "EventAttendance not found",
            "incorrect event_id"
        );

        Ok(())
    }
}
