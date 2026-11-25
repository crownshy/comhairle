use sea_query::extension::postgres::PgFunc;
use sea_query::{Alias, Condition, Expr, Func, LockType, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::PgConnection;
use uuid::Uuid;

use super::tables::{AreaLink, Feature};
use super::{AreaLinkIden, FeatureIden};
use crate::error::ComhairleError;
use crate::models::region::RegionIden;
use crate::models::region_area::RegionAreaIden;
use crate::models::translations::{
    TextContent, TextContentIden, TextTranslation, TextTranslationIden,
};

#[derive(Serialize, sqlx::FromRow)]
struct FeatureState {
    #[sqlx(flatten)]
    mapping: Feature,
    area_name: Option<String>,
    area_tags: Vec<String>,
    geometry: Option<Vec<u8>>,
    region_name: Uuid,
    region_type: String,
    official_id: Option<String>,
}

pub(super) async fn lock_and_hash(
    connection: &mut PgConnection,
    pipeline_id: Uuid,
) -> Result<String, ComhairleError> {
    let (sql, values) = Feature::owned_entities(pipeline_id)
        .column((FeatureIden::Table, sea_query::Asterisk))
        .expr_as(
            Expr::col((RegionAreaIden::Table, RegionAreaIden::Name)),
            Alias::new("area_name"),
        )
        .expr_as(
            Expr::col((RegionAreaIden::Table, RegionAreaIden::Tags)),
            Alias::new("area_tags"),
        )
        .expr_as(
            Func::cust(Alias::new("st_asbinary")).arg(Expr::col((
                RegionAreaIden::Table,
                RegionAreaIden::AreaGeometry,
            ))),
            Alias::new("geometry"),
        )
        .expr_as(
            Expr::col((RegionIden::Table, RegionIden::Name)),
            Alias::new("region_name"),
        )
        .columns([
            (RegionIden::Table, RegionIden::RegionType),
            (RegionIden::Table, RegionIden::OfficialId),
        ])
        .order_by((FeatureIden::Table, FeatureIden::ExternalId), Order::Asc)
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    let features: Vec<FeatureState> = sqlx::query_as_with(&sql, values)
        .fetch_all(&mut *connection)
        .await?;

    let mut name_ids: Vec<_> = features.iter().map(|feature| feature.region_name).collect();
    name_ids.sort();
    name_ids.dedup();
    let area_ids: Vec<_> = features
        .iter()
        .map(|feature| feature.mapping.region_area_id)
        .collect();
    let region_ids: Vec<_> = features
        .iter()
        .map(|feature| feature.mapping.region_id)
        .collect();

    let (contents, translations, links) = if features.is_empty() {
        (Vec::new(), Vec::new(), Vec::new())
    } else {
        let (sql, values) = Query::select()
            .column(sea_query::Asterisk)
            .from(TextContentIden::Table)
            .and_where(Expr::col(TextContentIden::Id).eq(PgFunc::any(name_ids.clone())))
            .order_by(TextContentIden::Id, Order::Asc)
            .lock(LockType::Update)
            .build_sqlx(PostgresQueryBuilder);
        let contents: Vec<TextContent> = sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *connection)
            .await?;
        if contents.len() != name_ids.len() {
            return Err(ComhairleError::CorruptedData(
                "A pipeline Region is missing its name content".to_string(),
            ));
        }

        let (sql, values) = Query::select()
            .column(sea_query::Asterisk)
            .from(TextTranslationIden::Table)
            .and_where(Expr::col(TextTranslationIden::ContentId).eq(PgFunc::any(name_ids)))
            .order_by(TextTranslationIden::ContentId, Order::Asc)
            .order_by(TextTranslationIden::Locale, Order::Asc)
            .order_by(TextTranslationIden::Id, Order::Asc)
            .lock(LockType::Update)
            .build_sqlx(PostgresQueryBuilder);
        let translations: Vec<TextTranslation> = sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *connection)
            .await?;

        let (sql, values) = Query::select()
            .columns([AreaLinkIden::RegionId, AreaLinkIden::RegionAreaId])
            .from(AreaLinkIden::Table)
            .cond_where(
                Condition::any()
                    .add(Expr::col(AreaLinkIden::RegionId).eq(PgFunc::any(region_ids)))
                    .add(Expr::col(AreaLinkIden::RegionAreaId).eq(PgFunc::any(area_ids))),
            )
            .order_by(AreaLinkIden::RegionId, Order::Asc)
            .order_by(AreaLinkIden::RegionAreaId, Order::Asc)
            .lock(LockType::Update)
            .build_sqlx(PostgresQueryBuilder);
        let links: Vec<AreaLink> = sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *connection)
            .await?;
        (contents, translations, links)
    };

    let state = serde_json::to_vec(&(features, contents, translations, links))?;
    Ok(hex::encode(Sha256::digest(state)))
}
