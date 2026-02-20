use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{event_attendance::EventAttendance, pagination::PaginatedResults};

/// Data transfer object (public API representation) for an EventAttendance.
///
/// This DTO is returned by event attendance related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendanceDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<EventAttendance> for EventAttendanceDto {
    fn from(e: EventAttendance) -> Self {
        Self {
            id: e.id,
            user_id: e.user_id,
            event_id: e.event_id,
            role: e.role,
            created_at: e.created_at,
        }
    }
}

impl From<PaginatedResults<EventAttendance>> for PaginatedResults<EventAttendanceDto> {
    fn from(r: PaginatedResults<EventAttendance>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
