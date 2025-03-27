use std::fmt;

use crate::{
    error::ComhairleError,
    routes::auth::{hash_pw, SignupRequest},
};
use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

/// Defines the type of authentication has been used to create
/// The user
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum UserAuthType {
    #[sqlx(rename = "annon")]
    Annon,
    #[sqlx(rename = "email_password")]
    EmailPassword,
    #[sqlx(rename = "scot_account")]
    ScotAccount,
}

impl Into<sea_query::Value> for UserAuthType {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
    }
}

impl fmt::Display for UserAuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            UserAuthType::Annon => "annon",
            UserAuthType::EmailPassword => "email_password",
            UserAuthType::ScotAccount => "scot_account",
        };
        write!(f, "{}", value)
    }
}

/// User table representation
/// user is a protected word in postgresql so
/// we actually use the comahirle_user table
#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
#[enum_def(table_name = "comhairle_user")]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub auth_type: UserAuthType,
    pub email: Option<String>,
}

/// Create a user from a signup request
pub async fn create_user(user: &SignupRequest, db: &PgPool) -> Result<User, ComhairleError> {
    let password = hash_pw(&user.password)?;
    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([
            UserIden::AuthType,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::Email,
        ])
        .values([
            UserAuthType::EmailPassword.into(),
            user.username.clone().into(),
            password.into(),
            user.avatar_url.clone().into(),
            user.email.clone().into(),
        ])
        .unwrap()
        .returning(Query::returning().columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AuthType,
            UserIden::AvatarUrl,
            UserIden::Email,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    // Check to see if the either a unique username or email has been
    // duplicated
    match user_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                if let Some(constraint) = pg_err.constraint() {
                    if constraint.contains("username") {
                        return Err(ComhairleError::DuplicateUsername(user.username.clone()));
                    } else if constraint.contains("email") {
                        return Err(ComhairleError::DuplicateEmail(user.email.clone()));
                    }
                }
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

/// Create an annon user
pub async fn create_annon_user(db: &PgPool) -> Result<User, ComhairleError> {
    let sudo_random_name: String = ppg::generate().replace(" ", "-");

    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([UserIden::Username, UserIden::AuthType])
        .values([sudo_random_name.into(), UserAuthType::Annon.into()])
        .unwrap()
        .returning(Query::returning().columns([
            UserIden::AuthType,
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::Email,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await?;
    Ok(user)
}

/// Return a user by ID
pub async fn get_user_by_id(id: &Uuid, db: &PgPool) -> Result<User, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await?;
    Ok(user)
}

/// Return a user by email
pub async fn get_user_by_email(email: &str, db: &PgPool) -> Result<User, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Email).eq(email))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await?;
    Ok(user)
}
