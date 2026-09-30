use crate::models::error::ModelError;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::ConversationError;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

#[derive(Serialize, Deserialize, FromRow, JsonSchema, Debug, Clone)]
#[enum_def(table_name = "bot_service_user_session")]
pub struct BotServiceUserSession {
    /// Unique indentifier for this session
    pub id: Uuid,
    /// Reference to the user the session belongs to
    pub user_id: Uuid,
    /// Determines session type for bot service
    pub context: String,
    /// Reference to the conversation the chat session belongs to if `context` is `qa_bot`
    pub conversation_id: Option<Uuid>,
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub enum BotServiceSessionContext {
    #[sqlx(rename = "qa_bot")]
    QaBot,
    #[sqlx(rename = "elicitation_bot")]
    ElicitationBot,
    #[sqlx(rename = "thinking_space")]
    ThinkingSpace,
}

impl From<BotServiceSessionContext> for sea_query::Value {
    fn from(session: BotServiceSessionContext) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(session.to_string())))
    }
}

impl std::fmt::Display for BotServiceSessionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BotServiceSessionContext::QaBot => "qa_bot",
            BotServiceSessionContext::ElicitationBot => "elicitation_bot",
            BotServiceSessionContext::ThinkingSpace => "thinking_space",
        };
        write!(f, "{}", value)
    }
}

pub const DEFAULT_COLUMNS: [BotServiceUserSessionIden; 8] = [
    BotServiceUserSessionIden::Id,
    BotServiceUserSessionIden::UserId,
    BotServiceUserSessionIden::Context,
    BotServiceUserSessionIden::ConversationId,
    BotServiceUserSessionIden::WorkflowStepId,
    BotServiceUserSessionIden::BotServiceSessionId,
    BotServiceUserSessionIden::CreatedAt,
    BotServiceUserSessionIden::UpdatedAt,
];

/// Data transfer object for creating a new bot service user session.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateBotServiceUserSession {
    pub context: BotServiceSessionContext,
    pub user_id: Uuid,
    pub conversation_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateBotServiceUserSessionWithSessionId {
    pub context: BotServiceSessionContext,
    pub user_id: Uuid,
    pub bot_service_session_id: String,
    pub conversation_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
}

impl CreateBotServiceUserSessionWithSessionId {
    pub fn columns(&self) -> Vec<BotServiceUserSessionIden> {
        let mut columns = vec![
            BotServiceUserSessionIden::Context,
            BotServiceUserSessionIden::UserId,
            BotServiceUserSessionIden::BotServiceSessionId,
        ];

        if self.conversation_id.is_some() {
            columns.push(BotServiceUserSessionIden::ConversationId);
        }
        if self.workflow_step_id.is_some() {
            columns.push(BotServiceUserSessionIden::WorkflowStepId);
        }

        columns
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values: Vec<sea_query::SimpleExpr> = vec![
            self.context.clone().into(),
            self.user_id.into(),
            self.bot_service_session_id.clone().into(),
        ];

        if let Some(value) = self.conversation_id {
            values.push(value.into());
        }
        if let Some(value) = self.workflow_step_id {
            values.push(value.into());
        }

        values
    }
}

/// Creates a new user session for a conversation tied to a ragflow bot session.
///
/// # Arguments
///
/// * `db` - Database conncection pool
/// * `bot_service` - RAG based bot service provider

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
/// Returns a `Result` containing the `BotServiceUserSession` if found or a
/// `ModelError` if not found.
#[instrument(err(Debug), skip(db))]
pub async fn get_by_conversation_id(
    db: &PgPool,
    user_id: Uuid,
    conversation_id: Uuid,
) -> Result<BotServiceUserSession, ModelError> {
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
            sqlx::Error::RowNotFound => ConversationError::NoBotUserSession,
            _ => e.into(),
        })?;

    Ok(bot_session)
}

/// Retrieves a user bot session by user_id and workflow_step_id.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `user_id` = user's ID
/// * `workflow_step_id` - relevant workflow_step's ID
///
/// # Returns
///
/// Returns a `Result` containing the `BotServiceUserSession` if found or a
/// `ModelError` if not found.
#[instrument(err(Debug), skip(db))]
pub async fn get_by_workflow_step_id(
    db: &PgPool,
    user_id: Uuid,
    workflow_step_id: Uuid,
) -> Result<BotServiceUserSession, ModelError> {
    let (sql, values) = Query::select()
        .from(BotServiceUserSessionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::UserId,
            ))
            .eq(user_id.to_owned()),
        )
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let bot_session = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ConversationError::NoBotUserSession,
            _ => e.into(),
        })?;

    Ok(bot_session)
}

/// Lists bot sessions by workflow step id and context.
#[instrument(err(Debug), skip(db))]
pub async fn list_by_workflow_step_id_and_context(
    db: &PgPool,
    workflow_step_id: &Uuid,
    context: BotServiceSessionContext,
) -> Result<Vec<BotServiceUserSession>, ModelError> {
    let (sql, values) = Query::select()
        .from(BotServiceUserSessionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        )
        .and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::Context,
            ))
            .eq(context),
        )
        .build_sqlx(PostgresQueryBuilder);

    let sessions = sqlx::query_as_with::<_, BotServiceUserSession, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(sessions)
}

/// Deletes bot sessions by local table id.
#[instrument(err(Debug), skip(db))]
pub async fn delete_by_ids(db: &PgPool, ids: &[Uuid]) -> Result<u64, ModelError> {
    if ids.is_empty() {
        return Ok(0);
    }

    let (sql, values) = Query::delete()
        .from_table(BotServiceUserSessionIden::Table)
        .and_where(Expr::col(BotServiceUserSessionIden::Id).is_in(ids.iter().copied()))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(db).await?;

    Ok(result.rows_affected())
}

/// Retrieves a user bot session by context, user_id and either conversation_id or
/// workflow_step_id. Will create and return a new entry if none found.
///
/// # Arguments
///
/// * `state` - Comhairle state, including Database connection pool, bot service implementation and
///   config

// Data transfer object for bot service user session
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct BotServiceUserSessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub context: String,
    pub conversation_id: Option<Uuid>,
    pub workflow_step_id: Option<Uuid>,
    pub bot_service_session_id: String,
}

impl From<BotServiceUserSession> for BotServiceUserSessionDto {
    fn from(s: BotServiceUserSession) -> Self {
        Self {
            id: s.id,
            user_id: s.user_id,
            context: s.context,
            conversation_id: s.conversation_id,
            workflow_step_id: s.workflow_step_id,
            bot_service_session_id: s.bot_service_session_id,
        }
    }
}
