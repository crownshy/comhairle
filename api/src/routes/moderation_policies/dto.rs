use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::moderation_policy::{ModerationPolicyReason, ModerationPolicyWithReasons};

/// Data transfer object (public API representation) for a moderation policy reason.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModerationPolicyReasonDto {
    pub id: Uuid,
    pub label: String,
    pub description: Option<String>,
    pub position: i32,
}

impl From<ModerationPolicyReason> for ModerationPolicyReasonDto {
    fn from(reason: ModerationPolicyReason) -> Self {
        Self {
            id: reason.id,
            label: reason.label,
            description: reason.description,
            position: reason.position,
        }
    }
}

/// Data transfer object (public API representation) for a moderation policy and its
/// reasons in display order.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModerationPolicyDto {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub name: String,
    pub reasons: Vec<ModerationPolicyReasonDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ModerationPolicyWithReasons> for ModerationPolicyDto {
    fn from(ModerationPolicyWithReasons { policy, reasons }: ModerationPolicyWithReasons) -> Self {
        Self {
            id: policy.id,
            conversation_id: policy.conversation_id,
            name: policy.name,
            reasons: reasons.into_iter().map(Into::into).collect(),
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        }
    }
}

/// A reason from the built-in default list, used by steps with no moderation policy.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefaultModerationPolicyReasonDto {
    pub label: String,
    pub description: String,
}
