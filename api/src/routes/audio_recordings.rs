//! Audio recording endpoints, mounted under
//! `/conversation/{conversation_id}/events/{event_id}/audio_recordings`.
//!
//! Each recording lives under an event. The UI adds recordings one
//! at a time: create a recording (which returns a presigned upload URL), upload the
//! file, then start processing. Status is tracked per recording.

pub mod dto;

use std::sync::Arc;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
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
    AudioRecordingDto, CreateRecordingRequest, CreateRecordingResponse, DeleteRecordingResponse,
    ProcessRecordingResponse, RecordingDetailResponse, RecordingDownloadUrls, SubmitReportResponse,
};
use crate::routes::auth::{verify_webhook_signature, RequiredAdminUser};
use crate::worker_service::process_video_call_transcriptions::TranscribeRecording;
use crate::ComhairleState;

/// Create an audio recording and return a presigned URL for uploading its audio.
///
/// # Errors
/// * `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * `ComhairleError::DuplicateRecordingName` if a recording with this name already exists for the event.
/// * `ComhairleError::DatabaseError` on database errors.
#[instrument(err(Debug), skip(state))]
async fn create_recording(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(request): Json<CreateRecordingRequest>,
) -> Result<(StatusCode, Json<CreateRecordingResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let recording_id = Uuid::new_v4();
    let extension = request.file_extension.extension();
    let s3_key_prefix = format!("events/{event_id}/recordings/{recording_id}");

    let recording = audio_recording::create(
        &state.db,
        &CreateAudioRecording {
            id: recording_id,
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
        Json(CreateRecordingResponse {
            recording: recording.into(),
            upload_url,
        }),
    ))
}

/// List all recordings for an event with their status.
///
/// # Errors
/// * `ComhairleError::DatabaseError` on database errors.
#[instrument(err(Debug), skip(state))]
async fn list_recordings(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<AudioRecordingDto>>), ComhairleError> {
    let recordings = audio_recording::list_by_event(&state.db, &event_id).await?;
    Ok((
        StatusCode::OK,
        Json(recordings.into_iter().map(AudioRecordingDto::from).collect()),
    ))
}

/// Get a recording's details and presigned download URLs (recording, transcript, report).
///
/// # Errors
/// * `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * `ComhairleError::ResourceNotFound` if the recording does not exist for this event.
#[instrument(err(Debug), skip(state))]
async fn get_recording(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id, recording_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<RecordingDetailResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let recording = audio_recording::get_by_id_and_event(&state.db, &recording_id, &event_id).await?;
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
        Json(RecordingDetailResponse {
            recording: recording.into(),
            downloads,
        }),
    ))
}

/// Start processing (transcription + categorization) for a single recording.
///
/// # Errors
/// * `ComhairleError::ResourceNotFound` if the recording does not exist for this event.
/// * `ComhairleError::NoWorkerServiceConfigured` if no worker service is configured.
#[instrument(err(Debug), skip(state))]
async fn process_recording(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id, recording_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<ProcessRecordingResponse>), ComhairleError> {
    let worker_service = state.required_worker_service()?;

    // Ensure the recording exists and belongs to this event before enqueuing work.
    let _recording = audio_recording::get_by_id_and_event(&state.db, &recording_id, &event_id).await?;

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
            recording_id,
            job_id: job.id,
        })
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProcessRecordingResponse {
            message: "Recording processing moved to a background job".to_string(),
            job_id: job.id,
        }),
    ))
}

/// Delete a recording and best-effort-clean its files from bulk storage.
///
/// Used to remove stuck/orphaned recordings (e.g. a mid-upload failure left the
/// row in `pending` forever). Storage deletes are idempotent and silent on
/// missing keys, so partially-uploaded recordings clean up cleanly. Any
/// in-flight processing job for this recording will fail to find the row and
/// drop on the floor — acceptable given the row is being abandoned.
///
/// # Errors
/// * `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * `ComhairleError::ResourceNotFound` if the recording does not exist for this event.
#[instrument(err(Debug), skip(state))]
async fn delete_recording(
    State(state): State<Arc<ComhairleState>>,
    Path((_conversation_id, event_id, recording_id)): Path<(Uuid, Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<DeleteRecordingResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let recording = audio_recording::get_by_id_and_event(&state.db, &recording_id, &event_id).await?;

    // Best-effort: log and swallow storage errors so a hung S3 doesn't block
    // the user from getting rid of a stuck row.
    let extension = recording.file_extension.extension();
    let prefix = &recording.s3_key_prefix;
    for path in [
        format!("{prefix}/recording.{extension}"),
        format!("{prefix}/transcript.json"),
        format!("{prefix}/report.json"),
    ] {
        if let Err(err) = bulk_storage_service.delete_file(&path).await {
            tracing::warn!(?err, path, "failed to delete recording file from bulk storage");
        }
    }

    let deleted = audio_recording::delete(&state.db, &recording_id, &event_id).await?;

    Ok((
        StatusCode::OK,
        Json(DeleteRecordingResponse {
            recording: deleted.into(),
        }),
    ))
}

/// Webhook receiver for a recording's categorization report.
///
/// Authenticated by the HMAC signature headers (not an admin JWT).
///
/// # Errors
/// * `ComhairleError::AuthWebhookSignatureError` if the signature is missing or invalid.
/// * `ComhairleError::ResourceNotFound` if the recording/event does not match the path.
#[instrument(err(Debug), skip(state))]
async fn submit_report(
    State(state): State<Arc<ComhairleState>>,
    headers: HeaderMap,
    Path((conversation_id, event_id, recording_id)): Path<(Uuid, Uuid, Uuid)>,
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

    let event = event::get_by_id(&state.db, &event_id).await?;
    if event.conversation_id != conversation_id {
        return Err(ComhairleError::ResourceNotFound(format!(
            "No event {event_id} found for conversation {conversation_id}"
        )));
    }
    let recording = audio_recording::get_by_id_and_event(&state.db, &recording_id, &event_id).await?;

    let path = format!("{}/report.json", recording.s3_key_prefix);
    let bytes = serde_json::to_vec(&payload)?;
    let metadata = FileMetadata {
        is_public: false,
        content_type: "application/json".to_string(),
    };

    let result = bulk_storage_service
        .upload_file(&path, bytes, metadata)
        .await?;

    audio_recording::update_status(
        &state.db,
        &recording.id,
        audio_recording::AudioRecordingStatus::BothAvailable,
    )
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
        .api_route(
            "/",
            post_with(create_recording, |op| {
                op.id("CreateAudioRecording")
                    .tag("Audio Recordings")
                    .summary("Create an audio recording and get an upload URL")
                    .description("Create a named audio recording for an event and return a presigned S3 URL for uploading its audio.")
                    .security_requirement("JWT")
                    .response::<201, Json<CreateRecordingResponse>>()
            }),
        )
        .api_route(
            "/",
            get_with(list_recordings, |op| {
                op.id("ListAudioRecordings")
                    .tag("Audio Recordings")
                    .summary("List audio recordings for an event")
                    .description("List all audio recordings for an event with their processing status.")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<AudioRecordingDto>>>()
            }),
        )
        .api_route(
            "/{recording_id}",
            get_with(get_recording, |op| {
                op.id("GetAudioRecording")
                    .tag("Audio Recordings")
                    .summary("Get an audio recording and its download URLs")
                    .description("Get an audio recording's details and presigned S3 URLs for its audio, transcript, and report.")
                    .security_requirement("JWT")
                    .response::<200, Json<RecordingDetailResponse>>()
            }),
        )
        .api_route(
            "/{recording_id}",
            delete_with(delete_recording, |op| {
                op.id("DeleteAudioRecording")
                    .tag("Audio Recordings")
                    .summary("Delete an audio recording")
                    .description("Delete an audio recording and best-effort-clean its files from bulk storage. Useful for clearing stuck rows left behind by a failed upload.")
                    .security_requirement("JWT")
                    .response::<200, Json<DeleteRecordingResponse>>()
            }),
        )
        .api_route(
            "/{recording_id}/process",
            post_with(process_recording, |op| {
                op.id("ProcessAudioRecording")
                    .tag("Audio Recordings")
                    .summary("Start processing an audio recording")
                    .description("Enqueue a background job to transcribe and categorize a single audio recording.")
                    .security_requirement("JWT")
                    .response::<200, Json<ProcessRecordingResponse>>()
            }),
        )
        .api_route(
            "/{recording_id}/report",
            post_with(submit_report, |op| {
                op.id("SubmitAudioRecordingReport")
                    .tag("Audio Recordings")
                    .summary("Categorization report webhook")
                    .description("Webhook for the categorization service to submit a recording's report. Authenticated by HMAC signature headers.")
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
    async fn test_create_recording_success_with_mocked_storage(
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

        let request = CreateRecordingRequest {
            name: "Main Room".to_string(),
            file_extension: AudioFormat::Wav,
        };

        let (status, response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio_recordings/",
                    conversation.id, event.id
                ),
                json!(request).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);
        let body: CreateRecordingResponse = serde_json::from_value(response)?;
        assert_eq!(body.recording.name, "Main Room");
        assert_eq!(body.recording.event_id, event.id);
        assert_eq!(body.upload_url, "https://s3.example.com/signed-upload-url");

        // The recording is persisted and listed for the event.
        let recordings = audio_recording::list_by_event(&pool, &event.id).await?;
        assert_eq!(recordings.len(), 1);
        assert_eq!(recordings[0].name, "Main Room");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_recording_duplicate_name_conflicts(
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
        let url = format!("/conversation/{}/events/{}/audio_recordings/", conversation.id, event.id);
        let request = CreateRecordingRequest {
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
    async fn test_list_recordings_empty(
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
                    "/conversation/{}/events/{}/audio_recordings/",
                    conversation.id, event.id
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);
        let recordings: Vec<AudioRecordingDto> = serde_json::from_value(response)?;
        assert_eq!(recordings.len(), 0);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_get_recording_not_found(
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
        let missing_recording = Uuid::new_v4();

        let (status, _, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio_recordings/{}",
                    conversation.id, event.id, missing_recording
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_delete_recording_removes_row_and_files(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        // We expect attempts to delete the three known suffixes; mock them all
        // as successful.
        storage_service
            .expect_delete_file()
            .times(3)
            .returning(|_| Box::pin(async move { Ok(()) }));

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

        let recording = audio_recording::create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Main Room".to_string(),
                s3_key_prefix: format!("events/{}/recordings/{}", event.id, Uuid::new_v4()),
                file_extension: AudioFormat::Wav,
            },
        )
        .await?;

        let (status, _resp, _) = session
            .delete(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio_recordings/{}",
                    conversation.id, event.id, recording.id
                ),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);

        // The row is gone.
        let recordings = audio_recording::list_by_event(&pool, &event.id).await?;
        assert_eq!(recordings.len(), 0);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_delete_recording_survives_storage_errors(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::bulk_storage_service::error::BulkStorageError;

        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        // Every storage delete fails (e.g. files were never uploaded).
        storage_service
            .expect_delete_file()
            .returning(|_| {
                Box::pin(async move { Err(BulkStorageError::FailedToDelete("nope".to_string())) })
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

        let recording = audio_recording::create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Main Room".to_string(),
                s3_key_prefix: format!("events/{}/recordings/{}", event.id, Uuid::new_v4()),
                file_extension: AudioFormat::Wav,
            },
        )
        .await?;

        let (status, _resp, _) = session
            .delete(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio_recordings/{}",
                    conversation.id, event.id, recording.id
                ),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);
        // The DB row is gone even though the storage cleanup failed.
        assert!(audio_recording::get_by_id(&pool, &recording.id).await.is_err());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_submit_report_marks_both_available(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::bulk_storage_service::UploadResult;
        use crate::routes::auth::build_webhook_signature;
        use axum::body::Body;
        use axum::http::{HeaderName, HeaderValue};
        use chrono::Utc;
        use std::str::FromStr;

        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();
        storage_service
            .expect_upload_file()
            .times(1)
            .returning(|_, _, _| {
                Box::pin(async move {
                    Ok(UploadResult {
                        url: "https://s3.example.com/report.json".to_string(),
                    })
                })
            });

        let mut config = test_config()?;
        config.bot_service = None;
        let webhook_secret = config
            .categorization_service
            .as_ref()
            .expect("test config has a categorization service")
            .webhook_secret
            .clone();

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

        // A recording that has its transcript but is awaiting its report.
        let recording = audio_recording::create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Main Room".to_string(),
                s3_key_prefix: format!("events/{}/recordings/{}", event.id, Uuid::new_v4()),
                file_extension: AudioFormat::Wav,
            },
        )
        .await?;

        // Sign the payload the way the categorization service would.
        let body = serde_json::json!({ "report": "done" }).to_string();
        let timestamp = Utc::now().timestamp().to_string();
        let signature = build_webhook_signature(&timestamp, &body, &webhook_secret)?;

        let (status, _resp, _) = session
            .post_with_headers(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio_recordings/{}/report",
                    conversation.id, event.id, recording.id
                ),
                Body::from(body),
                &[
                    (
                        HeaderName::from_str("X-Webhook-Timestamp")?,
                        HeaderValue::from_str(&timestamp)?,
                    ),
                    (
                        HeaderName::from_str("X-Webhook-Signature")?,
                        HeaderValue::from_str(&signature)?,
                    ),
                ],
            )
            .await?;

        assert_eq!(status, StatusCode::CREATED);

        // The recording is now both_available.
        let refreshed = audio_recording::get_by_id(&pool, &recording.id).await?;
        assert_eq!(
            refreshed.status,
            audio_recording::AudioRecordingStatus::BothAvailable
        );

        Ok(())
    }
}
