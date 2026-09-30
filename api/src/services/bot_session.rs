//! Bot service session orchestration.
//!
//! Getting or creating a session reaches into the bot service to open the remote
//! chat session, so it needs [`ComhairleState`]. The row and its queries stay in
//! [`crate::models::bot_service_user_session`].

use std::sync::Arc;

use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::bot_service::{ComhairleBotService, CreateChatSessionRequest};
use crate::config::ComhairleConfig;
use crate::error::ServiceError;
use crate::models::bot_service_user_session::{
    BotServiceSessionContext, BotServiceUserSession, BotServiceUserSessionIden,
    CreateBotServiceUserSession, CreateBotServiceUserSessionWithSessionId, DEFAULT_COLUMNS,
};
use crate::models::error::WorkflowError;
use crate::models::error::{ConversationError, DataError};
use crate::models::{conversation, workflow_step};
use crate::tools::ToolConfig;
use crate::{ComhairleState, error::ComhairleError};

/// * `context` - type of bot service session
/// * `user_id` - user's ID
/// * `conversation_id` - conversation's ID if for `qa_bot` type
/// * `workflow_step_id` - workflow_step's ID if for `elicitation_bot` type
///
/// # Returns
///
/// Returns a `Result` containing the `BotServiceUserSession` or a `ComhairleError` if a database
/// error occurs.
#[instrument(err(Debug), skip(state))]
pub async fn get_or_create(
    state: &ComhairleState,
    context: BotServiceSessionContext,
    user_id: &Uuid,
    conversation_id: Option<&Uuid>,
    workflow_step_id: Option<&Uuid>,
) -> Result<BotServiceUserSession, ComhairleError> {
    let bot_service = state.required_bot_service()?;

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
            create(&state.db, bot_service, &state.config, &create_session).await?
        }
    };

    Ok(session)
}

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
#[instrument(err(Debug), skip(bot_service))]
pub async fn create(
    db: &PgPool,
    bot_service: &Arc<dyn ComhairleBotService>,
    config: &ComhairleConfig,
    session: &CreateBotServiceUserSession,
) -> Result<BotServiceUserSession, ComhairleError> {
    let bot_service_config = config
        .bot_service
        .as_ref()
        .ok_or(ServiceError::NoBotServiceConfigured)?;

    let bot_service_session_id = match session.context {
        BotServiceSessionContext::QaBot => {
            let conversation_id = match session.conversation_id {
                Some(id) => id,
                None => {
                    return Err(DataError::CorruptedData(
                        "Missing conversation_id for qa_bot session".to_string(),
                    )
                    .into());
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
                .ok_or_else(|| ConversationError::NoConversationBotId)?;

            let (_, bot_service_session) = bot_service
                .create_chat_session(&chat_bot_id, create_chat_session)
                .await?;

            bot_service_session.id
        }
        BotServiceSessionContext::ElicitationBot => {
            let workflow_step_id = match session.workflow_step_id {
                Some(id) => id,
                None => {
                    return Err(DataError::CorruptedData(
                        "Missing workflow_step_id for elicitation_bot session".to_string(),
                    )
                    .into());
                }
            };

            let workflow_step = workflow_step::get_by_id(db, &workflow_step_id).await?;

            // TODO: think a bit harder here about if this is in preview mode or not
            let _tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config)
            {
                (Some(ToolConfig::ElicitationBot(config)), _) => config,
                (None, ToolConfig::ElicitationBot(config)) => config,
                _ => {
                    return Err(WorkflowError::ToolConfigError(
                        "Incorrect config type".to_string(),
                    )
                    .into());
                }
            };

            let (_, bot_service_session) = bot_service
                .create_agent_session(&bot_service_config.elicitation_bot_agent_id)
                .await?;

            bot_service_session.id
        }
        BotServiceSessionContext::ThinkingSpace => {
            let workflow_step_id = match session.workflow_step_id {
                Some(id) => id,
                None => {
                    return Err(DataError::CorruptedData(
                        "Missing workflow_step_id for thinking_space session".to_string(),
                    )
                    .into());
                }
            };

            let workflow_step = workflow_step::get_by_id(db, &workflow_step_id).await?;

            // TODO: think a bit harder here about if this is in preview mode or not
            let _tool_config = match (workflow_step.tool_config, workflow_step.preview_tool_config)
            {
                (Some(ToolConfig::ThinkingSpace(config)), _) => config,
                (None, ToolConfig::ThinkingSpace(config)) => config,
                _ => {
                    return Err(WorkflowError::ToolConfigError(
                        "Incorrect config type".to_string(),
                    )
                    .into());
                }
            };

            let (_, bot_service_session) = bot_service
                .create_agent_session(&bot_service_config.thinking_space_agent_id)
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
