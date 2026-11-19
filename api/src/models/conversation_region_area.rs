//! Optional geographic coverage for conversations, using existing region area boundaries.

use sea_query::{
    Alias, Expr, Func, Iden, JoinType, OnConflict, PostgresQueryBuilder, Query, SelectStatement,
};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use super::SqlxResultExt;
use super::conversation::ConversationIden;
use super::region_area::RegionAreaIden;
use crate::error::ComhairleError;

#[derive(Iden)]
enum ConversationRegionAreaIden {
    #[iden = "conversation_region_area"]
    Table,
    ConversationId,
    RegionAreaId,
}

#[derive(Iden)]
enum RegionRegionAreaIden {
    #[iden = "region_region_area"]
    Table,
    RegionId,
    RegionAreaId,
}

/// List the area IDs in a conversation's coverage, in deterministic order.
#[instrument(err(Debug), skip(db))]
pub async fn list_area_ids(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<Uuid>, ComhairleError> {
    let (sql, values) = Query::select()
        .column(ConversationRegionAreaIden::RegionAreaId)
        .from(ConversationRegionAreaIden::Table)
        .and_where(Expr::col(ConversationRegionAreaIden::ConversationId).eq(*conversation_id))
        .order_by(
            ConversationRegionAreaIden::RegionAreaId,
            sea_query::Order::Asc,
        )
        .build_sqlx(PostgresQueryBuilder);

    Ok(sqlx::query_scalar_with(&sql, values).fetch_all(db).await?)
}

/// Atomically replace coverage. An empty set removes all geographic coverage.
///
/// The conversation lock serializes replacements, including initially empty sets.
#[instrument(err(Debug), skip(db))]
pub async fn set_area_links(
    db: &PgPool,
    conversation_id: &Uuid,
    area_ids: &[Uuid],
) -> Result<(), ComhairleError> {
    let mut transaction = db.begin().await?;
    let (sql, values) = Query::select()
        .column(ConversationIden::Id)
        .from(ConversationIden::Table)
        .and_where(Expr::col(ConversationIden::Id).eq(*conversation_id))
        .lock_exclusive()
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await
        .resolve_db_err("Conversation")?;

    let (sql, values) = Query::delete()
        .from_table(ConversationRegionAreaIden::Table)
        .and_where(Expr::col(ConversationRegionAreaIden::ConversationId).eq(*conversation_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(&mut *transaction)
        .await?;

    if !area_ids.is_empty() {
        let mut query = Query::insert();
        query
            .into_table(ConversationRegionAreaIden::Table)
            .columns([
                ConversationRegionAreaIden::ConversationId,
                ConversationRegionAreaIden::RegionAreaId,
            ]);
        for area_id in area_ids {
            query.values([(*conversation_id).into(), (*area_id).into()])?;
        }
        let (sql, values) = query
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *transaction)
            .await
            .resolve_db_err("Conversation Region Area")?;
    }

    transaction.commit().await?;
    Ok(())
}

/// Attach an existing area. Attaching the same area again is idempotent.
#[instrument(err(Debug), skip(db))]
pub async fn add_area_link(
    db: &PgPool,
    conversation_id: &Uuid,
    area_id: &Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(ConversationRegionAreaIden::Table)
        .columns([
            ConversationRegionAreaIden::ConversationId,
            ConversationRegionAreaIden::RegionAreaId,
        ])
        .values([(*conversation_id).into(), (*area_id).into()])?
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .resolve_db_err("Conversation Region Area")?;
    Ok(())
}

/// Detach an area without deleting the shared region area itself.
#[instrument(err(Debug), skip(db))]
pub async fn remove_area_link(
    db: &PgPool,
    conversation_id: &Uuid,
    area_id: &Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ConversationRegionAreaIden::Table)
        .and_where(Expr::col(ConversationRegionAreaIden::ConversationId).eq(*conversation_id))
        .and_where(Expr::col(ConversationRegionAreaIden::RegionAreaId).eq(*area_id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Select conversation IDs whose coverage intersects the selected target areas.
///
/// `ST_Intersects` includes shared boundaries; missing geometries never match.
fn intersecting_conversations(target_areas: SelectStatement) -> SelectStatement {
    let target_area = Alias::new("target_area");
    Query::select()
        .column((
            ConversationRegionAreaIden::Table,
            ConversationRegionAreaIden::ConversationId,
        ))
        .from(ConversationRegionAreaIden::Table)
        .inner_join(
            RegionAreaIden::Table,
            Expr::col((RegionAreaIden::Table, RegionAreaIden::Id)).equals((
                ConversationRegionAreaIden::Table,
                ConversationRegionAreaIden::RegionAreaId,
            )),
        )
        .join_as(
            JoinType::InnerJoin,
            RegionAreaIden::Table,
            target_area.clone(),
            sea_query::SimpleExpr::from(Func::cust(Alias::new("ST_Intersects")).args([
                Expr::col((RegionAreaIden::Table, RegionAreaIden::AreaGeometry)).into(),
                Expr::col((target_area.clone(), RegionAreaIden::AreaGeometry)).into(),
            ])),
        )
        .and_where(Expr::col((target_area, RegionAreaIden::Id)).in_subquery(target_areas))
        .to_owned()
}

/// Select conversations overlapping any area attached to a region.
///
/// Use as a membership subquery so multiple intersections do not duplicate results.
pub fn intersecting_region(region_id: Uuid) -> SelectStatement {
    intersecting_conversations(
        Query::select()
            .column(RegionRegionAreaIden::RegionAreaId)
            .from(RegionRegionAreaIden::Table)
            .and_where(Expr::col(RegionRegionAreaIden::RegionId).eq(region_id))
            .to_owned(),
    )
}

/// Check whether a live, incomplete conversation overlaps the supplied area.
///
/// Conversations with no coverage and areas without geometry return false.
#[instrument(err(Debug), skip(db))]
pub async fn is_active_in_area(
    db: &PgPool,
    conversation_id: &Uuid,
    area_id: &Uuid,
) -> Result<bool, ComhairleError> {
    let matching_conversations = intersecting_conversations(
        Query::select()
            .column(RegionAreaIden::Id)
            .from(RegionAreaIden::Table)
            .and_where(Expr::col(RegionAreaIden::Id).eq(*area_id))
            .to_owned(),
    );
    let (sql, values) = Query::select()
        .expr(Expr::exists(
            Query::select()
                .column(ConversationIden::Id)
                .from(ConversationIden::Table)
                .and_where(Expr::col(ConversationIden::Id).eq(*conversation_id))
                .and_where(Expr::col(ConversationIden::IsLive).eq(true))
                .and_where(Expr::col(ConversationIden::IsComplete).eq(false))
                .and_where(Expr::col(ConversationIden::Id).in_subquery(matching_conversations))
                .to_owned(),
        ))
        .build_sqlx(PostgresQueryBuilder);

    Ok(sqlx::query_scalar_with(&sql, values).fetch_one(db).await?)
}
