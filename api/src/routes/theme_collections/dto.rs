use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::theme::{Theme, ThemeCollection};

/// Data transfer object (public API representation) for a theme.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThemeDto {
    pub id: Uuid,
    pub theme_collection_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by_ai: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Theme> for ThemeDto {
    fn from(theme: Theme) -> Self {
        Self {
            id: theme.id,
            theme_collection_id: theme.theme_collection_id,
            title: theme.title,
            description: theme.description,
            created_by_ai: theme.created_by_ai,
            created_at: theme.created_at,
            updated_at: theme.updated_at,
        }
    }
}

/// Data transfer object (public API representation) for a theme collection and its themes.
///
/// Serialized to JSON using camelCase field names for frontend (JavaScript) compatibility.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThemeCollectionDto {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub context: String,
    pub themes: Vec<ThemeDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ThemeCollectionDto {
    pub fn from_parts(collection: ThemeCollection, themes: Vec<Theme>) -> Self {
        Self {
            id: collection.id,
            workflow_step_id: collection.workflow_step_id,
            context: collection.context,
            themes: themes.into_iter().map(Into::into).collect(),
            created_at: collection.created_at,
            updated_at: collection.updated_at,
        }
    }
}

impl From<ThemeCollection> for ThemeCollectionDto {
    fn from(collection: ThemeCollection) -> Self {
        Self::from_parts(collection, vec![])
    }
}
