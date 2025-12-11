use std::str::FromStr;
use std::{fmt, sync::Arc};

use crate::{error::ComhairleError, translation_service::TranslationService};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

/// A type-safe wrapper around Uuid for referencing text content.
///
/// This wrapper provides type safety when referencing TextContent records from other models.
/// It ensures that only text content IDs are used where translatable content is expected,
/// preventing accidental confusion with other UUID fields.
///
/// # Examples
///
/// ```rust
/// use uuid::Uuid;
/// use comhairle::models::translations::TextContentId;
///
/// // Create from a UUID
/// let uuid = Uuid::new_v4();
/// let content_id = TextContentId::from(uuid);
///
/// // Convert back to UUID when needed
/// let uuid_back: Uuid = content_id.into();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct TextContentId(pub Uuid);

impl TextContentId {
    /// Creates a new TextContentId with a random UUID.
    ///
    /// # Returns
    ///
    /// A new TextContentId with a randomly generated UUID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a TextContentId from a UUID.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to wrap
    ///
    /// # Returns
    ///
    /// A new TextContentId wrapping the provided UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID value.
    ///
    /// # Returns
    ///
    /// The UUID wrapped by this TextContentId.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Consumes the TextContentId and returns the inner UUID.
    ///
    /// # Returns
    ///
    /// The UUID that was wrapped by this TextContentId.
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for TextContentId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for TextContentId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TextContentId> for Uuid {
    fn from(content_id: TextContentId) -> Self {
        content_id.0
    }
}

impl Into<sea_query::Value> for TextContentId {
    fn into(self) -> sea_query::Value {
        self.0.into()
    }
}

impl Into<sea_query::Value> for &TextContentId {
    fn into(self) -> sea_query::Value {
        self.0.into()
    }
}

impl fmt::Display for TextContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TextContentId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl sqlx::Type<sqlx::Postgres> for TextContentId {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for TextContentId {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let uuid = <Uuid as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self(uuid))
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for TextContentId {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <Uuid as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

/// Represents the format of text content that can be stored in the system.
///
/// This enum defines the supported text formats for content storage and rendering.
/// Each format determines how the content should be processed and displayed.
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "camelCase")]
pub enum TextFormat {
    /// Plain text format - no special formatting or markup
    #[sqlx(rename = "plain")]
    Plain,
    /// Markdown format - supports Markdown syntax for formatting
    #[sqlx(rename = "markdown")]
    Markdown,
    /// Rich text format - supports advanced formatting and styling
    #[sqlx(rename = "rich")]
    Rich,
}

impl Into<sea_query::Value> for TextFormat {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
    }
}

impl fmt::Display for TextFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            TextFormat::Plain => "plain",
            TextFormat::Markdown => "markdown",
            TextFormat::Rich => "rich",
        };
        write!(f, "{}", value)
    }
}

/// Represents text content that can be translated into multiple languages.
///
/// TextContent serves as the main container for content that needs to be available
/// in multiple locales. It defines the primary locale and format.
/// Individual translations are stored separately in TextTranslation records that reference this content.
#[derive(Serialize, Deserialize, JsonSchema, FromRow, Debug, PartialEq, Clone)]
#[enum_def(table_name = "text_content")]
pub struct TextContent {
    /// Unique identifier for this text content
    pub id: TextContentId,
    /// The primary locale/language code for this content (e.g., "en", "es", "fr")
    pub primary_locale: String,
    /// The format of the text content (plain, markdown, or rich)
    pub format: TextFormat,
    /// Timestamp when this content was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this content was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents a translation of text content in a specific locale.
///
/// TextTranslation stores the actual translated content for a specific language.
/// Each translation references a TextContent record and provides the translated
/// text in the specified locale. It also tracks whether the translation was
/// AI-generated and if it requires human validation.
#[derive(Serialize, Deserialize, JsonSchema, FromRow, Debug, PartialEq, Clone)]
#[enum_def(table_name = "text_translation")]
pub struct TextTranslation {
    /// Unique identifier for this translation
    pub id: Uuid,
    /// Reference to the TextContent this translation belongs to
    pub content_id: TextContentId,
    /// The locale/language code for this translation (e.g., "en", "es", "fr")
    pub locale: String,
    /// The actual translated text content
    pub content: String,
    /// Whether this translation was generated by AI
    pub ai_generated: bool,
    /// Whether this translation requires human validation
    pub requires_validation: bool,
    /// Timestamp when this translation was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when this translation was last updated
    pub updated_at: DateTime<Utc>,
}

const TEXT_CONTENT_DEFAULT_COLUMNS: [TextContentIden; 5] = [
    TextContentIden::Id,
    TextContentIden::PrimaryLocale,
    TextContentIden::Format,
    TextContentIden::CreatedAt,
    TextContentIden::UpdatedAt,
];

const TEXT_TRANSLATION_DEFAULT_COLUMNS: [TextTranslationIden; 8] = [
    TextTranslationIden::Id,
    TextTranslationIden::ContentId,
    TextTranslationIden::Locale,
    TextTranslationIden::Content,
    TextTranslationIden::AiGenerated,
    TextTranslationIden::RequiresValidation,
    TextTranslationIden::CreatedAt,
    TextTranslationIden::UpdatedAt,
];

/// Data transfer object for creating new text content.
///
/// This struct contains all the required fields for creating a new TextContent
/// record in the database. The ID and timestamps will be automatically generated.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateTextContent {
    /// The primary locale/language code for this content
    pub primary_locale: String,
    /// The format of the text content
    pub format: TextFormat,
}

impl CreateTextContent {
    /// Returns the database columns that will be inserted for this content.
    ///
    /// # Returns
    ///
    /// A vector of TextContentIden enum values representing the database columns.
    pub fn columns(&self) -> Vec<TextContentIden> {
        vec![TextContentIden::PrimaryLocale, TextContentIden::Format]
    }

    /// Returns the values to be inserted into the database columns.
    ///
    /// # Returns
    ///
    /// A vector of sea_query::SimpleExpr values corresponding to the columns.
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.primary_locale.clone().into(),
            self.format.to_string().into(),
        ]
    }
}

/// Data transfer object for creating new text translations.
///
/// This struct contains all the required fields for creating a new TextTranslation
/// record in the database. The ID and timestamps will be automatically generated.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateTextTranslation {
    /// Reference to the TextContent this translation belongs to
    pub content_id: TextContentId,
    /// The locale/language code for this translation
    pub locale: String,
    /// The actual translated text content
    pub content: String,
    /// Whether this translation was generated by AI (defaults to false)
    pub ai_generated: Option<bool>,
    /// Whether this translation requires human validation (defaults to false)
    pub requires_validation: Option<bool>,
}

impl CreateTextTranslation {
    /// Returns the database columns that will be inserted for this translation.
    ///
    /// # Returns
    ///
    /// A vector of TextTranslationIden enum values representing the database columns.
    pub fn columns(&self) -> Vec<TextTranslationIden> {
        vec![
            TextTranslationIden::ContentId,
            TextTranslationIden::Locale,
            TextTranslationIden::Content,
            TextTranslationIden::AiGenerated,
            TextTranslationIden::RequiresValidation,
        ]
    }

    /// Returns the values to be inserted into the database columns.
    ///
    /// # Returns
    ///
    /// A vector of sea_query::SimpleExpr values corresponding to the columns.
    /// Defaults ai_generated and requires_validation to false if not provided.
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let ai_generated = self.ai_generated.unwrap_or(false);
        let requires_validation = self.requires_validation.unwrap_or(false);

        vec![
            self.content_id.into(),
            self.locale.clone().into(),
            self.content.clone().into(),
            ai_generated.into(),
            requires_validation.into(),
        ]
    }
}

/// Data transfer object for updating existing text content.
///
/// This struct contains optional fields that can be updated on a TextContent record.
/// Only the provided (Some) fields will be updated in the database.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct UpdateTextContent {
    /// The new primary locale/language code for this content
    pub primary_locale: Option<String>,
    /// The new format of the text content
    pub format: Option<TextFormat>,
}

impl UpdateTextContent {
    /// Converts the update struct to database column-value pairs.
    ///
    /// Only fields that are Some(..) will be included in the update.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the column identifier and the new value.
    pub fn to_values(&self) -> Vec<(TextContentIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.primary_locale {
            values.push((TextContentIden::PrimaryLocale, value.into()));
        }
        if let Some(value) = &self.format {
            values.push((TextContentIden::Format, value.to_string().into()));
        }
        values
    }
}

/// Data transfer object for updating existing text translations.
///
/// This struct contains optional fields that can be updated on a TextTranslation record.
/// Only the provided (Some) fields will be updated in the database.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct UpdateTextTranslation {
    /// The new locale/language code for this translation
    pub locale: Option<String>,
    /// The new translated text content
    pub content: Option<String>,
    /// Whether this translation was generated by AI
    pub ai_generated: Option<bool>,
    /// Whether this translation requires human validation
    pub requires_validation: Option<bool>,
}

impl UpdateTextTranslation {
    /// Converts the update struct to database column-value pairs.
    ///
    /// Only fields that are Some(..) will be included in the update.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the column identifier and the new value.
    pub fn to_values(&self) -> Vec<(TextTranslationIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.locale {
            values.push((TextTranslationIden::Locale, value.into()));
        }
        if let Some(value) = &self.content {
            values.push((TextTranslationIden::Content, value.into()));
        }
        if let Some(value) = self.ai_generated {
            values.push((TextTranslationIden::AiGenerated, value.into()));
        }
        if let Some(value) = self.requires_validation {
            values.push((TextTranslationIden::RequiresValidation, value.into()));
        }
        values
    }
}

// TextContent CRUD operations

/// Creates a new text content record in the database.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `text_content` - The text content data to create
///
/// # Returns
///
/// Returns a `Result` containing the created `TextContent` on success,
/// or a `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * The database operation fails
#[instrument(err(Debug))]
pub async fn create_text_content(
    db: &PgPool,
    text_content: &CreateTextContent,
) -> Result<TextContent, ComhairleError> {
    let columns = text_content.columns();
    let values = text_content.values();

    let (sql, values) = Query::insert()
        .into_table(TextContentIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(TEXT_CONTENT_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_content = sqlx::query_as_with::<_, TextContent, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(text_content)
}

/// Retrieves a text content record by its ID.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text content to retrieve
///
/// # Returns
///
/// Returns a `Result` containing the `TextContent` if found,
/// or a `ComhairleError::ResourceNotFound` if not found.

#[instrument(err(Debug))]
pub async fn get_text_content_by_id(
    db: &PgPool,
    id: &TextContentId,
) -> Result<TextContent, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(TEXT_CONTENT_DEFAULT_COLUMNS)
        .from(TextContentIden::Table)
        .and_where(Expr::col(TextContentIden::Id).eq(id.as_uuid().to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let text_content = sqlx::query_as_with::<_, TextContent, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("TextContent".into()))?;

    Ok(text_content)
}

/// Updates an existing text content record.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text content to update
/// * `update` - The fields to update (only non-None fields will be updated)
///
/// # Returns
///
/// Returns a `Result` containing the updated `TextContent` on success,
/// or a `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * No valid updates are provided (all fields are None)
/// * The text content with the given ID does not exist
/// * The database operation fails
#[instrument(err(Debug))]
pub async fn update_text_content(
    db: &PgPool,
    id: &TextContentId,
    update: &UpdateTextContent,
) -> Result<TextContent, ComhairleError> {
    let mut values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    values.push((TextContentIden::CreatedAt, Utc::now().into()));

    let (sql, values) = Query::update()
        .table(TextContentIden::Table)
        .values(values)
        .and_where(Expr::col(TextContentIden::Id).eq(id.as_uuid().to_owned()))
        .returning(Query::returning().columns(TEXT_CONTENT_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_content = sqlx::query_as_with::<_, TextContent, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("TextContent".into()),
            _ => ComhairleError::DatabaseError(e),
        })?;

    Ok(text_content)
}

/// Deletes a text content record from the database.
///
/// This will also cascade delete all associated translations due to
/// the foreign key constraint in the database.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text content to delete
///
/// # Returns
///
/// Returns a `Result` containing the deleted `TextContent` on success,
/// or a `ComhairleError::ResourceNotFound` if the content doesn't exist.
#[instrument(err(Debug))]
pub async fn delete_text_content(
    db: &PgPool,
    id: &TextContentId,
) -> Result<TextContent, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(TextContentIden::Table)
        .and_where(Expr::col(TextContentIden::Id).eq(id.as_uuid().to_owned()))
        .returning(Query::returning().columns(TEXT_CONTENT_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_content = sqlx::query_as_with::<_, TextContent, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("TextContent".into()))?;

    Ok(text_content)
}

// TextTranslation CRUD operations

/// Creates a new text translation record in the database.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `text_translation` - The text translation data to create
///
/// # Returns
///
/// Returns a `Result` containing the created `TextTranslation` on success,
/// or a `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * The database operation fails
/// * The content_id references a non-existent text content
/// * A translation for the same content_id and locale already exists
#[instrument(err(Debug))]
pub async fn create_text_translation(
    db: &PgPool,
    text_translation: &CreateTextTranslation,
) -> Result<TextTranslation, ComhairleError> {
    let columns = text_translation.columns();
    let values = text_translation.values();

    let (sql, values) = Query::insert()
        .into_table(TextTranslationIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(TEXT_TRANSLATION_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_translation = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(text_translation)
}

/// Retrieves a text translation record by its ID.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text translation to retrieve
///
/// # Returns
///
/// Returns a `Result` containing the `TextTranslation` if found,
/// or a `ComhairleError::ResourceNotFound` if not found.
#[instrument(err(Debug))]
pub async fn get_text_translation_by_id(
    db: &PgPool,
    id: &Uuid,
) -> Result<TextTranslation, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(TEXT_TRANSLATION_DEFAULT_COLUMNS)
        .from(TextTranslationIden::Table)
        .and_where(Expr::col(TextTranslationIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let text_translation = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("TextTranslation".into()))?;

    Ok(text_translation)
}

/// Retrieves all text translation records for a specific text content.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `content_id` - The unique identifier of the text content
///
/// # Returns
///
/// Returns a `Result` containing a vector of `TextTranslation` records,
/// or a `ComhairleError` on database failure. Returns an empty vector
/// if no translations are found for the content.
#[instrument(err(Debug))]
pub async fn get_text_translations_by_content_id(
    db: &PgPool,
    content_id: &TextContentId,
) -> Result<Vec<TextTranslation>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(TEXT_TRANSLATION_DEFAULT_COLUMNS)
        .from(TextTranslationIden::Table)
        .and_where(Expr::col(TextTranslationIden::ContentId).eq(content_id.as_uuid().to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let text_translations = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(text_translations)
}

/// Retrieves a specific text translation by content ID and locale.
///
/// This is useful for finding a translation in a specific language
/// for a given piece of content.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `content_id` - The unique identifier of the text content
/// * `locale` - The locale/language code of the desired translation
///
/// # Returns
///
/// Returns a `Result` containing the `TextTranslation` if found,
/// or a `ComhairleError::ResourceNotFound` if no translation exists
/// for the given content and locale combination.
#[instrument(err(Debug))]
pub async fn get_text_translation_by_content_and_locale(
    db: &PgPool,
    content_id: &TextContentId,
    locale: &str,
) -> Result<TextTranslation, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(TEXT_TRANSLATION_DEFAULT_COLUMNS)
        .from(TextTranslationIden::Table)
        .and_where(Expr::col(TextTranslationIden::ContentId).eq(content_id.as_uuid().to_owned()))
        .and_where(Expr::col(TextTranslationIden::Locale).eq(locale))
        .build_sqlx(PostgresQueryBuilder);

    let text_translation = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("TextTranslation".into()))?;

    Ok(text_translation)
}

/// Updates an existing text translation record.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text translation to update
/// * `update` - The fields to update (only non-None fields will be updated)
///
/// # Returns
///
/// Returns a `Result` containing the updated `TextTranslation` on success,
/// or a `ComhairleError` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * No valid updates are provided (all fields are None)
/// * The text translation with the given ID does not exist
/// * The database operation fails
/// * The updated locale would create a duplicate (content_id, locale) pair
#[instrument(err(Debug))]
pub async fn update_text_translation(
    db: &PgPool,
    id: &Uuid,
    update: &UpdateTextTranslation,
) -> Result<TextTranslation, ComhairleError> {
    let values = update.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(TextTranslationIden::Table)
        .values(values)
        .and_where(Expr::col(TextTranslationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(TEXT_TRANSLATION_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_translation = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(text_translation)
}

/// Deletes a text translation record from the database.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - The unique identifier of the text translation to delete
///
/// # Returns
///
/// Returns a `Result` containing the deleted `TextTranslation` on success,
/// or a `ComhairleError::ResourceNotFound` if the translation doesn't exist.
#[instrument(err(Debug))]
pub async fn delete_text_translation(
    db: &PgPool,
    id: &Uuid,
) -> Result<TextTranslation, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(TextTranslationIden::Table)
        .and_where(Expr::col(TextTranslationIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(TEXT_TRANSLATION_DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let text_translation = sqlx::query_as_with::<_, TextTranslation, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("TextTranslation".into()))?;

    Ok(text_translation)
}

// Convenience functions for working with TextContentId

/// Creates a new text content and returns its ID for linking from other models.
///
/// This is a convenience function that creates text content and returns just the ID,
/// which can be used to link translatable content from other models like Conversation.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `text_content` - The text content data to create
///
/// # Returns
///
/// Returns a `Result` containing the `TextContentId` of the created content,
/// or a `ComhairleError` on failure.
#[instrument(err(Debug))]
pub async fn create_text_content_and_get_id(
    db: &PgPool,
    text_content: &CreateTextContent,
) -> Result<TextContentId, ComhairleError> {
    let content = create_text_content(db, text_content).await?;
    Ok(content.id)
}

/// Retrieves the translation for a specific text content and locale, if it exists.
///
/// This is a convenience function that returns an Option instead of an error
/// when no translation is found, making it easier to handle optional translations.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `content_id` - The unique identifier of the text content
/// * `locale` - The locale/language code of the desired translation
///
/// # Returns
///
/// Returns a `Result` containing `Some(TextTranslation)` if found,
/// `None` if no translation exists, or a `ComhairleError` on database failure.
#[instrument(err(Debug))]
pub async fn get_text_translation_optional(
    db: &PgPool,
    content_id: &TextContentId,
    locale: &str,
) -> Result<Option<TextTranslation>, ComhairleError> {
    match get_text_translation_by_content_and_locale(db, content_id, locale).await {
        Ok(translation) => Ok(Some(translation)),
        Err(ComhairleError::ResourceNotFound(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Translate all languages that exist for this
/// text content. Will use the primary_locale as the
/// base line and use the translator service to generate
/// all of the others
// TODO This can be improved, it currently looks up
// the text content for each translation. We can
// make this smoother
#[instrument(err(Debug), skip(translator))]
pub async fn auto_generate_all_translations(
    db: &PgPool,
    translator: &Arc<dyn TranslationService>,
    text_content_id: &TextContentId,
) -> Result<Vec<TextTranslation>, ComhairleError> {
    let text_content = get_text_content_by_id(db, text_content_id).await?;
    let translations = get_text_translations_by_content_id(db, text_content_id).await?;
    let mut result: Vec<TextTranslation> = vec![];
    for translation in translations.iter() {
        if translation.locale != text_content.primary_locale {
            let new_translation =
                auto_generate_translation(db, translator, &translation.id).await?;
            result.push(new_translation);
        }
    }
    return Ok(result);
}

/// Update this translation using the primary local
/// as a reference
#[instrument(err(Debug), skip(translator))]
pub async fn auto_generate_translation(
    db: &PgPool,
    translator: &Arc<dyn TranslationService>,
    text_translation_id: &Uuid,
) -> Result<TextTranslation, ComhairleError> {
    let translation = get_text_translation_by_id(db, text_translation_id).await?;
    let text_content = get_text_content_by_id(db, &translation.content_id).await?;
    let reference_text = get_text_translation_by_content_and_locale(
        db,
        &text_content.id,
        &text_content.primary_locale,
    )
    .await?;

    let translated_text = translator
        .translate_from_to(
            &reference_text.content,
            &reference_text.locale,
            &translation.locale,
        )
        .await?;

    let updated_translation = update_text_translation(
        db,
        &translation.id,
        &UpdateTextTranslation {
            content: Some(translated_text),
            ai_generated: Some(true),
            requires_validation: Some(true),
            ..Default::default()
        },
    )
    .await?;
    Ok(updated_translation)
}

#[instrument(err(Debug))]
pub async fn new_translation(
    db: &PgPool,
    locale: &str,
    content: &str,
    format: TextFormat,
) -> Result<TextContent, ComhairleError> {
    let translation = create_text_content(
        db,
        &CreateTextContent {
            primary_locale: locale.to_owned(),
            format,
        },
    )
    .await?;

    create_text_translation(
        db,
        &CreateTextTranslation {
            content_id: translation.id,
            locale: locale.to_owned(),
            content: content.to_owned(),
            ai_generated: Some(false),
            requires_validation: Some(false),
        },
    )
    .await?;
    Ok(translation)
}
