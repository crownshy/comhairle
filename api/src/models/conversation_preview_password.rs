use chrono::{DateTime, Utc};
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use sqlx::prelude::FromRow;
use sqlx::{PgPool, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Debug, FromRow, Clone)]
#[enum_def(table_name = "conversation_preview_password")]
pub struct ConversationPreviewPassword {
    pub conversation_id: Uuid,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ConversationPreviewPasswordIden; 4] = [
    ConversationPreviewPasswordIden::ConversationId,
    ConversationPreviewPasswordIden::PasswordHash,
    ConversationPreviewPasswordIden::CreatedAt,
    ConversationPreviewPasswordIden::UpdatedAt,
];

/// Sets or replaces the password. Replacing it moves `updated_at`, which is what
/// invalidates every preview grant issued against the old one.
#[instrument(err(Debug), skip(db, password_hash))]
pub async fn upsert(
    db: &PgPool,
    conversation_id: Uuid,
    password_hash: String,
) -> Result<ConversationPreviewPassword, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(ConversationPreviewPasswordIden::Table)
        .columns([
            ConversationPreviewPasswordIden::ConversationId,
            ConversationPreviewPasswordIden::PasswordHash,
        ])
        .values([conversation_id.into(), password_hash.into()])?
        .on_conflict(
            OnConflict::column(ConversationPreviewPasswordIden::ConversationId)
                .update_column(ConversationPreviewPasswordIden::PasswordHash)
                .value(
                    ConversationPreviewPasswordIden::UpdatedAt,
                    Expr::current_timestamp(),
                )
                .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let row = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(row)
}

#[instrument(err(Debug), skip(db))]
pub async fn get(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<Option<ConversationPreviewPassword>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationPreviewPasswordIden::Table)
        .and_where(Expr::col(ConversationPreviewPasswordIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let row = query_as_with(&sql, values).fetch_optional(db).await?;

    Ok(row)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, conversation_id: Uuid) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ConversationPreviewPasswordIden::Table)
        .and_where(Expr::col(ConversationPreviewPasswordIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;

    Ok(())
}
