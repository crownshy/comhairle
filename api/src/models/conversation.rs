use super::{
    pagination::{Order, PageOptions, PaginatedResults},
    translations::{new_translation, TextContentId, TextFormat},
    user_participation::UserParticipationIden,
    workflow::WorkflowIden,
};
use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::{prelude::FromRow, PgPool};
use tracing::info;
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "conversation")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct Conversation {
    #[partially(omit)]
    pub id: Uuid,
    pub title: TextContentId,
    pub short_description: TextContentId,
    pub description: TextContentId,
    #[partially(transparent)]
    pub video_url: Option<String>,
    pub image_url: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub is_complete: bool,
    #[partially(omit)]
    pub owner_id: Uuid,
    pub is_invite_only: bool,
    #[partially(transparent)]
    pub slug: Option<String>,
    #[partially(transparent)]
    pub default_workflow_id: Option<Uuid>,
    pub primary_locale: String,
    pub supported_languages: Vec<String>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ConversationIden; 17] = [
    ConversationIden::Id,
    ConversationIden::Title,
    ConversationIden::ShortDescription,
    ConversationIden::Description,
    ConversationIden::VideoUrl,
    ConversationIden::ImageUrl,
    ConversationIden::Tags,
    ConversationIden::IsPublic,
    ConversationIden::IsComplete,
    ConversationIden::IsInviteOnly,
    ConversationIden::Slug,
    ConversationIden::DefaultWorkflowId,
    ConversationIden::PrimaryLocale,
    ConversationIden::SupportedLanguages,
    ConversationIden::CreatedAt,
    ConversationIden::UpdatedAt,
    ConversationIden::OwnerId,
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
        if let Some(value) = &self.image_url {
            values.push((ConversationIden::ImageUrl, value.into()))
        };
        if let Some(value) = &self.tags {
            values.push((
                ConversationIden::Tags,
                sea_query::Value::Array(
                    sea_query::ArrayType::String,
                    Some(Box::new(
                        value.into_iter().map(sea_query::Value::from).collect(),
                    )),
                )
                .into(),
            ))
        };
        if let Some(value) = self.is_public {
            values.push((ConversationIden::IsPublic, value.into()))
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

        if let Some(value) = &self.supported_languages {
            values.push((
                ConversationIden::SupportedLanguages,
                sea_query::Value::Array(
                    sea_query::ArrayType::String,
                    Some(Box::new(
                        value.into_iter().map(sea_query::Value::from).collect(),
                    )),
                )
                .into(),
            ))
        };
        values
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ConversationFilterOptions {
    title: Option<String>,
    is_public: Option<bool>,
    is_complete: Option<bool>,
    is_invite_only: Option<bool>,
    owner_id: Option<Uuid>,
    created_before: Option<DateTime<Utc>>,
    created_after: Option<DateTime<Utc>>,
}

impl ConversationFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = &self.title {
            query = query
                .and_where(Expr::col(ConversationIden::Title).like(format!("%{value}%")))
                .to_owned();
        };
        if let Some(value) = self.is_public {
            query = query
                .and_where(Expr::col(ConversationIden::IsPublic).eq(value))
                .to_owned();
        };
        if let Some(value) = self.is_invite_only {
            query = query
                .and_where(Expr::col(ConversationIden::IsInviteOnly).eq(value))
                .to_owned();
        };
        if let Some(value) = self.is_complete {
            query = query
                .and_where(Expr::col(ConversationIden::IsComplete).eq(value))
                .to_owned();
        };
        if let Some(value) = &self.owner_id {
            query = query
                .and_where(Expr::col(ConversationIden::OwnerId).eq(value.to_string()))
                .to_owned();
        }
        if let Some(value) = &self.created_before {
            query = query
                .and_where(
                    Expr::col(ConversationIden::CreatedAt).lt(sea_query::SimpleExpr::Value(
                        sea_query::Value::ChronoDateTime(Some(Box::new(value.naive_utc()))),
                    )),
                )
                .to_owned();
        };
        if let Some(value) = &self.created_after {
            query = query
                .and_where(
                    Expr::col(ConversationIden::CreatedAt).gt(sea_query::SimpleExpr::Value(
                        sea_query::Value::ChronoDateTime(Some(Box::new(value.naive_utc()))),
                    )),
                )
                .to_owned();
        };
        query.to_owned()
    }
}

#[derive(Deserialize, Debug, Default, JsonSchema)]
pub struct ConversationOrderOptions {
    title: Option<Order>,
    created_at: Option<Order>,
}

impl ConversationOrderOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.title {
            query = query
                .order_by(ConversationIden::Title, order.into())
                .to_owned()
        }

        if let Some(order) = &self.created_at {
            query = query
                .order_by(ConversationIden::CreatedAt, order.into())
                .to_owned()
        }
        query
    }
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Conversation, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(conversation)
}
/// Get a conversation by ID
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<LocalisedConversation, ComhairleError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Id).eq(id.to_owned()))
        .to_owned();

    let (sql, values) = LocalisedConversation::query_to_localisation(select_query, "en")
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, LocalisedConversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(conversation)
}

pub async fn get_by_slug(db: &PgPool, slug: &str) -> Result<LocalisedConversation, ComhairleError> {
    let select_query = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Slug).eq(slug.to_owned()))
        .to_owned();

    let (sql, values) = LocalisedConversation::query_to_localisation(select_query, "en")
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, LocalisedConversation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(conversation)
}

pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update: &PartialConversation,
) -> Result<Conversation, ComhairleError> {
    info!("Updating conversation {id} with update {update:#?}");
    let values = update.to_values();

    if values.len() == 0 {
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

pub async fn list_for_user_participation(
    db: &PgPool,
    user_id: &Uuid,
) -> Result<Vec<Conversation>, ComhairleError> {
    let (sql, values) = Query::select()
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
        .build_sqlx(PostgresQueryBuilder);

    let conversations = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
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
    pub image_url: String,
    pub tags: Option<Vec<String>>,
    pub is_public: bool,
    pub is_invite_only: bool,
    pub slug: Option<String>,
    #[cfg_attr(test, dummy(expr = "None"))]
    pub default_workflow_id: Option<Uuid>,
    pub primary_locale: String,
    pub supported_languages: Vec<String>,
}

impl CreateConversation {
    pub fn columns(&self) -> Vec<ConversationIden> {
        vec![
            ConversationIden::VideoUrl,
            ConversationIden::ImageUrl,
            ConversationIden::Tags,
            ConversationIden::IsPublic,
            ConversationIden::IsInviteOnly,
            ConversationIden::PrimaryLocale,
            ConversationIden::SupportedLanguages,
        ]
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let tags = self.tags.to_owned().unwrap_or_else(|| vec![]);

        vec![
            self.video_url.to_owned().into(),
            self.image_url.to_owned().into(),
            tags.into(),
            self.is_public.into(),
            self.is_invite_only.into(),
            self.primary_locale.to_owned().into(),
            self.supported_languages.to_owned().into(),
        ]
    }
}

pub async fn create(
    db: &PgPool,
    conversation: &CreateConversation,
    owner_id: Uuid,
) -> Result<Conversation, ComhairleError> {
    let conversation_id = Uuid::new_v4();

    // Generate Translations
    let title = new_translation(
        &db,
        &conversation.primary_locale,
        &conversation.title,
        TextFormat::Plain,
        &conversation_id,
    )
    .await?;

    let description = new_translation(
        &db,
        &conversation.primary_locale,
        &conversation.description,
        TextFormat::Rich,
        &conversation_id,
    )
    .await?;

    let short_description = new_translation(
        &db,
        &conversation.primary_locale,
        &conversation.short_description,
        TextFormat::Rich,
        &conversation_id,
    )
    .await?;

    let mut columns = conversation.columns();
    let mut values = conversation.values();

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

    let (sql, values) = Query::insert()
        .into_table(ConversationIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let conversation_result = sqlx::query_as_with::<_, Conversation, _>(&sql, values)
        .fetch_one(db)
        .await;

    match conversation_result {
        Ok(conversation) => Ok(conversation),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                if let Some(constraint) = pg_err.constraint() {
                    if constraint.contains("slug") {
                        return Err(ComhairleError::DuplicateSlug(slug));
                    }
                }
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
) -> Result<PaginatedResults<Conversation>, ComhairleError> {
    let query = Query::select()
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ConversationIden::OwnerId).eq(owner_id.to_owned()))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);
    let query = LocalisedConversation::query_to_localisation(query, &locale.unwrap_or("en".into()));

    let conversations = page_options.fetch_paginated_results(db, query).await?;

    Ok(conversations)
}

pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: ConversationOrderOptions,
    filter_options: ConversationFilterOptions,
    locale: Option<String>,
) -> Result<PaginatedResults<LocalisedConversation>, ComhairleError> {
    let query = Query::select()
        .from(ConversationIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ConversationIden::IsPublic).eq(true))
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);
    let query = LocalisedConversation::query_to_localisation(query, &locale.unwrap_or("en".into()));

    let conversations = page_options.fetch_paginated_results(db, query).await?;

    Ok(conversations)
}
