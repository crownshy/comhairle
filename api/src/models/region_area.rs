use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{error::ComhairleError, models::SqlxResultExt};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "region_area")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct RegionArea {
    #[partially(omit)]
    pub id: Uuid,
    pub zip_prefix: String,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateRegionArea {
    pub zip_prefix: String,
}

const DEFAULT_COLUMNS: [RegionAreaIden; 4] = [
    RegionAreaIden::Id,
    RegionAreaIden::ZipPrefix,
    RegionAreaIden::CreatedAt,
    RegionAreaIden::UpdatedAt,
];

pub async fn create(
    db: &PgPool,
    create_request: CreateRegionArea,
) -> Result<RegionArea, ComhairleError> {
    let CreateRegionArea { zip_prefix } = create_request;

    let (sql, values) = Query::insert()
        .into_table(RegionAreaIden::Table)
        .columns([RegionAreaIden::ZipPrefix])
        .values([zip_prefix.into()])
        .expect("region_area create values should be valid")
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(area)
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<RegionArea, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RegionAreaIden::Table)
        .and_where(Expr::col(RegionAreaIden::Id).eq(*id))
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region Area")?;

    Ok(area)
}

pub async fn list(db: &PgPool) -> Result<Vec<RegionArea>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RegionAreaIden::Table)
        .order_by(RegionAreaIden::ZipPrefix, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let areas = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(areas)
}

pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_request: &PartialRegionArea,
) -> Result<RegionArea, ComhairleError> {
    let mut tx = db.begin().await?;

    let mut query = Query::update()
        .table(RegionAreaIden::Table)
        .and_where(Expr::col(RegionAreaIden::Id).eq(*id))
        .to_owned();

    let mut has_updates = false;
    if let Some(value) = &update_request.zip_prefix {
        query = query
            .value(RegionAreaIden::ZipPrefix, value.clone())
            .to_owned();
        has_updates = true;
    }

    if !has_updates {
        tx.rollback().await?;
        return get_by_id(db, id).await;
    }

    query = query
        .value(RegionAreaIden::UpdatedAt, Utc::now())
        .to_owned();

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .resolve_db_err("Region Area")?;

    tx.commit().await?;

    Ok(area)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<RegionArea, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(RegionAreaIden::Table)
        .and_where(Expr::col(RegionAreaIden::Id).eq(*id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region Area")?;

    Ok(area)
}
