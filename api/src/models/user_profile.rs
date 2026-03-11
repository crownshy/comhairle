use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "user_profile")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct UserProfile {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub user_id: Uuid,
    pub consented: bool,
    #[partially(transparent)]
    pub ethnicity: Option<String>,
    #[partially(transparent)]
    pub age: Option<i32>,
    #[partially(transparent)]
    pub gender: Option<String>,
    #[partially(transparent)]
    pub zipcode: Option<String>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
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
    update: &PartialUserProfile,
) -> Result<UserProfile, ComhairleError> {
    let mut query = Query::update()
        .table(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .to_owned();

    let mut has_updates = false;

    if let Some(value) = &update.consented {
        query = query
            .value(UserProfileIden::Consented, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = &update.ethnicity {
        query = query.value(UserProfileIden::Ethnicity, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = &update.age {
        query = query.value(UserProfileIden::Age, value.clone()).to_owned();
        has_updates = true;
    }
    if let Some(value) = &update.gender {
        query = query.value(UserProfileIden::Gender, value).to_owned();
        has_updates = true;
    }
    if let Some(value) = &update.zipcode {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::auth::SignupRequest;
    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test]
    async fn should_create_user_profile(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: Some("Asian".to_string()),
            age: Some(25),
            gender: Some("Female".to_string()),
            zipcode: Some("12345".to_string()),
        };

        let profile = create(&pool, &create_profile).await?;

        assert_eq!(profile.user_id, user.id, "incorrect user_id");
        assert_eq!(profile.consented, true, "incorrect consented");
        assert_eq!(
            profile.ethnicity,
            Some("Asian".to_string()),
            "incorrect ethnicity"
        );
        assert_eq!(profile.age, Some(25), "incorrect age");
        assert_eq!(
            profile.gender,
            Some("Female".to_string()),
            "incorrect gender"
        );
        assert_eq!(
            profile.zipcode,
            Some("12345".to_string()),
            "incorrect zipcode"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_profile_by_user_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: Some("Hispanic".to_string()),
            age: Some(30),
            gender: Some("Male".to_string()),
            zipcode: Some("67890".to_string()),
        };

        let created_profile = create(&pool, &create_profile).await?;

        let fetched_profile = get_by_user_id(&pool, &user.id).await?;

        assert_eq!(
            fetched_profile.id, created_profile.id,
            "incorrect profile id"
        );
        assert_eq!(fetched_profile.user_id, user.id, "incorrect user_id");
        assert_eq!(fetched_profile.consented, true, "incorrect consented");

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_profile(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: false,
            ethnicity: None,
            age: None,
            gender: None,
            zipcode: None,
        };

        let profile = create(&pool, &create_profile).await?;
        let original_updated_at = profile.updated_at;

        // Wait a moment to ensure updated_at changes
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let updated_profile = update(
            &pool,
            &profile.id,
            &PartialUserProfile {
                consented: Some(true),
                ethnicity: Some("Black".to_string()),
                age: Some(35),
                gender: Some("Non-binary".to_string()),
                zipcode: Some("54321".to_string()),
                ..PartialUserProfile::default()
            },
        )
        .await?;

        assert_eq!(updated_profile.consented, true, "consented not updated");
        assert_eq!(
            updated_profile.ethnicity,
            Some("Black".to_string()),
            "ethnicity not updated"
        );
        assert_eq!(updated_profile.age, Some(35), "age not updated");
        assert_eq!(
            updated_profile.gender,
            Some("Non-binary".to_string()),
            "gender not updated"
        );
        assert_eq!(
            updated_profile.zipcode,
            Some("54321".to_string()),
            "zipcode not updated"
        );
        assert!(
            updated_profile.updated_at > original_updated_at,
            "updated_at should be updated"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_enforce_one_profile_per_user(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: None,
            age: None,
            gender: None,
            zipcode: None,
        };

        // Create first profile
        let _profile = create(&pool, &create_profile).await?;

        // Attempt to create second profile for same user
        let result = create(&pool, &create_profile).await;

        assert!(
            result.is_err(),
            "should not allow multiple profiles for same user"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_cascade_delete_profile_when_user_deleted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: Some("White".to_string()),
            age: Some(40),
            gender: Some("Male".to_string()),
            zipcode: Some("11111".to_string()),
        };

        let profile = create(&pool, &create_profile).await?;

        // Delete the user
        sqlx::query("DELETE FROM comhairle_user WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await?;

        // Attempt to get the profile
        let result = get_by_id(&pool, &profile.id).await;

        assert!(
            result.is_err(),
            "profile should be deleted when user is deleted"
        );

        Ok(())
    }
}
