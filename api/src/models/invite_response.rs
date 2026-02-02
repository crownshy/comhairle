use chrono::{DateTime, Utc};
use comhairle_macros::DbStringEnum;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Deserialize, Serialize, Clone, JsonSchema, Debug, DbStringEnum, PartialEq, Eq)]
pub enum Response {
    Accept,
    Reject,
}

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "invite_response")]
#[partially(derive(Deserialize, Debug, JsonSchema, FromRow))]
pub struct InviteResponse {
    #[partially(omit)]
    pub id: Uuid,
    pub invite_id: Uuid,
    pub user_id: Uuid,
    pub response: Response,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [InviteResponseIden; 5] = [
    InviteResponseIden::Id,
    InviteResponseIden::InviteId,
    InviteResponseIden::UserId,
    InviteResponseIden::Response,
    InviteResponseIden::CreatedAt,
];

#[instrument(err(Debug))]
pub async fn create(
    db: &sqlx::PgPool,
    user_id: &Uuid,
    invite_id: &Uuid,
    response: Response,
) -> Result<InviteResponse, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(InviteResponseIden::Table)
        .columns(vec![
            InviteResponseIden::UserId,
            InviteResponseIden::InviteId,
            InviteResponseIden::Response,
        ])
        .values(vec![
            user_id.to_owned().into(),
            invite_id.to_owned().into(),
            response.into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, InviteResponse, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(ComhairleError::FailedToCreateInvite)
}
