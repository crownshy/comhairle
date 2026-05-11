use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
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

const DEFAULT_COLUMNS: [OtpIden; 7] = [
    OtpIden::Id,
    OtpIden::UserId,
    OtpIden::Code,
    OtpIden::Status,
    OtpIden::ExpiresAt,
    OtpIden::CreatedAt,
    OtpIden::UpdatedAt,
];

#[instrument(err(Debug))]
pub async fn create(db: &PgPool, user_id: &Uuid) -> Result<Otp, ComhairleError> {
    let user = users::get_user_by_id(user_id, db).await?;

    if user.auth_type == UserAuthType::Annon {
        return Err(ComhairleError::WrongUserType);
    }

    let random_code = gen_id();
    let expires_at = Utc::now() + Duration::minutes(10);

    let (sql, values) = Query::insert()
        .into_table(OtpIden::Table)
        .columns([OtpIden::UserId, OtpIden::Code, OtpIden::ExpiresAt])
        .values([(*user_id).into(), random_code.into(), expires_at.into()])?
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
pub async fn get_by_user_code(
    db: &PgPool,
    user_id: &Uuid,
    code: &str,
) -> Result<Otp, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(OtpIden::Table)
        .and_where(Expr::col(OtpIden::UserId).eq(user_id.to_owned()))
        .and_where(Expr::col(OtpIden::Code).eq(code.to_owned()))
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

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateOtp {
    status: Option<OtpStatus>,
}

#[instrument(err(Debug))]
pub async fn update(db: &PgPool, id: &Uuid, update: &UpdateOtp) -> Result<Otp, ComhairleError> {
    let mut values = vec![];

    if let Some(value) = update.status {
        values.push((OtpIden::Status, value.into()));
    }

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(OtpIden::Table)
        .values(values)
        .and_where(Expr::col(OtpIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let otp = sqlx::query_as_with(&sql, values).fetch_one(db).await?;

    Ok(otp)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::routes::auth::OtpSignupRequest;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_otp_for_user_with_expiry(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
            },
            &pool,
        )
        .await?;

        let before = Utc::now();
        let otp = create(&pool, &user.id).await?;
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

    #[sqlx::test]
    async fn should_fail_otp_create_for_annon_users(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_annon_user(&pool).await?;

        let error = create(&pool, &user.id).await.unwrap_err();

        match error {
            ComhairleError::WrongUserType => return Ok(()),
            _ => panic!("Wrong error type"),
        }
    }

    #[sqlx::test]
    async fn should_get_otp_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id).await?;

        let otp = get_by_id(&pool, &new_otp.id).await?;

        assert_eq!(otp.id, new_otp.id, "ids don't match");

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_otp_by_user_code(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id).await?;

        let otp = get_by_user_code(&pool, &user.id, &new_otp.code).await?;

        assert_eq!(otp.user_id, user.id, "incorrect user_id");
        assert_eq!(otp.code, new_otp.code, "incorrect code");

        Ok(())
    }

    #[sqlx::test]
    async fn should_return_error_if_user_or_code_incorrect(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id).await?;

        let first_error = get_by_user_code(&pool, &Uuid::new_v4(), &new_otp.code)
            .await
            .unwrap_err();
        let second_error = get_by_user_code(&pool, &user.id, "wrong code")
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

    #[sqlx::test]
    async fn should_update_otp_status(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_otp_user(
            &OtpSignupRequest {
                email: "test@user.com".to_string(),
            },
            &pool,
        )
        .await?;

        let new_otp = create(&pool, &user.id).await?;

        let otp = update(
            &pool,
            &new_otp.id,
            &UpdateOtp {
                status: Some(OtpStatus::Accepted),
            },
        )
        .await?;

        assert_eq!(otp.status, OtpStatus::Accepted, "incorrect status");

        Ok(())
    }
}
