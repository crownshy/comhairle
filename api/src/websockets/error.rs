use thiserror::Error;

use crate::websockets::handlers::{
    notifications::NotificationWSError, video_call::VideoCallWSError, workflow::WorkflowWSError,
};

#[derive(Error, Debug)]
pub enum WebsocketError {
    #[error("Video Call Error: {0}")]
    VideoCallError(#[from] VideoCallWSError),

    #[error("Notification Error: {0}")]
    NotificationError(#[from] NotificationWSError),

    #[error("Workflow Error: {0}")]
    WorkflowError(#[from] WorkflowWSError),

    #[error("WebSocket send error: {0}")]
    SendError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}
