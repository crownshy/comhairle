//! Failures about events, attendance, and event recordings.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Event at max capacity")]
    EventAtCapacity,

    #[error("Event has past")]
    EventHasPast,

    #[error("Event has incorrect signup mode for this action")]
    EventInvalidSignupMode,

    #[error("User is already registered for event: {0}")]
    UserAlreadyRegisteredForEvent(String),

    #[error("A recording named {0} already exists for this event")]
    DuplicateRecordingName(String),

    #[error("Event missing video_meeting_id")]
    NoVideoMeetingId,

    /// A persistence failure hit while serving an event request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for EventError {
    fn kind(&self) -> ErrorKind {
        match self {
            EventError::Data(err) => err.kind(),
            EventError::EventAtCapacity
            | EventError::UserAlreadyRegisteredForEvent(_)
            | EventError::DuplicateRecordingName(_) => ErrorKind::Conflict,
            EventError::EventHasPast => ErrorKind::Unprocessable,
            // Preserved oddity: on the flat enum this variant had no status arm
            // and fell into the `_ => 500` catch-all. Reads like Unprocessable;
            // worth a separate decision.
            EventError::EventInvalidSignupMode => ErrorKind::Internal,
            EventError::NoVideoMeetingId => ErrorKind::Internal,
        }
    }
}
