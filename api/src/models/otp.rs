use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    models::users::{self, UserAuthType},
    tools::id::gen_id,
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

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: &Uuid,
    redirect_url: Option<String>,
    custom_expiry: Option<DateTime<Utc>>,
) -> Result<Otp, ComhairleError> {
    let user = users::get_user_by_id(user_id, db).await?;

    if user.auth_type == UserAuthType::Annon {
        return Err(ComhairleError::WrongUserType);
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

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Otp, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(OtpIden::Table)
        .and_where(Expr::col(OtpIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let otp = sqlx::query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("One time passcode".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(otp)
}

#[instrument(err(Debug))]
pub async fn accept(
    db: &PgPool,
    user_id: &Uuid,
    code: &str,
    now: DateTime<Utc>,
) -> Result<Otp, ComhairleError> {
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
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("One time passcode".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(otp)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::routes::auth::OtpSignupRequest;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_otp_for_user_with_expiry(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let before = Utc::now();
        let otp = create(&pool, &user.id, None, None).await?;
        let after = Utc::now();

        let expiry_duration = Duration::minutes(10);
        let lower = before + expiry_duration;
        let upper = after + expiry_duration;

        assert_eq!(otp.user_id, user.id, "incorrect user_id");
        assert_eq!(otp.status, OtpStatus::Pending, "incorrect default status");
        assert!(otp.expires_at >= lower);
        assert!(otp.expires_at <= upper);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_mark_existing_otps_as_error_when_new_otp_created(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let first = create(&pool, &user.id, None, None).await?;
        let second = create(&pool, &user.id, None, None).await?;
        let third = create(&pool, &user.id, None, None).await?;

        let first = get_by_id(&pool, &first.id).await?;
        let second = get_by_id(&pool, &second.id).await?;

        assert_eq!(first.status, OtpStatus::Error, "incorrect first status");
        assert_eq!(second.status, OtpStatus::Error, "incorrect second status");
        assert_eq!(third.status, OtpStatus::Pending, "incorrect third status");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_fail_otp_create_for_annon_users(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_annon_user(&pool).await?;

        let error = create(&pool, &user.id, None, None).await.unwrap_err();

        match error {
            ComhairleError::WrongUserType => return Ok(()),
            _ => panic!("Wrong error type"),
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_otp_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let otp = get_by_id(&pool, &new_otp.id).await?;

        assert_eq!(otp.id, new_otp.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_otp_by_user_code(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now();
        let otp = accept(&pool, &user.id, &new_otp.code, now).await?;

        assert_eq!(otp.user_id, user.id, "incorrect user_id");
        assert_eq!(otp.code, new_otp.code, "incorrect code");
        assert_eq!(otp.status, OtpStatus::Accepted, "incorrect status");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_error_for_expired_otps(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now() + Duration::minutes(11);
        let error = accept(&pool, &user.id, &new_otp.code, now)
            .await
            .unwrap_err();

        match error {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(
                    e,
                    "One time passcode".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Wrong error type"),
        }

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_error_if_user_or_code_incorrect(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
                username: None,
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id, None, None).await?;

        let now = Utc::now();
        let first_error = accept(&pool, &Uuid::new_v4(), &new_otp.code, now)
            .await
            .unwrap_err();
        let second_error = accept(&pool, &user.id, "wrong code", now)
            .await
            .unwrap_err();

        match (first_error, second_error) {
            (ComhairleError::ResourceNotFound(first), ComhairleError::ResourceNotFound(second)) => {
                let message = "One time passcode".to_string();
                assert_eq!(first, message, "incorrect error message");
                assert_eq!(second, message, "incorrect error message")
            }
            _ => panic!("Wrong error type"),
        }

        Ok(())
    }
}
