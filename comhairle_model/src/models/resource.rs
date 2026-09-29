use crate::models::error::ModelError;
use std::fmt;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::DataError;

const PUT_EXPIRES: u64 = 600;
const GET_EXPIRES: u64 = 600;

const DEFAULT_COLUMNS: [ResourceIden; 9] = [
    ResourceIden::Id,
    ResourceIden::Name,
    ResourceIden::Description,
    ResourceIden::StorageType,
    ResourceIden::MediaType,
    ResourceIden::Url,
    ResourceIden::OwnerId,
    ResourceIden::CreatedAt,
    ResourceIden::UpdatedAt,
];

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResourceSource {
    S3,
    Url,
}

impl From<ResourceSource> for sea_query::Value {
    fn from(val: ResourceSource) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl fmt::Display for ResourceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ResourceSource::S3 => "s3",
            ResourceSource::Url => "url",
        };
        write!(f, "{}", value)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MediaType {
    Video,
    Image,
    Text,
}

impl From<MediaType> for sea_query::Value {
    fn from(val: MediaType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            MediaType::Video => "video",
            MediaType::Image => "image",
            MediaType::Text => "text",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, FromRow, Debug, PartialEq, Clone)]
#[enum_def(table_name = "resource")]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub storage_type: ResourceSource,
    pub url: String,
    pub media_type: MediaType,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateResource {
    pub name: String,
    pub description: String,
    pub storage_type: ResourceSource,
    pub url: String,
    pub media_type: MediaType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResourceResponse {
    pub id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub owner_id: Uuid,
}

#[instrument(err(Debug), skip(db))]
pub async fn create_resource(
    db: &PgPool,
    new_resource: CreateResource,
    owner_id: Uuid,
) -> Result<Resource, ModelError> {
    let (sql, values) = sea_query::Query::insert()
        .into_table(ResourceIden::Table)
        .columns([
            ResourceIden::Name,
            ResourceIden::Description,
            ResourceIden::StorageType,
            ResourceIden::MediaType,
            ResourceIden::Url,
            ResourceIden::OwnerId,
        ])
        .values([
            new_resource.name.into(),
            new_resource.description.into(),
            new_resource.storage_type.to_string().into(),
            new_resource.media_type.to_string().into(),
            new_resource.url.into(),
            owner_id.to_string().into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let resource = sqlx::query_as_with::<_, Resource, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| DataError::FailedToCreateResource {
            resource_type: "Resource".into(),
            error: e,
        })?;

    Ok(resource)
}

#[instrument(err(Debug), skip(db))]
pub async fn get(db: &PgPool, id: Uuid) -> Result<Resource, ModelError> {
    let (sql, values) = sea_query::Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ResourceIden::Table)
        .and_where(Expr::col(ResourceIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let resource = sqlx::query_as_with::<_, Resource, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| DataError::NoResourceFoundForId(id.to_owned()))?;
    Ok(resource)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResourceUploadResponse {
    pub url: String,
    pub id: Uuid,
}
