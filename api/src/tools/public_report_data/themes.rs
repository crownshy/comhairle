//! The step's latest theme collection with the contributions filed under each theme. A tool
//! that may not show the contributions serves `PublicTheme` instead: each theme's title,
//! description and how many contributions it groups (ADR-0061).

use sqlx::PgPool;
use uuid::Uuid;

use super::anonymous::{Count, GeneratedText, public_report_data};
use crate::error::ComhairleError;
use crate::models::statement::{self, Statement};
use crate::models::statement_theme;
use crate::models::theme::{self, Theme};

public_report_data! {
    #[derive(Clone)]
    pub struct PublicTheme {
        pub title: GeneratedText,
        pub description: GeneratedText,
        /// Contributions the sensemaker filed under this theme.
        pub contributions: Count,
    }
}

/// A theme with the contributions filed under it. These include contributions moderation
/// hasn't accepted, so a tool must match them against its own public data before serving any.
pub struct ThemeWithStatements {
    pub theme: Theme,
    pub statements: Vec<Statement>,
}

/// The latest theme collection the given tool produced for the step, or `None` when no themes
/// have been generated yet.
pub async fn latest_theme_collection(
    db: &PgPool,
    workflow_step_id: &Uuid,
    origin_tool: &str,
) -> Result<Option<Vec<ThemeWithStatements>>, ComhairleError> {
    let Some(collection) =
        theme::get_latest_for_workflow_step(db, *workflow_step_id, Some(origin_tool), None).await?
    else {
        return Ok(None);
    };
    let (collection, themes) = theme::list_themes(db, *workflow_step_id, collection.id).await?;
    let statements = statement::list_for_theme_collection(db, collection.id).await?;
    let assignments = statement_theme::list_for_theme_collection(db, collection.id).await?;

    let mut statements_by_theme = statement_theme::statements_by_theme(statements, assignments);

    Ok(Some(
        themes
            .into_iter()
            .map(|theme| ThemeWithStatements {
                statements: statements_by_theme.remove(&theme.id).unwrap_or_default(),
                theme,
            })
            .collect(),
    ))
}

pub fn public_theme(theme: &ThemeWithStatements) -> PublicTheme {
    PublicTheme {
        title: GeneratedText::from_sensemaking(theme.theme.title.clone()),
        description: GeneratedText::from_sensemaking(theme.theme.description.clone()),
        contributions: Count(theme.statements.len() as u64),
    }
}
