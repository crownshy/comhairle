//! The moderation state of a statement.
//!
//! Lives in the model layer because it is persisted (it is the
//! `polis_statement_aux.moderation_status` column) and because the model layer
//! owns the `sea_query::Value` conversion. Keeping it here avoids an orphan-rule
//! problem: that impl has two foreign types in it, so it can only be written in
//! the crate that defines this enum.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::error::ValidationError;

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone, JsonSchema, Default)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum ModerationStatus {
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "rejected")]
    Rejected,
    #[sqlx(rename = "pending")]
    #[default]
    Pending,
}

impl std::fmt::Display for ModerationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ModerationStatus::Accepted => "accepted",
            ModerationStatus::Rejected => "rejected",
            ModerationStatus::Pending => "pending",
        };
        write!(f, "{}", value)
    }
}

impl From<ModerationStatus> for sea_query::Value {
    fn from(val: ModerationStatus) -> Self {
        val.to_string().into()
    }
}

/// Polis encodes moderation as -1 / 0 / 1 on the wire.
impl TryFrom<i32> for ModerationStatus {
    type Error = ValidationError;

    fn try_from(value: i32) -> Result<Self, ValidationError> {
        match value {
            -1 => Ok(ModerationStatus::Rejected),
            1 => Ok(ModerationStatus::Accepted),
            0 => Ok(ModerationStatus::Pending),
            other => Err(ValidationError::BadRequest(format!(
                "Unknown moderation status: {other}"
            ))),
        }
    }
}

impl ModerationStatus {
    /// The numeric moderation value Polis stores (`mod` column).
    pub fn mod_value(self) -> i32 {
        match self {
            ModerationStatus::Accepted => 1,
            ModerationStatus::Rejected => -1,
            ModerationStatus::Pending => 0,
        }
    }

    /// Whether the statement is live for participants.
    pub fn active(self) -> bool {
        matches!(self, ModerationStatus::Accepted)
    }
}
