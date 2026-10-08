use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::extension::postgres::PgExpr;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, ReturningClause, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;
use crate::models::pagination::{PageOptions, PaginatedResults};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "region_area")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct RegionArea {
    #[partially(omit)]
    pub id: Uuid,
    pub zip_prefix: Option<String>,
    pub name: Option<String>,
    pub tags: Vec<String>,
    /// GeoJSON representation of the area's WGS84 MultiPolygon.
    pub area_geometry: Option<serde_json::Value>,
    /// Geodesic area of the boundary, maintained by the `region_area_metrics` trigger.
    #[partially(omit)]
    pub area_sqm: Option<f64>,
    #[partially(omit)]
    pub bbox_min_lng: Option<f64>,
    #[partially(omit)]
    pub bbox_min_lat: Option<f64>,
    #[partially(omit)]
    pub bbox_max_lng: Option<f64>,
    #[partially(omit)]
    pub bbox_max_lat: Option<f64>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct CreateRegionArea {
    pub zip_prefix: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub area_geometry: Option<serde_json::Value>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct RegionAreaListOptions {
    #[serde(default)]
    pub include_geometry: bool,

    pub name: Option<String>,
    pub tag: Option<String>,

    /// Comma separated area ids, for loading a known selection.
    pub ids: Option<String>,

    pub viewport_min_lng: Option<f64>,
    pub viewport_min_lat: Option<f64>,
    pub viewport_max_lng: Option<f64>,
    pub viewport_max_lat: Option<f64>,
    /// Drop areas smaller than this fraction of the viewport's own area.
    pub min_area_ratio: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub min_lng: f64,
    pub min_lat: f64,
    pub max_lng: f64,
    pub max_lat: f64,
}

impl RegionAreaListOptions {
    fn viewport(&self) -> Result<Option<Viewport>, ComhairleError> {
        let bounds = [
            self.viewport_min_lng,
            self.viewport_min_lat,
            self.viewport_max_lng,
            self.viewport_max_lat,
        ];
        if bounds.iter().all(Option::is_none) {
            return Ok(None);
        }
        let [Some(min_lng), Some(min_lat), Some(max_lng), Some(max_lat)] = bounds else {
            return Err(ComhairleError::BadRequest(
                "Provide all four viewport bounds, or none of them".into(),
            ));
        };
        if !(-180.0..=180.0).contains(&min_lng)
            || !(-180.0..=180.0).contains(&max_lng)
            || !(-90.0..=90.0).contains(&min_lat)
            || !(-90.0..=90.0).contains(&max_lat)
            || min_lng >= max_lng
            || min_lat >= max_lat
        {
            return Err(ComhairleError::BadRequest(
                "Use a WGS84 viewport with min values below max values, longitudes between -180 and 180 and latitudes between -90 and 90".into(),
            ));
        }
        Ok(Some(Viewport {
            min_lng,
            min_lat,
            max_lng,
            max_lat,
        }))
    }

    fn min_area_ratio(&self) -> Result<Option<f64>, ComhairleError> {
        match self.min_area_ratio {
            None => Ok(None),
            Some(ratio) if ratio.is_finite() && (0.0..1.0).contains(&ratio) => Ok(Some(ratio)),
            Some(_) => Err(ComhairleError::BadRequest(
                "Use a min_area_ratio between 0 and 1".into(),
            )),
        }
    }

    fn ids(&self) -> Result<Option<Vec<Uuid>>, ComhairleError> {
        let Some(ids) = &self.ids else {
            return Ok(None);
        };
        let ids = ids
            .split(',')
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(Uuid::parse_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                ComhairleError::BadRequest("Use a comma separated list of area ids".into())
            })?;
        Ok(Some(ids))
    }
}

const DEFAULT_COLUMNS: [RegionAreaIden; 11] = [
    RegionAreaIden::Id,
    RegionAreaIden::ZipPrefix,
    RegionAreaIden::Name,
    RegionAreaIden::Tags,
    RegionAreaIden::AreaSqm,
    RegionAreaIden::BboxMinLng,
    RegionAreaIden::BboxMinLat,
    RegionAreaIden::BboxMaxLng,
    RegionAreaIden::BboxMaxLat,
    RegionAreaIden::CreatedAt,
    RegionAreaIden::UpdatedAt,
];

/// Every scalar column plus the boundary as GeoJSON, for hand written queries.
const SELECT_COLUMNS_SQL: &str = "id, zip_prefix, name, tags,
            area_sqm, bbox_min_lng, bbox_min_lat, bbox_max_lng, bbox_max_lat,
            ST_AsGeoJSON(area_geometry, 15)::jsonb AS area_geometry,
            created_at, updated_at";

fn returning_columns() -> ReturningClause {
    Query::returning().exprs(
        DEFAULT_COLUMNS
            .map(|column| Expr::col(column).into())
            .into_iter()
            .chain([Expr::cust(
                "ST_AsGeoJSON(area_geometry, 15)::jsonb AS area_geometry",
            )]),
    )
}

fn returning_columns_with_null_geometry() -> ReturningClause {
    Query::returning().exprs(
        DEFAULT_COLUMNS
            .map(|column| Expr::col(column).into())
            .into_iter()
            .chain([Expr::cust("NULL::jsonb AS area_geometry")]),
    )
}

#[instrument(err(Debug), skip_all)]
pub async fn create(
    db: &PgPool,
    create_request: CreateRegionArea,
) -> Result<RegionArea, ComhairleError> {
    let mut areas = create_many(db, vec![create_request]).await?;
    areas.pop().ok_or_else(|| {
        ComhairleError::Conflict("A region area with this name and boundary already exists.".into())
    })
}

#[instrument(err(Debug), skip_all)]
pub async fn create_many(
    db: &PgPool,
    mut create_requests: Vec<CreateRegionArea>,
) -> Result<Vec<RegionArea>, ComhairleError> {
    if create_requests.is_empty() {
        return Ok(Vec::new());
    }

    let mut tx = db.begin().await?;

    let mut query = Query::insert()
        .into_table(RegionAreaIden::Table)
        .columns([
            RegionAreaIden::ZipPrefix,
            RegionAreaIden::Tags,
            RegionAreaIden::AreaGeometry,
            RegionAreaIden::Name,
        ])
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .to_owned();

    for request in create_requests.drain(..) {
        query
            .values([
                request.zip_prefix.into(),
                request.tags.into(),
                match request.area_geometry {
                    Some(geom) => Expr::cust_with_values(
                        "ST_Multi(ST_CollectionExtract(ST_MakeValid(ST_GeomFromGeoJSON($1)), 3))",
                        [geom.to_string()],
                    )
                    .into(),
                    None => Expr::cust("NULL").into(),
                },
                request.name.into(),
            ])
            .expect("columns match values length");
    }

    let (sql, values) = query
        // Null out geometry column in the returning clause to avoid fetching large geometries unnecessarily
        .returning(returning_columns_with_null_geometry())
        .build_sqlx(PostgresQueryBuilder);

    let areas = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_all(&mut *tx)
        .await
        .map_err(geometry_error)?;

    tx.commit().await?;

    Ok(areas)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<RegionArea, ComhairleError> {
    let area = sqlx::query_as::<_, RegionArea>(&format!(
        "SELECT {SELECT_COLUMNS_SQL}
         FROM region_area
         WHERE id = $1"
    ))
    .bind(id)
    .fetch_one(db)
    .await
    .resolve_db_err("Region Area")?;

    Ok(area)
}

#[instrument(err(Debug), skip(db))]
pub async fn list(db: &PgPool) -> Result<PaginatedResults<RegionArea>, ComhairleError> {
    list_with_geometry(db, RegionAreaListOptions::default(), PageOptions::default()).await
}

pub async fn list_with_geometry(
    db: &PgPool,
    options: RegionAreaListOptions,
    page_options: PageOptions,
) -> Result<PaginatedResults<RegionArea>, ComhairleError> {
    let viewport = options.viewport()?;
    let min_area_ratio = options.min_area_ratio()?;
    let ids = options.ids()?;

    let mut query = Query::select()
        .column(RegionAreaIden::Id)
        .column(RegionAreaIden::ZipPrefix)
        .column(RegionAreaIden::Name)
        .column(RegionAreaIden::Tags)
        .expr_as(
            if options.include_geometry {
                Expr::cust("ST_AsGeoJSON(area_geometry, 15)::jsonb")
            } else {
                Expr::cust("NULL")
            },
            RegionAreaIden::AreaGeometry,
        )
        .column(RegionAreaIden::AreaSqm)
        .column(RegionAreaIden::BboxMinLng)
        .column(RegionAreaIden::BboxMinLat)
        .column(RegionAreaIden::BboxMaxLng)
        .column(RegionAreaIden::BboxMaxLat)
        .column(RegionAreaIden::CreatedAt)
        .column(RegionAreaIden::UpdatedAt)
        .from(RegionAreaIden::Table)
        .order_by_with_nulls(
            RegionAreaIden::Name,
            sea_query::Order::Asc,
            sea_query::NullOrdering::Last,
        )
        .order_by_with_nulls(
            RegionAreaIden::ZipPrefix,
            sea_query::Order::Asc,
            sea_query::NullOrdering::Last,
        )
        .order_by(RegionAreaIden::Id, sea_query::Order::Asc)
        .to_owned();

    if let Some(name) = &options.name {
        query.and_where(Expr::col(RegionAreaIden::Name).ilike(format!("%{}%", name)));
    }

    if let Some(tag) = &options.tag {
        query.and_where(Expr::cust_with_values("$1 = ANY(tags)", [tag.clone()]));
    }

    if let Some(ids) = ids {
        query.and_where(Expr::col(RegionAreaIden::Id).is_in(ids));
    }

    if let Some(viewport) = viewport {
        query.and_where(Expr::cust_with_values(
            "bbox_min_lng <= $1 AND bbox_max_lng >= $2 AND bbox_min_lat <= $3 AND bbox_max_lat >= $4",
            [
                viewport.max_lng,
                viewport.min_lng,
                viewport.max_lat,
                viewport.min_lat,
            ],
        ));

        if let Some(ratio) = min_area_ratio {
            query.and_where(Expr::cust_with_values(
                "area_sqm >= ST_Area(ST_Segmentize(ST_MakeEnvelope($1, $2, $3, $4, 4326), 0.1)::geography) * $5",
                [
                    viewport.min_lng,
                    viewport.min_lat,
                    viewport.max_lng,
                    viewport.max_lat,
                    ratio,
                ],
            ));
        }
    }

    let results = page_options
        .fetch_paginated_results(db, query)
        .await
        .map_err(ComhairleError::DatabaseError)?;

    Ok(results)
}

#[instrument(err(Debug), skip(db))]
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
    if let Some(value) = &update_request.name {
        query.value(RegionAreaIden::Name, value.clone());
        has_updates = true;
    }
    if let Some(value) = &update_request.zip_prefix {
        query = query
            .value(RegionAreaIden::ZipPrefix, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = &update_request.tags {
        query = query.value(RegionAreaIden::Tags, value.clone()).to_owned();
        has_updates = true;
    }
    if let Some(value) = &update_request.area_geometry {
        query = query
            .value(
                RegionAreaIden::AreaGeometry,
                match value {
                    Some(geometry) => Expr::cust_with_values(
                        "ST_Multi(ST_CollectionExtract(ST_MakeValid(ST_GeomFromGeoJSON(?)), 3))",
                        [geometry.to_string()],
                    ),
                    None => Expr::cust("NULL"),
                },
            )
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
        .returning(returning_columns())
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(&mut *tx)
        .await
        .map_err(geometry_error)?;

    tx.commit().await?;

    Ok(area)
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AreaLocation {
    pub longitude: f64,
    pub latitude: f64,
}

impl AreaLocation {
    pub fn validate(&self) -> Result<(), ComhairleError> {
        if !self.longitude.is_finite()
            || !self.latitude.is_finite()
            || !(-180.0..=180.0).contains(&self.longitude)
            || !(-90.0..=90.0).contains(&self.latitude)
        {
            return Err(ComhairleError::BadRequest(
                "Use a WGS84 longitude between -180 and 180 and latitude between -90 and 90".into(),
            ));
        }
        Ok(())
    }
}

pub async fn intersecting(
    db: &PgPool,
    location: &AreaLocation,
) -> Result<Vec<RegionArea>, ComhairleError> {
    location.validate()?;
    let areas = sqlx::query_as::<_, RegionArea>(&format!(
        "SELECT {SELECT_COLUMNS_SQL}
         FROM region_area
         WHERE ST_Intersects(area_geometry, ST_SetSRID(ST_MakePoint($1, $2), 4326))
         ORDER BY name NULLS LAST, zip_prefix NULLS LAST, id"
    ))
    .bind(location.longitude)
    .bind(location.latitude)
    .fetch_all(db)
    .await?;
    Ok(areas)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<RegionArea, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(RegionAreaIden::Table)
        .and_where(Expr::col(RegionAreaIden::Id).eq(*id))
        .returning(returning_columns())
        .build_sqlx(PostgresQueryBuilder);

    let area = sqlx::query_as_with::<_, RegionArea, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Region Area")?;

    Ok(area)
}

fn geometry_error(error: sqlx::Error) -> ComhairleError {
    if let sqlx::Error::Database(ref database_error) = error {
        if matches!(
            database_error.code().as_deref(),
            Some("22023" | "XX000" | "23514")
        ) {
            return ComhairleError::BadRequest(
                "Invalid boundary. Use valid WGS84 Polygon or MultiPolygon geometry.".into(),
            );
        }
    }
    match error {
        sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Region Area".into()),
        error => ComhairleError::DatabaseError(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rejects_invalid_area_locations() {
        for (longitude, latitude) in [
            (181.0, 0.0),
            (0.0, -91.0),
            (f64::NAN, 0.0),
            (0.0, f64::INFINITY),
        ] {
            assert!(
                AreaLocation {
                    longitude,
                    latitude
                }
                .validate()
                .is_err()
            );
        }
        assert!(
            AreaLocation {
                longitude: -180.0,
                latitude: 90.0
            }
            .validate()
            .is_ok()
        );
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn intersects_overlaps_holes_edges_and_exclaves(
        pool: PgPool,
    ) -> Result<(), ComhairleError> {
        let outer = create(
            &pool,
            CreateRegionArea {
                name: Some("Outer".into()),
                area_geometry: Some(json!({"type":"MultiPolygon", "coordinates":[
                    [[[0,0],[10,0],[10,10],[0,10],[0,0]], [[2,2],[2,4],[4,4],[4,2],[2,2]]],
                    [[[20,20],[21,20],[21,21],[20,21],[20,20]]]
                ]})),
                ..Default::default()
            },
        )
        .await?;
        create(
            &pool,
            CreateRegionArea {
                name: Some("Overlap".into()),
                area_geometry: Some(
                    json!({"type":"Polygon", "coordinates":[[[5,5],[15,5],[15,15],[5,15],[5,5]]]}),
                ),
                ..Default::default()
            },
        )
        .await?;
        create(&pool, CreateRegionArea::default()).await?;
        assert!(
            list(&pool)
                .await?
                .records
                .iter()
                .all(|area| area.area_geometry.is_none())
        );
        assert_eq!(
            list_with_geometry(
                &pool,
                RegionAreaListOptions {
                    include_geometry: true,
                    ..Default::default()
                },
                PageOptions::default(),
            )
            .await?
            .records
            .iter()
            .filter(|area| area.area_geometry.is_some())
            .count(),
            2
        );
        for (longitude, latitude, expected) in [
            (6.0, 6.0, 2),
            (10.0, 6.0, 2),
            (3.0, 3.0, 0),
            (0.0, 5.0, 1),
            (20.5, 20.5, 1),
            (-1.0, -1.0, 0),
        ] {
            let matches = intersecting(
                &pool,
                &AreaLocation {
                    longitude,
                    latitude,
                },
            )
            .await?;
            assert_eq!(matches.len(), expected);
            if expected == 1 {
                assert_eq!(matches[0].id, outer.id);
            }
            assert!(matches.iter().all(|area| area.area_geometry.is_some()));
        }
        Ok(())
    }
}
