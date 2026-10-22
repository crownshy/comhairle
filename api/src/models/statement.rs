//! Statements: content attached to a workflow step, written by a participant
//! (`user_id` set) or generated (`ai_generated`), with `context` explaining where it
//! came from.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "statement")]
pub struct Statement {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub ai_generated: bool,
    pub context: String,
    pub source_tool: String,
    pub source_sub_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const STATEMENT_COLUMNS: [StatementIden; 10] = [
    StatementIden::Id,
    StatementIden::WorkflowStepId,
    StatementIden::UserId,
    StatementIden::Content,
    StatementIden::AiGenerated,
    StatementIden::Context,
    StatementIden::SourceTool,
    StatementIden::SourceSubId,
    StatementIden::CreatedAt,
    StatementIden::UpdatedAt,
];

/// Statements created manually via the API (e.g. an admin adding a statement by hand)
/// default to the "manual" source, since they don't originate from a tool.
fn default_source_tool() -> String {
    "manual".to_string()
}

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateStatement {
    pub user_id: Option<Uuid>,
    pub content: String,
    pub context: String,
    #[serde(default)]
    pub ai_generated: bool,
    #[serde(default = "default_source_tool")]
    pub source_tool: String,
    #[serde(default)]
    pub source_sub_id: Option<String>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateStatement {
    pub content: String,
    pub context: String,
}

fn clean_text(field: &str, value: &str) -> Result<String, ComhairleError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ComhairleError::BadRequest(format!(
            "{field} can't be blank"
        )));
    }
    Ok(value.to_string())
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    workflow_step_id: Uuid,
    payload: &CreateStatement,
) -> Result<Statement, ComhairleError> {
    let content = clean_text("Content", &payload.content)?;
    let context = clean_text("Context", &payload.context)?;

    let (sql, values) = Query::insert()
        .into_table(StatementIden::Table)
        .columns([
            StatementIden::WorkflowStepId,
            StatementIden::UserId,
            StatementIden::Content,
            StatementIden::Context,
            StatementIden::AiGenerated,
            StatementIden::SourceTool,
            StatementIden::SourceSubId,
        ])
        .values([
            workflow_step_id.into(),
            payload.user_id.into(),
            content.into(),
            context.into(),
            payload.ai_generated.into(),
            payload.source_tool.clone().into(),
            payload.source_sub_id.clone().into(),
        ])?
        .returning(Query::returning().columns(STATEMENT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Statement")
}

/// Inserts a statement if none exists yet for this
/// `(workflow_step_id, source_tool, source_sub_id)`, otherwise refreshes its content.
/// Used by tool harvesters to mirror source text into generic statements without
/// duplicating rows on repeated runs.
#[instrument(err(Debug), skip(db))]
pub async fn upsert_from_source(
    db: &PgPool,
    workflow_step_id: Uuid,
    payload: &CreateStatement,
) -> Result<Statement, ComhairleError> {
    let content = clean_text("Content", &payload.content)?;
    let context = clean_text("Context", &payload.context)?;

    let (sql, values) = Query::select()
        .columns(STATEMENT_COLUMNS)
        .from(StatementIden::Table)
        .and_where(Expr::col(StatementIden::WorkflowStepId).eq(workflow_step_id))
        .and_where(Expr::col(StatementIden::SourceTool).eq(payload.source_tool.clone()))
        .and_where(match &payload.source_sub_id {
            Some(sub_id) => Expr::col(StatementIden::SourceSubId).eq(sub_id.clone()),
            None => Expr::col(StatementIden::SourceSubId).is_null(),
        })
        .build_sqlx(PostgresQueryBuilder);

    let existing: Option<Statement> = query_as_with(&sql, values).fetch_optional(db).await?;

    if let Some(existing) = existing {
        let (sql, values) = Query::update()
            .table(StatementIden::Table)
            .values([
                (StatementIden::Content, content.into()),
                (StatementIden::UpdatedAt, Expr::current_timestamp().into()),
            ])
            .and_where(Expr::col(StatementIden::Id).eq(existing.id))
            .returning(Query::returning().columns(STATEMENT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        return query_as_with(&sql, values)
            .fetch_one(db)
            .await
            .resolve_db_err("Statement");
    }

    let (sql, values) = Query::insert()
        .into_table(StatementIden::Table)
        .columns([
            StatementIden::WorkflowStepId,
            StatementIden::UserId,
            StatementIden::Content,
            StatementIden::Context,
            StatementIden::AiGenerated,
            StatementIden::SourceTool,
            StatementIden::SourceSubId,
        ])
        .values([
            workflow_step_id.into(),
            payload.user_id.into(),
            content.into(),
            context.into(),
            payload.ai_generated.into(),
            payload.source_tool.clone().into(),
            payload.source_sub_id.clone().into(),
        ])?
        .returning(Query::returning().columns(STATEMENT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Statement")
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<Statement, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(STATEMENT_COLUMNS)
        .from(StatementIden::Table)
        .and_where(Expr::col(StatementIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Statement")
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_workflow_step(
    db: &PgPool,
    workflow_step_id: Uuid,
) -> Result<Vec<Statement>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(STATEMENT_COLUMNS)
        .from(StatementIden::Table)
        .and_where(Expr::col(StatementIden::WorkflowStepId).eq(workflow_step_id))
        .order_by(StatementIden::CreatedAt, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let statements = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(statements)
}

/// Lists the distinct statements classified under any theme in the given theme
/// collection, for embedding into a `ThemeCollectionDto` response. A statement
/// classified under more than one theme in the collection appears once here; see
/// `statement_theme::list_for_theme_collection` for the per-theme assignment pairs
/// used to group these statements back under each of their themes.
#[instrument(err(Debug), skip(db))]
pub async fn list_for_theme_collection(
    db: &PgPool,
    theme_collection_id: Uuid,
) -> Result<Vec<Statement>, ComhairleError> {
    let (sql, values) = Query::select()
        .distinct()
        .columns(STATEMENT_COLUMNS.map(|c| (StatementIden::Table, c)))
        .from(StatementIden::Table)
        .inner_join(
            crate::models::statement_theme::StatementThemeIden::Table,
            Expr::col((StatementIden::Table, StatementIden::Id)).equals((
                crate::models::statement_theme::StatementThemeIden::Table,
                crate::models::statement_theme::StatementThemeIden::StatementId,
            )),
        )
        .inner_join(
            crate::models::theme::ThemeIden::Table,
            Expr::col((
                crate::models::statement_theme::StatementThemeIden::Table,
                crate::models::statement_theme::StatementThemeIden::ThemeId,
            ))
            .equals((
                crate::models::theme::ThemeIden::Table,
                crate::models::theme::ThemeIden::Id,
            )),
        )
        .and_where(
            Expr::col((
                crate::models::theme::ThemeIden::Table,
                crate::models::theme::ThemeIden::ThemeCollectionId,
            ))
            .eq(theme_collection_id),
        )
        .order_by((StatementIden::Table, StatementIden::CreatedAt), Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let statements = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(statements)
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    workflow_step_id: Uuid,
    id: Uuid,
    payload: &UpdateStatement,
) -> Result<Statement, ComhairleError> {
    let content = clean_text("Content", &payload.content)?;
    let context = clean_text("Context", &payload.context)?;

    let (sql, values) = Query::update()
        .table(StatementIden::Table)
        .values([
            (StatementIden::Content, content.into()),
            (StatementIden::Context, context.into()),
            (StatementIden::UpdatedAt, Expr::current_timestamp().into()),
        ])
        .and_where(Expr::col(StatementIden::Id).eq(id))
        .and_where(Expr::col(StatementIden::WorkflowStepId).eq(workflow_step_id))
        .returning(Query::returning().columns(STATEMENT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Statement")
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(StatementIden::Table)
        .and_where(Expr::col(StatementIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_with(&sql, values).execute(db).await?;
    if deleted.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("Statement".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_json::json;

    use super::*;
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
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
    async fn should_create_and_list_statements(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        let created = create(
            &pool,
            workflow_step_id,
            &CreateStatement {
                user_id: None,
                content: "More buses".into(),
                context: "Submitted via form".into(),
                ai_generated: false,
                source_tool: "manual".into(),
                source_sub_id: None,
            },
        )
        .await?;
        assert!(!created.ai_generated, "ai_generated not saved");
        assert_eq!(created.user_id, None, "user_id should be optional");

        let statements = list_for_workflow_step(&pool, workflow_step_id).await?;
        assert_eq!(statements.len(), 1, "incorrect statement count");
        assert_eq!(statements[0].id, created.id, "wrong statement returned");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_and_delete_statement(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;
        let created = create(
            &pool,
            workflow_step_id,
            &CreateStatement {
                user_id: None,
                content: "Original".into(),
                context: "Context".into(),
                ai_generated: true,
                source_tool: "manual".into(),
                source_sub_id: None,
            },
        )
        .await?;

        let updated = update(
            &pool,
            workflow_step_id,
            created.id,
            &UpdateStatement {
                content: "Updated content".into(),
                context: "Updated context".into(),
            },
        )
        .await?;
        assert_eq!(updated.content, "Updated content", "content not updated");

        delete(&pool, created.id).await?;
        let statements = list_for_workflow_step(&pool, workflow_step_id).await?;
        assert!(statements.is_empty(), "statement not deleted");

        Ok(())
    }
}
