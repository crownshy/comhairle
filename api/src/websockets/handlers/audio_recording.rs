use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use crate::{
    ComhairleState,
    bulk_storage_service::{MultipartUploadPartNumber, StorageUploadID},
    error::ComhairleError,
    models::{audio_recording, event, event_attendance, live_audio_recording, users},
    routes::{
        audio_recordings::dto::{
            AckLiveAudioRecordingPartResponse, LiveAudioRecordingDto,
            PresignLiveAudioRecordingPartResponse,
        },
        auth::is_user_admin,
    },
    websockets::{
        WebSocketConnection, WebSocketMessageHandler, error::WebsocketError,
        messages::WebSocketMessage,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsPresignPartRequest {
    request_id: String,
    conversation_id: Uuid,
    event_id: Uuid,
    live_recording_id: Uuid,
    part_number: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsAckPartRequest {
    request_id: String,
    conversation_id: Uuid,
    event_id: Uuid,
    live_recording_id: Uuid,
    part_number: i32,
    etag: String,
    size_bytes: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WsResultPayload<T: Serialize> {
    request_id: String,
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Websocket handler for live audio recording domain.
///
/// This currently provides connect and disconnect debug hooks only.
pub struct AudioRecordingMessageHandler;

impl AudioRecordingMessageHandler {
    pub fn new() -> Self {
        Self
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
            .map_err(|err| WebsocketError::SerializationError(err.to_string()))?;
        connection
            .send_message(&WebSocketMessage::Custom {
                event: event.to_string(),
                data,
            })
            .await
            .map_err(|err| WebsocketError::SendError(err.to_string()))
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

    async fn handle_presign_part(
        request: WsPresignPartRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result = async {
            let event_record = event::get_by_id(&state.db, &request.event_id).await?;
            if event_record.conversation_id != request.conversation_id {
                return Err(ComhairleError::ResourceNotFound(format!(
                    "No event {} found for conversation {}",
                    request.event_id, request.conversation_id
                )));
            }

            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;

            let bulk_storage_service = state.required_bulk_storage_service()?;
            let live_recording = live_audio_recording::lock_for_user(
                &state.db,
                request.live_recording_id,
                connection.user.id,
            )
            .await?;

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

            Ok::<PresignLiveAudioRecordingPartResponse, ComhairleError>(
                PresignLiveAudioRecordingPartResponse {
                    upload_url,
                    part_number: request.part_number,
                },
            )
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
        request: WsAckPartRequest,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
        let result = async {
            let event_record = event::get_by_id(&state.db, &request.event_id).await?;
            if event_record.conversation_id != request.conversation_id {
                return Err(ComhairleError::ResourceNotFound(format!(
                    "No event {} found for conversation {}",
                    request.event_id, request.conversation_id
                )));
            }

            Self::ensure_audio_recording_access(state, request.event_id, connection.user.id)
                .await?;

            let _ = live_audio_recording::lock_for_user(
                &state.db,
                request.live_recording_id,
                connection.user.id,
            )
            .await?;

            let updated = live_audio_recording::append_uploaded_part(
                &state.db,
                request.live_recording_id,
                live_audio_recording::UploadedPart {
                    part_number: request.part_number,
                    etag: request.etag,
                    size_bytes: request.size_bytes,
                },
                request.part_number,
            )
            .await?;

            Ok::<AckLiveAudioRecordingPartResponse, ComhairleError>(
                AckLiveAudioRecordingPartResponse {
                    live_audio_recording: LiveAudioRecordingDto::from(updated),
                },
            )
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

                Self::handle_presign_part(request, connection, state).await?;
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

                Self::handle_ack_part(request, connection, state).await?;
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
        debug!(
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
        debug!(
            connection_id = ?connection.id,
            user_id = %connection.user.id,
            "audio recording websocket disconnected"
        );
        Ok(())
    }
}
