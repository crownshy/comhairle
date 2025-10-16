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
#[enum_def(table_name = "user_participation")]
pub struct UserParticipation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workflow_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [UserParticipationIden; 5] = [
    UserParticipationIden::Id,
    UserParticipationIden::UserId,
    UserParticipationIden::WorkflowId,
    UserParticipationIden::CreatedAt,
    UserParticipationIden::UpdatedAt,
];

pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<UserParticipation, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(UserParticipationIden::Table)
        .columns([
            UserParticipationIden::UserId,
            UserParticipationIden::WorkflowId,
        ])
        .values([user_id.to_owned().into(), workflow_id.to_owned().into()])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserParticipation, _>(&sql, values)
        .fetch_one(db)
        .await;

    match result {
        Ok(result) => Ok(result),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                return Err(ComhairleError::UserAlreadyParticipatingInWorkflow(
                    workflow_id.to_string(),
                ));
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

pub async fn get(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<Option<UserParticipation>, ComhairleError> {
    let (sql, values) = Query::select()
        .from(UserParticipationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(UserParticipationIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(workflow_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, UserParticipation, _>(&sql, values)
        .fetch_optional(db)
        .await?;
    Ok(result)
}

pub async fn delete(
    db: &PgPool,
    user_id: &Uuid,
    workflow_id: &Uuid,
) -> Result<UserParticipation, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(UserParticipationIden::Table)
        .and_where(Expr::col(UserParticipationIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(workflow_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let user_participation = sqlx::query_as_with::<_, UserParticipation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserParticipation".into()))?;

    Ok(user_participation)
}

pub async fn get_participant_user_ids_for_conversation(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<Uuid>, ComhairleError> {
    use crate::models::workflow::WorkflowIden;

    let (sql, values) = Query::select()
        .from(UserParticipationIden::Table)
        .column(UserParticipationIden::UserId)
        .join(
            sea_query::JoinType::InnerJoin,
            WorkflowIden::Table,
            Expr::col((WorkflowIden::Table, WorkflowIden::Id)).equals((
                UserParticipationIden::Table,
                UserParticipationIden::WorkflowId,
            )),
        )
        .and_where(
            Expr::col((WorkflowIden::Table, WorkflowIden::ConversationId))
                .eq(conversation_id.to_owned()),
        )
        .distinct()
        .build_sqlx(PostgresQueryBuilder);

    let user_ids: Vec<(Uuid,)> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

    Ok(user_ids.into_iter().map(|(id,)| id).collect())
}
