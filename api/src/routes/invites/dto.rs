use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::models::invites::{Invite, InviteStatus, InviteType, LoginBehaviour};

/// Data transfer object (public API representation) for an Invite.
///
/// This DTO is returned by invite related endpoints and is safe to expose to
/// clients. It intentionally omits fields such as:
///
/// * `created_at`
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InviteDto {
    pub id: Uuid,
    pub invite_type: InviteType,
    pub created_by: Uuid,
    pub status: InviteStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub conversation_id: Uuid,
    pub workflow_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
    pub login_behaviour: LoginBehaviour,
    pub tags: Vec<String>,
    pub accept_count: i32,
}

impl From<Invite> for InviteDto {
    fn from(i: Invite) -> Self {
        Self {
            id: i.id,
            invite_type: i.invite_type,
            created_by: i.created_by,
            status: i.status,
            expires_at: i.expires_at,
            conversation_id: i.conversation_id,
            workflow_id: i.workflow_id,
            workflow_step_id: i.workflow_step_id,
            login_behaviour: i.login_behaviour,
            tags: i.tags,
            accept_count: i.accept_count,
        }
    }
}
