//! Join table linking statements to the themes they were classified under. A
//! statement can match more than one theme, so this is many-to-many rather than a
//! `theme_id` column on `statement`.

use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::theme::ThemeIden;

#[derive(FromRow, Debug, Clone)]
#[enum_def(table_name = "statement_theme")]
pub struct StatementTheme {
    pub statement_id: Uuid,
    pub theme_id: Uuid,
}

const COLUMNS: [StatementThemeIden; 2] =
    [StatementThemeIden::StatementId, StatementThemeIden::ThemeId];

/// Replaces every theme assignment for `statement_ids` with `assignments`, so a
/// generation run's result fully replaces this batch of statements' previous
/// classification rather than accumulating stale pairs alongside new ones.
pub async fn replace_assignments(
    db: &PgPool,
    statement_ids: &[Uuid],
    assignments: &[(Uuid, Uuid)],
) -> Result<(), ComhairleError> {
    if statement_ids.is_empty() {
        return Ok(());
    }

    let mut tx = db.begin().await?;

    let (sql, values) = Query::delete()
        .from_table(StatementThemeIden::Table)
        .and_where(Expr::col(StatementThemeIden::StatementId).is_in(statement_ids.to_vec()))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    for (statement_id, theme_id) in assignments {
        let (sql, values) = Query::insert()
            .into_table(StatementThemeIden::Table)
            .columns(COLUMNS)
            .values([(*statement_id).into(), (*theme_id).into()])?
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&mut *tx).await?;
    }

    tx.commit().await?;

    Ok(())
}

/// Every (statement_id, theme_id) pair for a theme collection, so the API can group
/// statements under each of the themes they were classified into.
pub async fn list_for_theme_collection(
    db: &PgPool,
    theme_collection_id: Uuid,
) -> Result<Vec<StatementTheme>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(COLUMNS.map(|c| (StatementThemeIden::Table, c)))
        .from(StatementThemeIden::Table)
        .inner_join(
            ThemeIden::Table,
            Expr::col((StatementThemeIden::Table, StatementThemeIden::ThemeId))
                .equals((ThemeIden::Table, ThemeIden::Id)),
        )
        .and_where(
            Expr::col((ThemeIden::Table, ThemeIden::ThemeCollectionId)).eq(theme_collection_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::models::statement::{self, CreateStatement};
    use crate::models::theme::{self, CreateThemeCollection, NewTheme};
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::test_helpers::learn_tool_config;

    async fn get_random_workflow_step_id(
        app: &crate::App,
        session: &mut crate::test_helpers::UserSession,
    ) -> Result<Uuid, Box<dyn Error>> {
        let conversation_id = get_random_conversation_id(app, session).await?;
        let workflow_id = get_random_workflow_id(app, session, &conversation_id).await?;

        let (_, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow step",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": learn_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        Ok(workflow_step.id)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_assign_a_statement_to_multiple_themes(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        let collection = theme::create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "manual".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;
        let theme_a = theme::add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Buses".into(),
                description: "More buses".into(),
                created_by_ai: true,
            },
        )
        .await?;
        let theme_b = theme::add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Trains".into(),
                description: "More trains".into(),
                created_by_ai: true,
            },
        )
        .await?;

        let created_statement = statement::create(
            &pool,
            workflow_step_id,
            &CreateStatement {
                user_id: None,
                content: "More public transport".into(),
                context: "Submitted via form".into(),
                ai_generated: false,
                source_tool: "manual".into(),
                source_sub_id: None,
            },
        )
        .await?;

        replace_assignments(
            &pool,
            &[created_statement.id],
            &[
                (created_statement.id, theme_a.id),
                (created_statement.id, theme_b.id),
            ],
        )
        .await?;

        let mut assignments = list_for_theme_collection(&pool, collection.id).await?;
        assignments.sort_by_key(|a| a.theme_id);
        assert_eq!(
            assignments.len(),
            2,
            "statement should be assigned to both themes"
        );
        assert!(
            assignments
                .iter()
                .all(|a| a.statement_id == created_statement.id),
            "both assignments should be for the same statement"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn replace_assignments_clears_previous_assignments(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        let collection = theme::create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "manual".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;
        let theme_a = theme::add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Buses".into(),
                description: "More buses".into(),
                created_by_ai: true,
            },
        )
        .await?;

        let created_statement = statement::create(
            &pool,
            workflow_step_id,
            &CreateStatement {
                user_id: None,
                content: "More public transport".into(),
                context: "Submitted via form".into(),
                ai_generated: false,
                source_tool: "manual".into(),
                source_sub_id: None,
            },
        )
        .await?;

        replace_assignments(
            &pool,
            &[created_statement.id],
            &[(created_statement.id, theme_a.id)],
        )
        .await?;
        replace_assignments(&pool, &[created_statement.id], &[]).await?;

        let assignments = list_for_theme_collection(&pool, collection.id).await?;
        assert!(
            assignments.is_empty(),
            "previous assignment should be cleared"
        );

        Ok(())
    }
}
