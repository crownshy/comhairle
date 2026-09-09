// Context: We migrated user profile demographics to an EAV key-value schema using three tables:
// 1. `demographics_question`                     - Represents a demographics question
// (
//    slug TEXT PK,                               - Unique identifier for the demographics question
//    display_name TEXT NOT NULL,                 - Human-readable name for the demographics question
//    response_type 'string' | 'number' NOT NULL, - The type of response expected for the demographics question
//    bucket_config JSONB                         - Configuration for how responses should be bucketed (e.g., age ranges)
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

use crate::error::ComhairleError;
use crate::models::pagination::{PageOptions, PaginatedResults};

// ============================================================================
// Conversation Demographics Models
// ============================================================================

/// Represents an association between a conversation and a demographics question.
#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "conversation_demographics")]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
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

/// Represents demographics question response type (either 'string' or 'number').
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub enum DemographicsQuestionResponseType {
    String,
    Number,
}

impl Into<SimpleExpr> for DemographicsQuestionResponseType {
    fn into(self) -> SimpleExpr {
        Expr::val(self.as_ref()).cast_as(Alias::new("demographics_response_type"))
    }
}

impl AsRef<str> for DemographicsQuestionResponseType {
    fn as_ref(&self) -> &str {
        match self {
            Self::String => "string",
            Self::Number => "number",
        }
    }
}

impl Decode<'_, Postgres> for DemographicsQuestionResponseType {
    fn decode(
        row: <Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = row.as_str()?;
        match s {
            "string" => return Ok(DemographicsQuestionResponseType::String),
            "number" => return Ok(DemographicsQuestionResponseType::Number),
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
#[cfg_attr(test, derive(PartialEq))]
pub struct NumericBucket {
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct TextBucket {
    pub values: Option<Vec<String>>,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ValueBuckets {
    Numeric { buckets: Vec<NumericBucket> },
    Text { buckets: Vec<TextBucket> },
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
        ValueBuckets::Text {
            buckets: text_buckets,
        } => {
            for bucket in text_buckets {
                if let Some(values) = &bucket.values {
                    if values.contains(&value.to_string()) {
                        return bucket.label.clone();
                    }
                }
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
#[cfg_attr(test, derive(PartialEq))]
pub struct DemographicsQuestion {
    #[partially(omit)]
    pub slug: String,
    pub display_name: String,
    pub response_type: DemographicsQuestionResponseType,
    #[schemars(with = "Option<Vec<ValueBuckets>>")]
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
    #[schemars(with = "Option<Vec<ValueBuckets>>")]
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
            let json_expr = bucket_config
                .map(|json| serde_json::to_value(json.0).unwrap())
                .into();

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

/// Represents a demographics response from a user to a specific demographics question.
#[derive(Serialize, Deserialize, Partial, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "demographics_response")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct DemographicsResponse {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub question_slug: String,
    #[partially(omit)]
    pub user_id: Option<Uuid>,
    pub value: String,
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
) -> Result<PaginatedResults<ConversationDemographics>, ComhairleError> {
    let mut query = sea_query::Query::select()
        .columns(CONVERSATION_DEMOGRAPHICS_COLUMNS)
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
) -> Result<ConversationDemographics, ComhairleError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(ConversationDemographicsIden::Table)
        .columns(CreateConversationDemographics::columns())
        .values(new_conversation_demographics.values())?
        .returning(sea_query::Query::returning().columns(CONVERSATION_DEMOGRAPHICS_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let created = sqlx::query_as_with::<_, ConversationDemographics, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(created)
}

/// Remove an association between a conversation and a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn delete_conversation_demographics(
    db: &PgPool,
    conversation_id: Uuid,
    question_slug: String,
) -> Result<Option<ConversationDemographics>, ComhairleError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(ConversationDemographicsIden::Table)
        .and_where(Expr::col(ConversationDemographicsIden::ConversationId).eq(conversation_id))
        .and_where(Expr::col(ConversationDemographicsIden::QuestionSlug).eq(question_slug))
        .returning(sea_query::Query::returning().columns(CONVERSATION_DEMOGRAPHICS_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, ConversationDemographics, _>(&sql, values)
        .fetch_optional(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(deleted)
}

// ============================================================================
// Demographics questions - CRUD
// ============================================================================

/// Get a demographics questions with optional filters.
#[instrument(err(Debug), skip(db))]
pub async fn get_demographics_questions(
    db: &PgPool,
    filters: DemographicsQuestionsFilterOptions,
    page_options: PageOptions,
) -> Result<PaginatedResults<DemographicsQuestion>, ComhairleError> {
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
) -> Result<DemographicsQuestion, ComhairleError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(DemographicsQuestionIden::Table)
        .columns(CreateDemographicsQuestion::columns())
        .values(new_demographics_question.values())?
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(question)
}

/// Update a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn update_demographics_question(
    db: &PgPool,
    slug: String,
    updated_demographics_question: PartialDemographicsQuestion,
) -> Result<DemographicsQuestion, ComhairleError> {
    let (sql, values) = sea_query::Query::update()
        .table(DemographicsQuestionIden::Table)
        .values(updated_demographics_question.to_values())
        .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(slug))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(question)
}

/// Delete a demographics question.
#[instrument(err(Debug), skip(db))]
pub async fn delete_demographics_question(
    db: &PgPool,
    slug: String,
) -> Result<Option<DemographicsQuestion>, ComhairleError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(DemographicsQuestionIden::Table)
        .and_where(Expr::col(DemographicsQuestionIden::Slug).eq(slug))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_QUESTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted_question = sqlx::query_as_with::<_, DemographicsQuestion, _>(&sql, values)
        .fetch_optional(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

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
) -> Result<PaginatedResults<DemographicsResponse>, ComhairleError> {
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

/// Add a new response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn create_demographics_response(
    db: &PgPool,
    new_demographics_response: CreateDemographicsResponse,
) -> Result<DemographicsResponse, ComhairleError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(DemographicsResponseIden::Table)
        .columns(DEMOGRAPHICS_RESPONSE_COLUMNS)
        .values(new_demographics_response.values(Uuid::new_v4()))?
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(response)
}

/// Update a response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn update_demographics_response(
    db: &PgPool,
    question_slug: String,
    user_id: Uuid,
    updated_demographics_response: PartialDemographicsResponse,
) -> Result<DemographicsResponse, ComhairleError> {
    let (sql, values) = sea_query::Query::update()
        .table(DemographicsResponseIden::Table)
        .values(updated_demographics_response.to_values())
        .and_where(Expr::col(DemographicsResponseIden::QuestionSlug).eq(question_slug))
        .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let response = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(response)
}

/// Delete a response for a specific demographics question and user.
#[instrument(err(Debug), skip(db))]
pub async fn delete_demographics_response(
    db: &PgPool,
    question_slug: String,
    user_id: Uuid,
) -> Result<Option<DemographicsResponse>, ComhairleError> {
    let (sql, values) = sea_query::Query::delete()
        .from_table(DemographicsResponseIden::Table)
        .and_where(Expr::col(DemographicsResponseIden::QuestionSlug).eq(question_slug))
        .and_where(Expr::col(DemographicsResponseIden::UserId).eq(user_id))
        .returning(sea_query::Query::returning().columns(DEMOGRAPHICS_RESPONSE_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_as_with::<_, DemographicsResponse, _>(&sql, values)
        .fetch_optional(db)
        .await
        .map_err(|e| ComhairleError::DatabaseError(e))?;

    Ok(deleted)
}

#[cfg(test)]
mod tests {
    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::test_helpers::UserSession;

    use super::*;

    // Test can create, get, update and delete a demographics question.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_get_update_delete_demographics_question(
        db: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slug = "example_slug".to_string();
        let response_type = DemographicsQuestionResponseType::Number;

        // Ensure the question does not exist before creation
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            response.total, 0,
            "Question should not exist before creation"
        );

        // Create the demographics question and ensure it can be retrieved successfully
        let new_demographics_question = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, new_demographics_question).await?;
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(response.total, 1, "Question should exist after creation");
        assert_eq!(vec![question], response.records);

        // Update the demographics question and ensure the changes are reflected
        let update = PartialDemographicsQuestion {
            display_name: Some("Updated Display Name".to_string()),
            response_type: Some(DemographicsQuestionResponseType::String),
            bucket_config: None,
        };
        let updated_question = update_demographics_question(&db, slug.clone(), update).await?;
        assert_eq!(
            updated_question.response_type,
            DemographicsQuestionResponseType::String,
            "Question response type should be updated correctly"
        );
        assert_eq!(
            updated_question.display_name,
            "Updated Display Name".to_string(),
            "Question display name should be updated correctly"
        );

        // Delete the demographics question and ensure it is removed successfully
        let response = delete_demographics_question(&db, slug.clone()).await?;
        assert_eq!(
            Some(updated_question),
            response,
            "Question should be deleted successfully"
        );

        // Ensure the question is actually deleted
        let filters = DemographicsQuestionsFilterOptions {
            question_slug: Some(slug.clone()),
            ..Default::default()
        };
        let response = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            response.total, 0,
            "Question should not exist after deletion"
        );

        Ok(())
    }

    // Test can create a demographics question, associate it with a conversation, and then delete it.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_associate_delete_demographics_question(
        db: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let slug = "example_slug".to_string();
        let response_type = DemographicsQuestionResponseType::Number;

        // Create the demographics question
        let payload = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type,
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create a conversation
        let (app, mut session) = setup_default_app_and_session(&db).await?;
        let (status, response, _) = session.create_random_conversation(&app).await?;
        let conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get conversation id")?,
        )?;

        assert!(status.is_success(), "Failed to create random conversation");

        // Ensure the question is initially absent
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 0,
            "Question should not be initially associated with the conversation"
        );

        // Associate the demographics question with the conversation
        let payload = CreateConversationDemographics {
            conversation_id: conversation_id,
            question_slug: question.slug.clone(),
        };
        let response = create_conversation_demographics(&db, payload).await?;

        assert_eq!(
            conversation_id, response.conversation_id,
            "Conversation ID should match after associating demographics question"
        );
        assert_eq!(
            question.slug, response.question_slug,
            "Question slug should match after associating demographics question"
        );

        // Ensure the question is now associated with the conversation
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 1,
            "Question should be associated with the conversation after creation"
        );
        assert_eq!(
            questions.records[0].slug, question.slug,
            "The associated question slug should match the created question slug"
        );

        // Delete the demographics question and ensure it is removed successfully
        let response = delete_demographics_question(&db, question.slug.clone()).await?;
        assert_eq!(
            Some(question.slug.clone()),
            response.map(|r| r.slug),
            "Deleted question slug should match the original question slug"
        );

        // Ensure the question is actually deleted
        let filters = DemographicsQuestionsFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let questions = get_demographics_questions(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            questions.total, 0,
            "Question should not be associated with the conversation after deletion"
        );

        Ok(())
    }

    // Test can create, get, update and delete a demographics response.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_create_get_update_delete_demographics_response(
        db: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&db).await?;

        // Get the session user
        let (status, response, _) = session.current_user(&app).await?;
        assert!(status.is_success(), "Failed to get current user");
        let user_id = response.id;

        // Create a demographics question
        let slug = "test_question_slug".to_string();
        let response_type = DemographicsQuestionResponseType::String;
        let payload = CreateDemographicsQuestion {
            slug: slug.clone(),
            display_name: "Example Display Name".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create a demographics response
        let response_value = "test_response".to_string();
        let payload = CreateDemographicsResponse {
            question_slug: question.slug.clone(),
            user_id,
            value: response_value.clone(),
        };
        let demographics_response = create_demographics_response(&db, payload).await?;
        assert_eq!(
            demographics_response.value, response_value,
            "Demographics response value should match the provided response"
        );

        // Get the demographics response by question slug and user ID
        let filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            user_id: Some(user_id),
            conversation_id: None,
        };
        let fetched_responses =
            get_demographics_responses(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            fetched_responses.total, 1,
            "Fetched demographics responses should not be empty"
        );
        assert_eq!(
            demographics_response, fetched_responses.records[0],
            "Fetched demographics response should match the created response"
        );

        // Update the demographics response
        let new_response_value = "updated_test_response".to_string();
        let payload = PartialDemographicsResponse {
            value: Some(new_response_value.clone()),
        };
        let updated_demographics_response =
            update_demographics_response(&db, question.slug.clone(), user_id, payload).await?;
        assert_eq!(
            new_response_value, updated_demographics_response.value,
            "Updated demographics response value should match the new response"
        );

        // Delete the demographics response
        let deleted_response =
            delete_demographics_response(&db, question.slug.clone(), user_id).await?;
        assert_eq!(
            Some(updated_demographics_response),
            deleted_response,
            "Demographics response should be successfully deleted"
        );

        // Verify the demographics response has been deleted
        let filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            user_id: Some(user_id),
            conversation_id: None,
        };
        let fetched_responses_after_deletion =
            get_demographics_responses(&db, filters, PageOptions::default()).await?;
        assert_eq!(
            fetched_responses_after_deletion.total, 0,
            "Demographics response should be deleted"
        );

        Ok(())
    }

    // Test can create a demographics response for a specific demographics question, associated with a conversation, and list the response by question, conversation and user.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn test_list_response_by_conversation_question_and_user(
        db: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (app, mut session) = setup_default_app_and_session(&db).await?;

        // Get the session user
        let (status, user_dto, _) = session.current_user(&app).await?;
        assert!(status.is_success(), "Failed to get current user");
        let user_id = user_dto.id;

        // Create a conversation
        let (status, response, _) = session.create_random_conversation(&app).await?;
        assert!(status.is_success(), "Failed to create random conversation");
        let conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get conversation id")?,
        )?;

        // Create another conversation
        let (status, response, _) = session.create_random_conversation(&app).await?;
        assert!(status.is_success(), "Failed to create random conversation");
        let other_conversation_id: Uuid = serde_json::from_value(
            response
                .get("id")
                .cloned()
                .ok_or("Failed to get other conversation id")?,
        )?;

        // Create a second user for testing
        let mut other_session = UserSession::new_guest();
        let (status, other_user_dto, _) = other_session.signup_guest(&app).await?;
        assert!(status.is_success(), "Failed to create random user");
        let other_user_id: Uuid = serde_json::from_value(
            other_user_dto
                .get("id")
                .cloned()
                .flatten()
                .ok_or("Failed to get other user id")?,
        )?;

        // Create a demographics question
        let question_slug = "test_question".to_string();
        let response_type = DemographicsQuestionResponseType::String;
        let payload = CreateDemographicsQuestion {
            slug: question_slug.clone(),
            display_name: "Test Question".to_string(),
            response_type: response_type.clone(),
            bucket_config: None,
        };
        let question = create_demographics_question(&db, payload).await?;

        // Create another demographics question
        let other_question_slug = "other_test_question".to_string();
        let other_response_type = DemographicsQuestionResponseType::Number;
        let other_payload = CreateDemographicsQuestion {
            slug: other_question_slug.clone(),
            display_name: "Other Test Question".to_string(),
            response_type: other_response_type.clone(),
            bucket_config: None,
        };
        let other_question = create_demographics_question(&db, other_payload).await?;

        // Associate the demographics questions with the conversations
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: conversation_id,
                question_slug: question.slug.clone(),
            },
        )
        .await?;
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: conversation_id,
                question_slug: other_question.slug.clone(),
            },
        )
        .await?;
        let _ = create_conversation_demographics(
            &db,
            CreateConversationDemographics {
                conversation_id: other_conversation_id,
                question_slug: other_question.slug.clone(),
            },
        )
        .await?;

        // Create several demographics responses associated with the question and user
        let response_value = "test_response".to_string();
        let other_response_value = "15".to_string();
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: question.slug.clone(),
                user_id,
                value: response_value.clone(),
            },
        )
        .await?;
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: other_question.slug.clone(),
                user_id,
                value: other_response_value.clone(),
            },
        )
        .await?;
        let _ = create_demographics_response(
            &db,
            CreateDemographicsResponse {
                question_slug: question.slug.clone(),
                user_id: other_user_id,
                value: response_value.clone(),
            },
        )
        .await?;

        // List demographics responses by conversation
        let conversation_responses_filters = DemographicsResponsesFilterOptions {
            conversation_id: Some(conversation_id),
            ..Default::default()
        };
        let conversation_responses =
            get_demographics_responses(&db, conversation_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            conversation_responses.total, 3,
            "Expected 3 demographics responses for the conversation"
        );

        let other_conversation_responses_filters = DemographicsResponsesFilterOptions {
            conversation_id: Some(other_conversation_id),
            ..Default::default()
        };
        let other_conversation_responses = get_demographics_responses(
            &db,
            other_conversation_responses_filters,
            PageOptions::default(),
        )
        .await?;
        assert_eq!(
            other_conversation_responses.total, 1,
            "Expected 1 demographics response for the other conversation"
        );

        // List demographics responses by question
        let question_responses_filters = DemographicsResponsesFilterOptions {
            question_slug: Some(question.slug.clone()),
            ..Default::default()
        };
        let question_responses =
            get_demographics_responses(&db, question_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            question_responses.total, 2,
            "Expected 2 demographics responses for the question"
        );

        let other_question_responses_filters = DemographicsResponsesFilterOptions {
            question_slug: Some(other_question.slug.clone()),
            ..Default::default()
        };
        let other_question_responses = get_demographics_responses(
            &db,
            other_question_responses_filters,
            PageOptions::default(),
        )
        .await?;
        assert_eq!(
            other_question_responses.total, 1,
            "Expected 1 demographics response for the other question"
        );

        // List demographics responses by user
        let user_responses_filters = DemographicsResponsesFilterOptions {
            user_id: Some(user_id),
            ..Default::default()
        };
        let user_responses =
            get_demographics_responses(&db, user_responses_filters, PageOptions::default()).await?;
        assert_eq!(
            user_responses.total, 2,
            "Expected 2 demographics responses for the user"
        );

        let other_user_responses_filters = DemographicsResponsesFilterOptions {
            user_id: Some(other_user_id),
            ..Default::default()
        };
        let other_user_responses =
            get_demographics_responses(&db, other_user_responses_filters, PageOptions::default())
                .await?;
        assert_eq!(
            other_user_responses.total, 1,
            "Expected 1 demographics response for the other user"
        );

        assert_ne!(
            user_responses.total, other_user_responses.total,
            "Expected the user responses and other user responses to be different"
        );

        Ok(())
    }
}
