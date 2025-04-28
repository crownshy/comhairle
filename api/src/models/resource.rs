use std::fmt;
use std::time::Duration;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client};

use crate::ComhairleState;
use crate::{config::ComhairleConfig, error::ComhairleError};

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

impl Into<sea_query::Value> for ResourceSource {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
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

impl Into<sea_query::Value> for MediaType {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
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

impl Resource {
    pub async fn to_resource_response(
        &self,
        client: &Client,
        bucket: &str,
    ) -> Result<ResourceResponse, ComhairleError> {
        let url = self.resolve_url(client, bucket).await?;
        Ok(ResourceResponse {
            id: self.id,
            url,
            media_type: self.media_type,
            owner_id: self.owner_id,
        })
    }
}

pub async fn get_presigned_url(
    target: &str,
    client: &Client,
    bucket: &str,
) -> Result<String, ComhairleError> {
    let expires_in = Duration::from_secs(GET_EXPIRES);
    let url = client
        .get_object()
        .bucket(bucket)
        .key(target)
        .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        .await
        .map_err(|e| ComhairleError::FailedToGetDownloadPresign(e.to_string()))?;

    Ok(url.uri().into())
}
pub async fn get_signed_upload_url(
    target_dest: &str,
    client: &Client,
    bucket: &str,
) -> Result<String, ComhairleError> {
    let expires_in: Duration = std::time::Duration::from_secs(PUT_EXPIRES);

    let expires_in: aws_sdk_s3::presigning::PresigningConfig =
        PresigningConfig::expires_in(expires_in).unwrap();

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(target_dest)
        .presigned(expires_in)
        .await
        .map_err(|e| ComhairleError::FailedToGetUploadPresign(e.to_string()))?;

    Ok(presigned_request.uri().into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResourceUploadResponse {
    pub url: String,
    pub id: Uuid,
}

pub async fn create_resource(
    db: &PgPool,
    new_resource: CreateResource,
    owner_id: Uuid,
) -> Result<Resource, ComhairleError> {
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
        .map_err(|e| ComhairleError::FailedToCreateResource {
            resource_type: "Resource".into(),
            error: e,
        })?;

    Ok(resource)
}

pub async fn get(db: &PgPool, id: Uuid) -> Result<Resource, ComhairleError> {
    let (sql, values) = sea_query::Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ResourceIden::Table)
        .and_where(Expr::col(ResourceIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let resource = sqlx::query_as_with::<_, Resource, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::NoResourceFoundForId(id.to_owned()))?;
    Ok(resource)
}

impl Resource {
    pub async fn resolve_url(
        &self,
        client: &Client,
        bucket: &str,
    ) -> Result<String, ComhairleError> {
        match self.storage_type {
            ResourceSource::S3 => get_presigned_url(&self.url, client, bucket).await,
            ResourceSource::Url => Ok(self.url.clone()),
        }
    }
}
