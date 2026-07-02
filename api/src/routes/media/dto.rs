use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    media::{Media, MediaContentType},
    pagination::PaginatedResults,
};

/// Data transfer object (public API representation) for a Media record.
///
/// This DTO is returned by media related endpoints and is safe to expose
/// to clients. It intentionally omits fields such as:
///
/// * `updated_at`
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaDto {
    pub id: Uuid,
    pub url: String,
    pub store_name: String,
    pub storage_key: String,
    pub filename: String,
    pub content_type: MediaContentType,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<Media> for MediaDto {
    fn from(m: Media) -> Self {
        Self {
            id: m.id,
            url: m.url().clone(),
            store_name: m.store_name,
            storage_key: m.storage_key,
            filename: m.filename,
            content_type: m.content_type,
            owner_id: m.owner_id,
            created_at: m.created_at,
        }
    }
}

impl From<PaginatedResults<Media>> for PaginatedResults<MediaDto> {
    fn from(r: PaginatedResults<Media>) -> Self {
        Self {
            total: r.total,
            records: r.records.into_iter().map(Into::into).collect(),
        }
    }
}
