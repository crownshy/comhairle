use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;

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

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    create_follow_ups: &CreateFollowUpQuestions,
) -> Result<ThinkingSpaceFollowUpQuestion, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    update_follow_ups: &UpdateFollowUpQuestions,
) -> Result<ThinkingSpaceFollowUpQuestion, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceFollowUpQuestion, ModelError> {
    let (sql, values) = Query::select()
        .from(ThinkingSpaceFollowUpQuestionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ThinkingSpaceFollowUpQuestionIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Thinking Space Follow Up Questions")?;

    Ok(follow_ups)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ThinkingSpaceFollowUpQuestionFilterOptions {
    pub user_id: Option<Uuid>,
    pub root_question_id: Option<Uuid>,
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

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    workflow_step_id: &Uuid,
    filter_options: ThinkingSpaceFollowUpQuestionFilterOptions,
) -> Result<Vec<ThinkingSpaceFollowUpQuestion>, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: Uuid) -> Result<ThinkingSpaceFollowUpQuestion, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(ThinkingSpaceFollowUpQuestionIden::Table)
        .and_where(Expr::col(ThinkingSpaceFollowUpQuestionIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let follow_ups = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(follow_ups)
}
