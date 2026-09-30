//! Failures about conversations and participation in them.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum ConversationError {
    #[error("Conversation not live")]
    ConversationNotLive,

    #[error("Conversation already live")]
    ConversationAlreadyLive,

    #[error("Slug {0} already taken")]
    DuplicateSlug(String),

    #[error("User is not signed up to participate in the conversation")]
    UserIsNotParticipatingInTheConversation,

    #[error("No bot_id was found for this conversation")]
    NoConversationBotId,

    #[error("No chat session was found for this bot on this conversation")]
    NoBotUserSession,

    /// A persistence failure hit while serving a conversation request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for ConversationError {
    fn kind(&self) -> ErrorKind {
        match self {
            ConversationError::Data(err) => err.kind(),
            ConversationError::ConversationAlreadyLive | ConversationError::DuplicateSlug(_) => {
                ErrorKind::Conflict
            }
            ConversationError::ConversationNotLive => ErrorKind::Unprocessable,
            ConversationError::UserIsNotParticipatingInTheConversation
            | ConversationError::NoConversationBotId
            | ConversationError::NoBotUserSession => ErrorKind::Internal,
        }
    }
}
