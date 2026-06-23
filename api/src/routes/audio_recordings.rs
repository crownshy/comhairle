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
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::audio_recording::{self, CreateAudioRecording};
use crate::routes::audio_recordings::dto::{
    AudioRecordingDto, RecordingDownloadUrls, RequestUploadUrlsRequest, RequestUploadUrlsResponse,
    SignedDownloadUrls,
};
use crate::routes::auth::RequiredAdminUser;
use crate::ComhairleState;

/// Request signed URLs for uploading audio recordings (main + breakout rooms)
///
/// # Errors
/// * Returns `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * Returns `ComhairleError::DatabaseError` if there is a database error when creating the audio recording record.
#[instrument(err(Debug), skip(state))]
async fn request_upload_urls(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
    Json(request): Json<RequestUploadUrlsRequest>,
) -> Result<(StatusCode, Json<RequestUploadUrlsResponse>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let event_id_str = event_id.to_string();
    let base_key_prefix = format!("events/{}", event_id_str);
    let extension = request.file_extension.extension();

    let create_recording = CreateAudioRecording {
        event_id,
        breakout_room_ids: request.breakout_rooms.clone(),
        s3_key_prefix: base_key_prefix.clone(),
        file_extension: request.file_extension,
    };
    let _ = audio_recording::create(&state.db, &create_recording).await?;

    let main_signed_url = bulk_storage_service
        .get_write_file_url(&format!("{}/recording.{}", base_key_prefix, extension))
        .await?;

    let mut breakout_room_urls = Vec::new();
    for room_id in request.breakout_rooms {
        let breakout_key_prefix = format!("{}/rooms/{}", event_id_str, room_id);

        let breakout_signed_url = bulk_storage_service
            .get_write_file_url(&format!("{}/recording.{}", breakout_key_prefix, extension))
            .await?;

        breakout_room_urls.push((room_id, breakout_signed_url));
    }

    Ok((
        StatusCode::OK,
        Json(RequestUploadUrlsResponse {
            main: main_signed_url,
            breakout_rooms: breakout_room_urls,
        }),
    ))
}

/// Get signed URLs for downloading transcript and report
///
/// # Errors
/// * Returns `ComhairleError::NoBulkStorageServiceConfigured` if no bulk storage service is configured.
/// * Returns `ComhairleError::ResourceNotFound` if there is no audio recording for the given event.
/// * Returns `ComhairleError::DatabaseError` if there is a database error when retrieving the audio recording.
#[instrument(err(Debug), skip(state))]
async fn get_download_urls(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<SignedDownloadUrls>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    // Get the audio recording for this event
    let recording = audio_recording::get_by_event(&state.db, &event_id).await?;
    let extension = recording.file_extension.extension();

    // Generate signed URLs for transcript and report
    let main_recording_url = bulk_storage_service
        .get_read_file_url(&format!(
            "{}/recording.{}",
            recording.s3_key_prefix, extension
        ))
        .await?;
    let main_transcript_url = bulk_storage_service
        .get_read_file_url(&format!("{}/transcript.json", recording.s3_key_prefix))
        .await?;
    let main_report_url = bulk_storage_service
        .get_read_file_url(&format!("{}/report.json", recording.s3_key_prefix))
        .await?;

    let mut output = SignedDownloadUrls {
        main: RecordingDownloadUrls {
            recording_url: main_recording_url,
            transcript_url: main_transcript_url,
            report_url: main_report_url,
        },
        breakout_rooms: Vec::new(),
    };
    for room in &recording.breakout_room_ids {
        let breakout_key_prefix = format!("{}/rooms/{}", event_id, room);

        let breakout_recording_url = bulk_storage_service
            .get_read_file_url(&format!(
                "{}/recording.{}",
                breakout_key_prefix, extension
            ))
            .await?;
        let breakout_transcript_url = bulk_storage_service
            .get_read_file_url(&format!("{}/transcript.json", breakout_key_prefix))
            .await?;
        let breakout_report_url = bulk_storage_service
            .get_read_file_url(&format!("{}/report.json", breakout_key_prefix))
            .await?;

        // Add to the response
        output.breakout_rooms.push((
            room.clone(),
            RecordingDownloadUrls {
                recording_url: breakout_recording_url,
                transcript_url: breakout_transcript_url,
                report_url: breakout_report_url,
            },
        ));
    }

    Ok((StatusCode::OK, Json(output)))
}

/// List audio recordings for an event
///
/// # Errors
/// * Returns `ComhairleError::DatabaseError` if there is a database error when retrieving the audio recording.
#[instrument(err(Debug), skip(state))]
async fn get_recording_for_event(
    State(state): State<Arc<ComhairleState>>,
    Path((conversation_id, event_id)): Path<(Uuid, Uuid)>,
    RequiredAdminUser(_user): RequiredAdminUser,
) -> Result<(StatusCode, Json<Vec<AudioRecordingDto>>), ComhairleError> {
    // Get the single recording for this event
    match audio_recording::get_by_event(&state.db, &event_id).await {
        Ok(recording) => {
            let dto = AudioRecordingDto::from(recording);
            Ok((StatusCode::OK, Json(vec![dto])))
        }
        Err(ComhairleError::ResourceNotFound(_)) => Ok((StatusCode::OK, Json(vec![]))),
        Err(e) => Err(e),
    }
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/upload",
            post_with(request_upload_urls, |op| {
                op.id("RequestAudioUploadUrls")
                    .tag("Audio Recordings")
                    .summary("Request signed URLs for uploading audio recordings")
                    .description("Request signed S3 URLs for uploading main and breakout room audio recordings. Returns presigned URLs and creates DB records.")
                    .security_requirement("JWT")
                    .response::<200, Json<RequestUploadUrlsResponse>>()
            }),
        )
        .api_route(
            "/download",
            get_with(get_download_urls, |op| {
                op.id("GetAudioDownloadUrls")
                    .tag("Audio Recordings")
                    .summary("Get signed URLs for downloading transcript and report")
                    .description("Get presigned S3 URLs for downloading transcript.json and report.json for a recording.")
                    .security_requirement("JWT")
                    .response::<200, Json<SignedDownloadUrls>>()
            }),
        )
        .api_route(
            "/",
            get_with(get_recording_for_event, |op| {
                op.id("ListAudioRecordings")
                    .tag("Audio Recordings")
                    .summary("List audio recordings for an event")
                    .description("List all audio recordings for an event. Optionally filter by room_id.")
                    .security_requirement("JWT")
                    .response::<200, Json<Vec<AudioRecordingDto>>>()
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

    #[sqlx::test]
    async fn test_request_upload_urls_event_not_found(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, _event) = create_random_event(&mut session, &app).await?;
        let conversation_id: String = conversation.id.to_string();
        let fake_event = Uuid::new_v4();

        let request = RequestUploadUrlsRequest {
            breakout_rooms: vec!["room1".to_string(), "room2".to_string()],
            file_extension: AudioFormat::Wav,
        };

        let (status, _response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/upload",
                    conversation_id, fake_event
                ),
                json!(request).to_string().into(),
            )
            .await?;

        // Should fail with a database error because the event does not exist
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_download_urls_recording_not_found(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;
        let conversation_id: String = conversation.id.to_string();
        let event_id: String = event.id.to_string();

        let (status, _response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/download",
                    conversation_id, event_id
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::NOT_FOUND);
        Ok(())
    }

    // TODO: Mock bulk storage service for further tests.

    #[sqlx::test]
    async fn test_request_upload_urls_success_with_mocked_storage(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();

        // Mock the get_write_file_url calls for main and breakout rooms
        storage_service
            .expect_get_write_file_url()
            .times(3) // 1 main + 2 breakout rooms
            .returning(|_| {
                Box::pin(async move {
                    Ok("https://s3.example.com/signed-upload-url".to_string())
                })
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
        let conversation_id: String = conversation.id.to_string();
        let event_id: String = event.id.to_string();

        let request = RequestUploadUrlsRequest {
            breakout_rooms: vec!["room1".to_string(), "room2".to_string()],
            file_extension: AudioFormat::Wav,
        };

        let (status, response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/upload",
                    conversation_id, event_id
                ),
                json!(request).to_string().into(),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let upload_response: RequestUploadUrlsResponse = serde_json::from_value(response)?;
        
        // Verify main room URL
        assert!(!upload_response.main.is_empty());
        assert_eq!(upload_response.main, "https://s3.example.com/signed-upload-url");

        // Verify breakout room URLs
        assert_eq!(upload_response.breakout_rooms.len(), 2);
        for (i, room_urls) in upload_response.breakout_rooms.iter().enumerate() {
            assert_eq!(room_urls.0, format!("room{}", i + 1));
            assert_eq!(room_urls.1, "https://s3.example.com/signed-upload-url");
        }

        // Verify recording was created in database
        let recording = audio_recording::get_by_event(&pool, &Uuid::parse_str(&event_id)?).await?;
        assert_eq!(recording.breakout_room_ids, vec!["room1".to_string(), "room2".to_string()]);
        assert_eq!(
            recording.status,
            audio_recording::AudioRecordingStatus::Pending
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_download_urls_after_status_update(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage_service = crate::bulk_storage_service::MockBulkStorageService::new();

        // Mock the get_write_file_url calls for upload
        storage_service
            .expect_get_write_file_url()
            .times(3) // 1 main + 2 breakout rooms
            .returning(|_| {
                Box::pin(async move {
                    Ok("https://s3.example.com/signed-upload-url".to_string())
                })
            });

        // Mock the get_read_file_url calls for download (3 files per room: recording, transcript, report)
        // Main room: 3 files, Breakout rooms: 2 * 3 = 6 files, Total = 9 files
        storage_service
            .expect_get_read_file_url()
            .times(9) // 3 main files + 6 breakout files (2 rooms * 3 files each)
            .returning(|path| {
                let url = format!("https://s3.example.com/signed-read-{}", path);
                Box::pin(async move { Ok(url) })
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
        let conversation_id: String = conversation.id.to_string();
        let event_id: String = event.id.to_string();

        let request = RequestUploadUrlsRequest {
            breakout_rooms: vec!["room1".to_string(), "room2".to_string()],
            file_extension: AudioFormat::Wav,
        };

        let (status, _response, _) = session
            .post(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/upload",
                    conversation_id, event_id
                ),
                json!(request).to_string().into(),
            )
            .await?;
        assert_eq!(status, StatusCode::OK);

        let recording = audio_recording::get_by_event(&pool, &event.id).await?;
        audio_recording::update_status(
            &pool,
            &recording.id,
            audio_recording::AudioRecordingStatus::Completed,
        )
        .await?;

        let (status, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/download",
                    conversation_id, event_id
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let download_response: SignedDownloadUrls = serde_json::from_value(response)?;

        // Verify main room download URLs
        assert!(!download_response.main.recording_url.is_empty());
        assert!(!download_response.main.transcript_url.is_empty());
        assert!(!download_response.main.report_url.is_empty());

        // Verify main room URLs contain expected paths
        assert!(download_response
            .main
            .recording_url
            .contains("signed-read"));
        assert!(download_response
            .main
            .transcript_url
            .contains("signed-read"));
        assert!(download_response.main.report_url.contains("signed-read"));

        // Verify breakout room download URLs
        assert_eq!(download_response.breakout_rooms.len(), 2);
        for (i, (room_id, urls)) in download_response.breakout_rooms.iter().enumerate() {
            assert_eq!(room_id, &format!("room{}", i + 1));
            assert!(!urls.recording_url.is_empty());
            assert!(!urls.transcript_url.is_empty());
            assert!(!urls.report_url.is_empty());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_recording_for_event_empty_list(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = test_config()?;
        config.bot_service = None;
        let state = Arc::new(test_state().db(pool).config(config).call()?);
        let app = setup_server(state.clone()).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (conversation, event) = create_random_event(&mut session, &app).await?;
        let conversation_id: String = conversation.id.to_string();
        let event_id: String = event.id.to_string();

        let (status, response, _) = session
            .get(
                &app,
                &format!(
                    "/conversation/{}/events/{}/audio-recordings/",
                    conversation_id, event_id
                ),
            )
            .await?;

        assert_eq!(status, StatusCode::OK);

        let recordings: Vec<AudioRecordingDto> = serde_json::from_value(response)?;
        assert_eq!(recordings.len(), 0, "Should return empty list when no recording exists");

        Ok(())
    }
}
