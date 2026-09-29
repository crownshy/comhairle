use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::media::{Media, MediaContentType, MediaResolver};

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
    pub name: String,
    pub alt: String,
    pub content_type: MediaContentType,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<Media> for MediaDto {
    fn from(m: Media) -> Self {
        Self {
            id: m.id,
            url: m.url(),
            store_name: m.store_name,
            storage_key: m.storage_key,
            filename: m.filename,
            name: m.name,
            alt: m.alt,
            content_type: m.content_type,
            owner_id: m.owner_id,
            created_at: m.created_at,
        }
    }
}

/// Converts a value into `Self` using a [`MediaResolver`] to resolve any
/// media references (e.g. `Uuid` fields) into their corresponding URLs.
///
/// This mirrors [`From`], but for conversions that need previously-resolved
/// media data rather than performing async DB lookups inline. Callers are
/// expected to batch-load a [`MediaResolver`] for all relevant IDs up front
/// (e.g. via [`MediaResolver::load`]) before calling `from_with_media`.
pub trait FromWithMedia<T> {
    /// Performs the conversion from `model` to `Self`, resolving media
    /// references via `media`.
    ///
    /// If a media reference is absent (e.g. an optional field with no
    /// associated media) or its ID could not be resolved by `media`,
    /// `fallback` is used in its place instead.
    fn from_with_media(model: T, media: &MediaResolver, fallback: &str) -> Self;
}
