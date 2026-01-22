use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::{
    bot_service::ComhairleBotService, error::ComhairleError, models::conversation,
    routes::bot::chat_sessions::CreateChatSessionRequest,
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
    /// Reference to the workflow step if attached to a particular conversation step tool (i.e.
    /// elicitation bot)
    pub workflow_step_id: Option<Uuid>,
    /// Identifier of the session in bot service system
    pub bot_service_session_id: String,
    /// Timestamp when this session was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this session was last updated
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [BotServiceUserSessionIden; 7] = [
    BotServiceUserSessionIden::Id,
    BotServiceUserSessionIden::UserId,
    BotServiceUserSessionIden::ConversationId,
    BotServiceUserSessionIden::WorkflowStepId,
    BotServiceUserSessionIden::BotServiceSessionId,
    BotServiceUserSessionIden::CreatedAt,
    BotServiceUserSessionIden::UpdatedAt,
];

/// Data transfer object for creating a new bot service user session.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateBotServiceUserSession {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub workflow_step_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
struct CreateBotServiceUserSessionWithSessionId {
    conversation_id: Uuid,
    user_id: Uuid,
    bot_service_session_id: String,
    workflow_step_id: Option<Uuid>,
}

impl CreateBotServiceUserSessionWithSessionId {
    fn columns(&self) -> Vec<BotServiceUserSessionIden> {
        vec![
            BotServiceUserSessionIden::ConversationId,
            BotServiceUserSessionIden::UserId,
            BotServiceUserSessionIden::BotServiceSessionId,
            BotServiceUserSessionIden::WorkflowStepId,
        ]
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.conversation_id.into(),
            self.user_id.into(),
            self.bot_service_session_id.clone().into(),
            self.workflow_step_id.clone().into(),
        ]
    }
}

/// Creates a new user session for a conversation tied to a ragflow bot session.
///
/// # Arguments
///
/// * `db` - Database conncection pool
/// * `bot_service` - RAG based bot service provider
/// * `session` - request params containing `user_id` and `conversation_id`
///
/// # Returns
///
/// Returns a `Result` containing the created `BotServiceUserSession` or  a
/// `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * The database operation fails
/// * bot service request fails
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
        .ok_or_else(|| ComhairleError::NoConversationBotId)?;

    let (_, bot_service_session) = bot_service
        .create_chat_session(&chat_bot_id, create_chat_session)
        .await?;

    let session = CreateBotServiceUserSessionWithSessionId {
        conversation_id: session.conversation_id,
        user_id: session.user_id,
        bot_service_session_id: bot_service_session.id,
        workflow_step_id: session.workflow_step_id,
    };

    let columns = session.columns();
    let values = session.values();

    let (sql, values) = Query::insert()
        .into_table(BotServiceUserSessionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let bot_session_result = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(bot_session_result)
}

/// Data transfer object for creating a new bot service user session for an
/// elicitation bot workflow step on a conversation.
pub struct CreateWorkflowStepBotServiceUserSession {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub workflow_step_id: Uuid,
    pub agent_id: String,
}

/// Creates a new user session for an elicitation bot workflow step on a conversation,
/// tied to a ragflow agent session.
///
/// # Arguments
///
/// * `db` - Database conncection pool
/// * `bot_service` - RAG based bot service provider
/// * `session` - request params containing `user_id`, `conversation_id`, `workflow_step_id` and
/// `agent_id`
///
/// # Returns
///
/// Returns a `Result` containing the created `BotServiceUserSession` or  a
/// `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * The database operation fails
/// * bot service request fails
pub async fn create_workflow_step_session(
    db: &PgPool,
    bot_service: &Arc<dyn ComhairleBotService>,
    session: &CreateWorkflowStepBotServiceUserSession,
) -> Result<BotServiceUserSession, ComhairleError> {
    let (_, bot_service_session) = bot_service.create_agent_session(&session.agent_id).await?;

    let session = CreateBotServiceUserSessionWithSessionId {
        conversation_id: session.conversation_id,
        user_id: session.user_id,
        bot_service_session_id: bot_service_session.id,
        workflow_step_id: Some(session.workflow_step_id),
    };

    let columns = session.columns();
    let values = session.values();

    let (sql, values) = Query::insert()
        .into_table(BotServiceUserSessionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let bot_session_result = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(bot_session_result)
}

/// Retrieves a user bot session by user_id and conversation_id.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `user_id` - user's ID
/// * `conversation_id` - relevant conversation's ID
///
/// # Returns
///
/// Returns a `Result` containing the `BotServicerUserSession` if found or a
/// `ComhairleError` if not found.
pub async fn get_by_conversation_id(
    db: &PgPool,
    user_id: Uuid,
    conversation_id: Uuid,
) -> Result<BotServiceUserSession, ComhairleError> {
    let (sql, values) = Query::select()
        .from(BotServiceUserSessionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::ConversationId,
            ))
            .eq(conversation_id.to_owned()),
        )
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::UserId,
            ))
            .eq(user_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let bot_session = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::NoBotUserSession,
            _ => e.into(),
        })?;

    Ok(bot_session)
}

// Data transfer object for bot service user session
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct BotServiceUserSessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub bot_service_session_id: String,
    pub workflow_step_id: Option<Uuid>,
}

impl From<BotServiceUserSession> for BotServiceUserSessionDto {
    fn from(s: BotServiceUserSession) -> Self {
        Self {
            id: s.id,
            conversation_id: s.conversation_id,
            user_id: s.user_id,
            bot_service_session_id: s.bot_service_session_id,
            workflow_step_id: s.workflow_step_id,
        }
    }
}
