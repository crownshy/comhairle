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
    bulk_storage::{extract_room_id_from_key, FileMetadata},
    error::ComhairleError,
    models::{
        event::{
            self, get_by_id, CreateEvent, EventFilterOptions, EventOrderOptions,
            EventWithTranslations, PartialEvent,
        },
        event_attendance,
        job::{self, CreateJob},
        pagination::{PageOptions, PaginatedResults},
    },
    routes::{
        auth::{generate_jwt, is_user_admin, RequiredAdminUser, RequiredUser},
        events::dto::{EventDto, LocalizedEventDto},
        translations::LocaleExtractor,
    },
    worker_service::process_video_call_transcriptions::TranscribeRecording,
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
    agenda: Option<crate::models::event::EventAgenda>,
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
        agenda: payload.agenda,
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
    let video_meeting_id = event
        .video_meeting_id
        .ok_or(ComhairleError::NoVideoMeetingId)?;
    let video_call_config = &state
        .config
        .video_call_service
        .as_ref()
        .ok_or(ComhairleError::NoVideoServiceConfigured)?;

    let is_moderator = attendance.role == "facilitator";

    let claims = VideoEventJwtClaims {
        iss: &video_call_config.jwt_app_id,
        aud: &video_call_config.jwt_app_id,
        room: &video_meeting_id.to_string(),
        context: VideoEventJwtContext {
            user: VideoEventJwtUser {
                name: user.username.as_deref(),
                id: &user.id.to_string(),
                moderator: attendance.role == "facilitator",
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

    Ok((StatusCode::OK, Json(JwtResponse { jwt, is_moderator })))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct ProcessTranscriptionResponse {
    message: String,
    job_ids: Vec<Uuid>,
}

#[instrument(err(Debug), skip(state))]
async fn process_transcriptions(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<ProcessTranscriptionResponse>), ComhairleError> {
    let db_event = event::read(&state.db, event_id).await?;
    if db_event.conversation_id != conversation_id {
        return Err(ComhairleError::ResourceNotFound(format!(
            "event {event_id} for conversation {conversation_id}"
        )));
    }
    let worker_service = state.required_worker_service()?;

    let entries = state
        .bulk_storage_service
        .list_keys("comhairle-media", Some(&format!("events/{event_id}/")))
        .await?;

    let is_missing_main_recording = !entries
        .iter()
        .any(|entry| entry.contains("recording.wav") && !entry.contains("rooms/"));

    if is_missing_main_recording {
        return Err(ComhairleError::ResourceNotFound(format!(
            "recording.wav for event {event_id}"
        )));
    }

    let br_room_entries: Vec<String> = entries
        .into_iter()
        .filter(|entry| entry.contains("rooms/") && entry.contains("recording.wav"))
        .collect();

    let create_core_event_job = CreateJob {
        progress: Some(0.0),
        ..Default::default()
    };
    let core_event_job = job::create(&state.db, create_core_event_job).await?;

    worker_service
        .push_transcription_job(TranscribeRecording {
            event_id,
            conversation_id,
            room_id: None,
            job_id: core_event_job.id,
        })
        .await?;

    let mut br_room_job_ids = vec![];
    for entry in br_room_entries {
        let room_id = extract_room_id_from_key(&entry);

        if let Some(room_id) = room_id {
            let create_job = CreateJob {
                progress: Some(0.0),
                ..Default::default()
            };
            let job = job::create(&state.db, create_job).await?;
            br_room_job_ids.push(job.id);

            worker_service
                .push_transcription_job(TranscribeRecording {
                    event_id,
                    conversation_id,
                    room_id: Some(room_id.to_string()),
                    job_id: job.id,
                })
                .await?;
        }
    }

    let mut job_ids = vec![core_event_job.id];
    job_ids.extend(br_room_job_ids);

    Ok((
        StatusCode::OK,
        Json(ProcessTranscriptionResponse {
            message: "Transcription processing moved to background jobs".to_string(),
            job_ids,
        }),
    ))
}

#[derive(Deserialize, JsonSchema, Debug, Default)]
struct SubmitReportParams {
    room_id: Option<String>,
}

#[derive(Deserialize, JsonSchema, Debug, Default)]
struct SubmitReportRequest {
    result: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct SubmitReportResponse {
    url: String,
    success: bool,
}

#[instrument(err(Debug), skip(state))]
async fn submit_report(
    State(state): State<Arc<ComhairleState>>,
    Query(params): Query<SubmitReportParams>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SubmitReportRequest>,
) -> Result<(StatusCode, Json<SubmitReportResponse>), ComhairleError> {
    let event = get_by_id(&state.db, &event_id).await?;

    if event.conversation_id != conversation_id {
        return Err(StatusCode::NOT_FOUND.into());
    }
    let path = if let Some(room_id) = params.room_id {
        format!("events/{}/rooms/{}/report.json", event_id, room_id)
    } else {
        format!("events/{}/report.json", event_id)
    };

    let bytes = serde_json::to_vec(&payload.result)?;
    let metadata = FileMetadata {
        is_public: false,
        content_type: "application/json".to_string(),
    };

    let result = state
        .bulk_storage_service
        .upload_file(&path, bytes, metadata)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(SubmitReportResponse {
            success: true,
            url: result.url,
        }),
    ))
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
        .api_route("/{event_id}/transcriptions", 
            post_with(process_transcriptions, |op| {
                op.id("ProcessVideoCallTranscriptions")
                    .tag("Events")
                    .summary("Process video call transcription")
                    .description("Triggers transcription processing in a background worker")
                    .security_requirement("JWT")
                    .response::<200, Json<ProcessTranscriptionResponse>>()

        }))
        .api_route("/{event_id}/report", 
            post_with(submit_report, |op| {
                op.id("SubmitEventReport")
                    .tag("Events")
                    .summary("Categorization report")
                    .description("Submit categorization report to bulk storage")
                    .response::<201, Json<SubmitReportResponse>>()

        }))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use serde_json::json;
    use sqlx::PgPool;
    use std::error::Error;

    use crate::{
        bulk_storage::{MockBulkStorageService, UploadResult},
        models::model_test_helpers::{get_random_conversation_id, setup_default_app_and_session},
        routes::conversations::dto::ConversationDto,
        setup_server,
        test_helpers::{test_state, UserSession},
        worker_service::MockWorkerService,
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
            agenda: None,
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

    #[sqlx::test]
    async fn should_start_transcription_single_pipeline_for_event(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut worker_service = MockWorkerService::new();

        worker_service
            .expect_push_transcription_job()
            .once()
            .returning(|_| Box::pin(async move { Ok(()) }));

        let mut storage_service = MockBulkStorageService::new();

        storage_service
            .expect_list_keys()
            .once()
            .returning(|_, _| Box::pin(async move { Ok(vec!["recording.wav".to_string()]) }));

        let state = test_state()
            .db(pool)
            .worker_service(Arc::new(worker_service))
            .bulk_storage_service(Arc::new(storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/transcriptions",
                    conversation_id, event.id
                ),
                Body::empty(),
            )
            .await?;
        let response: ProcessTranscriptionResponse = serde_json::from_value(value)?;

        assert_eq!(
            response.job_ids.len(),
            1,
            "incorrect number of jobs spawned"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_err_if_recording_missing(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut storage_service = MockBulkStorageService::new();

        storage_service
            .expect_list_keys()
            .once()
            .returning(|_, _| Box::pin(async move { Ok(vec!["not-a-recording.pdf".to_string()]) }));

        let state = test_state()
            .db(pool)
            .bulk_storage_service(Arc::new(storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/transcriptions",
                    conversation_id, event.id
                ),
                Body::empty(),
            )
            .await?;

        assert_eq!(
            value.get("err").and_then(|v| v.as_str()).unwrap(),
            &format!("recording.wav for event {} not found", event.id),
            "incorrect error message"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_start_transcription_pipelines_for_event_with_breakout_rooms(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut worker_service = MockWorkerService::new();

        worker_service
            .expect_push_transcription_job()
            .times(5)
            .returning(|_| Box::pin(async move { Ok(()) }));

        let mut storage_service = MockBulkStorageService::new();

        storage_service.expect_list_keys().once().returning(|_, _| {
            Box::pin(async move {
                Ok(vec![
                    "recording.wav".to_string(),
                    ".secret-file.temp".to_string(),
                    "rooms/1234/recording.wav".to_string(),
                    "rooms/1234/.secret-file.temp".to_string(),
                    "rooms/4321/recording.wav".to_string(),
                    "rooms/5678/recording.wav".to_string(),
                    "rooms/8765/recording.wav".to_string(),
                ])
            })
        });

        let state = test_state()
            .db(pool)
            .worker_service(Arc::new(worker_service))
            .bulk_storage_service(Arc::new(storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let (_, response, _) = session
            .create_random_event(&app, &conversation_id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(response)?;

        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/transcriptions",
                    conversation_id, event.id
                ),
                Body::empty(),
            )
            .await?;
        let response: ProcessTranscriptionResponse = serde_json::from_value(value)?;

        assert_eq!(
            response.job_ids.len(),
            5,
            "incorrect number of jobs spawned"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_upload_report_to_bulk_storage(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bulk_storage_service = MockBulkStorageService::new();

        bulk_storage_service
            .expect_upload_file()
            .once()
            .returning(|_, _, _| {
                Box::pin(async move {
                    Ok(UploadResult {
                        url: "https://storage.com/some_file".to_owned(),
                    })
                })
            });

        let state = test_state()
            .db(pool)
            .bulk_storage_service(Arc::new(bulk_storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (_, value, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(value)?;
        let (_, value, _) = session
            .create_random_event(&app, &conversation.id.to_string())
            .await?;
        let event: EventDto = serde_json::from_value(value)?;

        let body = include_str!("../../../fixtures/tttc-report.json");
        let (_, value, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/report",
                    conversation.id, event.id
                ),
                body.into(),
            )
            .await?;
        let response: SubmitReportResponse = serde_json::from_value(value)?;

        assert!(response.success, "incorrect success");
        assert_eq!(
            response.url,
            "https://storage.com/some_file".to_string(),
            "incorrect url"
        );

        Ok(())
    }
}
