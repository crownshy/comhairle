use crate::models::error::ModelError;
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::models::error::UserError;
use crate::models::id::gen_id;
use crate::models::{
    SqlxResultExt,
    users::{self, UserAuthType},
};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "one_time_passcode")]
pub struct Otp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub status: OtpStatus,
    pub redirect_url: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum OtpStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "error")]
    Error,
}

impl From<OtpStatus> for sea_query::Value {
    fn from(val: OtpStatus) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for OtpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OtpStatus::Pending => "pending",
            OtpStatus::Accepted => "accepted",
            OtpStatus::Error => "error",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [OtpIden; 8] = [
    OtpIden::Id,
    OtpIden::UserId,
    OtpIden::Code,
    OtpIden::Status,
    OtpIden::RedirectUrl,
    OtpIden::ExpiresAt,
    OtpIden::CreatedAt,
    OtpIden::UpdatedAt,
];

#[instrument(err(Debug), skip(db))]
pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    redirect_url: Option<String>,
    custom_expiry: Option<DateTime<Utc>>,
) -> Result<Otp, ModelError> {
    let user = users::get_user_by_id(user_id, db).await?;

    if user.auth_type == UserAuthType::Guest {
        return Err(UserError::WrongUserType.into());
    }

    // Set any existing, pending otps to `error` so that only one pending otp
    // exists for a user
    let (sql, values) = Query::update()
        .table(OtpIden::Table)
        .values([(OtpIden::Status, OtpStatus::Error.into())])
        .and_where(Expr::col(OtpIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(OtpIden::Status).eq(OtpStatus::Pending))
        .build_sqlx(PostgresQueryBuilder);

    let _ = sqlx::query_as_with::<_, Otp, _>(&sql, values)
        .fetch_all(db)
        .await?;

    let random_code = gen_id();
    let expires_at = custom_expiry.unwrap_or(Utc::now() + Duration::minutes(10));

    let mut columns = vec![OtpIden::UserId, OtpIden::Code, OtpIden::ExpiresAt];
    let mut values = vec![(*user_id).into(), random_code.into(), expires_at.into()];

    if let Some(value) = redirect_url {
        columns.push(OtpIden::RedirectUrl);
        values.push(value.into());
    }

    let (sql, values) = Query::insert()
        .into_table(OtpIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let otp = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(otp)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Otp, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(OtpIden::Table)
        .and_where(Expr::col(OtpIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let otp = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("One Time Passcode")?;

    Ok(otp)
}

#[instrument(err(Debug), skip(db))]
pub async fn accept(
    db: &PgPool,
    user_id: &Uuid,
    code: &str,
    now: DateTime<Utc>,
) -> Result<Otp, ModelError> {
    let (sql, values) = Query::update()
        .table(OtpIden::Table)
        .values([(OtpIden::Status, OtpStatus::Accepted.into())])
        .and_where(Expr::col(OtpIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(OtpIden::Code).eq(code.to_owned()))
        .and_where(Expr::col(OtpIden::Status).eq(OtpStatus::Pending))
        .and_where(Expr::col(OtpIden::ExpiresAt).gt(now))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let otp = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("One Time Passcode")?;

    Ok(otp)
}
