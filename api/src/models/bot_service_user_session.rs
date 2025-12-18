use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    bot_service::ComhairleBotService, error::ComhairleError, models::conversation,
    routes::bot::sessions::CreateChatSessionRequest,
};

#[derive(Serialize, Deserialize, FromRow, JsonSchema, Debug, Clone)]
#[enum_def(table_name = "bot_service_user_session")]
pub struct BotServiceUserSession {
    /// Unique indentifier for this session
    pub id: Uuid,
    /// Reference to the user the session belongs to
    pub user_id: Uuid,
    /// Reference to the conversation the chat session belongs to
    pub conversation_id: Uuid,
    /// Identifier of the session in bot service system
    pub bot_service_session_id: String,
    /// Timestamp when this session was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this session was last updated
    pub updated_at: DateTime<Utc>,
}

const BOT_SERVICE_USER_SESSION_DEFAULT_COLUMNS: [BotServiceUserSessionIden; 6] = [
    BotServiceUserSessionIden::Id,
    BotServiceUserSessionIden::UserId,
    BotServiceUserSessionIden::ConversationId,
    BotServiceUserSessionIden::BotServiceSessionId,
    BotServiceUserSessionIden::CreatedAt,
    BotServiceUserSessionIden::UpdatedAt,
];

/// Data transfer object for creating a new bot service user session.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateBotServiceUserSession {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
struct CreateBotServiceUserSessionWithSessionId {
    conversation_id: Uuid,
    user_id: Uuid,
    bot_service_session_id: String,
}

impl CreateBotServiceUserSessionWithSessionId {
    fn columns(&self) -> Vec<BotServiceUserSessionIden> {
        vec![
            BotServiceUserSessionIden::ConversationId,
            BotServiceUserSessionIden::UserId,
            BotServiceUserSessionIden::BotServiceSessionId,
        ]
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.conversation_id.into(),
            self.user_id.into(),
            self.bot_service_session_id.clone().into(),
        ]
    }
}

// #[instrument(err(Debug))] // TODO: can't add because of ComhairleBotService
pub async fn create(
    db: &PgPool,
    bot_service: &Arc<dyn ComhairleBotService>,
    session: &CreateBotServiceUserSession,
) -> Result<BotServiceUserSession, ComhairleError> {
    let conversation = conversation::get_localised_by_id(db, &session.conversation_id).await?;

    let create_chat_session = CreateChatSessionRequest {
        name: conversation.title.clone(),
        ..Default::default()
    };

    let chat_bot_id = conversation
        .chat_bot_id
        // TODO: use a different error here
        .ok_or_else(|| {
            ComhairleError::DbError("Missing chat_bot_id on conversation".to_string())
        })?;

    let (_, bot_service_session) = bot_service
        .create_chat_session(&chat_bot_id, create_chat_session)
        .await?;

    let session = CreateBotServiceUserSessionWithSessionId {
        conversation_id: session.conversation_id,
        user_id: session.user_id,
        bot_service_session_id: bot_service_session.id,
    };

    let columns = session.columns();
    let values = session.values();

    let (sql, values) = Query::insert()
        .into_table(BotServiceUserSessionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(BOT_SERVICE_USER_SESSION_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let bot_session_result = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(bot_session_result)
}

// Data transfer object for bot service user session
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct BotServiceUserSessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub bot_service_session_id: String,
}

impl From<BotServiceUserSession> for BotServiceUserSessionDto {
    fn from(s: BotServiceUserSession) -> Self {
        Self {
            id: s.id,
            conversation_id: s.conversation_id,
            user_id: s.user_id,
            bot_service_session_id: s.bot_service_session_id,
        }
    }
}
