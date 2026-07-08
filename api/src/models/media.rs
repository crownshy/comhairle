use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        pagination::{Order, PageOptions, PaginatedResults},
    },
};

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
#[cfg_attr(test, derive(Dummy))]
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
        };
        write!(f, "{}", value)
    }
}

impl MediaContentType {
    pub fn try_from_mime(source: &str) -> Result<Self, ComhairleError> {
        match source {
            "image/jpeg" => Ok(Self::Jpeg),
            "image/png" => Ok(Self::Png),
            "image/gif" => Ok(Self::Gif),
            "image/webp" => Ok(Self::Webp),
            "video/mp4" => Ok(Self::Mp4),
            "video/mpeg" => Ok(Self::Mpeg),
            "video/webm" => Ok(Self::Webm),
            "audio/mpeg" => Ok(Self::Mp3),
            ct => Err(ComhairleError::UnsupportedContentType(ct.to_string())),
        }
    }
}

impl MediaContentType {
    pub fn try_from_extension(extension: &str) -> Result<Self, ComhairleError> {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "gif" => Ok(Self::Gif),
            "webp" => Ok(Self::Webp),
            "mp4" => Ok(Self::Mp4),
            "mpeg" | "mpg" => Ok(Self::Mpeg),
            "webm" => Ok(Self::Webm),
            "mp3" => Ok(Self::Mp3),
            ext => Err(ComhairleError::UnsupportedContentType(ext.to_string())),
        }
    }
}

const DEFAULT_COLUMNS: [MediaIden; 8] = [
    MediaIden::Id,
    MediaIden::StoreName,
    MediaIden::StorageKey,
    MediaIden::Filename,
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
    /// Returns [`ComhairleError`] if the underlying query fails.
    pub async fn load(db: &PgPool, ids: &[Uuid]) -> Result<Self, ComhairleError> {
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

/// Converts a value into `Self` using a [`MediaResolver`] to resolve any
/// media references (e.g. `Uuid` fields) into their corresponding URLs.
///
/// This mirrors [`From`], but for conversions that need previously-resolved
/// media data rather than performing async DB lookups inline. Callers are
/// expected to batch-load a [`MediaResolver`] for all relevant IDs up front
/// (e.g. via [`MediaResolver::load`]) before calling `from_with_media`.
pub trait FromWithMedia<T> {
    /// Performs the conversion from `model` to `Self`, resolving media
    /// references via `media`.
    ///
    /// If a media reference is absent (e.g. an optional field with no
    /// associated media) or its ID could not be resolved by `media`,
    /// `fallback` is used in its place instead.
    fn from_with_media(model: T, media: &MediaResolver, fallback: &str) -> Self;
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateMedia {
    pub store_name: String,
    pub storage_key: String,
    pub filename: String,
    pub content_type: MediaContentType,
}

impl CreateMedia {
    fn columns(&self) -> Vec<MediaIden> {
        vec![
            MediaIden::StoreName,
            MediaIden::StorageKey,
            MediaIden::Filename,
            MediaIden::ContentType,
        ]
    }

    fn values(&self) -> Vec<SimpleExpr> {
        vec![
            (*self.store_name).into(),
            (*self.storage_key).into(),
            (*self.filename).into(),
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
/// `ComhairleError` if the query fails.
#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    create_media: &CreateMedia,
    user_id: &Uuid,
) -> Result<Media, ComhairleError> {
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
/// a `ComhairleError::ResourceNotFound` if not found, or a
/// `ComhairleError` if the query fails for any other reason.
#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Media, ComhairleError> {
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
/// or a `ComhairleError` if the query fails.
#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    order_options: MediaOrderOptions,
    filter_options: MediaFilterOptions,
) -> Result<PaginatedResults<Media>, ComhairleError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(MediaIden::Table)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let media = page_options.fetch_paginated_results(db, query).await?;

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
/// Returns a `Result` containing the deleted `Media` record, or `ComhairleError`
/// if the query fails.
#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Media, ComhairleError> {
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{model_test_helpers::setup_default_app_and_session, users},
        test_helpers::TEST_PASSWORD,
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_media_record(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let media = create(&pool, &params, &user.id).await?;

        assert_eq!(
            media.filename,
            "test-image.jpg".to_string(),
            "incorrect filename"
        );
        assert_eq!(
            media.content_type.to_string(),
            "image/jpeg".to_string(),
            "incorrect deserialized content_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_media_record_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let created_media = create(&pool, &params, &user.id).await?;

        let media = get_by_id(&pool, &created_media.id).await?;

        assert_eq!(
            media.filename,
            "test-image.jpg".to_string(),
            "incorrect filename"
        );
        assert_eq!(media.id, created_media.id, "mis-matching ids");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_paginated_list_of_media_records(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params_1 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-b.jpg".to_string(),
            filename: "image-b.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_2 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-a.jpg".to_string(),
            filename: "image-a.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_3 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-d.jpg".to_string(),
            filename: "image-d.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_4 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-c.jpg".to_string(),
            filename: "image-c.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_5 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-e.jpg".to_string(),
            filename: "image-e.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let _ = create(&pool, &params_1, &user.id).await?;
        let _ = create(&pool, &params_2, &user.id).await?;
        let media_3 = create(&pool, &params_3, &user.id).await?;
        let media_4 = create(&pool, &params_4, &user.id).await?;
        let media_5 = create(&pool, &params_5, &user.id).await?;

        let page_options = PageOptions {
            offset: Some(2),
            limit: Some(3),
        };
        let order_options = MediaOrderOptions {
            filename: Some(Order::Asc),
            created_at: None,
        };
        let filter_options = MediaFilterOptions {
            ..Default::default()
        };

        let results = list(&pool, page_options, order_options, filter_options).await?;

        assert_eq!(results.records.len(), 3, "incorrect number of results");
        assert_eq!(results.records[0].id, media_4.id, "incorrect first id");
        assert_eq!(results.records[1].id, media_3.id, "incorrect second id");
        assert_eq!(results.records[2].id, media_5.id, "incorrect third id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_media_by_owner(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;

        let user_1 = users::create_annon_user(&pool).await?;
        let user_2 = users::create_annon_user(&pool).await?;

        let params_1 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-b.jpg".to_string(),
            filename: "image-b.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_2 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-a.jpg".to_string(),
            filename: "image-a.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_3 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-d.jpg".to_string(),
            filename: "image-d.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let _ = create(&pool, &params_1, &user_2.id).await?;
        let media_2 = create(&pool, &params_2, &user_1.id).await?;
        let media_3 = create(&pool, &params_3, &user_1.id).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = MediaOrderOptions {
            ..Default::default()
        };
        let filter_options = MediaFilterOptions {
            owner_id: Some(user_1.id),
            ..Default::default()
        };

        let results = list(&pool, page_options, order_options, filter_options).await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert_eq!(results.records[0].id, media_2.id, "incorrect first id");
        assert_eq!(results.records[1].id, media_3.id, "incorrect second id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_media_record(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let created_media = create(&pool, &params, &user.id).await?;

        let _ = delete(&pool, &created_media.id).await?;

        let err = get_by_id(&pool, &created_media.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(e, "Media".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
