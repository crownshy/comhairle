use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::statement::Statement;
use crate::models::theme::{Theme, ThemeCollection};
use crate::routes::statements::dto::StatementDto;

/// Data transfer object (public API representation) for a theme and the statements
/// classified under it.
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
    pub pinned: bool,
    pub statements: Vec<StatementDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ThemeDto {
    pub fn from_parts(theme: Theme, statements: Vec<Statement>) -> Self {
        Self {
            id: theme.id,
            theme_collection_id: theme.theme_collection_id,
            title: theme.title,
            description: theme.description,
            created_by_ai: theme.created_by_ai,
            pinned: theme.pinned,
            statements: statements.into_iter().map(Into::into).collect(),
            created_at: theme.created_at,
            updated_at: theme.updated_at,
        }
    }
}

impl From<Theme> for ThemeDto {
    fn from(theme: Theme) -> Self {
        Self::from_parts(theme, vec![])
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
    pub origin_tool: String,
    pub origin_sub_id: Option<String>,
    pub requested_by_user_id: Option<Uuid>,
    pub themes: Vec<ThemeDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ThemeCollectionDto {
    pub fn from_parts(collection: ThemeCollection, themes: Vec<Theme>) -> Self {
        Self::from_parts_with_statements(collection, themes, vec![])
    }

    /// Groups `statements` by their `theme_id` and attaches each group to the matching
    /// theme, so a single call returns the full theme collection, themes, and the
    /// statements classified under each theme.
    pub fn from_parts_with_statements(
        collection: ThemeCollection,
        themes: Vec<Theme>,
        statements: Vec<Statement>,
    ) -> Self {
        let mut statements_by_theme: HashMap<Uuid, Vec<Statement>> = HashMap::new();
        for statement in statements {
            if let Some(theme_id) = statement.theme_id {
                statements_by_theme
                    .entry(theme_id)
                    .or_default()
                    .push(statement);
            }
        }

        let themes = themes
            .into_iter()
            .map(|theme| {
                let statements = statements_by_theme.remove(&theme.id).unwrap_or_default();
                ThemeDto::from_parts(theme, statements)
            })
            .collect();

        Self {
            id: collection.id,
            workflow_step_id: collection.workflow_step_id,
            context: collection.context,
            origin_tool: collection.origin_tool,
            origin_sub_id: collection.origin_sub_id,
            requested_by_user_id: collection.requested_by_user_id,
            themes,
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
