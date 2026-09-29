// Context: We migrated user profile demographics to an EAV key-value schema using three tables:
// 1. `demographics_question`                     - Represents a demographics question
// (
//    slug TEXT PK,                               - Unique identifier for the demographics question
//    display_name TEXT NOT NULL,                 - Human-readable name for the demographics question
//    response_type 'number' | 'string' NOT NULL, - The type of response expected for the demographics question
//    bucket_config JSONB                         - For 'number': bucket ranges (e.g. age ranges). For 'string': the configurable set of selectable options.
// )
// 2. `demographics_response`                     - Represents a user's response to a demographics question
// (
//    question_slug FK,                           - Foreign key referencing the demographics question
//    user_id FK,                                 - Foreign key referencing the user
//    value NOT NULL                              - The response value provided by the user
// )
// 3. `conversation_demographics`                 - Represents which demographics questions are associated with a conversation
// (
//    conversation_id FK,                         - Foreign key referencing the conversation
//    question_slug FK                            - Foreign key referencing the demographics question
// )

use crate::models::error::ModelError;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Alias, Expr, PostgresQueryBuilder, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use sqlx::{Decode, Postgres, Type};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;
use crate::models::error::ValidationError;
use crate::models::pagination::{PageOptions, PaginatedResults};

// ============================================================================
// Conversation Demographics Models
// ============================================================================

/// Represents an association between a conversation and a demographics question.
#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "conversation_demographics")]
#[serde(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub struct ConversationDemographics {
    pub conversation_id: Uuid,
    pub question_slug: String,
}

const CONVERSATION_DEMOGRAPHICS_COLUMNS: [ConversationDemographicsIden; 2] = [
    ConversationDemographicsIden::ConversationId,
    ConversationDemographicsIden::QuestionSlug,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationDemographics {
    pub conversation_id: Uuid,
    pub question_slug: String,
}

impl CreateConversationDemographics {
    fn columns() -> [ConversationDemographicsIden; 2] {
        CONVERSATION_DEMOGRAPHICS_COLUMNS
    }

    fn values(self) -> [SimpleExpr; 2] {
        [
            self.conversation_id.into(),
            self.question_slug.clone().into(),
        ]
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct ConversationDemographicsFilterOptions {
    pub conversation_id: Option<Uuid>,
    pub question_slug: Option<String>,
}

impl ConversationDemographicsFilterOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(conversation_id) = self.conversation_id {
            query = query
                .and_where(
                    Expr::col(ConversationDemographicsIden::ConversationId).eq(conversation_id),
                )
                .to_owned();
        }

        if let Some(question_slug) = self.question_slug.clone() {
            query = query
                .and_where(Expr::col(ConversationDemographicsIden::QuestionSlug).eq(question_slug))
                .to_owned();
        }

        query
    }
}

/// Represents demographics question response type (either 'number' or 'string').
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub enum DemographicsQuestionResponseType {
    Number,
    String,
}

impl Into<SimpleExpr> for DemographicsQuestionResponseType {
    fn into(self) -> SimpleExpr {
        Expr::val(self.as_ref()).cast_as(Alias::new("demographics_response_type"))
    }
}

impl AsRef<str> for DemographicsQuestionResponseType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Number => "number",
            Self::String => "string",
        }
    }
}

impl Decode<'_, Postgres> for DemographicsQuestionResponseType {
    fn decode(
        row: <Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = row.as_str()?;
        match s {
            "number" => return Ok(DemographicsQuestionResponseType::Number),
            "string" => return Ok(DemographicsQuestionResponseType::String),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid demographics question response type: {s}"),
                )));
            }
        }
    }
}

impl Type<Postgres> for DemographicsQuestionResponseType {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <Postgres as sqlx::Database>::TypeInfo::with_name("demographics_response_type").into()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub struct NumericBucket {
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub label: String,
}

/// A single selectable answer for a 'string' demographics question.
#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub struct StringOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ValueBuckets {
    Numeric { buckets: Vec<NumericBucket> },
    String { options: Vec<StringOption> },
}

impl ValueBuckets {
    pub fn accepts_value(&self, value: &str) -> bool {
        match self {
            ValueBuckets::Numeric { .. } => value.parse::<i64>().is_ok(),
            ValueBuckets::String { options } => {
                options.is_empty() || options.iter().any(|option| option.value == value)
            }
        }
    }
}

/// Safely maps a raw value into a defined category bucket label based on the provided bucket configuration.
pub fn resolve_category_bucket(value: &str, buckets: &ValueBuckets) -> String {
    match buckets {
        ValueBuckets::Numeric {
            buckets: numeric_buckets,
        } => {
            if let Ok(num) = value.parse::<i64>() {
                for bucket in numeric_buckets {
                    let greater_than_or_equal_to_min = bucket.min.map_or(true, |m| num >= m);
                    let less_than_or_equal_to_max = bucket.max.map_or(true, |m| num <= m);
                    if greater_than_or_equal_to_min && less_than_or_equal_to_max {
                        return bucket.label.clone();
                    }
                }
            }
        }
        ValueBuckets::String { options } => {
            // No options configured yet: report the raw value rather than hiding it.
            if options.is_empty() {
                return value.to_string();
            }
            if let Some(option) = options.iter().find(|option| option.value == value) {
                return option.label.clone();
            }
        }
    }
    "Uncategorized".to_string()
}

/// Represents a demographics question.
#[derive(Serialize, Deserialize, Partial, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "demographics_question")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub struct DemographicsQuestion {
    #[partially(omit)]
    pub slug: String,
    pub display_name: String,
    pub response_type: DemographicsQuestionResponseType,
    #[schemars(with = "Option<ValueBuckets>")]
    pub bucket_config: Option<sqlx::types::Json<ValueBuckets>>,
}

const DEMOGRAPHICS_QUESTION_COLUMNS: [DemographicsQuestionIden; 4] = [
    DemographicsQuestionIden::Slug,
    DemographicsQuestionIden::DisplayName,
    DemographicsQuestionIden::ResponseType,
    DemographicsQuestionIden::BucketConfig,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDemographicsQuestion {
    pub slug: String,
    pub display_name: String,
    pub response_type: DemographicsQuestionResponseType,
    #[schemars(with = "Option<ValueBuckets>")]
    pub bucket_config: Option<sqlx::types::Json<ValueBuckets>>,
}

impl CreateDemographicsQuestion {
    fn columns() -> [DemographicsQuestionIden; 4] {
        DEMOGRAPHICS_QUESTION_COLUMNS
    }

    fn values(self) -> [SimpleExpr; 4] {
        [
            self.slug.into(),
            self.display_name.into(),
            self.response_type.into(),
            self.bucket_config
                .map(|json| serde_json::to_value(json.0).unwrap())
                .into(),
        ]
    }
}

impl PartialDemographicsQuestion {
    fn to_values(self) -> Vec<(DemographicsQuestionIden, SimpleExpr)> {
        let mut values = Vec::new();
        if let Some(display_name) = self.display_name {
            values.push((DemographicsQuestionIden::DisplayName, display_name.into()));
        }

        if let Some(response_type) = self.response_type {
            values.push((DemographicsQuestionIden::ResponseType, response_type.into()));
        }

        if let Some(bucket_config) = self.bucket_config {
            let json_expr: SimpleExpr = match bucket_config {
                Some(json) => Expr::val(serde_json::to_value(json.0).unwrap()).into(),
                None => sea_query::Keyword::Null.into(),
            };

            values.push((DemographicsQuestionIden::BucketConfig, json_expr));
        }
        values
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct DemographicsQuestionsFilterOptions {
    pub conversation_id: Option<Uuid>,
    pub question_slug: Option<String>,
}

impl DemographicsQuestionsFilterOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(conversation_id) = self.conversation_id {
            query = query
                .inner_join(
                    ConversationDemographicsIden::Table,
                    sea_query::Expr::col(DemographicsQuestionIden::Slug)
                        .equals(ConversationDemographicsIden::QuestionSlug),
                )
                .and_where(
                    Expr::col(ConversationDemographicsIden::ConversationId).eq(conversation_id),
                )
                .to_owned();
        }

        if let Some(question_slug) = self.question_slug.clone() {
            query = query
                .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(question_slug))
                .to_owned();
        }

        query
    }
}

/// A demographics response value is always a JSON number or string on the wire, never
/// stored/parsed as arbitrary JSON. Schema-only type: response bodies still serialize
/// `value` as a plain `String` internally.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TypedValue {
    Number(i64),
    Text(String),
}

/// Represents a demographics response from a user to a specific demographics question.
#[derive(Deserialize, Partial, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "demographics_response")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-util"), derive(PartialEq))]
pub struct DemographicsResponse {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub question_slug: String,
    #[partially(omit)]
    pub user_id: Option<Uuid>,
    #[schemars(with = "TypedValue")]
    pub value: String,
}

impl Serialize for DemographicsResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("DemographicsResponse", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("questionSlug", &self.question_slug)?;
        state.serialize_field("userId", &self.user_id)?;
        match self.value.parse::<i64>() {
            Ok(number) => state.serialize_field("value", &number)?,
            Err(_) => state.serialize_field("value", &self.value)?,
        }
        state.end()
    }
}

/// Accepts either a JSON number or JSON string for a response value and stores it as text.
fn deserialize_typed_value<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match TypedValue::deserialize(deserializer)? {
        TypedValue::Number(number) => number.to_string(),
        TypedValue::Text(text) => text,
    })
}

const DEMOGRAPHICS_RESPONSE_COLUMNS: [DemographicsResponseIden; 4] = [
    DemographicsResponseIden::Id,
    DemographicsResponseIden::QuestionSlug,
    DemographicsResponseIden::UserId,
    DemographicsResponseIden::Value,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateDemographicsResponse {
    pub question_slug: String,
    pub user_id: Uuid,
    #[serde(deserialize_with = "deserialize_typed_value")]
    #[schemars(with = "TypedValue")]
    pub value: String,
}

impl CreateDemographicsResponse {
    fn values(self, id: Uuid) -> [SimpleExpr; 4] {
        [
            id.into(),
            self.question_slug.into(),
            self.user_id.into(),
            self.value.into(),
        ]
    }
}

impl PartialDemographicsResponse {
    fn to_values(&self) -> Vec<(DemographicsResponseIden, SimpleExpr)> {
        let mut values = Vec::new();
        if let Some(value) = &self.value {
            values.push((DemographicsResponseIden::Value, value.clone().into()));
        }
        values
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct DemographicsResponsesFilterOptions {
    pub conversation_id: Option<Uuid>,
    pub question_slug: Option<String>,
    pub user_id: Option<Uuid>,
}

impl DemographicsResponsesFilterOptions {
    pub fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(conversation_id) = self.conversation_id {
            query = query
                .inner_join(
                    ConversationDemographicsIden::Table,
                    sea_query::Expr::col((
                        DemographicsResponseIden::Table,
                        DemographicsResponseIden::QuestionSlug,
                    ))
                    .equals((
                        ConversationDemographicsIden::Table,
                        ConversationDemographicsIden::QuestionSlug,
                    )),
                )
                .and_where(
                    Expr::col(ConversationDemographicsIden::ConversationId).eq(conversation_id),
                )
                .to_owned();
        }

        if let Some(question_slug) = self.question_slug.clone() {
            query = query
                .and_where(
                    Expr::col((
                        DemographicsResponseIden::Table,
                        DemographicsResponseIden::QuestionSlug,
                    ))
                    .eq(question_slug),
                )
                .to_owned();
        }

        if let Some(user_id) = self.user_id {
            query = query
                .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
                .to_owned();
        }

        query
    }
}

// ============================================================================
// Conversation-demographics associations - CR.D
// ============================================================================

/// Get all associations between conversations and demographics questions, with optional filters for conversation ID and question slug.
#[instrument(err(Debug), skip(db))]
pub async fn get_conversation_demographics(
    db: &PgPool,
    filters: ConversationDemographicsFilterOptions,
    page_options: PageOptions,
) -> Result<PaginatedResults<ConversationDemographics>, ModelError> {
    let mut query = sea_query::Query::select()
        .columns(CONVERSATION_DEMOGRAPHICS_COLUMNS)
        .from(ConversationDemographicsIden::Table)
        .to_owned();

    query = filters.apply(query);

    let fetched: PaginatedResults<ConversationDemographics> =
        page_options.fetch_paginated_results(db, query).await?;

    Ok(fetched)
}

/// Create a new association between a conversation and a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn create_conversation_demographics(
    db: &PgPool,
    new_conversation_demographics: CreateConversationDemographics,
) -> Result<ConversationDemographics, ModelError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(ConversationDemographicsIden::Table)
        .columns(CreateConversationDemographics::columns())
        .values(new_conversation_demographics.values())?
        .returning(sea_query::Query::returning().columns(CONVERSATION_DEMOGRAPHICS_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let created = sqlx::query_as_with::<_, ConversationDemographics, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(created)
}

/// Remove an association between a conversation and a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn delete_conversation_demographics(
    db: &PgPool,
    conversation_id: Uuid,
    question_slug: String,
) -> Result<Option<ConversationDemographics>, ModelError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(ConversationDemographicsIden::Table)
        .and_where(Expr::col(ConversationDemographicsIden::ConversationId).eq(conversation_id))
        .and_where(Expr::col(ConversationDemographicsIden::QuestionSlug).eq(question_slug))
        .returning(sea_query::Query::returning().columns(CONVERSATION_DEMOGRAPHICS_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, ConversationDemographics, _>(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("conversation_demographics")?;

    Ok(deleted)
}

// ============================================================================
// Demographics questions - CRUD
// ============================================================================

/// Get a single demographics question by its slug.
#[instrument(err(Debug), skip(db))]
pub async fn get_demographics_question_by_slug(
    db: &PgPool,
    slug: &str,
) -> Result<DemographicsQuestion, ModelError> {
    let (sql, values) = sea_query::Query::select()
        .columns(DEMOGRAPHICS_QUESTION_COLUMNS)
        .from(DemographicsQuestionIden::Table)
        .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(slug))
        .build_sqlx(PostgresQueryBuilder);

    let question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics_question")?;

    Ok(question)
}

/// Get a demographics questions with optional filters.
#[instrument(err(Debug), skip(db))]
pub async fn get_demographics_questions(
    db: &PgPool,
    filters: DemographicsQuestionsFilterOptions,
    page_options: PageOptions,
) -> Result<PaginatedResults<DemographicsQuestion>, ModelError> {
    let mut query = sea_query::Query::select()
        .columns(DEMOGRAPHICS_QUESTION_COLUMNS)
        .from(DemographicsQuestionIden::Table)
        .to_owned();

    query = filters.apply(query);

    let questions: PaginatedResults<DemographicsQuestion> =
        page_options.fetch_paginated_results(db, query).await?;

    Ok(questions)
}

/// Create a new demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn create_demographics_question(
    db: &PgPool,
    new_demographics_question: CreateDemographicsQuestion,
) -> Result<DemographicsQuestion, ModelError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(DemographicsQuestionIden::Table)
        .columns(CreateDemographicsQuestion::columns())
        .values(new_demographics_question.values())?
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics_question")?;

    Ok(question)
}

/// Update a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn update_demographics_question(
    db: &PgPool,
    slug: String,
    updated_demographics_question: PartialDemographicsQuestion,
) -> Result<DemographicsQuestion, ModelError> {
    let update_values = updated_demographics_question.to_values();

    if update_values.is_empty() {
        return get_demographics_question_by_slug(db, &slug).await;
    }

    let (sql, values) = sea_query::Query::update()
        .table(DemographicsQuestionIden::Table)
        .values(update_values)
        .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(slug))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics_question")?;

    Ok(question)
}

/// Delete a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn delete_demographics_question(
    db: &PgPool,
    slug: String,
) -> Result<Option<DemographicsQuestion>, ModelError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(DemographicsQuestionIden::Table)
        .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(slug))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted_question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("demographics_question")?;

    Ok(deleted_question)
}

// ============================================================================
// Demographics responses - CRUD
// ============================================================================

/// Get responses with optional filters.
#[instrument(err(Debug), skip(db))]
pub async fn get_demographics_responses(
    db: &PgPool,
    filters: DemographicsResponsesFilterOptions,
    page_options: PageOptions,
) -> Result<PaginatedResults<DemographicsResponse>, ModelError> {
    let mut query = sea_query::Query::select();

    query = query
        .columns(
            DEMOGRAPHICS_RESPONSE_COLUMNS
                .into_iter()
                .map(|col| (DemographicsResponseIden::Table, col)),
        )
        .from(DemographicsResponseIden::Table)
        .to_owned();

    query = filters.apply(query);

    let responses: PaginatedResults<DemographicsResponse> =
        page_options.fetch_paginated_results(db, query).await?;

    Ok(responses)
}

async fn validate_response_value(
    db: &PgPool,
    question_slug: &str,
    value: &str,
) -> Result<(), ModelError> {
    let question = get_demographics_question_by_slug(db, question_slug).await?;

    if let Some(bucket_config) = &question.bucket_config {
        if !bucket_config.0.accepts_value(value) {
            return Err(ValidationError::BadRequest(format!(
                "'{value}' is not a valid answer for question '{question_slug}'"
            ))
            .into());
        }
    }

    Ok(())
}

/// Add a new response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn create_demographics_response(
    db: &PgPool,
    new_demographics_response: CreateDemographicsResponse,
) -> Result<DemographicsResponse, ModelError> {
    validate_response_value(
        db,
        &new_demographics_response.question_slug,
        &new_demographics_response.value,
    )
    .await?;

    let (sql, values) = sea_query::Query::insert()
        .into_table(DemographicsResponseIden::Table)
        .columns(DEMOGRAPHICS_RESPONSE_COLUMNS)
        .values(new_demographics_response.values(Uuid::new_v4()))?
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics_response")?;

    Ok(response)
}

/// Update a response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn update_demographics_response(
    db: &PgPool,
    question_slug: String,
    user_id: Uuid,
    updated_demographics_response: PartialDemographicsResponse,
) -> Result<DemographicsResponse, ModelError> {
    if let Some(value) = &updated_demographics_response.value {
        validate_response_value(db, &question_slug, value).await?;
    }

    let update_values = updated_demographics_response.to_values();

    if update_values.is_empty() {
        let (sql, query_values) = sea_query::Query::select()
            .columns(DEMOGRAPHICS_RESPONSE_COLUMNS)
            .from(DemographicsResponseIden::Table)
            .and_where(Expr::col(DemographicsResponseIden::QuestionSlug).eq(question_slug))
            .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);

        let response = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, query_values)
            .fetch_one(db)
            .await
            .resolve_db_err("demographics_response")?;

        return Ok(response);
    }

    let (sql, values) = sea_query::Query::update()
        .table(DemographicsResponseIden::Table)
        .values(update_values)
        .and_where(Expr::col(DemographicsResponseIden::QuestionSlug).eq(question_slug))
        .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("demographics_response")?;

    Ok(response)
}

/// Delete a response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn delete_demographics_response(
    db: &PgPool,
    question_slug: String,
    user_id: Uuid,
) -> Result<Option<DemographicsResponse>, ModelError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(DemographicsResponseIden::Table)
        .and_where(Expr::col(DemographicsResponseIden::QuestionSlug).eq(question_slug))
        .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_optional(db)
        .await
        .resolve_db_err("demographics_response")?;

    Ok(deleted)
}
