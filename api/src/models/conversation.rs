use std::sync::Arc;

use super::{
    pagination::{Order, PageOptions, PaginatedResults},
    translations::{TextContentId, TextFormat, new_translation},
    user_participation::UserParticipationIden,
    workflow::WorkflowIden,
};
use crate::ComhairleState;
use crate::bot_service::{
    ComhairleBotService, ComhairlePrompt, CreateChatRequest, DEFAULT_CHAT_NOT_FOUND_RESPONSE,
    DEFAULT_CHAT_OPENER, DEFAULT_CHAT_PROMPT,
};
use crate::config::ComhairleConfig;
use crate::error::ComhairleError;
use crate::models::permissions::{Action, ResourcePermissionIden, ResourceType, Role};
use crate::models::{self, SqlxResultExt};
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{
    Cond, Expr, JoinType, PostgresQueryBuilder, Query, enum_def, extension::postgres::PgExpr,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

/// For extracting an id or slug from Path
#[derive(Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum IdOrSlug {
    Id(Uuid),
    Slug(String),
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "conversation")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
pub struct Conversation {
    #[partially(omit)]
    pub id: Uuid,
    pub title: TextContentId,
    pub short_description: TextContentId,
    pub description: TextContentId,
    #[partially(transparent)]
    pub video_url: Option<String>,
    #[partially(transparent)]
    pub image: Option<Uuid>,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_complete: bool,
    #[partially(omit)]
    pub owner_id: Uuid,
    #[partially(omit)]
    pub organization_id: Option<Uuid>,
    pub is_invite_only: bool,
    #[partially(transparent)]
    pub slug: Option<String>,
    #[partially(transparent)]
    pub default_workflow_id: Option<Uuid>,
    pub primary_locale: String,
    pub knowledge_base_id: Option<String>,
    pub chat_bot_id: Option<String>,
    pub enable_qa_chat_bot: bool,
    pub supported_languages: Vec<String>,
    #[partially(transparent)]
    pub privacy_policy: Option<TextContentId>,
    #[partially(transparent)]
    pub short_privacy_policy: Option<TextContentId>,
    #[partially(transparent)]
    pub faqs: Option<TextContentId>,
    #[partially(transparent)]
    pub thank_you_message: Option<TextContentId>,
    #[partially(transparent)]
    pub call_to_action: Option<TextContentId>,
    pub enable_signup_prompts: bool,
    pub show_thank_you_page_annon_instructions: bool,
    pub metadata: serde_json::Value,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ConversationIden; 30] = [
    ConversationIden::Id,
    ConversationIden::Title,
    ConversationIden::ShortDescription,
    ConversationIden::Description,
    ConversationIden::VideoUrl,
    ConversationIden::Image,
    ConversationIden::Tags,
    ConversationIden::IsPublic,
    ConversationIden::IsLive,
    ConversationIden::IsComplete,
    ConversationIden::IsInviteOnly,
    ConversationIden::Slug,
    ConversationIden::DefaultWorkflowId,
    ConversationIden::PrimaryLocale,
    ConversationIden::KnowledgeBaseId,
    ConversationIden::ChatBotId,
    ConversationIden::EnableQaChatBot,
    ConversationIden::SupportedLanguages,
    ConversationIden::CreatedAt,
    ConversationIden::UpdatedAt,
    ConversationIden::OwnerId,
    ConversationIden::OrganizationId,
    ConversationIden::PrivacyPolicy,
    ConversationIden::ShortPrivacyPolicy,
    ConversationIden::Faqs,
    ConversationIden::ThankYouMessage,
    ConversationIden::CallToAction,
    ConversationIden::EnableSignupPrompts,
    ConversationIden::ShowThankYouPageAnnonInstructions,
    ConversationIden::Metadata,
];

impl PartialConversation {
    pub fn to_values(&self) -> Vec<(ConversationIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.title {
            values.push((ConversationIden::Title, value.into()))
        };
        if let Some(value) = &self.short_description {
            values.push((ConversationIden::ShortDescription, value.into()))
        };
        if let Some(value) = &self.description {
            values.push((ConversationIden::Description, value.into()))
        };
        if let Some(value) = &self.video_url {
            values.push((ConversationIden::VideoUrl, value.into()))
        };
        if let Some(value) = &self.image {
            values.push((ConversationIden::Image, (*value).into()))
        };
        if let Some(value) = &self.tags {
            values.push((
                ConversationIden::Tags,
                sea_query::Value::Array(
                    sea_query::ArrayType::String,
                    Some(Box::new(value.iter().map(sea_query::Value::from).collect())),
                )
                .into(),
            ))
        };
        if let Some(value) = self.is_public {
            values.push((ConversationIden::IsPublic, value.into()))
        };
        if let Some(value) = self.is_live {
            values.push((ConversationIden::IsLive, value.into()))
        };
        if let Some(value) = self.is_complete {
            values.push((ConversationIden::IsComplete, value.into()))
        };
        if let Some(value) = self.is_invite_only {
            values.push((ConversationIden::IsInviteOnly, value.into()))
        };
        if let Some(value) = &self.slug {
            values.push((ConversationIden::Slug, value.into()))
        };
        if let Some(value) = &self.default_workflow_id {
            values.push((ConversationIden::DefaultWorkflowId, (*value).into()))
        };
        if let Some(value) = &self.primary_locale {
            values.push((ConversationIden::PrimaryLocale, value.into()))
        };
        if let Some(value) = &self.knowledge_base_id {
            values.push((ConversationIden::KnowledgeBaseId, value.clone().into()))
        };
        if let Some(value) = &self.enable_qa_chat_bot {
            values.push((ConversationIden::EnableQaChatBot, (*value).into()))
        };
        if let Some(value) = &self.privacy_policy {
            values.push((ConversationIden::PrivacyPolicy, (*value).into()))
        };
        if let Some(value) = &self.short_privacy_policy {
            values.push((ConversationIden::ShortPrivacyPolicy, (*value).into()))
        };
        if let Some(value) = &self.faqs {
            values.push((ConversationIden::Faqs, (*value).into()))
        };
        if let Some(value) = &self.thank_you_message {
            values.push((ConversationIden::ThankYouMessage, (*value).into()))
        };
        if let Some(value) = &self.call_to_action {
            values.push((ConversationIden::CallToAction, (*value).into()))
        };
        if let Some(value) = &self.enable_signup_prompts {
            values.push((ConversationIden::EnableSignupPrompts, (*value).into()))
        };
        if let Some(value) = &self.show_thank_you_page_annon_instructions {
            values.push((
                ConversationIden::ShowThankYouPageAnnonInstructions,
                (*value).into(),
            ))
        };
        if let Some(value) = &self.metadata {
            values.push((ConversationIden::Metadata, value.clone().into()))
        };

        if let Some(value) = &self.supported_languages {
            values.push((
                ConversationIden::SupportedLanguages,
                sea_query::Value::Array(
                    sea_query::ArrayType::String,
                    Some(Box::new(value.iter().map(sea_query::Value::from).collect())),
                )
                .into(),
            ))
        };
        values
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ConversationFilterOptions {
    keyword: Option<String>,
    is_public: Option<bool>,
    is_live: Option<bool>,
    is_complete: Option<bool>,
    is_invite_only: Option<bool>,
    owner_id: Option<Uuid>,
    organization_id: Option<Uuid>,
    created_before: Option<DateTime<Utc>>,
    created_after: Option<DateTime<Utc>>,
}

impl ConversationFilterOptions {
    pub fn enforce_live(&mut self) {
        self.is_live = Some(true)
    }

    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = self.is_public {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::IsPublic)).eq(value),
                )
                .to_owned();
        };
        if let Some(value) = self.is_live {
            query = query
                .and_where(Expr::col((ConversationIden::Table, ConversationIden::IsLive)).eq(value))
                .to_owned();
        };
        if let Some(value) = self.is_invite_only {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::IsInviteOnly)).eq(value),
                )
                .to_owned();
        };
        if let Some(value) = self.is_complete {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::IsComplete)).eq(value),
                )
                .to_owned();
        };
        if let Some(value) = &self.owner_id {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::OwnerId))
                        .eq(value.to_string()),
                )
                .to_owned();
        }
        if let Some(value) = &self.organization_id {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::OrganizationId))
                        .eq(*value),
                )
                .to_owned();
        }
        if let Some(value) = &self.created_before {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::CreatedAt)).lt(
                        sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(
                            Box::new(value.naive_utc()),
                        ))),
                    ),
                )
                .to_owned();
        };
        if let Some(value) = &self.created_after {
            query = query
                .and_where(
                    Expr::col((ConversationIden::Table, ConversationIden::CreatedAt)).gt(
                        sea_query::SimpleExpr::Value(sea_query::Value::ChronoDateTime(Some(
                            Box::new(value.naive_utc()),
                        ))),
                    ),
                )
                .to_owned();
        };
        query.to_owned()
    }

    /// Apply filters after localization joins have been made
    /// This version can filter on the localized text content
    fn apply_to_localized(
        &self,
        mut query: sea_query::SelectStatement,
    ) -> sea_query::SelectStatement {
        use crate::models::translations::TextTranslationIden;
        use sea_query::Alias;

        if let Some(value) = &self.keyword {
            // Filter on the actual translation table column, not the alias
            let tt_title_alias = Alias::new("tt_title");
            let tt_short_description_alias = Alias::new("tt_short_description");
            query = query
                .cond_where(
                    Cond::any()
                        .add(
                            Expr::col((tt_title_alias, TextTranslationIden::Content))
                                .ilike(format!("%{value}%")),
                        )
                        .add(
                            Expr::col((tt_short_description_alias, TextTranslationIden::Content))
                                .ilike(format!("%{value}%")),
                        ),
                )
                .to_owned();
        };

        self.apply(query)
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ConversationOrderOptions {
    title: Option<Order>,
    created_at: Option<Order>,
}

impl Default for ConversationOrderOptions {
    fn default() -> Self {
        Self {
            title: None,
            created_at: Some(Order::Desc),
        }
    }
}

impl ConversationOrderOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (ConversationIden::Table, ConversationIden::CreatedAt),
                    order.into(),
                )
                .to_owned()
        }
        query
    }

    /// Apply ordering after localization joins have been made
    /// This version can order by the localized text content
    pub fn apply_to_localized(
        &self,
        mut query: sea_query::SelectStatement,
    ) -> sea_query::SelectStatement {
        use crate::models::translations::TextTranslationIden;
        use sea_query::Alias;

        if let Some(order) = &self.title {
            // Order by the actual translation table column, not the alias
            let tt_title_alias = Alias::new("tt_title");
            query = query
                .order_by((tt_title_alias, TextTranslationIden::Content), order.into())
                .to_owned()
        }
        self.apply(query)
    }
}

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
        .inspect_err(|e| println!("{e:#?}"))
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

pub async fn get_by_id_or_slug(
    db: &PgPool,
    id_or_slug: &IdOrSlug,
) -> Result<Conversation, ComhairleError> {
    let conversation = match id_or_slug {
        IdOrSlug::Id(id) => get_by_id(db, id).await?,
        IdOrSlug::Slug(slug) => get_by_slug(db, slug).await?,
    };
    Ok(conversation)
}

#[instrument(err(Debug))]
pub async fn get_localised_by_id_or_slug(
    db: &PgPool,
    id_or_slug: &IdOrSlug,
    lang_code: &str,
) -> Result<LocalizedConversation, ComhairleError> {
    let original_conversation = match id_or_slug {
        IdOrSlug::Id(id) => get_localised_by_id(db, id, lang_code).await?,
        IdOrSlug::Slug(slug) => get_localised_by_slug(db, slug, lang_code).await?,
    };
    Ok(original_conversation)
}
/// Get a conversation by ID (original struct, not localized)
#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Conversation, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Conversation")?;

    Ok(conversation)
}

/// Get a conversation by ID
#[instrument(err(Debug))]
pub async fn get_localised_by_id(
    db: &PgPool,
    id: &Uuid,
    lang_code: &str,
) -> Result<LocalizedConversation, ComhairleError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (ConversationIden::Table, col)))
        .from(ConversationIden::Table)
        .and_where(Expr::col((ConversationIden::Table, ConversationIden::Id)).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = LocalizedConversation::query_to_localisation(select_query, lang_code)
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, LocalizedConversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .inspect_err(|e| println!("{e:#?}"))
        .resolve_db_err("Conversation")?;

    Ok(conversation)
}

/// Get a conversation by slug (original struct, not localized)
#[instrument(err(Debug))]
pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Conversation, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Slug).eq(slug.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Conversation")?;

    Ok(conversation)
}

#[instrument(err(Debug))]
pub async fn get_localised_by_slug(
    db: &PgPool,
    slug: &str,
    lang_code: &str,
) -> Result<LocalizedConversation, ComhairleError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (ConversationIden::Table, col)))
        .from(ConversationIden::Table)
        .and_where(Expr::col((ConversationIden::Table, ConversationIden::Slug)).eq(slug.to_owned()))
        .to_owned();

    let (sql, values) = LocalizedConversation::query_to_localisation(select_query, lang_code)
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, LocalizedConversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Conversation")?;

    Ok(conversation)
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update: &PartialConversation,
) -> Result<Conversation, ComhairleError> {
    //TODO we need something here to generate new translations
    //if the supported lanagues change
    //or I guess if primary_locale changes

    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(ConversationIden::Table)
        .values(values)
        .and_where(Expr::col(ConversationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(conversation)
}

/// Merge the supplied object into the conversation's `metadata` jsonb column at
/// the top level. Existing keys are overwritten by the patch, keys not present
/// in the patch are left untouched. This is a shallow merge — nested objects
/// are replaced, not merged recursively. `patch` must be a JSON object.
pub async fn patch_metadata(
    db: &PgPool,
    id: &Uuid,
    patch: &serde_json::Value,
) -> Result<Conversation, ComhairleError> {
    if !patch.is_object() {
        return Err(ComhairleError::BadRequest(
            "metadata patch must be a JSON object".into(),
        ));
    }

    let conversation = sqlx::query_as::<_, Conversation>(
        "UPDATE conversation
            SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb,
                updated_at = NOW()
            WHERE id = $2
            RETURNING *",
    )
    .bind(patch)
    .bind(id)
    .fetch_one(db)
    .await
    .resolve_db_err("Conversation")?;

    Ok(conversation)
}

#[instrument(err(Debug))]
pub async fn list_for_user_participation(
    db: &PgPool,
    user_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedConversation>, ComhairleError> {
    let query = Query::select()
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ConversationIden::Table, col)))
        .join(
            sea_query::JoinType::InnerJoin,
            WorkflowIden::Table,
            Expr::col((WorkflowIden::Table, WorkflowIden::ConversationId))
                .equals((ConversationIden::Table, ConversationIden::Id)),
        )
        .join(
            sea_query::JoinType::InnerJoin,
            UserParticipationIden::Table,
            Expr::col((
                UserParticipationIden::Table,
                UserParticipationIden::WorkflowId,
            ))
            .equals((WorkflowIden::Table, WorkflowIden::Id)),
        )
        .and_where(
            Expr::col((UserParticipationIden::Table, UserParticipationIden::UserId))
                .eq(user_id.to_owned()),
        )
        // .order_by(
        //     (
        //         UserParticipationIden::Table,
        //         UserParticipationIden::CreatedAt,
        //     ),
        //     sea_query::Order::Desc,
        // )
        .distinct()
        .to_owned();

    let (sql, values) = LocalizedConversation::query_to_localisation(query, locale)
        .build_sqlx(PostgresQueryBuilder);

    let conversations = sqlx::query_as_with::<_, LocalizedConversation, _>(&sql, values)
        .fetch_all(db)
        .await?;
    Ok(conversations)
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Dummy))]
pub struct CreateConversation {
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub video_url: Option<String>,
    #[cfg_attr(test, dummy(expr = "None"))]
    pub image: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    #[cfg_attr(test, dummy(expr = "None"))]
    pub default_workflow_id: Option<Uuid>,
    pub primary_locale: String,
    pub supported_languages: Vec<String>,
    pub enable_qa_chat_bot: Option<bool>,
}

impl CreateConversation {
    pub fn columns(&self) -> Vec<ConversationIden> {
        let mut columns = vec![
            ConversationIden::VideoUrl,
            ConversationIden::Tags,
            ConversationIden::IsPublic,
            ConversationIden::IsLive,
            ConversationIden::IsInviteOnly,
            ConversationIden::PrimaryLocale,
            ConversationIden::SupportedLanguages,
        ];

        if self.image.is_some() {
            columns.push(ConversationIden::Image);
        }

        columns
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let tags = self.tags.to_owned().unwrap_or_default();

        let mut values = vec![
            self.video_url.to_owned().into(),
            tags.into(),
            self.is_public.into(),
            self.is_live.into(),
            self.is_invite_only.into(),
            self.primary_locale.to_owned().into(),
            self.supported_languages.to_owned().into(),
        ];

        if let Some(image) = self.image {
            values.push(image.into());
        }

        values
    }
}

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
                return Err(ComhairleError::DuplicateSlug(slug));
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

pub async fn list_owned(
    db: &PgPool,
    owner_id: Uuid,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ComhairleError> {
    // 1. Build base query with conversation table columns
    let query = Query::select()
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ConversationIden::Table, col)))
        .and_where(
            Expr::col((ConversationIden::Table, ConversationIden::OwnerId)).eq(owner_id.to_owned()),
        )
        .to_owned();

    // 2. Apply localization joins first to get text content
    let query = LocalizedConversation::query_to_localisation(query, &locale.unwrap_or("en".into()));

    // 3. Apply filters and ordering to the localized data
    let query = filter_options.apply_to_localized(query);
    let query = order_options.apply_to_localized(query);

    let conversations = page_options.fetch_paginated_results(db, query).await?;

    Ok(conversations)
}

pub async fn launch(
    db: &PgPool,
    conversation_id: Uuid,
    state: &Arc<ComhairleState>,
) -> Result<Conversation, ComhairleError> {
    let workflows = models::workflow::list(db, conversation_id, None).await?;
    for workflow in workflows {
        models::workflow::launch(db, &workflow.id, state).await?;
    }

    update(
        db,
        &conversation_id,
        &PartialConversation {
            is_live: Some(true),
            ..Default::default()
        },
    )
    .await?;

    let conversation = get_by_id(db, &conversation_id).await?;

    Ok(conversation)
}
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ComhairleError> {
    // 1. Build base query with conversation table columns
    let query = Query::select()
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (ConversationIden::Table, col)))
        .and_where(Expr::col((ConversationIden::Table, ConversationIden::IsPublic)).eq(true))
        .and_where(Expr::col((ConversationIden::Table, ConversationIden::IsLive)).eq(true))
        .to_owned();

    // 2. Apply localization joins first to get text content
    let query = LocalizedConversation::query_to_localisation(query, &locale.unwrap_or("en".into()));

    // 3. Apply filters and ordering to the localized data
    let query = filter_options.apply_to_localized(query);
    let query = order_options.apply_to_localized(query);

    let conversations = page_options.fetch_paginated_results(db, query).await?;

    Ok(conversations)
}

#[instrument(err(Debug))]
pub async fn list_for_permitted_user(
    db: &PgPool,
    user_id: Uuid,
    organization_id: Option<Uuid>,
    is_super_admin: bool,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ComhairleError> {
    let mut query = Query::select();
    query
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS.map(|c| (ConversationIden::Table, c)))
        .distinct();

    if !is_super_admin {
        let read_role_names: Vec<String> = Role::all()
            .filter(|role| {
                role.resource_type() == ResourceType::Conversation
                    && role.actions().contains(&Action::ConversationRead)
            })
            .map(|role| role.as_ref().to_string())
            .collect();

        let actor_condition = match organization_id {
            Some(org_id) => Cond::any()
                .add(
                    Expr::col((
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::UserId,
                    ))
                    .eq(user_id),
                )
                .add(
                    Expr::col((
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::OrganizationId,
                    ))
                    .eq(org_id),
                ),
            None => Cond::all().add(
                Expr::col((
                    ResourcePermissionIden::Table,
                    ResourcePermissionIden::UserId,
                ))
                .eq(user_id),
            ),
        };

        let join_condition = Cond::all()
            .add(
                Expr::col((ConversationIden::Table, ConversationIden::Id)).equals((
                    ResourcePermissionIden::Table,
                    ResourcePermissionIden::ResourceId,
                )),
            )
            .add(
                Expr::col((
                    ResourcePermissionIden::Table,
                    ResourcePermissionIden::ResourceType,
                ))
                .eq(ResourceType::Conversation.as_ref()),
            )
            .add(
                Expr::col((
                    ResourcePermissionIden::Table,
                    ResourcePermissionIden::RoleName,
                ))
                .is_in(read_role_names),
            )
            .add(actor_condition);

        query.join(
            JoinType::LeftJoin,
            ResourcePermissionIden::Table,
            join_condition,
        );

        query.and_where(
            Cond::any()
                .add(Expr::col((ConversationIden::Table, ConversationIden::OwnerId)).eq(user_id))
                .add(
                    Expr::col((ResourcePermissionIden::Table, ResourcePermissionIden::Id))
                        .is_not_null(),
                )
                .into(),
        );
    }

    let query = query.to_owned();

    let query = LocalizedConversation::query_to_localisation(query, &locale.unwrap_or("en".into()));

    let query = filter_options.apply_to_localized(query);
    let query = order_options.apply_to_localized(query);

    let conversations = page_options.fetch_paginated_results(db, query).await?;

    Ok(conversations)
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use serde_json::json;

    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::models::permissions::{GrantRoleRequest, Role, UserOrOrganizationId, grant_role};
    use crate::models::users::{self, UpdateUserRequest, create_user, update_user};
    use crate::routes::auth::SignupRequest;
    use crate::routes::conversations::dto::ConversationDto;
    use crate::routes::organizations::dto::OrganizationDto;
    use crate::setup_server;
    use crate::test_helpers::{UserSession, test_state};

    use super::*;
    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_conversation_with_oranganization_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state.clone())).await?;

        let mut admin_session = UserSession::new_admin();
        admin_session.signup(&app).await?;

        let (_, response, _) = admin_session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;
        let user = create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test_email".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;
        let user = update_user(
            &user.id,
            &UpdateUserRequest {
                organization_id: Some(organization.id),
                ..Default::default()
            },
            &pool,
        )
        .await?;

        let conversation = create(
            &pool,
            &state.bot_service,
            &state.config,
            &Faker.fake(),
            user.id,
            user.organization_id,
        )
        .await?;

        assert!(
            conversation.organization_id.is_some(),
            "incorrect organization_id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_conversations_by_case_insensitive_keyword(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A title about the moon",
                    "short_description" : "Scotlands ambitions to leave the planet",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "moon_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A conversation about golf",
                    "short_description" : "LIV vs the PGA",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "golf_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;
        let _ = session
            .create_conversation(
                &app,
                json! ({
                    "title" : "A conversation about AI",
                    "short_description" : "Some text about artificial intelligence",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : true,
                    "is_live": true,
                    "is_invite_only" : false,
                    "slug" : "ai_conversation",
                    "primary_locale" : "en",
                    "supported_languages" : ["en"]
                }),
            )
            .await?;

        let filter_options_1 = ConversationFilterOptions {
            keyword: Some("moon".to_string()),
            ..Default::default()
        };
        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };

        let results_1 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_1,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_1.total, 1, "incorrect first total");
        assert_eq!(
            results_1.records[0].title,
            "A title about the moon".to_string(),
            "incorrect first title"
        );

        let filter_options_2 = ConversationFilterOptions {
            keyword: Some("liv".to_string()),
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_2 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_2,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_2.total, 1, "incorrect second total");
        assert_eq!(
            results_2.records[0].title,
            "A conversation about golf".to_string(),
            "incorrect second title"
        );

        let filter_options_3 = ConversationFilterOptions {
            keyword: Some("intelligence".to_string()),
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_3 = list(
            &pool,
            page_options.clone(),
            order_options,
            filter_options_3,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results_3.total, 1, "incorrect third total");
        assert_eq!(
            results_3.records[0].title,
            "A conversation about AI".to_string(),
            "incorrect third title"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_conversations_by_organization_id(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, response, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(response)?;

        // Before user has organization_id
        let _ = session.create_random_conversation(&app).await?;
        let _ = session.create_random_conversation(&app).await?;
        let _ = session.create_random_conversation(&app).await?;

        let _ = session
            .put(
                &app,
                "/user/details",
                json!({ "organization_id": organization.id })
                    .to_string()
                    .into(),
            )
            .await?;

        // After user has organization_id
        let (_, response_1, _) = session.create_random_conversation(&app).await?;
        let (_, response_2, _) = session.create_random_conversation(&app).await?;
        let (_, response_3, _) = session.create_random_conversation(&app).await?;
        let conversation_1: ConversationDto = serde_json::from_value(response_1)?;
        let conversation_2: ConversationDto = serde_json::from_value(response_2)?;
        let conversation_3: ConversationDto = serde_json::from_value(response_3)?;

        let page_options = PageOptions {
            limit: None,
            offset: None,
        };
        let order_options = ConversationOrderOptions {
            created_at: Some(Order::Asc),
            title: None,
        };
        let filter_options = ConversationFilterOptions {
            organization_id: Some(organization.id),
            ..Default::default()
        };
        let results = list(
            &pool,
            page_options,
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(results.total, 3, "incorrect total filtered conversations");
        assert_eq!(
            results.records[0].id, conversation_1.id,
            "incorrect first id"
        );
        assert_eq!(
            results.records[1].id, conversation_2.id,
            "incorrect second id"
        );
        assert_eq!(
            results.records[2].id, conversation_3.id,
            "incorrect third id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_conversations_for_permitted_user(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = Arc::new(test_state().db(pool).call()?);
        let (app, mut session) = setup_default_app_and_session(&state.db).await?;

        let (_, value, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(value)?;

        let user_a = users::create_annon_user(&state.db).await?;
        let user_b = users::create_annon_user(&state.db).await?;

        let grant_request_a_a = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_a.id),
            permission_triplet: Role::ConversationContentEditor.triplet(&conversation.id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a_a).await?;

        let grant_request_a_b = GrantRoleRequest {
            actor_id: UserOrOrganizationId::User(user_a.id),
            permission_triplet: Role::Tester.triplet(&conversation.id),
            granted_by: &session.id.unwrap(),
            grant_reason: "Testing",
        };
        grant_role(&state, grant_request_a_b).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };

        let results_user_a_a = list_for_permitted_user(
            &state.db,
            user_a.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_user_b_a = list_for_permitted_user(
            &state.db,
            user_b.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(
            results_user_a_a.total, 1,
            "incorrect permitted conversation total for user_a"
        );
        assert_eq!(
            results_user_b_a.total, 0,
            "incorrect permitted conversation total for user_b"
        );

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_owner = list_for_permitted_user(
            &state.db,
            session.id.unwrap(),
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        let filter_options = ConversationFilterOptions {
            ..Default::default()
        };
        let order_options = ConversationOrderOptions {
            ..Default::default()
        };
        let results_user_b_b = list_for_permitted_user(
            &state.db,
            user_b.id,
            None,
            false,
            page_options.clone(),
            order_options,
            filter_options,
            Some("en".to_string()),
        )
        .await?;

        assert_eq!(
            results_owner.total, 1,
            "incorrect permitted conversation total for owner"
        );
        assert_eq!(
            results_user_b_b.total, 0,
            "incorrect permitted conversation total for user_b"
        );

        Ok(())
    }
}
