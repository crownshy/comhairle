use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "report_impact")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct ReportImpact {
    pub id: Uuid,
    pub created_by: Uuid,
    pub report_id: Uuid,
    pub details: String,
    pub kind: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [ReportImpactIden; 8] = [
    ReportImpactIden::Id,
    ReportImpactIden::CreatedBy,
    ReportImpactIden::ReportId,
    ReportImpactIden::Title,
    ReportImpactIden::Details,
    ReportImpactIden::Kind,
    ReportImpactIden::CreatedAt,
    ReportImpactIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateImpactDTO {
    pub title: String,
    pub details: String,
    pub kind: String,
}

impl PartialReportImpact {
    pub fn to_values(&self) -> Vec<(ReportImpactIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.title {
            values.push((ReportImpactIden::Title, value.into()));
        }
        if let Some(value) = &self.details {
            values.push((ReportImpactIden::Details, value.into()));
        }
        if let Some(value) = &self.kind {
            values.push((ReportImpactIden::Kind, value.into()));
        }
        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    update_request: PartialReportImpact,
    impact_id: &Uuid,
    user_id: &Uuid,
) -> Result<ReportImpact, ComhairleError> {
    let values = update_request.to_values();

    let (sql, values) = Query::update()
        .table(ReportImpactIden::Table)
        .values(values)
        .and_where(Expr::col(ReportImpactIden::Id).eq(*impact_id))
        .and_where(Expr::col(ReportImpactIden::CreatedBy).eq(*user_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(ComhairleError::FailedToUpdateImpact)
}

pub async fn create(
    db: &PgPool,
    create_request: CreateImpactDTO,
    report_id: &Uuid,
    user_id: &Uuid,
) -> Result<ReportImpact, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(ReportImpactIden::Table)
        .columns(vec![
            ReportImpactIden::CreatedBy,
            ReportImpactIden::ReportId,
            ReportImpactIden::Title,
            ReportImpactIden::Details,
            ReportImpactIden::Kind,
        ])
        .values(vec![
            user_id.to_owned().into(),
            report_id.to_owned().into(),
            create_request.title.into(),
            create_request.details.into(),
            create_request.kind.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_e| ComhairleError::FailedToCreateImpact)
}

pub async fn get_for_report(
    db: &PgPool,
    report_id: &Uuid,
) -> Result<Vec<ReportImpact>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(ReportImpactIden::Table)
        .and_where(Expr::col(ReportImpactIden::ReportId).eq(report_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, ReportImpact, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("ReportImpacts".into()))
}
