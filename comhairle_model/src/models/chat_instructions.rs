use crate::models::error::ModelError;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

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

pub trait ChatInstructionsExt {
    fn to_prompt_variables(&self) -> HashMap<String, String>;
}

impl ChatInstructionsExt for Option<ChatInstructions> {
    fn to_prompt_variables(&self) -> HashMap<String, String> {
        let mut var_map = HashMap::new();

        // Extend with other fields as prompt requirements change
        var_map.insert(
            "target_reading_age".to_string(),
            self.as_ref()
                .and_then(|inst| inst.target_reading_age)
                .unwrap_or(9)
                .to_string(),
        );

        var_map
    }
}

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
    payload: &UpsertChatInstructions,
) -> Result<ChatInstructions, ModelError> {
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
) -> Result<ChatInstructions, ModelError> {
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
