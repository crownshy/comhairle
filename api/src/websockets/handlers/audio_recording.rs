use crate::websockets::ConnectionId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use crate::ComhairleState;
use crate::bulk_storage_service::{MultipartUploadPartNumber, StorageUploadID};
use crate::error::ComhairleError;
use crate::models::{audio_recording, event_attendance, live_audio_recording, users};
use crate::routes::audio_recordings::dto::{
    AckLiveAudioRecordingPartResponse, LiveAudioRecordingDto, PresignLiveAudioRecordingPartResponse,
};
use crate::routes::auth::is_user_admin;
use crate::websockets::error::WebsocketError;
use crate::websockets::messages::WebSocketMessage;
use crate::websockets::{WebSocketConnection, WebSocketMessageHandler};

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

#[derive(Debug, Serialize)]
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
}

impl AudioRecordingMessageHandler {
    pub fn new() -> Self {
        Self {
            live_recording_id_map: Arc::new(Mutex::new(HashMap::new())),
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

            {
                let live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_id_map is poisoned: {}",
                        err
                    )))
                })?;
                if live_recording_id_map.get(&connection.id).is_some() {
                    return Err(ComhairleError::Conflict(
                        "Connection already has an active recording session".to_string(),
                    ));
                }
            }

            // Ensure the recording belongs to the event from the acquire request.
            let _ = live_audio_recording::get_by_id_and_event(
                &state.db,
                request.live_recording_id,
                request.event_id,
            )
            .await?;

            // Acquire the lock for this recording session.
            let _ = live_audio_recording::lock_for_user(
                &state.db,
                request.live_recording_id,
                connection.user.id,
            )
            .await?;

            let mut live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                Err(ComhairleError::CorruptedData(format!(
                    "live_recording_id_map is poisoned: {}",
                    err
                )))
            })?;
            live_recording_id_map.insert(
                connection.id.clone(),
                LiveConnectionInfo {
                    event_id: request.event_id,
                    live_recording_id: request.live_recording_id,
                },
            );
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

            let live_recording_id = match self
                .live_recording_id_map
                .lock()
                .or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_id_map is poisoned: {}",
                        err
                    )))
                })?
                .get(&connection.id)
            {
                Some(info) => info.live_recording_id,
                None => {
                    return Err(ComhairleError::ResourceNotFound(
                        "No live recording associated with this connection".to_string(),
                    ));
                }
            };

            let live_recording =
                live_audio_recording::get_by_id(&state.db, live_recording_id).await?;

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

            let live_connection_info = match self
                .live_recording_id_map
                .lock()
                .unwrap()
                .get(&connection.id)
            {
                Some(info) => *info,
                None => {
                    return Err(ComhairleError::ResourceNotFound(
                        "No live recording associated with this connection".to_string(),
                    ));
                }
            };

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
        let result = async {
            let live_connection_info = {
                let live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_id_map is poisoned: {}",
                        err
                    )))
                })?;
                match live_recording_id_map.get(&connection.id) {
                    Some(info) => *info,
                    None => {
                        return Err(ComhairleError::ResourceNotFound(
                            "No live recording associated with this connection".to_string(),
                        ));
                    }
                }
            };

            Self::ensure_audio_recording_access(
                state,
                live_connection_info.event_id,
                connection.user.id,
            )
            .await?;

            live_audio_recording::unlock_for_user(
                &state.db,
                live_connection_info.live_recording_id,
                connection.user.id,
            )
            .await?;

            let mut live_recording_id_map = self.live_recording_id_map.lock().or_else(|err| {
                Err(ComhairleError::CorruptedData(format!(
                    "live_recording_id_map is poisoned: {}",
                    err
                )))
            })?;
            live_recording_id_map.remove(&connection.id);

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
        let live_connection_info = {
            let mut live_recording_id_map = self
                .live_recording_id_map
                .lock()
                .or_else(|err| {
                    Err(ComhairleError::CorruptedData(format!(
                        "live_recording_id_map is poisoned: {}",
                        err
                    )))
                })
                .map_err(|err| WebsocketError::AudioRecordingError(err.to_string()))?;
            live_recording_id_map.remove(&connection.id)
        };

        if let Some(live_connection_info) = live_connection_info {
            // We can safely ignore the result of unlock_for_user here, since
            // we expect the user to have already released the lock before
            // disconnecting in most cases.
            let _ = live_audio_recording::unlock_for_user(
                &_state.db,
                live_connection_info.live_recording_id,
                connection.user.id,
            )
            .await;
        }

        debug!(
            connection_id = ?connection.id,
            user_id = %connection.user.id,
            "audio recording websocket disconnected"
        );
        Ok(())
    }
}
