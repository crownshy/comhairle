use std::sync::Arc;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::{
    bot_service::{ComhairleBotService, CreateChatSessionRequest},
    config::ComhairleConfig,
    error::ComhairleError,
    models::{conversation, workflow_step},
    tools::ToolConfig,
    ComhairleState,
};

#[cfg(test)]
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
#[cfg_attr(test, derive(Dummy))]
pub enum BotServiceSessionContext {
    #[sqlx(rename = "qa_bot")]
    QaBot,
    #[sqlx(rename = "elicitation_bot")]
    ElicitationBot,
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
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [BotServiceUserSessionIden; 8] = [
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
struct CreateBotServiceUserSessionWithSessionId {
    context: BotServiceSessionContext,
    user_id: Uuid,
    bot_service_session_id: String,
    conversation_id: Option<Uuid>,
    workflow_step_id: Option<Uuid>,
}

impl CreateBotServiceUserSessionWithSessionId {
    fn columns(&self) -> Vec<BotServiceUserSessionIden> {
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

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
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
/// * `config` - Comhairle config state
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
    config: &ComhairleConfig,
    session: &CreateBotServiceUserSession,
) -> Result<BotServiceUserSession, ComhairleError> {
    let elicitation_bot_agent_id = match &config.elicitation_bot_agent_id {
        Some(e_id) => e_id,
        None => return Err(ComhairleError::UninitializedBotService),
    };

    let bot_service_session_id = match session.context {
        BotServiceSessionContext::QaBot => {
            let conversation_id = match session.conversation_id {
                Some(id) => id,
                None => {
                    return Err(ComhairleError::CorruptedData(
                        "Missing conversation_id for qa_bot session".to_string(),
                    ))
                }
            };
            // TODO: need to make this default to the local of the conversation
            let conversation =
                conversation::get_localised_by_id(db, &conversation_id, "en").await?;

            let create_chat_session = CreateChatSessionRequest {
                name: conversation.title.clone(),
            };

            let chat_bot_id = conversation
                .chat_bot_id
                .ok_or_else(|| ComhairleError::NoConversationBotId)?;

            let (_, bot_service_session) = bot_service
                .create_chat_session(&chat_bot_id, create_chat_session)
                .await?;

            bot_service_session.id
        }
        BotServiceSessionContext::ElicitationBot => {
            let workflow_step_id = match session.workflow_step_id {
                Some(id) => id,
                None => {
                    return Err(ComhairleError::CorruptedData(
                        "Missing workflow_step_id for elicitation_bot session".to_string(),
                    ))
                }
            };

            let workflow_step = workflow_step::get_by_id(db, &workflow_step_id).await?;

            // TODO: think a bit harder here about if this is in preview mode or not
            let _tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config)
            {
                (Some(ToolConfig::ElicitationBot(config)), _) => config,
                (None, ToolConfig::ElicitationBot(config)) => config,
                _ => {
                    return Err(ComhairleError::ToolConfigError(
                        "Incorrect config type".to_string(),
                    ))
                }
            };

            let (_, bot_service_session) = bot_service
                .create_agent_session(elicitation_bot_agent_id)
                .await?;

            bot_service_session.id
        }
    };

    let session = CreateBotServiceUserSessionWithSessionId {
        context: session.context.clone(),
        user_id: session.user_id,
        conversation_id: session.conversation_id,
        workflow_step_id: session.workflow_step_id,
        bot_service_session_id,
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
/// Returns a `Result` containing the `BotServiceUserSession` if found or a
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
/// `ComhairleError` if not found.
pub async fn get_by_workflow_step_id(
    db: &PgPool,
    user_id: Uuid,
    workflow_step_id: Uuid,
) -> Result<BotServiceUserSession, ComhairleError> {
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
            sqlx::Error::RowNotFound => ComhairleError::NoBotUserSession,
            _ => e.into(),
        })?;

    Ok(bot_session)
}

/// Retrieves a user bot session by context, user_id and either conversation_id or
/// workflow_step_id. Will create and return a new entry if none found.
///
/// # Arguments
///
/// * `state` - Comhairle state, including Database connection pool, bot service implementation and
///   config
/// * `context` - type of bot service session
/// * `user_id` - user's ID
/// * `conversation_id` - conversation's ID if for `qa_bot` type
/// * `workflow_step_id` - workflow_step's ID if for `elicitation_bot` type
///
/// # Returns
///
/// Returns a `Result` containing the `BotServiceUserSession` or a `ComhairleError` if a database
/// error occurs.
pub async fn get_or_create(
    state: &ComhairleState,
    context: BotServiceSessionContext,
    user_id: &Uuid,
    conversation_id: Option<&Uuid>,
    workflow_step_id: Option<&Uuid>,
) -> Result<BotServiceUserSession, ComhairleError> {
    let bot_service = match &state.bot_service {
        Some(bs) => bs,
        None => return Err(ComhairleError::UninitializedBotService),
    };

    let mut query = Query::select();
    query
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
                BotServiceUserSessionIden::Context,
            ))
            .eq(context.to_owned()),
        );

    if let Some(conversation_id) = conversation_id {
        query.and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::ConversationId,
            ))
            .eq(conversation_id.to_owned()),
        );
    }

    if let Some(workflow_step_id) = workflow_step_id {
        query.and_where(
            Expr::col((
                BotServiceUserSessionIden::Table,
                BotServiceUserSessionIden::WorkflowStepId,
            ))
            .eq(workflow_step_id.to_owned()),
        );
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let session = match sqlx::query_as_with(&sql, values)
        .fetch_optional(&state.db)
        .await?
    {
        Some(session) => session,
        None => {
            let create_session = CreateBotServiceUserSession {
                context,
                user_id: *user_id,
                conversation_id: conversation_id.copied(),
                workflow_step_id: workflow_step_id.copied(),
            };
            create(&state.db, &bot_service, &state.config, &create_session).await?
        }
    };

    Ok(session)
}

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
