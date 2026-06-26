//! Per-room audio recording endpoints, mounted under
//! `/conversation/{conversation_id}/events/{event_id}/rooms`.
//!
//! Each room is a single named recording within an event. The UI adds rooms one
//! at a time: create a room (which returns a presigned upload URL), upload the
//! file, then start processing. Status is tracked per room.

pub mod dto;

use std::sync::Arc;

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use hyper::HeaderMap;
use tracing::instrument;
use uuid::Uuid;

use crate::bulk_storage_service::FileMetadata;
use crate::error::ComhairleError;
use crate::models::audio_recording::{self, CreateAudioRecording};
use crate::models::event;
use crate::models::job::{self, CreateJob};
use crate::routes::audio_recordings::dto::{
    AudioRecordingDto, CreateRoomRequest, CreateRoomResponse, ProcessRoomResponse,
    RecordingDownloadUrls, RoomDetailResponse, SubmitReportResponse,
};
use crate::routes::auth::{verify_webhook_signature, RequiredAdminUser};
use crate::worker_service::process_video_call_transcriptions::TranscribeRecording;
use crate::ComhairleState;

/// Create a room and return a presigned URL for uploading its recording.
///
/// # Errors
/// * `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * `ComhairleError::DuplicateRoomName` if a room with this name already exists for the event.
/// * `ComhairleError::DatabaseError` on database errors.
#[instrument(err(Debug), skip(state))]
async fn create_room(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(request): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<CreateRoomResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    // Generate the room id up front so the S3 key prefix is fixed before the row
    // is inserted. The human name lives only in the DB; the S3 path uses the id.
    let room_id = Uuid::new_v4();
    let extension = request.file_extension.extension();
    let s3_key_prefix = format!("events/{event_id}/rooms/{room_id}");

    let recording = audio_recording::create(
        &state.db,
        &CreateAudioRecording {
            id: room_id,
            event_id,
            name: request.name,
            s3_key_prefix: s3_key_prefix.clone(),
            file_extension: request.file_extension,
        },
    )
    .await?;

    let upload_url = bulk_storage_service
        .get_write_file_url(&format!("{s3_key_prefix}/recording.{extension}"))
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRoomResponse {
            room: recording.into(),
            upload_url,
        }),
    ))
}

/// List all rooms for an event with their per-room status.
///
/// # Errors
/// * `ComhairleError::DatabaseError` on database errors.
#[instrument(err(Debug), skip(state))]
async fn list_rooms(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<AudioRecordingDto>>), ComhairleError> {
    let rooms = audio_recording::list_by_event(&state.db, &event_id).await?;
    Ok((
        StatusCode::OK,
        Json(rooms.into_iter().map(AudioRecordingDto::from).collect()),
    ))
}

/// Get a room's details and presigned download URLs (recording, transcript, report).
///
/// # Errors
/// * `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * `ComhairleError::ResourceNotFound` if the room does not exist for this event.
#[instrument(err(Debug), skip(state))]
async fn get_room(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id, room_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RoomDetailResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let recording = get_room_for_event(&state, &event_id, &room_id).await?;
    let extension = recording.file_extension.extension();
    let prefix = &recording.s3_key_prefix;

    let downloads = RecordingDownloadUrls {
        recording_url: bulk_storage_service
            .get_read_file_url(&format!("{prefix}/recording.{extension}"))
            .await?,
        transcript_url: bulk_storage_service
            .get_read_file_url(&format!("{prefix}/transcript.json"))
            .await?,
        report_url: bulk_storage_service
            .get_read_file_url(&format!("{prefix}/report.json"))
            .await?,
    };

    Ok((
        StatusCode::OK,
        Json(RoomDetailResponse {
            room: recording.into(),
            downloads,
        }),
    ))
}

/// Start processing (transcription + categorization) for a single room.
///
/// # Errors
/// * `ComhairleError::ResourceNotFound` if the room does not exist for this event.
/// * `ComhairleError::NoWorkerServiceConfigured` if no worker service is configured.
#[instrument(err(Debug), skip(state))]
async fn process_room(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id, room_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ProcessRoomResponse>), ComhairleError> {
    let worker_service = state.required_worker_service()?;

    // Ensure the room exists and belongs to this event before enqueuing work.
    let _recording = get_room_for_event(&state, &event_id, &room_id).await?;

    let job = job::create(
        &state.db,
        CreateJob {
            progress: Some(0.0),
            ..Default::default()
        },
    )
    .await?;

    worker_service
        .push_transcription_job(TranscribeRecording {
            event_id,
            conversation_id,
            room_id,
            job_id: job.id,
        })
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProcessRoomResponse {
            message: "Room processing moved to a background job".to_string(),
            job_id: job.id,
        }),
    ))
}

/// Webhook receiver for a room's categorization report.
///
/// Authenticated by the HMAC signature headers (not an admin JWT). Stores the
/// report payload at the room's S3 prefix.
///
/// # Errors
/// * `ComhairleError::AuthWebhookSignatureError` if the signature is missing or invalid.
/// * `ComhairleError::ResourceNotFound` if the room/event does not match the path.
#[instrument(err(Debug), skip(state))]
async fn submit_report(
    State(state): State<Arc<ComhairleState>>,
    headers: HeaderMap,
    Path((conversation_id, event_id, room_id)): Path<(Uuid, Uuid, Uuid)>,
    payload: String,
) -> Result<(StatusCode, Json<SubmitReportResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;
    let webhook_secret = &state
        .config
        .categorization_service
        .as_ref()
        .ok_or(ComhairleError::NoCategorizationServiceConfigured)?
        .webhook_secret;

    let webhook_timestamp = headers
        .get("X-Webhook-Timestamp")
        .ok_or(ComhairleError::AuthWebhookSignatureError(
            "Missing X-Webhook-Timestamp".to_string(),
        ))?
        .to_str()
        .map_err(|_| {
            ComhairleError::AuthWebhookSignatureError("Invalid X-Webhook-Timestamp".to_string())
        })?;
    let webhook_signature = headers
        .get("X-Webhook-Signature")
        .ok_or(ComhairleError::AuthWebhookSignatureError(
            "Missing X-Webhook-Signature".to_string(),
        ))?
        .to_str()
        .map_err(|_| {
            ComhairleError::AuthWebhookSignatureError("Invalid X-Webhook-Signature".to_string())
        })?;

    if !verify_webhook_signature(webhook_signature, webhook_timestamp, &payload, webhook_secret)? {
        return Err(ComhairleError::AuthWebhookSignatureError(
            "Invalid X-Webhook-Signature".to_string(),
        ));
    }

    // Confirm the event belongs to the conversation, then the room to the event.
    let event = event::get_by_id(&state.db, &event_id).await?;
    if event.conversation_id != conversation_id {
        return Err(ComhairleError::ResourceNotFound(format!(
            "No event {event_id} found for conversation {conversation_id}"
        )));
    }
    let recording = get_room_for_event(&state, &event_id, &room_id).await?;

    let path = format!("{}/report.json", recording.s3_key_prefix);
    let bytes = serde_json::to_vec(&payload)?;
    let metadata = FileMetadata {
        is_public: false,
        content_type: "application/json".to_string(),
    };

    let result = bulk_storage_service
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

/// Fetch a room and verify it belongs to the given event.
async fn get_room_for_event(
    state: &Arc<ComhairleState>,
    event_id: &Uuid,
    room_id: &Uuid,
) -> Result<audio_recording::AudioRecording, ComhairleError> {
    let recording = audio_recording::get_by_id(&state.db, room_id).await?;
    if recording.event_id != *event_id {
        return Err(ComhairleError::ResourceNotFound(format!(
            "No room {room_id} found for event {event_id}"
        )));
    }
    Ok(recording)
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_room, |op| {
                op.id("CreateRoom")
                    .tag("Rooms")
                    .summary("Create a room and get an upload URL")
                    .description("Create a named room for an event and return a presigned S3 URL for uploading its recording.")
                    .security_requirement("JWT")
                    .response::<201, Json<CreateRoomResponse>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_rooms, |op| {
                op.id("ListRooms")
                    .tag("Rooms")
                    .summary("List rooms for an event")
                    .description("List all rooms for an event with their per-room processing status.")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<AudioRecordingDto>>>()
            }),
        )
        .api_route(
            "/{room_id}",
            get_with(get_room, |op| {
                op.id("GetRoom")
                    .tag("Rooms")
                    .summary("Get a room and its download URLs")
                    .description("Get a room's details and presigned S3 URLs for its recording, transcript, and report.")
                    .security_requirement("JWT")
                    .response::<200, Json<RoomDetailResponse>>()
            }),
        )
        .api_route(
            "/{room_id}/process",
            post_with(process_room, |op| {
                op.id("ProcessRoom")
                    .tag("Rooms")
                    .summary("Start processing a room")
                    .description("Enqueue a background job to transcribe and categorize a single room's recording.")
                    .security_requirement("JWT")
                    .response::<200, Json<ProcessRoomResponse>>()
            }),
        )
        .api_route(
            "/{room_id}/report",
            post_with(submit_report, |op| {
                op.id("SubmitRoomReport")
                    .tag("Rooms")
                    .summary("Categorization report webhook")
                    .description("Webhook for the categorization service to submit a room's report. Authenticated by HMAC signature headers.")
                    .response::<201, Json<SubmitReportResponse>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use std::sync::Arc;

    use crate::models::audio_recording::AudioFormat;
    use crate::routes::conversations::dto::ConversationDto;
    use crate::routes::events::dto::EventDto;
    use crate::setup_server;
    use crate::test_helpers::{test_config, test_state, UserSession};

    async fn create_random_event(
        session: &mut UserSession,
        app: &axum::Router,
    ) -> Result<(ConversationDto, EventDto), Box<dyn std::error::Error>> {
        let conversation_response = session.create_random_conversation(app).await?;
        let conversation: ConversationDto = serde_json::from_value(conversation_response.1)?;
        let conversation_id: String = conversation.id.to_string();

        let event_response = session.create_random_event(app, &conversation_id).await?;
        let event: EventDto = serde_json::from_value(event_response.1)?;

        Ok((conversation, event))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_room_success_with_mocked_storage(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        storage_service
            .expect_get_write_file_url()
            .times(1)
            .returning(|_| {
                Box::pin(async move { Ok("https://s3.example.com/signed-upload-url".to_string()) })
            });

        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .config(config)
                .bulk_storage_service(Arc::new(storage_service))
                .call()?,
        );
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;

        let request = CreateRoomRequest {
            name: "Main Room".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let (status, response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/rooms/",
                    conversation.id, event.id
                ),
                json!(request).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);
        let body: CreateRoomResponse = serde_json::from_value(response)?;
        assert_eq!(body.room.name, "Main Room");
        assert_eq!(body.room.event_id, event.id);
        assert_eq!(body.upload_url, "https://s3.example.com/signed-upload-url");

        // The room is persisted and listed for the event.
        let rooms = audio_recording::list_by_event(&pool, &event.id).await?;
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].name, "Main Room");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_room_duplicate_name_conflicts(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        storage_service
            .expect_get_write_file_url()
            .returning(|_| Box::pin(async move { Ok("https://s3.example.com/url".to_string()) }));

        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .config(config)
                .bulk_storage_service(Arc::new(storage_service))
                .call()?,
        );
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;
        let url = format!("/conversation/{}/events/{}/rooms/", conversation.id, event.id);
        let request = CreateRoomRequest {
            name: "Room A".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let (status, _, _) = session
            .post(&app, &url, json!(request).to_string().into())
            .await?;
        assert_eq!(status, StatusCode::CREATED);

        let (status, _, _) = session
            .post(&app, &url, json!(request).to_string().into())
            .await?;
        assert_eq!(status, StatusCode::CONFLICT);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_list_rooms_empty(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;

        let (status, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/rooms/",
                    conversation.id, event.id
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let rooms: Vec<AudioRecordingDto> = serde_json::from_value(response)?;
        assert_eq!(rooms.len(), 0);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_room_not_found(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        storage_service.expect_get_read_file_url().returning(|path| {
            let url = format!("https://s3.example.com/{path}");
            Box::pin(async move { Ok(url) })
        });

        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(
            test_state()
                .db(pool)
                .config(config)
                .bulk_storage_service(Arc::new(storage_service))
                .call()?,
        );
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;
        let missing_room = Uuid::new_v4();

        let (status, _, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/rooms/{}",
                    conversation.id, event.id, missing_room
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);
        Ok(())
    }
}
