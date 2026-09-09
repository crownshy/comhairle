use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "chat_instructions")]
pub struct ChatInstructions {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub target_reading_age: Option<i32>,
    pub max_length: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ChatInstructionsIden; 6] = [
    ChatInstructionsIden::Id,
    ChatInstructionsIden::ConversationId,
    ChatInstructionsIden::TargetReadingAge,
    ChatInstructionsIden::MaxLength,
    ChatInstructionsIden::CreatedAt,
    ChatInstructionsIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpsertChatInstructions {
    pub target_reading_age: Option<i32>,
    pub max_length: Option<i32>,
}

impl UpsertChatInstructions {
    fn columns(&self) -> Vec<ChatInstructionsIden> {
        let mut columns = vec![];

        if self.target_reading_age.is_some() {
            columns.push(ChatInstructionsIden::TargetReadingAge);
        }
        if self.max_length.is_some() {
            columns.push(ChatInstructionsIden::MaxLength);
        }

        columns
    }

    fn values(&self) -> Vec<SimpleExpr> {
        let mut values = vec![];

        if let Some(value) = self.target_reading_age {
            values.push(value.into());
        }
        if let Some(value) = self.max_length {
            values.push(value.into());
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn upsert_for_conversation(
    db: &PgPool,
    conversation_id: Uuid,
    payload: UpsertChatInstructions,
) -> Result<ChatInstructions, ComhairleError> {
    let mut columns = payload.columns();
    let mut values = payload.values();

    columns.push(ChatInstructionsIden::ConversationId);
    values.push(conversation_id.into());

    let (sql, values) = Query::insert()
        .into_table(ChatInstructionsIden::Table)
        .columns(columns)
        .values(values)?
        .on_conflict(
            OnConflict::column(ChatInstructionsIden::ConversationId)
                .update_columns(payload.columns())
                .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let chat_instructions = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(chat_instructions)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_conversation_id(
    db: &PgPool,
    conversation_id: Uuid,
) -> Result<ChatInstructions, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ChatInstructionsIden::Table)
        .and_where(Expr::col(ChatInstructionsIden::ConversationId).eq(conversation_id))
        .build_sqlx(PostgresQueryBuilder);

    let chat_instructions = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Chat Instructions")?;

    Ok(chat_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_instructions_for_conversation(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(17),
            max_length: Some(500),
        };

        let instructions = upsert_for_conversation(&pool, conversation_id, payload).await?;

        assert_eq!(
            instructions.conversation_id, conversation_id,
            "incorrect conversation_id"
        );
        assert_eq!(
            instructions.target_reading_age,
            Some(17),
            "incorrect reading age"
        );
        assert_eq!(instructions.max_length, Some(500), "incorrect max_length");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_instructions_if_already_exists(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(10),
            max_length: Some(100),
        };

        let new_instructions = upsert_for_conversation(&pool, conversation_id, payload).await?;

        assert_eq!(
            new_instructions.conversation_id, conversation_id,
            "incorrect conversation_id before update"
        );
        assert_eq!(
            new_instructions.target_reading_age,
            Some(10),
            "incorrect reading age before update"
        );
        assert_eq!(
            new_instructions.max_length,
            Some(100),
            "incorrect max_length before update"
        );

        let payload = UpsertChatInstructions {
            target_reading_age: Some(15),
            max_length: Some(200),
        };

        let updated_instructions = upsert_for_conversation(&pool, conversation_id, payload).await?;

        assert_eq!(
            updated_instructions.conversation_id, conversation_id,
            "incorrect conversation_id after update"
        );
        assert_eq!(
            updated_instructions.target_reading_age,
            Some(15),
            "incorrect reading age after update"
        );
        assert_eq!(
            updated_instructions.max_length,
            Some(200),
            "incorrect max_length after update"
        );
        assert_eq!(
            new_instructions.id, updated_instructions.id,
            "ids don't match for instructions"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_instructions_for_conversation(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let payload = UpsertChatInstructions {
            target_reading_age: Some(17),
            max_length: Some(500),
        };

        let new_instructions = upsert_for_conversation(&pool, conversation_id, payload).await?;

        let fetched_instructions = get_by_conversation_id(&pool, conversation_id).await?;

        assert_eq!(
            new_instructions.id, fetched_instructions.id,
            "ids don't match for instructions"
        );

        Ok(())
    }
}
