use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as_with, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "thinking_space_summary")]
pub struct ThinkingSpaceSummary {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub summary: String,
    pub is_ai_generated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ThinkingSpaceSummaryIden; 7] = [
    ThinkingSpaceSummaryIden::Id,
    ThinkingSpaceSummaryIden::WorkflowStepId,
    ThinkingSpaceSummaryIden::UserId,
    ThinkingSpaceSummaryIden::Summary,
    ThinkingSpaceSummaryIden::IsAiGenerated,
    ThinkingSpaceSummaryIden::CreatedAt,
    ThinkingSpaceSummaryIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateSummary {
    pub summary: String,
    pub is_ai_generated: Option<bool>,
}

impl CreateSummary {
    fn columns(&self) -> Vec<ThinkingSpaceSummaryIden> {
        let mut columns = vec![ThinkingSpaceSummaryIden::Summary];
        if self.is_ai_generated.is_some() {
            columns.push(ThinkingSpaceSummaryIden::IsAiGenerated);
        }

        columns
    }

    fn values(&self) -> Vec<SimpleExpr> {
        let mut values = vec![self.summary.clone().into()];
        if let Some(value) = &self.is_ai_generated {
            values.push((*value).into());
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    workflow_step_id: Uuid,
    create_summary: &CreateSummary,
) -> Result<ThinkingSpaceSummary, ComhairleError> {
    let mut columns = create_summary.columns();
    let mut values = create_summary.values();

    columns.push(ThinkingSpaceSummaryIden::UserId);
    values.push(user_id.into());
    columns.push(ThinkingSpaceSummaryIden::WorkflowStepId);
    values.push(workflow_step_id.into());

    let (sql, values) = Query::insert()
        .into_table(ThinkingSpaceSummaryIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateSummary {
    pub summary: String,
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_summary: &UpdateSummary,
) -> Result<ThinkingSpaceSummary, ComhairleError> {
    let values = vec![(
        ThinkingSpaceSummaryIden::Summary,
        update_summary.summary.clone().into(),
    )];

    let (sql, values) = Query::update()
        .table(ThinkingSpaceSummaryIden::Table)
        .values(values)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceSummary, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ThinkingSpaceSummaryIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("Thinking space summary".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(summary)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ThinkingSpaceSummaryFilterOptions {
    // Server-set only — handlers must scope this to the authenticated user.
    // Hidden from the public schema/query string so callers can't spoof it.
    #[serde(skip_deserializing)]
    #[schemars(skip)]
    pub user_id: Option<Uuid>,
    pub is_ai_generated: Option<bool>,
}

impl ThinkingSpaceSummaryFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.user_id {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceSummaryIden::Table,
                        ThinkingSpaceSummaryIden::UserId,
                    ))
                    .eq(value),
                )
                .to_owned();
        }
        if let Some(value) = &self.is_ai_generated {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceSummaryIden::Table,
                        ThinkingSpaceSummaryIden::IsAiGenerated,
                    ))
                    .eq(*value),
                )
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    workflow_step_id: &Uuid,
    filter_options: ThinkingSpaceSummaryFilterOptions,
) -> Result<Vec<ThinkingSpaceSummary>, ComhairleError> {
    let query = Query::select()
        .from(ThinkingSpaceSummaryIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ThinkingSpaceSummaryIden::Table, col)))
        .and_where(
            Expr::col((
                ThinkingSpaceSummaryIden::Table,
                ThinkingSpaceSummaryIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let summaries = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(summaries)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceSummary, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThinkingSpaceSummaryIden::Table)
        .and_where(Expr::col(ThinkingSpaceSummaryIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let summary = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            users,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::thinking_space_tool_config,
    };

    use super::*;

    use serde_json::json;
    use std::error::Error;

    async fn create_thinking_space_resources(
        pool: &PgPool,
    ) -> Result<(Uuid, Uuid), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let workflow_id = get_random_workflow_id(&app, &mut session).await?;

        let (_, value, _) = session
            .create_workflow_step(
                &app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow_step with prioritization",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": thinking_space_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        let user = users::create_annon_user(pool).await?;

        Ok((user.id, workflow_step.id))
    }

    #[sqlx::test]
    async fn should_create_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        assert_eq!(
            summary.summary,
            "Some summary text".to_string(),
            "incorrect summary text"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            ..Default::default()
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        assert_eq!(
            summary.summary,
            "Some summary text".to_string(),
            "incorrect summary text before update"
        );

        let update_summary = UpdateSummary {
            summary: "Some updated text".to_string(),
        };

        let summary = update(&pool, summary.id, &update_summary).await?;

        assert_eq!(
            summary.summary,
            "Some updated text".to_string(),
            "incorrect summary text after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_thinking_space_summary_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let created_summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        let summary = get_by_id(&pool, created_summary.id).await?;

        assert_eq!(created_summary.id, summary.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_thinking_space_summaries(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_a_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let user_b = users::create_annon_user(&pool).await?;

        let params_a = CreateSummary {
            summary: "Summary_a".to_string(),
            ..Default::default()
        };
        create(&pool, user_a_id, workflow_step_id, &params_a).await?;

        let params_b = CreateSummary {
            summary: "Summary_b".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_b).await?;
        let params_c = CreateSummary {
            summary: "Summary_c".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_c).await?;
        let params_d = CreateSummary {
            summary: "Summary_d".to_string(),
            is_ai_generated: Some(true),
        };
        create(&pool, user_a_id, workflow_step_id, &params_d).await?;
        let params_e = CreateSummary {
            summary: "Summary_e".to_string(),
            is_ai_generated: Some(true),
        };
        let summary_e = create(&pool, user_b.id, workflow_step_id, &params_e).await?;

        let filter_options = ThinkingSpaceSummaryFilterOptions {
            user_id: Some(user_a_id),
            is_ai_generated: Some(true),
        };
        let summaries = list(&pool, &workflow_step_id, filter_options).await?;

        assert!(
            summaries.iter().all(|s| s.is_ai_generated),
            "not all edited summaries"
        );
        assert!(
            !summaries.iter().any(|s| s.id == summary_e.id),
            "user_b summary is included"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_thinking_space_summary(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id) = create_thinking_space_resources(&pool).await?;

        let create_summary = CreateSummary {
            summary: "Some summary text".to_string(),
            is_ai_generated: Some(true),
        };

        let summary = create(&pool, user_id, workflow_step_id, &create_summary).await?;

        delete(&pool, summary.id).await?;

        let err = get_by_id(&pool, summary.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(message) => {
                assert!(message.contains("Thinking space summary"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
