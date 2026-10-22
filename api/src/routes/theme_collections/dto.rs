use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::statement::Statement;
use crate::models::statement_theme::StatementTheme;
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
        Self::from_parts_with_statements(collection, themes, vec![], vec![])
    }

    /// Groups `statements` under every theme they were assigned to per `assignments`
    /// (a statement can be assigned to more than one theme), so a single call returns
    /// the full theme collection, themes, and the statements classified under each.
    pub fn from_parts_with_statements(
        collection: ThemeCollection,
        themes: Vec<Theme>,
        statements: Vec<Statement>,
        assignments: Vec<StatementTheme>,
    ) -> Self {
        let statements_by_id: HashMap<Uuid, Statement> =
            statements.into_iter().map(|s| (s.id, s)).collect();

        let mut statements_by_theme: HashMap<Uuid, Vec<Statement>> = HashMap::new();
        for assignment in assignments {
            if let Some(statement) = statements_by_id.get(&assignment.statement_id) {
                statements_by_theme
                    .entry(assignment.theme_id)
                    .or_default()
                    .push(statement.clone());
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

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    fn test_collection() -> ThemeCollection {
        ThemeCollection {
            id: Uuid::new_v4(),
            workflow_step_id: Uuid::new_v4(),
            context: "Context".into(),
            origin_tool: "manual".into(),
            origin_sub_id: None,
            requested_by_user_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn test_theme(theme_collection_id: Uuid, title: &str) -> Theme {
        Theme {
            id: Uuid::new_v4(),
            theme_collection_id,
            title: title.into(),
            description: "Description".into(),
            created_by_ai: true,
            pinned: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn test_statement(workflow_step_id: Uuid, content: &str) -> Statement {
        Statement {
            id: Uuid::new_v4(),
            workflow_step_id,
            user_id: None,
            content: content.into(),
            ai_generated: false,
            context: "Context".into(),
            source_tool: "manual".into(),
            source_sub_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn a_statement_assigned_to_multiple_themes_appears_under_each() {
        let collection = test_collection();
        let theme_a = test_theme(collection.id, "Buses");
        let theme_b = test_theme(collection.id, "Trains");
        let statement = test_statement(collection.workflow_step_id, "More public transport");

        let dto = ThemeCollectionDto::from_parts_with_statements(
            collection,
            vec![theme_a.clone(), theme_b.clone()],
            vec![statement.clone()],
            vec![
                StatementTheme {
                    statement_id: statement.id,
                    theme_id: theme_a.id,
                },
                StatementTheme {
                    statement_id: statement.id,
                    theme_id: theme_b.id,
                },
            ],
        );

        for theme in &dto.themes {
            assert_eq!(
                theme.statements.len(),
                1,
                "statement should appear under both themes"
            );
            assert_eq!(theme.statements[0].id, statement.id);
        }
    }
}
