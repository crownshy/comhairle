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
#[enum_def(table_name = "thinking_space_follow_up_question")]
pub struct ThinkingSpaceFollowUpQuestion {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub root_question_id: Uuid,
    pub follow_up_questions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ThinkingSpaceFollowUpQuestionIden; 7] = [
    ThinkingSpaceFollowUpQuestionIden::Id,
    ThinkingSpaceFollowUpQuestionIden::WorkflowStepId,
    ThinkingSpaceFollowUpQuestionIden::UserId,
    ThinkingSpaceFollowUpQuestionIden::RootQuestionId,
    ThinkingSpaceFollowUpQuestionIden::FollowUpQuestions,
    ThinkingSpaceFollowUpQuestionIden::CreatedAt,
    ThinkingSpaceFollowUpQuestionIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateFollowUpQuestions {
    pub root_question_id: Uuid,
    pub follow_up_questions: Vec<String>,
    pub workflow_step_id: Uuid,
}

impl CreateFollowUpQuestions {
    fn columns(&self) -> Vec<ThinkingSpaceFollowUpQuestionIden> {
        vec![
            ThinkingSpaceFollowUpQuestionIden::RootQuestionId,
            ThinkingSpaceFollowUpQuestionIden::FollowUpQuestions,
            ThinkingSpaceFollowUpQuestionIden::WorkflowStepId,
        ]
    }

    fn values(&self) -> Vec<SimpleExpr> {
        vec![
            self.root_question_id.into(),
            self.follow_up_questions.clone().into(),
            self.workflow_step_id.into(),
        ]
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    create_follow_ups: &CreateFollowUpQuestions,
) -> Result<ThinkingSpaceFollowUpQuestion, ComhairleError> {
    let mut columns = create_follow_ups.columns();
    let mut values = create_follow_ups.values();

    columns.push(ThinkingSpaceFollowUpQuestionIden::UserId);
    values.push(user_id.into());

    let (sql, values) = Query::insert()
        .into_table(ThinkingSpaceFollowUpQuestionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(follow_ups)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateFollowUpQuestions {
    pub follow_up_questions: Vec<String>,
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_follow_ups: &UpdateFollowUpQuestions,
) -> Result<ThinkingSpaceFollowUpQuestion, ComhairleError> {
    let values = vec![(
        ThinkingSpaceFollowUpQuestionIden::FollowUpQuestions,
        update_follow_ups.follow_up_questions.clone().into(),
    )];

    let (sql, values) = Query::update()
        .table(ThinkingSpaceFollowUpQuestionIden::Table)
        .values(values)
        .and_where(Expr::col(ThinkingSpaceFollowUpQuestionIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(follow_ups)
}

#[instrument(err(Debug))]
pub async fn get_by_id(
    db: &PgPool,
    id: Uuid,
) -> Result<ThinkingSpaceFollowUpQuestion, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ThinkingSpaceFollowUpQuestionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ThinkingSpaceFollowUpQuestionIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("Thinking space follow up questions".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(follow_ups)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ThinkingSpaceFollowUpQuestionFilterOptions {
    user_id: Option<Uuid>,
    root_question_id: Option<Uuid>,
}

impl ThinkingSpaceFollowUpQuestionFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.user_id {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceFollowUpQuestionIden::Table,
                        ThinkingSpaceFollowUpQuestionIden::UserId,
                    ))
                    .eq(value),
                )
                .to_owned();
        }
        if let Some(value) = self.root_question_id {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceFollowUpQuestionIden::Table,
                        ThinkingSpaceFollowUpQuestionIden::RootQuestionId,
                    ))
                    .eq(value),
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
    filter_options: ThinkingSpaceFollowUpQuestionFilterOptions,
) -> Result<Vec<ThinkingSpaceFollowUpQuestion>, ComhairleError> {
    let query = Query::select()
        .from(ThinkingSpaceFollowUpQuestionIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ThinkingSpaceFollowUpQuestionIden::Table, col)))
        .and_where(
            Expr::col((
                ThinkingSpaceFollowUpQuestionIden::Table,
                ThinkingSpaceFollowUpQuestionIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(follow_ups)
}

#[instrument(err(Debug))]
pub async fn delete(
    db: &PgPool,
    id: Uuid,
) -> Result<ThinkingSpaceFollowUpQuestion, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThinkingSpaceFollowUpQuestionIden::Table)
        .and_where(Expr::col(ThinkingSpaceFollowUpQuestionIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(follow_ups)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::{
            model_test_helpers::{
                get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
            },
            users,
        },
        routes::workflow_steps::dto::WorkflowStepDto,
        test_helpers::thinking_space_tool_config,
        tools::{thinking_space::ThinkingSpaceToolConfig, ToolConfig},
    };

    use serde_json::json;
    use std::error::Error;

    async fn create_thinking_space_resources(
        pool: &PgPool,
    ) -> Result<(Uuid, Uuid, ThinkingSpaceToolConfig), Box<dyn Error>> {
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

        let tool_config = match workflow_step.preview_tool_config {
            ToolConfig::ThinkingSpace(config) => config,
            _ => panic!("Wrong tool config type"),
        };

        Ok((user.id, workflow_step.id, tool_config))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_follow_up_questions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        assert_eq!(
            follow_ups.root_question_id,
            tool_config.root_questions.first().unwrap().id,
            "incorrect root_question_id"
        );
        assert_eq!(
            follow_ups.follow_up_questions.len(),
            2,
            "incorrect number of follow ups"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_follow_up_questions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        let update_follow_ups = UpdateFollowUpQuestions {
            follow_up_questions: vec![
                "An updated question".to_string(),
                "Another updated question".to_string(),
            ],
        };

        let follow_ups = update(&pool, follow_ups.id, &update_follow_ups).await?;

        assert_eq!(
            follow_ups.follow_up_questions,
            vec![
                "An updated question".to_string(),
                "Another updated question".to_string(),
            ],
            "follow up questions incorrect after update"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_follow_up_questions_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let created_follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        let follow_ups = get_by_id(&pool, created_follow_ups.id).await?;

        assert_eq!(created_follow_ups.id, follow_ups.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_follow_up_questions_for_workflow_step(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (user_a_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let user_b = users::create_annon_user(&pool).await?;

        let params_a = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_a_id, &params_a).await?;

        let params_b = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_a_id, &params_b).await?;

        let params_c = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };
        create(&pool, user_b.id, &params_c).await?;

        let filter_options = ThinkingSpaceFollowUpQuestionFilterOptions {
            user_id: Some(user_b.id),
            ..Default::default()
        };
        let follow_ups = list(&pool, &workflow_step_id, filter_options).await?;

        assert_eq!(follow_ups.len(), 1, "incorrect total");
        assert!(
            !follow_ups.iter().any(|f| f.user_id == user_a_id),
            "should not include user_a follow_ups"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_follow_up_questions_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (user_id, workflow_step_id, tool_config) =
            create_thinking_space_resources(&pool).await?;

        let create_follow_ups = CreateFollowUpQuestions {
            root_question_id: tool_config.root_questions.first().unwrap().id,
            follow_up_questions: vec![
                "A follow up question".to_string(),
                "Another follow up question".to_string(),
            ],
            workflow_step_id,
        };

        let follow_ups = create(&pool, user_id, &create_follow_ups).await?;

        delete(&pool, follow_ups.id).await?;

        let err = get_by_id(&pool, follow_ups.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(message) => {
                assert!(message.contains("Thinking space follow up questions"))
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
