use super::{
    pagination::{Order, PageOptions, PaginatedResults},
    translations::TextContentId,
    user_participation::UserParticipationIden,
    workflow::WorkflowIden,
};
use crate::models::SqlxResultExt;
use crate::models::error::ModelError;
use crate::models::error::ValidationError;
use crate::models::permissions::{Action, ResourcePermissionIden, ResourceType, Role};
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{
    Cond, Expr, JoinType, PostgresQueryBuilder, Query, enum_def, extension::postgres::PgExpr,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
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
    pub show_thankyou_page_feedback_button: bool,
    /// Whether a participant may return to the workflow's steps after finishing (i.e. once
    /// every step is done). False seals them: no step is reachable and their step writes are
    /// rejected. Orthogonal to the per-step `can_revisit` flag, which governs navigation
    /// *before* they finish. See ADR-0016.
    pub allow_revisit_after_finishing: bool,
    pub metadata: serde_json::Value,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

pub const DEFAULT_COLUMNS: [ConversationIden; 32] = [
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
    ConversationIden::ShowThankyouPageFeedbackButton,
    ConversationIden::AllowRevisitAfterFinishing,
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
        if let Some(value) = &self.organization_id {
            values.push((ConversationIden::OrganizationId, (*value).into()))
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
        if let Some(value) = &self.show_thankyou_page_feedback_button {
            values.push((
                ConversationIden::ShowThankyouPageFeedbackButton,
                (*value).into(),
            ))
        };
        if let Some(value) = &self.allow_revisit_after_finishing {
            values.push((
                ConversationIden::AllowRevisitAfterFinishing,
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
    pub keyword: Option<String>,
    pub is_public: Option<bool>,
    pub is_live: Option<bool>,
    pub is_complete: Option<bool>,
    pub is_invite_only: Option<bool>,
    pub owner_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub created_before: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
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
    pub title: Option<Order>,
    pub created_at: Option<Order>,
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

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id_or_slug(
    db: &PgPool,
    id_or_slug: &IdOrSlug,
) -> Result<Conversation, ModelError> {
    let conversation = match id_or_slug {
        IdOrSlug::Id(id) => get_by_id(db, id).await?,
        IdOrSlug::Slug(slug) => get_by_slug(db, slug).await?,
    };
    Ok(conversation)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_localised_by_id_or_slug(
    db: &PgPool,
    id_or_slug: &IdOrSlug,
    lang_code: &str,
) -> Result<LocalizedConversation, ModelError> {
    let original_conversation = match id_or_slug {
        IdOrSlug::Id(id) => get_localised_by_id(db, id, lang_code).await?,
        IdOrSlug::Slug(slug) => get_localised_by_slug(db, slug, lang_code).await?,
    };
    Ok(original_conversation)
}
/// Get a conversation by ID (original struct, not localized)
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Conversation, ModelError> {
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
#[instrument(err(Debug), skip(db))]
pub async fn get_localised_by_id(
    db: &PgPool,
    id: &Uuid,
    lang_code: &str,
) -> Result<LocalizedConversation, ModelError> {
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
        .resolve_db_err("Conversation")?;

    Ok(conversation)
}

/// Get a conversation by slug (original struct, not localized)
#[instrument(err(Debug), skip(db))]
pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<Conversation, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn get_localised_by_slug(
    db: &PgPool,
    slug: &str,
    lang_code: &str,
) -> Result<LocalizedConversation, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update: &PartialConversation,
) -> Result<Conversation, ModelError> {
    //TODO we need something here to generate new translations
    //if the supported lanagues change
    //or I guess if primary_locale changes

    let values = update.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
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
#[instrument(err(Debug), skip(db))]
pub async fn patch_metadata(
    db: &PgPool,
    id: &Uuid,
    patch: &serde_json::Value,
) -> Result<Conversation, ModelError> {
    if !patch.is_object() {
        return Err(
            ValidationError::BadRequest("metadata patch must be a JSON object".into()).into(),
        );
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

#[instrument(err(Debug), skip(db))]
pub async fn list_for_user_participation(
    db: &PgPool,
    user_id: &Uuid,
    locale: &str,
) -> Result<Vec<LocalizedConversation>, ModelError> {
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

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub struct CreateConversation {
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub video_url: Option<String>,
    #[cfg_attr(any(test, feature = "test-util"), dummy(expr = "None"))]
    pub image: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub is_public: bool,
    pub is_live: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    #[cfg_attr(any(test, feature = "test-util"), dummy(expr = "None"))]
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

#[instrument(err(Debug), skip(db))]
pub async fn list_owned(
    db: &PgPool,
    owner_id: Uuid,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn list_for_permitted_user(
    db: &PgPool,
    user_id: Uuid,
    organization_id: Option<Uuid>,
    is_super_admin: bool,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalizedConversation>, ModelError> {
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
