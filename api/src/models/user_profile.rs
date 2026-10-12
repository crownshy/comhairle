use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        demographics::defaults::{SignupAnswers, record_signup_answers},
    },
};
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::instrument;
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
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateUserProfile {
    pub user_id: Uuid,
    pub consented: bool,
    pub age: Option<i32>,
    pub ethnicity: Option<String>,
    pub gender: Option<String>,
    pub zipcode: Option<String>,
    pub political_party: Option<String>,
}

const DEFAULT_COLUMNS: [UserProfileIden; 5] = [
    UserProfileIden::Id,
    UserProfileIden::UserId,
    UserProfileIden::Consented,
    UserProfileIden::CreatedAt,
    UserProfileIden::UpdatedAt,
];

impl CreateUserProfile {
    pub fn columns(&self) -> Vec<UserProfileIden> {
        vec![UserProfileIden::UserId, UserProfileIden::Consented]
    }

    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![self.user_id.into(), self.consented.into()]
    }
}

#[instrument(err(Debug), skip(db))]
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

    let new_profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await?;

    record_signup_answers(
        db,
        new_profile.user_id,
        SignupAnswers {
            age: profile.age,
            ethnicity: profile.ethnicity.clone(),
            gender: profile.gender.clone(),
            zipcode: profile.zipcode.clone(),
            political_party: profile.political_party.clone(),
        },
    )
    .await?;

    Ok(new_profile)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("User Profile")?;

    Ok(profile)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_by_user_id(db: &PgPool, user_id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::UserId).eq(user_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("User Profile")?;

    Ok(profile)
}

#[instrument(err(Debug), skip(db))]
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
        query = query.value(UserProfileIden::Consented, *value).to_owned();
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

#[instrument(err(Debug), skip(db))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<UserProfile, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(UserProfileIden::Table)
        .and_where(Expr::col(UserProfileIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let profile = sqlx::query_as_with::<_, UserProfile, _>(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("User Profile")?;

    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::demographics::defaults::{self, AGE_VERSION_ID, GENDER_VERSION_ID};
    use crate::routes::auth::SignupRequest;

    async fn create_test_user(pool: &PgPool) -> Result<crate::models::users::User, Box<dyn Error>> {
        Ok(crate::models::users::create_user(
            &SignupRequest {
                username: "test_user".to_string(),
                password: "test_pw".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            },
            pool,
        )
        .await?)
    }

    async fn answer_for_version(
        pool: &PgPool,
        user_id: Uuid,
        version_id: Uuid,
    ) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
        let value = sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT value FROM demographics_response WHERE user_id = $1 AND question_version_id = $2",
        )
        .bind(user_id)
        .bind(version_id)
        .fetch_optional(pool)
        .await?;
        Ok(value)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_user_profile_and_record_signup_answers(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = create_test_user(&pool).await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: Some("Asian".to_string()),
            age: Some(30),
            gender: Some("Male".to_string()),
            zipcode: Some("67890".to_string()),
            political_party: None,
        };
        create(&pool, &create_profile).await?;

        assert_eq!(
            answer_for_version(&pool, user.id, AGE_VERSION_ID).await?,
            Some(serde_json::json!(30))
        );
        assert_eq!(
            answer_for_version(&pool, user.id, GENDER_VERSION_ID).await?,
            Some(serde_json::json!("Male"))
        );
        assert_eq!(
            answer_for_version(&pool, user.id, defaults::POSTCODE_VERSION_ID).await?,
            Some(serde_json::json!({ "postcode": "67890" }))
        );
        assert_eq!(
            answer_for_version(&pool, user.id, defaults::POLITICAL_PARTY_VERSION_ID).await?,
            None,
            "unanswered signup questions are not recorded"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_profile_by_user_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = create_test_user(&pool).await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: None,
            age: None,
            gender: None,
            zipcode: None,
            political_party: None,
        };
        let created_profile = create(&pool, &create_profile).await?;

        let fetched_profile = get_by_user_id(&pool, &user.id).await?;

        assert_eq!(
            fetched_profile.id, created_profile.id,
            "incorrect profile id"
        );
        assert_eq!(fetched_profile.user_id, user.id, "incorrect user_id");
        assert!(fetched_profile.consented, "incorrect consented");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_profile(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = create_test_user(&pool).await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: false,
            ethnicity: None,
            age: None,
            gender: None,
            zipcode: None,
            political_party: None,
        };
        let profile = create(&pool, &create_profile).await?;
        let original_updated_at = profile.updated_at;

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let updated_profile = update(
            &pool,
            &profile.id,
            &PartialUserProfile {
                consented: Some(true),
                ..PartialUserProfile::default()
            },
        )
        .await?;

        assert!(updated_profile.consented, "consented not updated");
        assert!(
            updated_profile.updated_at > original_updated_at,
            "updated_at should change"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_enforce_one_profile_per_user(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = create_test_user(&pool).await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: None,
            age: None,
            gender: None,
            zipcode: None,
            political_party: None,
        };
        create(&pool, &create_profile).await?;

        let result = create(&pool, &create_profile).await;

        assert!(
            result.is_err(),
            "should not allow multiple profiles for same user"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_cascade_delete_profile_but_keep_answers_when_user_deleted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let user = create_test_user(&pool).await?;

        let create_profile = CreateUserProfile {
            user_id: user.id,
            consented: true,
            ethnicity: None,
            age: Some(40),
            gender: None,
            zipcode: None,
            political_party: None,
        };
        let profile = create(&pool, &create_profile).await?;

        sqlx::query("DELETE FROM comhairle_user WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await?;

        assert!(
            get_by_id(&pool, &profile.id).await.is_err(),
            "profile should be deleted when user is deleted"
        );

        let orphaned_answers: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM demographics_response WHERE user_id IS NULL AND question_version_id = $1",
        )
        .bind(AGE_VERSION_ID)
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            orphaned_answers, 1,
            "answers should be kept with user_id set to null"
        );

        Ok(())
    }
}
