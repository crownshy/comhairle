use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "feedback")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct Feedback {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    created_by: Uuid,
    #[partially(omit)]
    pub conversation_id: Uuid,
    pub content: String,
    #[partially(omit)]
    created_at: DateTime<Utc>,
    #[partially(omit)]
    updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [FeedbackIden; 6] = [
    FeedbackIden::Id,
    FeedbackIden::CreatedBy,
    FeedbackIden::ConversationId,
    FeedbackIden::Content,
    FeedbackIden::CreatedAt,
    FeedbackIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateFeedbackDTO {
    pub content: String,
}

pub async fn update(
    db: &PgPool,
    update_request: PartialFeedback,
    feedback_id: &Uuid,
    user_id: &Uuid,
) -> Result<Feedback, ComhairleError> {
    let (sql, values) = Query::update()
        .table(FeedbackIden::Table)
        .values(vec![(FeedbackIden::Content, update_request.content.into())])
        .and_where(Expr::col(FeedbackIden::Id).eq(*feedback_id))
        .and_where(Expr::col(FeedbackIden::CreatedBy).eq(*user_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_e| ComhairleError::FailedToUpdateFeedback)
}

pub async fn create(
    db: &PgPool,
    create_request: CreateFeedbackDTO,
    conversation_id: &Uuid,
    user_id: &Uuid,
) -> Result<Feedback, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(FeedbackIden::Table)
        .columns(vec![
            FeedbackIden::CreatedBy,
            FeedbackIden::ConversationId,
            FeedbackIden::Content,
        ])
        .values(vec![
            user_id.to_owned().into(),
            conversation_id.to_owned().into(),
            create_request.content.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, Feedback, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_e| ComhairleError::FailedToCreateFeedback)
}
pub async fn list_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<Feedback>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(FeedbackIden::Table)
        .and_where(Expr::col(FeedbackIden::ConversationId).eq(conversation_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let feedback = sqlx::query_as_with::<_, Feedback, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Feedback".into()))?;

    Ok(feedback)
}

pub async fn list_for_user_on_conversation(
    db: &PgPool,
    user_id: &Uuid,
    conversation_id: &Uuid,
) -> Result<Vec<Feedback>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(FeedbackIden::Table)
        .and_where(Expr::col(FeedbackIden::ConversationId).eq(*conversation_id))
        .and_where(Expr::col(FeedbackIden::CreatedBy).eq(*user_id))
        .build_sqlx(PostgresQueryBuilder);

    let feedback = sqlx::query_as_with::<_, Feedback, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Feedback".into()))?;

    Ok(feedback)
}
