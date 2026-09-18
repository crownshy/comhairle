//! The central resource registry. Domain tables that permissions can target
//! directly (conversation, organization, ...) hold their id as a foreign key
//! into `resources`, so a resource must be registered here before the owning
//! row can be created, and is cleaned up when that row is deleted.

use bon::Builder;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "resources")]
pub struct Resource {
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub resource_type: String,
    pub created_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ResourceIden; 4] = [
    ResourceIden::Id,
    ResourceIden::OwnerId,
    ResourceIden::ResourceType,
    ResourceIden::CreatedAt,
];

#[derive(Debug, Deserialize, Serialize, Clone, Builder)]
pub struct CreateResource {
    pub owner_id: Option<Uuid>,
    pub resource_type: String,
}

impl CreateResource {
    pub fn columns() -> [ResourceIden; 4] {
        DEFAULT_COLUMNS
    }

    pub fn values(&self) -> [sea_query::SimpleExpr; 4] {
        [
            Uuid::new_v4().into(),
            self.owner_id.into(),
            self.resource_type.clone().into(),
            Utc::now().into(),
        ]
    }
}

/// Registers a new resource, returning its generated id.
#[instrument(err(Debug), skip(executor))]
pub async fn create<'e, E>(executor: E, create: CreateResource) -> Result<Resource, ComhairleError>
where
    E: PgExecutor<'e>,
{
    let (sql, values) = Query::insert()
        .into_table(ResourceIden::Table)
        .columns(CreateResource::columns())
        .values(create.values())?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let resource = query_as_with(&sql, values).fetch_one(executor).await?;

    Ok(resource)
}

/// Fetches a resource by id, e.g. to resolve its owner for permission checks.
#[instrument(err(Debug), skip(executor))]
pub async fn get_by_id<'e, E>(executor: E, id: Uuid) -> Result<Resource, ComhairleError>
where
    E: PgExecutor<'e>,
{
    let (sql, values) = Query::select()
        .from(ResourceIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ResourceIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let resource = query_as_with(&sql, values)
        .fetch_one(executor)
        .await
        .resolve_db_err("Resource")?;

    Ok(resource)
}

/// Deletes a resource. Domain rows referencing it are removed via
/// `ON DELETE CASCADE`, so this should only be called once the referencing
/// row has already been deleted (or to remove one that never got claimed).
#[instrument(err(Debug), skip(executor))]
pub async fn delete<'e, E>(executor: E, id: Uuid) -> Result<Resource, ComhairleError>
where
    E: PgExecutor<'e>,
{
    let (sql, values) = Query::delete()
        .from_table(ResourceIden::Table)
        .and_where(Expr::col(ResourceIden::Id).eq(id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let resource = query_as_with(&sql, values)
        .fetch_one(executor)
        .await
        .resolve_db_err("Resource")?;

    Ok(resource)
}
