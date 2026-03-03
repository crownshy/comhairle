use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        event::{Event, LocalizedEvent, LocalizedEventWithAttendance},
        pagination::PaginatedResults,
        translations::TextContentId,
    },
    schema_helpers::{example_localized_text, example_uuid},
};

/// Data transfer object (public API representation) for an Event.
///
/// This DTO is returned by event related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_uuid")]
    pub name: TextContentId,
    #[schemars(example = "example_uuid")]
    pub description: TextContentId,
    pub capacity: Option<i32>,
    #[schemars(example = "example_uuid")]
    pub conversation_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_mode: String,
    pub video_meeting_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Data transfer object (public API representation) for LocalizedEvent.
///
/// This DTO is returned by event related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedEventDto {
    #[schemars(example = "example_uuid")]
    pub id: Uuid,
    #[schemars(example = "example_localized_text")]
    pub name: String,
    #[schemars(example = "example_localized_text")]
    pub description: String,
    pub capacity: Option<i32>,
    #[schemars(example = "example_uuid")]
    pub conversation_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_mode: String,
    pub current_attendance: Option<i64>,
    pub video_meeting_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<Event> for EventDto {
    fn from(e: Event) -> Self {
        Self {
            id: e.id,
            name: e.name,
            description: e.description,
            capacity: e.capacity,
            conversation_id: e.conversation_id,
            start_time: e.start_time,
            end_time: e.end_time,
            signup_mode: e.signup_mode,
            video_meeting_id: e.video_meeting_id,
            created_at: e.created_at,
        }
    }
}

impl From<LocalizedEventWithAttendance> for LocalizedEventDto {
    fn from(e: LocalizedEventWithAttendance) -> Self {
        Self {
            id: e.event.id,
            name: e.event.name,
            description: e.event.description,
            capacity: e.event.capacity,
            conversation_id: e.event.conversation_id,
            start_time: e.event.start_time,
            end_time: e.event.end_time,
            signup_mode: e.event.signup_mode,
            created_at: e.event.created_at,
            video_meeting_id: e.event.video_meeting_id,
            current_attendance: Some(e.current_attendance),
        }
    }
}

impl From<LocalizedEvent> for LocalizedEventDto {
    fn from(e: LocalizedEvent) -> Self {
        Self {
            id: e.id,
            name: e.name,
            description: e.description,
            capacity: e.capacity,
            conversation_id: e.conversation_id,
            start_time: e.start_time,
            end_time: e.end_time,
            signup_mode: e.signup_mode,
            created_at: e.created_at,
            video_meeting_id: e.video_meeting_id,
            current_attendance: None,
        }
    }
}

impl From<PaginatedResults<LocalizedEventWithAttendance>> for PaginatedResults<LocalizedEventDto> {
    fn from(r: PaginatedResults<LocalizedEventWithAttendance>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
