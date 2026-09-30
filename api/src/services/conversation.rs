//! Conversation lifecycle.
//!
//! Creating and deleting a conversation also provisions and tears down its chat
//! bot, so these need the bot service and app config. The conversation row and
//! its queries stay in [`crate::models::conversation`].

use std::sync::Arc;

use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use slugify::slugify;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::bot_service::{
    ComhairleBotService, ComhairlePrompt, CreateChatRequest, DEFAULT_CHAT_NOT_FOUND_RESPONSE,
    DEFAULT_CHAT_OPENER, DEFAULT_CHAT_PROMPT, Variable,
};
use crate::config::ComhairleConfig;
use crate::error::ComhairleError;
use crate::models::SqlxResultExt;
use crate::models::conversation::{
    Conversation, ConversationIden, CreateConversation, DEFAULT_COLUMNS,
};
use crate::models::error::{ConversationError, DataError};
use crate::models::translations::{TextFormat, new_translation};

#[instrument(err(Debug), skip(db, bot_service))]
pub async fn delete(
    db: &PgPool,
    bot_service: &Option<Arc<dyn ComhairleBotService>>,
    id: &Uuid,
) -> Result<Conversation, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Conversation")?;

    if let Some(bot_service) = bot_service {
        if let Some(ref knowledge_base_id) = conversation.knowledge_base_id {
            let _ = bot_service.delete_knowledge_base(knowledge_base_id).await?;
        }

        if let Some(ref chat_bot_id) = conversation.chat_bot_id {
            let _ = bot_service.delete_chat(chat_bot_id).await?;
        }
    }

    Ok(conversation)
}

#[instrument(err(Debug), skip(db, bot_service))]
pub async fn create(
    db: &PgPool,
    bot_service: &Option<Arc<dyn ComhairleBotService>>,
    config: &ComhairleConfig,
    conversation: &CreateConversation,
    owner_id: Uuid,
    organization_id: Option<Uuid>,
) -> Result<Conversation, ComhairleError> {
    let conversation_id = Uuid::new_v4();

    // Generate Translations
    let title = new_translation(
        db,
        &conversation.primary_locale,
        &conversation.title,
        TextFormat::Plain,
    )
    .await?;

    let description = new_translation(
        db,
        &conversation.primary_locale,
        &conversation.description,
        TextFormat::Rich,
    )
    .await?;

    let short_description = new_translation(
        db,
        &conversation.primary_locale,
        &conversation.short_description,
        TextFormat::Rich,
    )
    .await?;

    let mut columns = conversation.columns();
    let mut values = conversation.values();

    if let (Some(bot_service), Some(bot_service_config)) = (bot_service, &config.bot_service) {
        let (_, knowledge_base) = bot_service
            .create_knowledge_base(conversation_id.to_string(), None)
            .await?;

        let create_chat = CreateChatRequest {
            name: conversation_id.to_string(),
            knowledge_base_ids: Some(vec![bot_service_config.default_knowledge_base_id.clone()]),
            prompt: Some(ComhairlePrompt {
                llm_prompt: Some(DEFAULT_CHAT_PROMPT.to_string()),
                opener: Some(DEFAULT_CHAT_OPENER.to_string()),
                empty_response: Some(DEFAULT_CHAT_NOT_FOUND_RESPONSE.to_string()),
                cross_languages: None,
                variables: Some(vec![
                    Variable {
                        key: "knowledge".to_string(),
                        optional: Some(false),
                    },
                    Variable {
                        key: "target_reading_age".to_string(),
                        optional: Some(false),
                    },
                ]),
            }),
            ..Default::default()
        };

        let (_, chat) = bot_service.create_chat(create_chat).await?;

        columns.push(ConversationIden::KnowledgeBaseId);
        values.push(knowledge_base.id.into());

        columns.push(ConversationIden::ChatBotId);
        values.push(chat.id.into());

        if let Some(enable_qa_chat_bot) = conversation.enable_qa_chat_bot {
            columns.push(ConversationIden::EnableQaChatBot);
            values.push(enable_qa_chat_bot.into());
        }
    }

    columns.push(ConversationIden::Title);
    values.push(title.id.into());

    columns.push(ConversationIden::Description);
    values.push(description.id.into());

    columns.push(ConversationIden::ShortDescription);
    values.push(short_description.id.into());

    // Generate Slug

    let slug = conversation
        .slug
        .to_owned()
        .unwrap_or_else(|| slugify!(&conversation.title));

    columns.push(ConversationIden::Slug);
    values.push(slug.clone().into());

    columns.push(ConversationIden::Id);
    values.push(conversation_id.into());

    columns.push(ConversationIden::IsComplete);
    values.push(false.into());

    columns.push(ConversationIden::OwnerId);
    values.push(owner_id.into());

    if let Some(default_workflow_id) = conversation.default_workflow_id {
        columns.push(ConversationIden::DefaultWorkflowId);
        values.push(default_workflow_id.into());
    }

    if let Some(org_id) = organization_id {
        columns.push(ConversationIden::OrganizationId);
        values.push(org_id.into());
    }

    let (sql, values) = Query::insert()
        .into_table(ConversationIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let conversation_result = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await;

    match conversation_result {
        Ok(conversation) => Ok(conversation),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505"
                && let Some(constraint) = pg_err.constraint()
                && constraint.contains("slug")
            {
                return Err(ConversationError::DuplicateSlug(slug).into());
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}
