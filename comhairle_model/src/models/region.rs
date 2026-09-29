use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;

use crate::models::error::ValidationError;
use crate::models::{
    SqlxResultExt,
    organization::OrganizationIden,
    pagination::{Order, PageOptions, PaginatedResults},
    translations::{TextContentId, TextFormat, new_translation},
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "region")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
pub struct Region {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub name: TextContentId,
    #[partially(omit)]
    pub description: TextContentId,
    pub region_type: RegionType,
    #[partially(transparent)]
    pub official_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema,
)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub enum RegionType {
    #[sqlx(rename = "custom")]
    Custom,
    #[sqlx(rename = "official")]
    #[default]
    Official,
}

impl From<RegionType> for sea_query::Value {
    fn from(val: RegionType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RegionType::Custom => "custom",
            RegionType::Official => "official",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [RegionIden; 8] = [
    RegionIden::Id,
    RegionIden::Name,
    RegionIden::Description,
    RegionIden::RegionType,
    RegionIden::OfficialId,
    RegionIden::Metadata,
    RegionIden::CreatedAt,
    RegionIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateRegion {
    pub name: String,
    pub description: String,
    pub region_type: RegionType,
    pub official_id: Option<String>,
}

impl CreateRegion {
    fn columns(&self) -> Vec<RegionIden> {
        let mut columns = vec![RegionIden::RegionType];
        if self.official_id.is_some() {
            columns.push(RegionIden::OfficialId);
        }

        columns
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec![self.region_type.clone().into()];
        if let Some(value) = &self.official_id {
            values.push(value.into());
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    new_region: &CreateRegion,
    locale: &str,
) -> Result<Region, ModelError> {
    let mut columns = new_region.columns();
    let mut values = new_region.values();

    let name_translation = new_translation(db, locale, &new_region.name, TextFormat::Plain).await?;
    let description_translation =
        new_translation(db, locale, &new_region.description, TextFormat::Plain).await?;

    columns.push(RegionIden::Name);
    values.push(name_translation.id.into());

    columns.push(RegionIden::Description);
    values.push(description_translation.id.into());

    let (sql, values) = Query::insert()
        .into_table(RegionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(region)
}

#[instrument(err(Debug), skip(db))]
pub async fn set_area_links(
    db: &PgPool,
    region_id: &Uuid,
    area_ids: &[Uuid],
) -> Result<(), ModelError> {
    let mut tx = db.begin().await?;

    sqlx::query("DELETE FROM region_region_area WHERE region_id = $1")
        .bind(region_id)
        .execute(&mut *tx)
        .await?;

    for area_id in area_ids {
        sqlx::query(
            "INSERT INTO region_region_area (region_id, region_area_id)
                VALUES ($1, $2)
                ON CONFLICT (region_id, region_area_id) DO NOTHING",
        )
        .bind(region_id)
        .bind(area_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn list_area_ids(db: &PgPool, region_id: &Uuid) -> Result<Vec<Uuid>, ModelError> {
    let area_ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT region_area_id
            FROM region_region_area
            WHERE region_id = $1
            ORDER BY region_area_id",
    )
    .bind(region_id)
    .fetch_all(db)
    .await?;

    Ok(area_ids)
}

#[instrument(err(Debug), skip(db))]
pub async fn add_area_link(
    db: &PgPool,
    region_id: &Uuid,
    area_id: &Uuid,
) -> Result<(), ModelError> {
    sqlx::query(
        "INSERT INTO region_region_area (region_id, region_area_id)
            VALUES ($1, $2)
            ON CONFLICT (region_id, region_area_id) DO NOTHING",
    )
    .bind(region_id)
    .bind(area_id)
    .execute(db)
    .await?;

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn remove_area_link(
    db: &PgPool,
    region_id: &Uuid,
    area_id: &Uuid,
) -> Result<(), ModelError> {
    sqlx::query("DELETE FROM region_region_area WHERE region_id = $1 AND region_area_id = $2")
        .bind(region_id)
        .bind(area_id)
        .execute(db)
        .await?;

    Ok(())
}

impl PartialRegion {
    pub fn to_values(&self) -> Vec<(RegionIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.region_type {
            values.push((RegionIden::RegionType, value.clone().into()));
        }
        if let Some(value) = &self.official_id {
            values.push((RegionIden::OfficialId, value.into()));
        }
        if let Some(value) = &self.metadata {
            values.push((RegionIden::Metadata, value.clone().into()));
        }

        values
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_region: &PartialRegion,
) -> Result<Region, ModelError> {
    let values = update_region.to_values();

    if values.is_empty() {
        return Err(ValidationError::NoValidUpdates.into());
    }

    let (sql, values) = Query::update()
        .table(RegionIden::Table)
        .values(values)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(region)
}

/// Get the metadata for a region by its ID.
#[instrument(err(Debug), skip(db))]
pub async fn get_metadata(db: &PgPool, id: &Uuid) -> Result<Option<serde_json::Value>, ModelError> {
    let (sql, values) = Query::select()
        .columns([RegionIden::Metadata])
        .from(RegionIden::Table)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let (metadata,) = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(metadata)
}

/// Merge the supplied object into the region's `metadata` jsonb column at
/// the top level. Existing keys are overwritten by the patch, keys not present
/// in the patch are left untouched.
pub async fn patch_metadata(
    db: &PgPool,
    id: &Uuid,
    patch: serde_json::Value,
) -> Result<Region, ModelError> {
    if !patch.is_object() {
        return Err(
            ValidationError::BadRequest("metadata patch must be a JSON object".into()).into(),
        );
    }

    let region = sqlx::query_as::<_, Region>(
        "UPDATE region
            SET metadata = COALESCE(metadata, '{}'::jsonb) || $1::jsonb,
                updated_at = NOW()
            WHERE id = $2
            RETURNING *",
    )
    .bind(patch)
    .bind(id)
    .fetch_one(db)
    .await
    .resolve_db_err("Region")?;

    Ok(region)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct RegionOrderOptions {
    pub name: Option<Order>,
    pub created_at: Option<Order>,
}

impl RegionOrderOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by((RegionIden::Table, RegionIden::CreatedAt), order.into())
                .to_owned();
        }
        query
    }

    fn apply_to_localized(
        &self,
        mut query: sea_query::SelectStatement,
    ) -> sea_query::SelectStatement {
        use crate::models::translations::TextTranslationIden;
        use sea_query::Alias;

        if let Some(order) = &self.name {
            let tt_name_alias = Alias::new("tt_name");
            query = query
                .order_by((tt_name_alias, TextTranslationIden::Content), order.into())
                .to_owned();
        }
        self.apply(query)
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct RegionFilterOptions {
    pub organization_id: Option<Uuid>,
}

impl RegionFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = self.organization_id {
            query = query
                .join(
                    sea_query::JoinType::InnerJoin,
                    OrganizationIden::Table,
                    Expr::cust("region.id = ANY(organization.regions)"),
                )
                .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(value))
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: RegionFilterOptions,
    order_options: RegionOrderOptions,
    locale: &str,
) -> Result<PaginatedResults<LocalizedRegion>, ModelError> {
    let query = Query::select()
        .from(RegionIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (RegionIden::Table, col)))
        .to_owned();

    let query = LocalizedRegion::query_to_localisation(query, locale);

    let query = filter_options.apply(query);
    let query = order_options.apply_to_localized(query);
    let regions = page_options.fetch_paginated_results(db, query).await?;

    Ok(regions)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedRegion, ModelError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (RegionIden::Table, col)))
        .from(RegionIden::Table)
        .and_where(Expr::col((RegionIden::Table, RegionIden::Id)).eq(id.to_owned()))
        .to_owned();

    let query = LocalizedRegion::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region")?;

    Ok(region)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Region, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RegionIden::Table)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region")?;

    Ok(region)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Region, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(RegionIden::Table)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region")?;

    Ok(region)
}
