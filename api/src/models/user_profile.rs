use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "user_profile")]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub consented: bool,
    pub ethnicity: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateUserProfile {
    pub user_id: Uuid,
    pub consented: bool,
    pub ethnicity: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
}

const DEFAULT_COLUMNS: [UserProfileIden; 9] = [
    UserProfileIden::Id,
    UserProfileIden::UserId,
    UserProfileIden::Consented,
    UserProfileIden::Ethnicity,
    UserProfileIden::Age,
    UserProfileIden::Gender,
    UserProfileIden::Zipcode,
    UserProfileIden::CreatedAt,
    UserProfileIden::UpdatedAt,
];

impl CreateUserProfile {
    pub fn columns(&self) -> Vec<UserProfileIden> {
        let mut columns = vec![UserProfileIden::UserId, UserProfileIden::Consented];

        if self.ethnicity.is_some() {
            columns.push(UserProfileIden::Ethnicity);
        }
        if self.age.is_some() {
            columns.push(UserProfileIden::Age);
        }
        if self.gender.is_some() {
            columns.push(UserProfileIden::Gender);
        }
        if self.zipcode.is_some() {
            columns.push(UserProfileIden::Zipcode);
        }

        columns
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values: Vec<sea_query::SimpleExpr> =
            vec![self.user_id.into(), self.consented.into()];

        if let Some(ref ethnicity) = self.ethnicity {
            values.push(ethnicity.clone().into());
        }
        if let Some(age) = self.age {
            values.push(age.into());
        }
        if let Some(ref gender) = self.gender {
            values.push(gender.clone().into());
        }
        if let Some(ref zipcode) = self.zipcode {
            values.push(zipcode.clone().into());
        }

        values
    }
}

pub async fn create(
    db: &PgPool,
    profile: &CreateUserProfile,
) -> Result<UserProfile, ComhairleError> {
    let columns = profile.columns();
    let values = profile.values();

    let (sql, values) = Query::insert()
        .into_table(UserProfileIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(profile)
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserProfile".into()))?;

    Ok(profile)
}

pub async fn get_by_user_id(db: &PgPool, user_id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::UserId).eq(user_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserProfile".into()))?;

    Ok(profile)
}

pub async fn update(
    db: &PgPool,
    id: &Uuid,
    consented: Option<bool>,
    ethnicity: Option<String>,
    age: Option<i32>,
    gender: Option<String>,
    zipcode: Option<String>,
) -> Result<UserProfile, ComhairleError> {
    let mut query = Query::update()
        .table(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .to_owned();

    let mut has_updates = false;

    if let Some(value) = consented {
        query = query.value(UserProfileIden::Consented, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = ethnicity {
        query = query.value(UserProfileIden::Ethnicity, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = age {
        query = query.value(UserProfileIden::Age, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = gender {
        query = query.value(UserProfileIden::Gender, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = zipcode {
        query = query.value(UserProfileIden::Zipcode, value).to_owned();
        has_updates = true;
    }

    if !has_updates {
        return get_by_id(db, id).await;
    }

    // Always update the updated_at timestamp when there are changes
    query = query
        .value(UserProfileIden::UpdatedAt, Utc::now())
        .to_owned();

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(profile)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("UserProfile".into()))?;

    Ok(profile)
}
