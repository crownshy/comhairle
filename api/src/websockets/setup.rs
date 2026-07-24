use std::sync::Arc;
use tracing::info;

use crate::ComhairleState;

use super::handlers::{
    audio_recording::AudioRecordingMessageHandler, notifications::NotificationMessageHandler,
    video_call::VideoCallMessageHandler, workflow::WorkflowMessageHandler,
};

/// Register all WebSocket message handlers with the application state.
///
/// This function initializes and registers all domain-specific WebSocket message handlers.
/// It should be called once during application startup, after the state is created but
/// before the server starts accepting connections.
///
/// # Registered Handlers
///
/// - **NotificationMessageHandler**: Handles real-time notifications
///   - Domain: `"notification"`
///   - Events: `notification:new`, `notification:mark_read`, etc.
///
/// - **WorkflowMessageHandler**: Handles workflow progress tracking
///   - Domain: `"workflow"`
///   - Events: `user_started_workflow_step`, `user_finished_workflow_step`, etc.
///
/// - **VideoCallMessageHandler**: Handles video call state and interactions
///   - Domain: `"video_call"`
///   - Events: `video_call:user_joined`, `video_call:user_left`, `video_call:change_state`, etc.
///
/// # Adding New Handlers
///
/// To add a new handler:
///
/// 1. Create a handler struct implementing `WebSocketMessageHandler`
/// 2. Import it in this module
/// 3. Register it in this function:
///
/// ```rust,ignore
/// // Example (assuming you have a MyHandler implementation):
/// let my_handler = Arc::new(MyHandler::new());
/// state.websockets.register_handler(my_handler);
/// ```
///
/// # Example
///
/// ```rust,ignore
/// // In main.rs, after creating the application state:
/// let state = Arc::new(ComhairleState { /* ... */ });
///
/// // Register all WebSocket handlers
/// comhairle::websockets::setup::register_handlers(&state);
///
/// // Now start the server
/// let app = setup_server(state.clone()).await?;
/// ```
///
/// # Parameters
///
/// - `state`: The application state containing the WebSocket service
pub fn register_handlers(state: &Arc<ComhairleState>) {
    info!("Registering WebSocket message handlers");

    // Register notification handler
    let notification_handler = Arc::new(NotificationMessageHandler::new());
    state.websockets.register_handler(notification_handler);

    // Register workflow handler
    let workflow_handler = Arc::new(WorkflowMessageHandler::new());
    state.websockets.register_handler(workflow_handler);

    // Register video call handler
    let video_call_handler = Arc::new(VideoCallMessageHandler::new());
    state.websockets.register_handler(video_call_handler);

    // Register audio recording handler
    let audio_recording_handler = Arc::new(AudioRecordingMessageHandler::new());
    state.websockets.register_handler(audio_recording_handler);

    info!("WebSocket message handlers registered successfully");
}
