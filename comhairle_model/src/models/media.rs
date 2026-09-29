use crate::models::error::ModelError;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use std::path::Path;
use tracing::instrument;
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    pagination::{Order, PageOptions, PaginatedResults},
};

/// File type to handle the incoming File from the HTTP Request
#[derive(Debug)]
pub struct File {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub content_type: String,
}

/// Incoming form type from the upload request
#[derive(Debug)]
pub struct UploadMediaForm {
    pub file: File,
    pub name: String,
    pub alt: String,
}

impl UploadMediaForm {
    pub fn new() -> Self {
        Self {
            file: File {
                filename: "".to_string(),
                bytes: Vec::new(),
                content_type: "".to_string(),
            },
            name: "".to_string(),
            alt: "".to_string(),
        }
    }
}

/// A media record, which references an upload in the bulk_storage_service.
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "media")]
pub struct Media {
    pub id: Uuid,
    /// Store name in bulk_storage_service
    pub store_name: String,
    /// Identifier in bulk_storage_service
    pub storage_key: String,
    pub filename: String,
    /// User defined identifier
    pub name: String,
    /// Alt text for the media
    pub alt: String,
    /// MIME type of the media uploaded
    pub content_type: MediaContentType,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Media {
    pub fn url(&self) -> String {
        format!(
            "https://{}.s3.amazonaws.com/{}",
            self.store_name, self.storage_key
        )
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub enum MediaContentType {
    #[sqlx(rename = "image/jpeg")]
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[sqlx(rename = "image/png")]
    #[serde(rename = "image/png")]
    Png,
    #[sqlx(rename = "image/gif")]
    #[serde(rename = "image/gif")]
    Gif,
    #[sqlx(rename = "image/webp")]
    #[serde(rename = "image/webp")]
    Webp,
    #[sqlx(rename = "video/mp4")]
    #[serde(rename = "video/mp4")]
    Mp4,
    #[sqlx(rename = "video/mpeg")]
    #[serde(rename = "video/mpeg")]
    Mpeg,
    #[sqlx(rename = "video/webm")]
    #[serde(rename = "video/webm")]
    Webm,
    #[sqlx(rename = "audio/mpeg")]
    #[serde(rename = "audio/mpeg")]
    Mp3,
    #[sqlx(rename = "audio/mp4")]
    #[serde(rename = "audio/mp4")]
    M4a,
    #[sqlx(rename = "audio/webm")]
    #[serde(rename = "audio/webm")]
    Weba,
    #[sqlx(rename = "audio/wav")]
    #[serde(rename = "audio/wav")]
    Wav,
    #[sqlx(rename = "audio/ogg")]
    #[serde(rename = "audio/ogg")]
    Oga,
}

impl From<MediaContentType> for sea_query::Value {
    fn from(val: MediaContentType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for MediaContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            MediaContentType::Jpeg => "image/jpeg",
            MediaContentType::Png => "image/png",
            MediaContentType::Gif => "image/gif",
            MediaContentType::Webp => "image/webp",
            MediaContentType::Mp4 => "video/mp4",
            MediaContentType::Mpeg => "video/mpeg",
            MediaContentType::Webm => "video/webm",
            MediaContentType::Mp3 => "audio/mpeg",
            MediaContentType::M4a => "audio/mp4",
            MediaContentType::Weba => "audio/webm",
            MediaContentType::Wav => "audio/wav",
            MediaContentType::Oga => "audio/ogg",
        };
        write!(f, "{}", value)
    }
}

impl MediaContentType {
    fn try_from_mime(source: &str) -> Result<Self, ModelError> {
        match source {
            "image/jpeg" => Ok(Self::Jpeg),
            "image/png" => Ok(Self::Png),
            "image/gif" => Ok(Self::Gif),
            "image/webp" => Ok(Self::Webp),
            "video/mp4" => Ok(Self::Mp4),
            "video/mpeg" => Ok(Self::Mpeg),
            "video/webm" => Ok(Self::Webm),
            "audio/mpeg" => Ok(Self::Mp3),
            "audio/m4a" => Ok(Self::M4a),
            "audio/webm" => Ok(Self::Weba),
            "audio/wav" => Ok(Self::Wav),
            "audio/ogg" => Ok(Self::Oga),
            ct => Err(ValidationError::UnsupportedContentType(ct.to_string()).into()),
        }
    }
    fn try_from_extension(extension: &str) -> Result<Self, ModelError> {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "gif" => Ok(Self::Gif),
            "webp" => Ok(Self::Webp),
            "mp4" => Ok(Self::Mp4),
            "mpeg" | "mpg" => Ok(Self::Mpeg),
            "webm" => Ok(Self::Webm),
            "mp3" => Ok(Self::Mp3),
            "weba" => Ok(Self::Weba),
            "wav" => Ok(Self::Wav),
            "oga" => Ok(Self::Oga),
            ext => Err(ValidationError::UnsupportedContentType(ext.to_string()).into()),
        }
    }
    pub fn get_type(file: &File) -> Result<Self, ModelError> {
        let mime = MediaContentType::try_from_mime(&file.content_type);
        if mime.is_ok() {
            return mime;
        }

        let extension = Path::new(&file.filename).extension();
        let Some(extension) = extension else {
            return Err(ValidationError::UnsupportedContentType(file.filename.clone()).into());
        };
        let Some(extension) = extension.to_str() else {
            return Err(ValidationError::UnsupportedContentType(file.filename.clone()).into());
        };
        let extension = MediaContentType::try_from_extension(extension);
        extension
    }
}

const DEFAULT_COLUMNS: [MediaIden; 10] = [
    MediaIden::Id,
    MediaIden::StoreName,
    MediaIden::StorageKey,
    MediaIden::Filename,
    MediaIden::Name,
    MediaIden::Alt,
    MediaIden::ContentType,
    MediaIden::OwnerId,
    MediaIden::CreatedAt,
    MediaIden::UpdatedAt,
];

/// A batch-loaded lookup table mapping media IDs to their resolved URLs.
///
/// `MediaResolver` exists to avoid N+1 queries when converting DB models
/// (which reference media by [`Uuid`]) into DTOs (which need the resolved
/// URL string). Load it once with [`MediaResolver::load`] for a batch of
/// IDs, then query it synchronously via [`MediaResolver::url_for`] while
/// assembling DTOs.
#[derive(Debug)]
pub struct MediaResolver(HashMap<Uuid, String>);

impl MediaResolver {
    /// Fetches media rows for the given `ids` and builds a resolver from them.
    ///
    /// Only rows matching `ids` are loaded, so this is safe to call with a
    /// list of IDs collected from an arbitrary batch of records. IDs with no
    /// matching row are simply absent from the resulting map.
    ///
    /// # Errors
    ///
    /// Returns [`crate::models::error::ModelError`] if the underlying query fails.
    pub async fn load(db: &PgPool, ids: &[Uuid]) -> Result<Self, ModelError> {
        let (sql, values) = Query::select()
            .from(MediaIden::Table)
            .columns(DEFAULT_COLUMNS)
            .and_where(Expr::col(MediaIden::Id).is_in(ids.to_owned()))
            .build_sqlx(PostgresQueryBuilder);

        let media = query_as_with::<_, Media, _>(&sql, values)
            .fetch_all(db)
            .await?;

        Ok(Self(
            media.into_iter().map(|row| (row.id, row.url())).collect(),
        ))
    }

    /// Returns the resolved URL for a given media `id`, if it was loaded.
    ///
    /// Returns `None` if `id` was not present in the batch passed to
    /// [`MediaResolver::load`]
    pub fn url_for(&self, id: Uuid) -> Option<String> {
        self.0.get(&id).map(|url| url.to_owned())
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateMedia {
    pub store_name: String,
    pub storage_key: String,
    pub filename: String,
    pub name: String,
    pub alt: String,
    pub content_type: MediaContentType,
}

impl CreateMedia {
    fn columns(&self) -> Vec<MediaIden> {
        vec![
            MediaIden::StoreName,
            MediaIden::StorageKey,
            MediaIden::Filename,
            MediaIden::Name,
            MediaIden::Alt,
            MediaIden::ContentType,
        ]
    }

    fn values(&self) -> Vec<SimpleExpr> {
        vec![
            (*self.store_name).into(),
            (*self.storage_key).into(),
            (*self.filename).into(),
            (*self.name).into(),
            (*self.alt).into(),
            self.content_type.clone().into(),
        ]
    }
}

/// Creates a new media record referencing media uploaded via the bulk_storage_service.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `create_media` - Params for the new media record to create
/// * `user_id` - Unique identifier of the owner of the created media
///
/// # Returns
///
/// Returns a `Result` containing the created `Media` record if successful or a
/// `ModelError` if the query fails.
#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    create_media: &CreateMedia,
    user_id: &Uuid,
) -> Result<Media, ModelError> {
    let mut columns = create_media.columns();
    let mut values = create_media.values();

    columns.push(MediaIden::OwnerId);
    values.push((*user_id).into());

    let (sql, values) = Query::insert()
        .into_table(MediaIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let media = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(media)
}

/// Retrieves a media record by its ID.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - Unique identifier of the media record to retrieve
///
/// # Returns
///
/// Returns a `Result` containing the `Media` record if found,
/// a `DataError::ResourceNotFound` if not found, or a
/// `ModelError` if the query fails for any other reason.
#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Media, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(MediaIden::Table)
        .and_where(Expr::col(MediaIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let media = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Media")?;

    Ok(media)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
pub struct MediaEditableFields {
    pub name: Option<String>,
    pub alt: Option<String>,
}

impl MediaEditableFields {
    fn to_values(&self) -> Vec<(MediaIden, sea_query::SimpleExpr)> {
        let mut values = Vec::new();

        if let Some(value) = &self.name {
            values.push((MediaIden::Name, value.into()));
        }

        if let Some(value) = &self.alt {
            values.push((MediaIden::Alt, value.into()));
        }

        values
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct MediaOrderOptions {
    pub filename: Option<Order>,
    pub created_at: Option<Order>,
}

impl MediaOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(ref order) = self.filename {
            query = query
                .order_by((MediaIden::Table, MediaIden::Filename), order.into())
                .to_owned()
        }
        if let Some(ref order) = self.created_at {
            query = query
                .order_by((MediaIden::Table, MediaIden::CreatedAt), order.into())
                .to_owned()
        }

        query
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct MediaFilterOptions {
    pub owner_id: Option<Uuid>,
    pub content_type: Option<MediaContentType>,
}

impl MediaFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.owner_id {
            query = query
                .and_where(Expr::col((MediaIden::Table, MediaIden::OwnerId)).eq(value))
                .to_owned()
        }
        if let Some(ref value) = self.content_type {
            query = query
                .and_where(Expr::col((MediaIden::Table, MediaIden::ContentType)).eq(value.clone()))
                .to_owned()
        }

        query
    }
}

/// Retrieves a paginated list of media records.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `page_options` - params for paginating results
/// * `order_options` - params for ordering results
/// * `filter_options` - params for filtering results
///
/// # Returns
///
/// Returns a `Result` containing a `PaginatedResults<Media>` if successful,
/// or a `ModelError` if the query fails.
#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: MediaOrderOptions,
    filter_options: MediaFilterOptions,
) -> Result<PaginatedResults<Media>, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(MediaIden::Table)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let media = page_options.fetch_paginated_results(db, query).await?;

    Ok(media)
}

/// Update a media record by its ID.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - Unique identifier of the media record to delete
/// * `update_media` - MediaEditableFields of the fields that you want to change
///
/// # Returns
///
/// Returns a `Result` containing the updated `Media` record, or `ModelError`
/// if the query fails.
#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_media: &MediaEditableFields,
) -> Result<Media, ModelError> {
    let values = update_media.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(MediaIden::Table)
        .values(values.into_iter())
        .and_where(Expr::col(MediaIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let media = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(media)
}

/// Deletes a media record by its ID.
///
/// # Arguments
///
/// * `db` - Database connection pool
/// * `id` - Unique identifier of the media record to delete
///
/// # Returns
///
/// Returns a `Result` containing the deleted `Media` record, or `ModelError`
/// if the query fails.
#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Media, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(MediaIden::Table)
        .and_where(Expr::col(MediaIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let media = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Media")?;

    Ok(media)
}
