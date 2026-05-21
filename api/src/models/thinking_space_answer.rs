use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as_with, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[cfg(test)]
use fake::Dummy;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "thinking_space_answer")]
pub struct ThinkingSpaceAnswer {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub user_id: Uuid,
    pub root_question_id: Option<Uuid>,
    pub is_follow_up: bool,
    pub question: String,
    pub answer: String,
    pub other_questions: Vec<String>,
    pub status: AnswerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum AnswerStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "approved")]
    Approved,
    #[sqlx(rename = "declined")]
    Declined,
}

impl From<AnswerStatus> for sea_query::Value {
    fn from(val: AnswerStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for AnswerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AnswerStatus::Pending => "pending",
            AnswerStatus::Approved => "approved",
            AnswerStatus::Declined => "declined",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [ThinkingSpaceAnswerIden; 11] = [
    ThinkingSpaceAnswerIden::Id,
    ThinkingSpaceAnswerIden::WorkflowStepId,
    ThinkingSpaceAnswerIden::UserId,
    ThinkingSpaceAnswerIden::RootQuestionId,
    ThinkingSpaceAnswerIden::IsFollowUp,
    ThinkingSpaceAnswerIden::Question,
    ThinkingSpaceAnswerIden::Answer,
    ThinkingSpaceAnswerIden::OtherQuestions,
    ThinkingSpaceAnswerIden::Status,
    ThinkingSpaceAnswerIden::CreatedAt,
    ThinkingSpaceAnswerIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct CreateAnswer {
    question: String,
    answer: String,
    other_questions: Option<Vec<String>>,
    root_question_id: Option<Uuid>,
    is_follow_up: Option<bool>,
}

impl CreateAnswer {
    fn columns(&self) -> Vec<ThinkingSpaceAnswerIden> {
        let mut columns = vec![
            ThinkingSpaceAnswerIden::Question,
            ThinkingSpaceAnswerIden::Answer,
        ];

        if self.other_questions.is_some() {
            columns.push(ThinkingSpaceAnswerIden::OtherQuestions);
        }
        if self.root_question_id.is_some() {
            columns.push(ThinkingSpaceAnswerIden::RootQuestionId);
        }
        if self.is_follow_up.is_some() {
            columns.push(ThinkingSpaceAnswerIden::IsFollowUp);
        }

        columns
    }

    fn values(&self) -> Vec<SimpleExpr> {
        let mut values = vec![(*self.question).into(), (*self.answer).into()];

        if let Some(value) = &self.other_questions {
            values.push(value.clone().into());
        }
        if let Some(value) = &self.root_question_id {
            values.push((*value).into());
        }
        if let Some(value) = &self.is_follow_up {
            values.push((*value).into());
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    workflow_step_id: &Uuid,
    user_id: &Uuid,
    new_answer: &CreateAnswer,
) -> Result<ThinkingSpaceAnswer, ComhairleError> {
    let mut columns = new_answer.columns();
    let mut values = new_answer.values();

    columns.push(ThinkingSpaceAnswerIden::WorkflowStepId);
    values.push((*workflow_step_id).into());

    columns.push(ThinkingSpaceAnswerIden::UserId);
    values.push((*user_id).into());

    let (sql, values) = Query::insert()
        .into_table(ThinkingSpaceAnswerIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let answer = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(answer)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceAnswer, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ThinkingSpaceAnswerIden::Table)
        .and_where(Expr::col(ThinkingSpaceAnswerIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let answer = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("Thinking space answer".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(answer)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ThinkingSpaceAnswerFilterOptions {
    user_id: Option<Uuid>,
    status: Option<AnswerStatus>,
}

impl ThinkingSpaceAnswerFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.user_id {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceAnswerIden::Table,
                        ThinkingSpaceAnswerIden::UserId,
                    ))
                    .eq(value),
                )
                .to_owned();
        }
        if let Some(value) = &self.status {
            query = query
                .and_where(
                    Expr::col((
                        ThinkingSpaceAnswerIden::Table,
                        ThinkingSpaceAnswerIden::Status,
                    ))
                    .eq(value.to_string()),
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
    filter_options: ThinkingSpaceAnswerFilterOptions,
) -> Result<Vec<ThinkingSpaceAnswer>, ComhairleError> {
    let query = Query::select()
        .from(ThinkingSpaceAnswerIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ThinkingSpaceAnswerIden::Table, col)))
        .and_where(
            Expr::col((
                ThinkingSpaceAnswerIden::Table,
                ThinkingSpaceAnswerIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .to_owned();

    let query = filter_options.apply(query);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let answers = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(answers)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateAnswer {
    pub answer: Option<String>,
    pub status: Option<AnswerStatus>,
}

impl UpdateAnswer {
    fn to_values(&self) -> Vec<(ThinkingSpaceAnswerIden, SimpleExpr)> {
        let mut values = vec![];

        if let Some(value) = &self.answer {
            values.push((ThinkingSpaceAnswerIden::Answer, value.clone().into()))
        }
        if let Some(value) = &self.status {
            values.push((ThinkingSpaceAnswerIden::Status, value.clone().into()))
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_answer: &UpdateAnswer,
) -> Result<ThinkingSpaceAnswer, ComhairleError> {
    let values = update_answer.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(ThinkingSpaceAnswerIden::Table)
        .values(values)
        .and_where(Expr::col(ThinkingSpaceAnswerIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let answer = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(answer)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceAnswer, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThinkingSpaceAnswerIden::Table)
        .and_where(Expr::col(ThinkingSpaceAnswerIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let answer = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

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

    use std::error::Error;

    // TODO: requires tool_config for thinking space
    #[ignore]
    #[sqlx::test]
    async fn should_create_new_thinking_space_answer(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
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

        let user = users::create_annon_user(&pool).await?;

        let create_answer = CreateAnswer {
            question: "A test question".to_string(),
            answer: "A test answer".to_string(),
            ..Default::default()
        };
        let answer = create(&pool, &workflow_step.id, &user.id, &create_answer).await?;

        assert_eq!(
            answer.workflow_step_id, workflow_step.id,
            "incorrect workflow_step_id"
        );

        Ok(())
    }
}
