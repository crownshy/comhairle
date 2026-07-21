use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use crate::{
    ComhairleState,
    websockets::{
        WebSocketConnection, WebSocketMessageHandler, error::WebsocketError,
        messages::WebSocketMessage,
    },
};

/// Websocket handler for live audio recording domain.
///
/// This currently provides connect and disconnect debug hooks only.
pub struct AudioRecordingMessageHandler;

impl AudioRecordingMessageHandler {
    pub fn new() -> Self {
        Self
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
        _message: &WebSocketMessage,
        _connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), WebsocketError> {
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
