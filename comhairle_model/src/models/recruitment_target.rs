use crate::models::error::ModelError;
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, OnConflict, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "recruitment_target")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct RecruitmentTarget {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub workflow_id: Uuid,
    pub metric: String,
    pub bucket: String,
    pub target_count: i32,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateRecruitmentTarget {
    pub metric: String,
    pub bucket: String,
    pub target_count: i32,
}

const DEFAULT_COLUMNS: [RecruitmentTargetIden; 7] = [
    RecruitmentTargetIden::Id,
    RecruitmentTargetIden::WorkflowId,
    RecruitmentTargetIden::Metric,
    RecruitmentTargetIden::Bucket,
    RecruitmentTargetIden::TargetCount,
    RecruitmentTargetIden::CreatedAt,
    RecruitmentTargetIden::UpdatedAt,
];

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    workflow_id: &Uuid,
    create_request: &CreateRecruitmentTarget,
) -> Result<RecruitmentTarget, ModelError> {
    let (sql, values) = Query::insert()
        .into_table(RecruitmentTargetIden::Table)
        .columns([
            RecruitmentTargetIden::WorkflowId,
            RecruitmentTargetIden::Metric,
            RecruitmentTargetIden::Bucket,
            RecruitmentTargetIden::TargetCount,
        ])
        .values([
            (*workflow_id).into(),
            create_request.metric.clone().into(),
            create_request.bucket.clone().into(),
            create_request.target_count.into(),
        ])
        .unwrap()
        .on_conflict(
            OnConflict::columns([
                RecruitmentTargetIden::WorkflowId,
                RecruitmentTargetIden::Metric,
                RecruitmentTargetIden::Bucket,
            ])
            .update_columns([
                RecruitmentTargetIden::TargetCount,
                RecruitmentTargetIden::UpdatedAt,
            ])
            .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(target)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<RecruitmentTarget, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Recruitment Target")?;

    Ok(target)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_for_workflow(
    db: &PgPool,
    workflow_id: &Uuid,
) -> Result<Vec<RecruitmentTarget>, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::WorkflowId).eq(*workflow_id))
        .order_by(RecruitmentTargetIden::Metric, sea_query::Order::Asc)
        .order_by(RecruitmentTargetIden::Bucket, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let targets = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(targets)
}

#[instrument(err(Debug), skip(db))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_request: &PartialRecruitmentTarget,
) -> Result<RecruitmentTarget, ModelError> {
    let mut query = Query::update()
        .table(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .to_owned();

    let mut has_updates = false;

    if let Some(value) = &update_request.metric {
        query = query
            .value(RecruitmentTargetIden::Metric, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = &update_request.bucket {
        query = query
            .value(RecruitmentTargetIden::Bucket, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = update_request.target_count {
        query = query
            .value(RecruitmentTargetIden::TargetCount, value)
            .to_owned();
        has_updates = true;
    }

    if !has_updates {
        return get_by_id(db, id).await;
    }

    query = query
        .value(RecruitmentTargetIden::UpdatedAt, Utc::now())
        .to_owned();

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Recruitment Target")?;

    Ok(target)
}

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<RecruitmentTarget, ModelError> {
    let (sql, values) = Query::delete()
        .from_table(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Recruitment Target")?;

    Ok(target)
}
