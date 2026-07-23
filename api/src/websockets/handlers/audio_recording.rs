use crate::websockets::ConnectionId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::ComhairleState;
use crate::bulk_storage_service::{MultipartUploadPartNumber, StorageEntityTag, StorageUploadID};
use crate::error::ComhairleError;
use crate::models::job::{self, CreateJob};
use crate::models::{audio_recording, event_attendance, live_audio_recording, users};
use crate::routes::audio_recordings::dto::{
    AckLiveAudioRecordingPartResponse, LiveAudioRecordingDto, LiveAudioRecordingStateResponse,
    PresignLiveAudioRecordingPartResponse, ProcessRecordingResponse,
};
use crate::routes::auth::is_user_admin;
use crate::websockets::error::WebsocketError;
use crate::websockets::messages::WebSocketMessage;
use crate::websockets::{WebSocketConnection, WebSocketMessageHandler};
use crate::worker_service::process_video_call_transcriptions::TranscribeRecording;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsAcquireRequest {
    request_id: String,
    event_id: Uuid,
    live_recording_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsPresignPartRequest {
    request_id: String,
    part_number: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsAckPartRequest {
    request_id: String,
    part_number: i32,
    etag: String,
    size_bytes: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsReleaseRequest {
    request_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsDisconnectSessionsRequest {
    request_id: String,
    event_id: Uuid,
    live_recording_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsCompleteRequest {
    request_id: String,
    conversation_id: Uuid,
    event_id: Uuid,
    live_recording_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsDeleteRequest {
    request_id: String,
    event_id: Uuid,
    live_recording_id: Uuid,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename_all = "camelCase")]
struct WsDisconnectSessionResponse {
    disconnected_sessions: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename_all = "camelCase")]
struct WsResultPayload<T: Serialize> {
    request_id: String,
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Copy, Clone)]
struct LiveConnectionInfo {
    event_id: Uuid,
    live_recording_id: Uuid,
}

/// Websocket handler for live audio recording domain.
///
/// This currently provides connect and disconnect debug hooks only.
pub struct AudioRecordingMessageHandler {
    live_recording_id_map: Arc<Mutex<HashMap<ConnectionId, LiveConnectionInfo>>>,
    live_recording_lock_map: Arc<Mutex<HashMap<Uuid, ConnectionId>>>,
}

impl AudioRecordingMessageHandler {
    pub fn new() -> Self {
        Self {
            live_recording_id_map: Arc::new(Mutex::new(HashMap::new())),
            live_recording_lock_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn acquire_connection_lock(
        &self,
        connection_id: &ConnectionId,
        connection_info: LiveConnectionInfo,
    ) -> Result<(), ComhairleError> {
        let mut live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
            Err(ComhairleError::CorruptedData(format!(
                "live_recording_id_map is poisoned: {}",
                err
            )))
        })?;

        if live_recording_id_map.get(connection_id).is_some() {
            return Err(ComhairleError::Conflict(
                "Connection already has an active recording session".to_string(),
            ));
        }

        let mut live_recording_lock_map = self.live_recording_lock_map.lock().or_else(|err| {
            Err(ComhairleError::CorruptedData(format!(
                "live_recording_lock_map is poisoned: {}",
                err
            )))
        })?;

        if let Some(owner_connection_id) =
            live_recording_lock_map.get(&connection_info.live_recording_id)
            && owner_connection_id != connection_id
        {
            return Err(ComhairleError::LiveAudioRecordingLocked);
        }

        live_recording_lock_map.insert(connection_info.live_recording_id, connection_id.clone());
        live_recording_id_map.insert(connection_id.clone(), connection_info);
        Ok(())
    }

    fn release_connection_lock(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<Option<LiveConnectionInfo>, ComhairleError> {
        let removed = {
            let mut live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                Err(ComhairleError::CorruptedData(format!(
                    "live_recording_id_map is poisoned: {}",
                    err
                )))
            })?;
            live_recording_id_map.remove(connection_id)
        };

        if let Some(info) = removed {
            let mut live_recording_lock_map =
                self.live_recording_lock_map.lock().or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_lock_map is poisoned: {}",
                        err
                    )))
                })?;
            if live_recording_lock_map
                .get(&info.live_recording_id)
                .is_some_and(|owner| owner == connection_id)
            {
                live_recording_lock_map.remove(&info.live_recording_id);
            }
            return Ok(Some(info));
        }

        Ok(None)
    }

    fn get_live_connection_info(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<LiveConnectionInfo, ComhairleError> {
        self.live_recording_id_map
            .lock()
            .or_else(|err| {
                Err(ComhairleError::CorruptedData(format!(
                    "live_recording_id_map is poisoned: {}",
                    err
                )))
            })?
            .get(connection_id)
            .copied()
            .ok_or_else(|| {
                ComhairleError::ResourceNotFound(
                    "No live recording associated with this connection".to_string(),
                )
            })
    }

    async fn is_admin_user(
        state: &Arc<ComhairleState>,
        user_id: Uuid,
    ) -> Result<bool, ComhairleError> {
        match users::get_user_by_id(&user_id, &state.db).await {
            Ok(user) => Ok(is_user_admin(state, &user).await),
            Err(ComhairleError::ResourceNotFound(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }

    async fn ensure_audio_recording_access(
        state: &Arc<ComhairleState>,
        event_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ComhairleError> {
        if let Ok(user) = users::get_user_by_id(&user_id, &state.db).await
            && is_user_admin(state, &user).await
        {
            return Ok(());
        }

        match event_attendance::get_by_event_and_user(&state.db, &event_id, &user_id).await {
            Ok(_) => Ok(()),
            Err(ComhairleError::ResourceNotFound(_)) => Err(ComhairleError::UserNotAuthorized),
            Err(err) => Err(err),
        }
    }

    async fn send_custom_result<T: Serialize>(
        connection: &WebSocketConnection,
        event: &str,
        payload: WsResultPayload<T>,
    ) -> Result<(), WebsocketError> {
        let data = serde_json::to_value(payload)
            .map_err(|e| WebsocketError::SerializationError(e.to_string()))?;
        connection
            .send_message(&WebSocketMessage::Custom {
                event: event.to_string(),
                data,
            })
            .await
            .map_err(|e| WebsocketError::SendError(e.to_string()))
    }

    async fn send_error_result(
        connection: &WebSocketConnection,
        event: &str,
        request_id: Option<String>,
        error: String,
    ) -> Result<(), WebsocketError> {
        let request_id = request_id.unwrap_or_else(|| "unknown-request".to_string());
        Self::send_custom_result(
            connection,
            event,
            WsResultPayload::<serde_json::Value> {
                request_id,
                success: false,
                data: None,
                error: Some(error),
            },
        )
        .await
    }

    /// Handles the acquire message and associates the live recording ID with the connection.
    async fn handle_acquire(
        &self,
        request: WsAcquireRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<(), ComhairleError> = async {
            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;

            let _ = live_audio_recording::get(&state.db, request.live_recording_id).await?;

            self.acquire_connection_lock(
                &connection.id,
                LiveConnectionInfo {
                    event_id: request.event_id,
                    live_recording_id: request.live_recording_id,
                },
            )?;
            Ok(())
        }
        .await;

        match result {
            Ok(_) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:acquire_result",
                    WsResultPayload::<serde_json::Value> {
                        request_id: request.request_id,
                        success: true,
                        data: None,
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:acquire_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_presign_part(
        &self,
        request: WsPresignPartRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result = async {
            if self
                .live_recording_id_map
                .lock()
                .unwrap()
                .get(&connection.id)
                .is_none()
            {
                return Err(ComhairleError::ResourceNotFound(
                    "No live recording associated with this connection".to_string(),
                ));
            }

            let bulk_storage_service = state.required_bulk_storage_service()?;
            let live_recording_id = self
                .get_live_connection_info(&connection.id)?
                .live_recording_id;

            let live_recording = live_audio_recording::get(&state.db, live_recording_id).await?;

            if request.part_number != live_recording.next_part_number {
                return Err(ComhairleError::CorruptedData(format!(
                    "Expected part_number {}, got {}",
                    live_recording.next_part_number, request.part_number
                )));
            }

            let recording =
                audio_recording::get_by_id(&state.db, live_recording.audio_recording_id).await?;
            let extension = recording.file_extension.extension();
            let recording_path = format!("{}/recording.{}", recording.s3_key_prefix, extension);

            let upload_url = bulk_storage_service
                .get_multipart_file_write_url(
                    &recording_path,
                    &StorageUploadID(live_recording.multipart_upload_id),
                    MultipartUploadPartNumber(request.part_number),
                )
                .await?;

            Ok(PresignLiveAudioRecordingPartResponse {
                upload_url,
                part_number: request.part_number,
            })
        }
        .await;

        match result {
            Ok(response) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:presign_part_result",
                    WsResultPayload {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:presign_part_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_ack_part(
        &self,
        request: WsAckPartRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<AckLiveAudioRecordingPartResponse, ComhairleError> = async {
            let live_connection_info = self.get_live_connection_info(&connection.id)?;

            let updated = live_audio_recording::append_uploaded_part(
                &state.db,
                live_connection_info.live_recording_id,
                live_audio_recording::UploadedPart {
                    part_number: request.part_number,
                    etag: request.etag,
                    size_bytes: request.size_bytes,
                },
                request.part_number,
            )
            .await?;

            Ok(AckLiveAudioRecordingPartResponse {
                live_audio_recording: LiveAudioRecordingDto::from(updated),
            })
        }
        .await;

        match result {
            Ok(response) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:ack_part_result",
                    WsResultPayload {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:ack_part_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_release(
        &self,
        request: WsReleaseRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<(), ComhairleError> = async {
            let live_connection_info = self.get_live_connection_info(&connection.id)?;

            Self::ensure_audio_recording_access(
                state,
                live_connection_info.event_id,
                connection.user.id,
            )
            .await?;

            let _ = self.release_connection_lock(&connection.id)?;

            Ok(())
        }
        .await;

        match result {
            Ok(_) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:release_result",
                    WsResultPayload::<serde_json::Value> {
                        request_id: request.request_id,
                        success: true,
                        data: None,
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:release_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_disconnect_session(
        &self,
        request: WsDisconnectSessionsRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<WsDisconnectSessionResponse, ComhairleError> = async {
            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;

            // Pick one existing session for the same user, event and recording.
            let existing_session: Option<ConnectionId> = {
                let live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_id_map is poisoned: {}",
                        err
                    )))
                })?;

                live_recording_id_map.iter().find_map(|(conn_id, info)| {
                    if *conn_id == connection.id
                        || info.live_recording_id != request.live_recording_id
                    {
                        return None;
                    }

                    Some(conn_id.clone())
                })
            };

            let mut disconnected_sessions = 0;
            if let Some(conn_id) = existing_session {
                // Send a disconnect message to the other session.
                let disconnect_message = WebSocketMessage::Custom {
                    event: "audio_recording:disconnect".to_string(),
                    data: json!({
                        "eventId": request.event_id,
                        "liveRecordingId": request.live_recording_id,
                        "reason": "session_replaced"
                    }),
                };

                state
                    .websockets
                    .send_to_connections(&[conn_id.clone()], &disconnect_message)
                    .await?;

                // Remove the session from the live_recording_id_map.
                let _ = self.release_connection_lock(&conn_id)?;
                disconnected_sessions = 1;
            }

            Ok(WsDisconnectSessionResponse {
                disconnected_sessions,
            })
        }
        .await;

        match result {
            Ok(response) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:disconnect_sessions_result",
                    WsResultPayload {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:disconnect_sessions_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_complete(
        &self,
        request: WsCompleteRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<ProcessRecordingResponse, ComhairleError> = async {
            let live_connection_info = self.get_live_connection_info(&connection.id)?;
            if live_connection_info.event_id != request.event_id
                || live_connection_info.live_recording_id != request.live_recording_id
            {
                return Err(ComhairleError::UserNotAuthorized);
            }

            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;

            let bulk_storage_service = state.required_bulk_storage_service()?;
            let worker_service = state.required_worker_service()?;

            let live_recording =
                live_audio_recording::get(&state.db, request.live_recording_id).await?;
            let recording =
                audio_recording::get_by_id(&state.db, live_recording.audio_recording_id).await?;
            let extension = recording.file_extension.extension();
            let recording_path = format!("{}/recording.{}", recording.s3_key_prefix, extension);

            let mut sorted_uploaded_parts = live_recording.uploaded_parts.clone();
            sorted_uploaded_parts.sort_by_key(|part| part.part_number);
            let parts: Vec<(MultipartUploadPartNumber, StorageEntityTag)> = sorted_uploaded_parts
                .iter()
                .map(|p| {
                    (
                        MultipartUploadPartNumber(p.part_number),
                        StorageEntityTag(p.etag.clone()),
                    )
                })
                .collect();

            bulk_storage_service
                .complete_multipart_upload(
                    &recording_path,
                    &StorageUploadID(live_recording.multipart_upload_id),
                    &parts,
                )
                .await?;

            live_audio_recording::delete(&state.db, request.live_recording_id).await?;

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
                    event_id: request.event_id,
                    conversation_id: request.conversation_id,
                    recording_id: recording.id,
                    job_id: job.id,
                })
                .await?;

            audio_recording::update_status(
                &state.db,
                recording.id,
                audio_recording::AudioRecordingStatus::Transcribing,
            )
            .await?;

            let _ = self.release_connection_lock(&connection.id)?;

            Ok(ProcessRecordingResponse {
                message: "Live audio recording processing moved to a background job".to_string(),
                job_id: job.id,
            })
        }
        .await;

        match result {
            Ok(response) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:complete_result",
                    WsResultPayload {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:complete_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }

    async fn handle_delete(
        &self,
        request: WsDeleteRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result: Result<LiveAudioRecordingStateResponse, ComhairleError> = async {
            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;
            let is_admin = Self::is_admin_user(state, connection.user.id).await?;

            let current_owner_connection = {
                let live_recording_lock_map = self.live_recording_lock_map.lock().or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_lock_map is poisoned: {}",
                        err
                    )))
                })?;
                live_recording_lock_map
                    .get(&request.live_recording_id)
                    .cloned()
            };

            if let Some(owner_connection_id) = current_owner_connection
                && owner_connection_id != connection.id
            {
                if !is_admin {
                    return Err(ComhairleError::LiveAudioRecordingLocked);
                }

                let disconnect_message = WebSocketMessage::Custom {
                    event: "audio_recording:disconnect".to_string(),
                    data: json!({
                        "eventId": request.event_id,
                        "liveRecordingId": request.live_recording_id,
                        "reason": "admin_deleted"
                    }),
                };
                let _ = state
                    .websockets
                    .send_to_connections(&[owner_connection_id.clone()], &disconnect_message)
                    .await;
                let _ = self.release_connection_lock(&owner_connection_id)?;
            }

            let bulk_storage_service = state.required_bulk_storage_service()?;
            let live_recording = live_audio_recording::get(&state.db, request.live_recording_id).await?;
            let recording = audio_recording::get_by_id(&state.db, live_recording.audio_recording_id).await?;
            let extension = recording.file_extension.extension();
            let recording_path = format!("{}/recording.{}", recording.s3_key_prefix, extension);

            if let Err(err) = bulk_storage_service
                .abort_multipart_upload(
                    &recording_path,
                    &StorageUploadID(live_recording.multipart_upload_id.clone()),
                )
                .await
            {
                tracing::warn!(?err, live_recording_id = %request.live_recording_id, "failed to abort multipart upload");
            }

            live_audio_recording::delete(&state.db, request.live_recording_id).await?;

            if recording.status == audio_recording::AudioRecordingStatus::AwaitingUpload {
                let _ = audio_recording::delete(&state.db, recording.id, request.event_id).await?;
            }

            {
                let owner_connection_ids = {
                    let live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                        Err(ComhairleError::CorruptedData(format!(
                            "live_recording_id_map is poisoned: {}",
                            err
                        )))
                    })?;
                    live_recording_id_map
                        .iter()
                        .filter_map(|(conn_id, info)| {
                            if info.live_recording_id == request.live_recording_id {
                                Some(conn_id.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                };

                for owner_connection_id in owner_connection_ids {
                    let _ = self.release_connection_lock(&owner_connection_id)?;
                }
            }

            Ok(LiveAudioRecordingStateResponse {
                live_audio_recording: live_recording.into(),
            })
        }
        .await;

        match result {
            Ok(response) => {
                Self::send_custom_result(
                    connection,
                    "audio_recording:delete_result",
                    WsResultPayload {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                    },
                )
                .await
            }
            Err(err) => {
                Self::send_error_result(
                    connection,
                    "audio_recording:delete_result",
                    Some(request.request_id),
                    err.to_string(),
                )
                .await
            }
        }
    }
}

impl Default for AudioRecordingMessageHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WebSocketMessageHandler for AudioRecordingMessageHandler {
    fn domain(&self) -> &str {
        "audio_recording"
    }

    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let WebSocketMessage::Custom { event, data } = message else {
            return Ok(());
        };

        match event.as_str() {
            "audio_recording:acquire" => {
                let request = match serde_json::from_value::<WsAcquireRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:acquire_result",
                            request_id,
                            format!("Invalid acquire request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_acquire(request, connection, state).await?;
            }
            "audio_recording:presign_part" => {
                let request = match serde_json::from_value::<WsPresignPartRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:presign_part_result",
                            request_id,
                            format!("Invalid presign request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_presign_part(request, connection, state).await?;
            }
            "audio_recording:ack_part" => {
                let request = match serde_json::from_value::<WsAckPartRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:ack_part_result",
                            request_id,
                            format!("Invalid ack request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_ack_part(request, connection, state).await?;
            }
            "audio_recording:release" => {
                let request = match serde_json::from_value::<WsReleaseRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:release_result",
                            request_id,
                            format!("Invalid release request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_release(request, connection, state).await?;
            }
            "audio_recording:disconnect_sessions" => {
                let request =
                    match serde_json::from_value::<WsDisconnectSessionsRequest>(data.clone()) {
                        Ok(request) => request,
                        Err(err) => {
                            let request_id = data
                                .get("requestId")
                                .and_then(|value| value.as_str())
                                .map(ToString::to_string);
                            Self::send_error_result(
                                connection,
                                "audio_recording:disconnect_sessions_result",
                                request_id,
                                format!("Invalid disconnect sessions request payload: {err}"),
                            )
                            .await?;
                            return Ok(());
                        }
                    };

                self.handle_disconnect_session(request, connection, state)
                    .await?;
            }
            "audio_recording:complete" => {
                let request = match serde_json::from_value::<WsCompleteRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:complete_result",
                            request_id,
                            format!("Invalid complete request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_complete(request, connection, state).await?;
            }
            "audio_recording:delete" => {
                let request = match serde_json::from_value::<WsDeleteRequest>(data.clone()) {
                    Ok(request) => request,
                    Err(err) => {
                        let request_id = data
                            .get("requestId")
                            .and_then(|value| value.as_str())
                            .map(ToString::to_string);
                        Self::send_error_result(
                            connection,
                            "audio_recording:delete_result",
                            request_id,
                            format!("Invalid delete request payload: {err}"),
                        )
                        .await?;
                        return Ok(());
                    }
                };

                self.handle_delete(request, connection, state).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn on_connected(
        &self,
        connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        tracing::info!(
            connection_id = ?connection.id,
            user_id = %connection.user.id,
            "audio recording websocket connected"
        );
        Ok(())
    }

    async fn on_disconnected(
        &self,
        connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let _ = self
            .release_connection_lock(&connection.id)
            .map_err(|e| WebsocketError::AudioRecordingError(e.to_string()))?;

        tracing::info!(
            connection_id = ?connection.id,
            user_id = %connection.user.id,
            "audio recording websocket disconnected"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use axum::extract::ws::Message;
    use chrono::{Duration, Utc};
    use tokio::sync::mpsc::UnboundedReceiver;
    use tokio::time::{Duration as TokioDuration, timeout};

    use crate::config::ComhairleConfig;
    use crate::models::{audio_recording::CreateAudioRecording, conversation, event};
    use crate::routes::auth::SignupRequest;
    use crate::test_helpers::{TEST_PASSWORD, test_config, test_state};
    use crate::websockets::{ComhairleWebSocketService, WebSocketService};

    struct TestContext {
        state: Arc<ComhairleState>,
        websockets: Arc<ComhairleWebSocketService>,
        user: users::User,
        event_id: Uuid,
        live_recording_id: Uuid,
    }

    async fn build_test_context(
        pool: sqlx::PgPool,
    ) -> Result<TestContext, Box<dyn std::error::Error>> {
        let mut config: ComhairleConfig = test_config()?;
        config.bot_service = None;

        let websockets = Arc::new(ComhairleWebSocketService::new(None).await?);
        let websocket_service: Arc<dyn WebSocketService> = websockets.clone();
        let state = Arc::new(
            test_state()
                .db(pool.clone())
                .config(config.clone())
                .websockets(websocket_service)
                .call()?,
        );

        let user = users::create_user(
            &SignupRequest {
                username: "admin".to_string(),
                password: TEST_PASSWORD.to_string(),
                avatar_url: None,
                email: "admin@crown-shy.com".to_string(),
            },
            &pool,
        )
        .await?;

        let conversation = conversation::create(
            &pool,
            &None,
            &config,
            &conversation::CreateConversation {
                title: "Test Conversation".to_string(),
                short_description: "Short description".to_string(),
                description: "Description".to_string(),
                video_url: None,
                image: None,
                tags: None,
                is_public: false,
                is_live: true,
                is_invite_only: false,
                slug: None,
                default_workflow_id: None,
                primary_locale: "en".to_string(),
                supported_languages: vec!["en".to_string()],
                enable_qa_chat_bot: None,
            },
            user.id,
            None,
        )
        .await?;

        let event = event::create(
            &pool,
            &conversation.id,
            &event::CreateEvent {
                name: "Test Event".to_string(),
                description: "Event description".to_string(),
                capacity: None,
                start_time: Utc::now(),
                end_time: Utc::now() + Duration::hours(1),
                signup_mode: "open".to_string(),
                agenda: None,
                location: None,
                default_time_zone: Some("UTC".to_string()),
            },
        )
        .await?;

        let recording = audio_recording::create(
            &pool,
            &CreateAudioRecording {
                id: Uuid::new_v4(),
                event_id: event.id,
                name: "Main Room".to_string(),
                s3_key_prefix: format!("events/{}/recordings/{}", event.id, Uuid::new_v4()),
                file_extension: audio_recording::AudioFormat::Webm,
            },
        )
        .await?;

        let live_recording = live_audio_recording::create(
            &pool,
            &live_audio_recording::CreateLiveAudioRecording {
                audio_recording_id: recording.id,
                multipart_upload_id: "multipart-upload-id".to_string(),
                owner_id: Some(user.id),
            },
        )
        .await?;

        Ok(TestContext {
            state,
            websockets,
            user,
            event_id: event.id,
            live_recording_id: live_recording.id,
        })
    }

    fn parse_ws_message(message: Message) -> WebSocketMessage {
        serde_json::from_str(message.to_text().expect("websocket message should be text"))
            .expect("websocket message should deserialize")
    }

    async fn next_ws_message(receiver: &mut UnboundedReceiver<Message>) -> WebSocketMessage {
        let message = timeout(TokioDuration::from_millis(200), receiver.recv())
            .await
            .expect("expected websocket message within timeout")
            .expect("websocket channel should stay open");
        parse_ws_message(message)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_disconnect_session_clears_stale_lock_without_websocket_session(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context = build_test_context(pool.clone()).await?;

        let handler = AudioRecordingMessageHandler::new();
        let (connection, mut receiver) =
            WebSocketConnection::new(context.user.clone(), "127.0.0.1:4101".parse().unwrap());

        handler
            .handle_disconnect_session(
                WsDisconnectSessionsRequest {
                    request_id: "req-stale-lock".to_string(),
                    event_id: context.event_id,
                    live_recording_id: context.live_recording_id,
                },
                &connection,
                &context.state,
            )
            .await?;

        let response_message = next_ws_message(&mut receiver).await;
        let WebSocketMessage::Custom { event, data } = response_message else {
            panic!("expected custom websocket response");
        };
        assert_eq!(event, "audio_recording:disconnect_sessions_result");

        let payload: WsResultPayload<WsDisconnectSessionResponse> = serde_json::from_value(data)?;
        assert!(payload.success);
        assert_eq!(
            payload
                .data
                .expect("disconnect response data")
                .disconnected_sessions,
            0
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_disconnect_session_replaces_existing_tab_and_allows_reacquire(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context = build_test_context(pool.clone()).await?;

        let handler = AudioRecordingMessageHandler::new();
        let (old_connection, mut old_receiver) =
            WebSocketConnection::new(context.user.clone(), "127.0.0.1:4102".parse().unwrap());
        let (new_connection, mut new_receiver) =
            WebSocketConnection::new(context.user.clone(), "127.0.0.1:4103".parse().unwrap());

        context.websockets.add_connection(old_connection.clone());
        context.websockets.add_connection(new_connection.clone());
        handler.live_recording_id_map.lock().unwrap().insert(
            old_connection.id.clone(),
            LiveConnectionInfo {
                event_id: context.event_id,
                live_recording_id: context.live_recording_id,
            },
        );
        handler
            .live_recording_lock_map
            .lock()
            .unwrap()
            .insert(context.live_recording_id, old_connection.id.clone());

        handler
            .handle_disconnect_session(
                WsDisconnectSessionsRequest {
                    request_id: "req-takeover".to_string(),
                    event_id: context.event_id,
                    live_recording_id: context.live_recording_id,
                },
                &new_connection,
                &context.state,
            )
            .await?;

        let disconnect_message = next_ws_message(&mut old_receiver).await;
        let WebSocketMessage::Custom { event, data } = disconnect_message else {
            panic!("expected custom disconnect websocket message");
        };
        assert_eq!(event, "audio_recording:disconnect");
        assert_eq!(data["eventId"], serde_json::json!(context.event_id));
        assert_eq!(
            data["liveRecordingId"],
            serde_json::json!(context.live_recording_id)
        );
        assert_eq!(data["reason"], serde_json::json!("session_replaced"));

        let response_message = next_ws_message(&mut new_receiver).await;
        let WebSocketMessage::Custom { event, data } = response_message else {
            panic!("expected custom disconnect result message");
        };
        assert_eq!(event, "audio_recording:disconnect_sessions_result");
        let payload: WsResultPayload<WsDisconnectSessionResponse> = serde_json::from_value(data)?;
        assert!(payload.success);
        assert_eq!(
            payload
                .data
                .expect("disconnect response data")
                .disconnected_sessions,
            1
        );

        assert!(
            handler
                .live_recording_id_map
                .lock()
                .unwrap()
                .get(&old_connection.id)
                .is_none()
        );
        assert!(
            handler
                .live_recording_lock_map
                .lock()
                .unwrap()
                .get(&context.live_recording_id)
                .is_none()
        );

        handler
            .handle_acquire(
                WsAcquireRequest {
                    request_id: "req-reacquire".to_string(),
                    event_id: context.event_id,
                    live_recording_id: context.live_recording_id,
                },
                &new_connection,
                &context.state,
            )
            .await?;

        let acquire_message = next_ws_message(&mut new_receiver).await;
        let WebSocketMessage::Custom { event, data } = acquire_message else {
            panic!("expected custom acquire result message");
        };
        assert_eq!(event, "audio_recording:acquire_result");
        let payload: WsResultPayload<serde_json::Value> = serde_json::from_value(data)?;
        assert!(payload.success);

        assert_eq!(
            handler
                .live_recording_id_map
                .lock()
                .unwrap()
                .get(&new_connection.id)
                .expect("new connection should own live recording")
                .live_recording_id,
            context.live_recording_id
        );
        assert!(
            handler
                .live_recording_lock_map
                .lock()
                .unwrap()
                .get(&context.live_recording_id)
                .is_some_and(|owner| owner == &new_connection.id)
        );

        Ok(())
    }
}
