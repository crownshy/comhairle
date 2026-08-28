use crate::{
    error::ComhairleError,
    models::{
        SqlxResultExt,
        demographics::{self, ValueBuckets},
    },
};
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, types::Json};
use std::collections::HashMap;
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

    let _ = create_default_user_profile_demographics(
        db,
        new_profile.user_id,
        profile.age.to_owned(),
        profile.ethnicity.to_owned(),
        profile.gender.to_owned(),
        profile.zipcode.to_owned(),
        profile.political_party.to_owned(),
    )
    .await?;

    Ok(new_profile)
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
        .resolve_db_err("User Profile")?;

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
        .resolve_db_err("User Profile")?;

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

pub async fn create_default_user_profile_demographics(
    db: &PgPool,
    user_id: Uuid,
    age: Option<i32>,
    ethnicity: Option<String>,
    gender: Option<String>,
    zipcode: Option<String>,
    political_party: Option<String>,
) -> Result<Vec<demographics::DemographicsResponse>, ComhairleError> {
    let demographics = std::iter::once(("age", age.map(|v| v.to_string())))
        .chain(std::iter::once(("ethnicity", ethnicity)))
        .chain(std::iter::once(("gender", gender)))
        .chain(std::iter::once(("zipcode", zipcode)))
        .chain(std::iter::once(("political_party", political_party)));

    let mut responses = Vec::new();
    for (question_slug, value) in demographics {
        if let Some(value) = value {
            let response = demographics::create_demographics_response(
                db,
                demographics::CreateDemographicsResponse {
                    question_slug: question_slug.to_string(),
                    user_id,
                    value,
                },
            )
            .await?;
            responses.push(response);
        }
    }

    Ok(responses)
}

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DemographicCount {
    pub display_name: String,
    pub value: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DemographicReport {
    pub total_participants: i64,
    pub categories: HashMap<String, Vec<DemographicCount>>,
}

/// Generate a demographic report for users participating in a workflow
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfileDemographicsExport {
    pub question_slug: String,
    pub display_name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfileExport {
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub demographics: Json<HashMap<String, UserProfileDemographicsExport>>,
}

pub async fn get_demographics_for_export(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<UserProfileExport>, ComhairleError> {
    // We join conversation_demographics to ensure we only get required questions,
    // and use jsonb_object_agg to map question_slug -> value dynamically.
    let query = r#"
        SELECT
            up.user_id,
            up.created_at,
            COALESCE(
                jsonb_object_agg(
                    cd.question_slug,
                    jsonb_build_object(
                        'question_slug', cd.question_slug,
                        'display_name', dq.display_name,
                        'value', dr.value
                    )
                ), 
                '{}'::jsonb
            ) as demographics
        FROM user_profile up
        INNER JOIN comhairle_user u ON u.id = up.user_id
        INNER JOIN user_participation upart ON upart.user_id = u.id
        INNER JOIN workflow w ON w.id = upart.workflow_id
        LEFT JOIN conversation_demographics cd ON cd.conversation_id = w.conversation_id
        LEFT JOIN demographics_question dq ON dq.slug = cd.question_slug
        LEFT JOIN demographics_response dr ON dr.user_id = up.user_id AND dr.question_slug = cd.question_slug
        WHERE w.conversation_id = $1
        AND up.consented = true
        GROUP BY up.user_id, up.created_at
        ORDER BY up.created_at DESC
    "#;

    let profiles = sqlx::query_as::<_, UserProfileExport>(query)
        .bind(conversation_id)
        .fetch_all(db)
        .await?;

    Ok(profiles)
}

pub async fn get_demographic_report(
    db: &PgPool,
    workflow_id: &Uuid,
) -> Result<DemographicReport, ComhairleError> {
    // 1. Get total participants (unchanged)
    let total_query = r#"
        SELECT COUNT(DISTINCT up.user_id)::BIGINT as count
        FROM user_participation up
        WHERE up.workflow_id = $1
    "#;
    let total_participants: i64 = sqlx::query_scalar(total_query)
        .bind(workflow_id)
        .fetch_one(db)
        .await?;

    // 2. Get dynamically grouped counts for ALL string-based demographics
    let dynamic_report_query = r#"
        SELECT
            dq.slug as category_name,
            dq.display_name as display_name,
            dq.bucket_config as bucket_config,
            dr.value as value,
            COUNT(up.user_id)::BIGINT as count
        FROM user_participation up
        INNER JOIN workflow w ON w.id = up.workflow_id
        INNER JOIN conversation_demographics cd ON cd.conversation_id = w.conversation_id
        INNER JOIN demographics_question dq ON dq.slug = cd.question_slug
        INNER JOIN user_profile prof ON prof.user_id = up.user_id AND prof.consented = true
        LEFT JOIN demographics_response dr ON dr.user_id = up.user_id AND dr.question_slug = cd.question_slug
        WHERE up.workflow_id = $1
        GROUP BY dq.slug, dq.bucket_config, dr.value
        ORDER BY dq.slug, count DESC
    "#;

    // Use a temporary struct to hold the flat DB rows
    #[derive(sqlx::FromRow)]
    struct FlatDemographicRow {
        category_name: String,
        display_name: String,
        bucket_config: Option<sqlx::types::Json<ValueBuckets>>,
        value: String,
        count: i64,
    }

    let flat_rows: Vec<FlatDemographicRow> = sqlx::query_as(dynamic_report_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    // 3. Transform the flat rows into a nested HashMap for the UI
    let mut categories: HashMap<String, Vec<DemographicCount>> = HashMap::new();

    for row in flat_rows {
        if let Some(bucket_config) = &row.bucket_config {
            let bucket_value = demographics::resolve_category_bucket(&row.value, &bucket_config.0);
            let category_counts = categories.entry(row.category_name).or_insert_with(Vec::new);

            if let Some(category_count) =
                category_counts.iter_mut().find(|c| c.value == bucket_value)
            {
                category_count.count += row.count;
            } else {
                category_counts.push(DemographicCount {
                    display_name: row.display_name,
                    value: bucket_value,
                    count: row.count,
                });
            }
        } else {
            categories
                .entry(row.category_name)
                .or_insert_with(Vec::new)
                .push(DemographicCount {
                    display_name: row.display_name,
                    value: row.value,
                    count: row.count,
                });
        };
    }

    Ok(DemographicReport {
        total_participants,
        categories,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use sqlx::PgPool;
    use std::error::Error;

    use crate::models::demographics::{
        DemographicsResponse, DemographicsResponsesFilterOptions, get_demographics_responses,
    };
    use crate::routes::auth::SignupRequest;

    async fn get_demographic(
        pool: &PgPool,
        question_slug: String,
        user_id: Uuid,
    ) -> Result<Option<DemographicsResponse>, Box<dyn Error>> {
        let record = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some(question_slug),
                user_id: Some(user_id),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();
        Ok(record)
    }

    async fn update_demographic(
        pool: &PgPool,
        question_slug: String,
        user_id: Uuid,
        value: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        crate::models::demographics::update_demographics_response(
            &pool,
            question_slug,
            user_id,
            crate::models::demographics::PartialDemographicsResponse { value: value },
        )
        .await?;
        Ok(())
    }

    async fn add_default_demographics_to_conversation(
        pool: &PgPool,
        conversation_id: Uuid,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::demographics::{
            CreateConversationDemographics, create_conversation_demographics,
        };

        let default_questions = vec!["age", "ethnicity", "gender", "zipcode", "political_party"];
        for question_slug in default_questions {
            let _ = create_conversation_demographics(
                &pool,
                CreateConversationDemographics {
                    conversation_id,
                    question_slug: question_slug.to_string(),
                },
            )
            .await?;
        }
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            political_party: None,
        };

        let profile = create(&pool, &create_profile).await?;

        assert_eq!(profile.user_id, user.id, "incorrect user_id");
        assert!(profile.consented, "incorrect consented");

        let age = get_demographic(&pool, "age".to_string(), user.id).await?;
        let ethnicity = get_demographic(&pool, "ethnicity".to_string(), user.id).await?;
        let gender = get_demographic(&pool, "gender".to_string(), user.id).await?;
        let zipcode = get_demographic(&pool, "zipcode".to_string(), user.id).await?;

        assert_eq!(
            age.map(|r| r.value),
            Some("25".to_string()),
            "incorrect age"
        );
        assert_eq!(
            ethnicity.map(|r| r.value),
            Some("Asian".to_string()),
            "incorrect ethnicity"
        );
        assert_eq!(
            gender.map(|r| r.value),
            Some("Female".to_string()),
            "incorrect gender"
        );
        assert_eq!(
            zipcode.map(|r| r.value),
            Some("12345".to_string()),
            "incorrect zipcode"
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            ethnicity: Some("Hispanic".to_string()),
            age: None,
            gender: None,
            zipcode: Some("67890".to_string()),
            political_party: None,
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
                ..PartialUserProfile::default()
            },
        )
        .await?;

        update_demographic(
            &pool,
            "ethnicity".to_string(),
            user.id,
            Some("Black".to_string()),
        )
        .await?;
        update_demographic(
            &pool,
            "zipcode".to_string(),
            user.id,
            Some("54321".to_string()),
        )
        .await?;

        assert!(updated_profile.consented, "consented not updated");
        assert!(
            updated_profile.updated_at > original_updated_at,
            "updated_at should be updated"
        );

        let age = get_demographic(&pool, "age".to_string(), user.id).await?;
        let ethnicity = get_demographic(&pool, "ethnicity".to_string(), user.id).await?;
        let gender = get_demographic(&pool, "gender".to_string(), user.id).await?;
        let zipcode = get_demographic(&pool, "zipcode".to_string(), user.id).await?;
        let political_party =
            get_demographic(&pool, "political_party".to_string(), user.id).await?;

        assert_eq!(age, None, "age updated unexpectedly");
        assert_eq!(
            ethnicity.map(|r| r.value),
            Some("Black".to_string()),
            "ethnicity not updated"
        );
        assert_eq!(gender, None, "gender updated unexpectedly");
        assert_eq!(
            zipcode.map(|r| r.value),
            Some("54321".to_string()),
            "zipcode not updated"
        );
        assert_eq!(
            political_party, None,
            "political_party updated unexpectedly"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            political_party: None,
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
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
            political_party: None,
        };

        let profile = create(&pool, &create_profile).await?;

        let age = get_demographic(&pool, "age".to_string(), user.id).await?;
        let ethnicity = get_demographic(&pool, "ethnicity".to_string(), user.id).await?;
        let gender = get_demographic(&pool, "gender".to_string(), user.id).await?;
        let zipcode = get_demographic(&pool, "zipcode".to_string(), user.id).await?;
        let political_party =
            get_demographic(&pool, "political_party".to_string(), user.id).await?;

        assert_eq!(
            age.map(|r| r.value),
            Some("40".to_string()),
            "age should be correctly set"
        );
        assert_eq!(
            ethnicity.map(|r| r.value),
            Some("White".to_string()),
            "ethnicity should be correctly set"
        );
        assert_eq!(
            gender.map(|r| r.value),
            Some("Male".to_string()),
            "gender should be correctly set"
        );
        assert_eq!(
            zipcode.map(|r| r.value),
            Some("11111".to_string()),
            "zipcode should be correctly set"
        );
        assert_eq!(
            political_party, None,
            "political_party should be correctly set"
        );

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

        // We should keep demographics and simply set the user to null on delete.
        let age = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some("age".to_string()),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();

        let ethnicity = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some("ethnicity".to_string()),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();

        let gender = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some("gender".to_string()),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();

        let zipcode = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some("zipcode".to_string()),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();

        let political_party = get_demographics_responses(
            &pool,
            DemographicsResponsesFilterOptions {
                question_slug: Some("political_party".to_string()),
                ..Default::default()
            },
            Default::default(),
        )
        .await?
        .records
        .drain(..)
        .next();

        assert_eq!(
            age.map(|r| r.value),
            Some("40".to_string()),
            "age should not be deleted via cascade"
        );
        assert_eq!(
            ethnicity.map(|r| r.value),
            Some("White".to_string()),
            "ethnicity should not be deleted via cascade"
        );
        assert_eq!(
            gender.map(|r| r.value),
            Some("Male".to_string()),
            "gender should not be deleted via cascade"
        );
        assert_eq!(
            zipcode.map(|r| r.value),
            Some("11111".to_string()),
            "zipcode should not be deleted via cascade"
        );
        assert_eq!(
            political_party, None,
            "political_party should be not be set"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_generate_demographic_report_for_workflow(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{
            model_test_helpers::setup_default_app_and_session, user_participation,
        };

        // Setup app and session
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        // Create conversation and workflow via API
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: crate::routes::conversations::dto::ConversationDto =
            serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: crate::routes::workflows::dto::WorkflowDto =
            serde_json::from_value(workflow)?;
        let workflow_id = workflow.id;

        // Add demographics questions to conversation
        add_default_demographics_to_conversation(&pool, conversation.id).await?;

        // Create users with different demographics
        let user1 = crate::models::users::create_user(
            &SignupRequest {
                username: "user1".to_string(),
                password: "pw1".to_string(),
                email: "user1@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: user1.id,
                consented: true,
                ethnicity: Some("Asian".to_string()),
                age: Some(25),
                gender: Some("Female".to_string()),
                zipcode: Some("12345".to_string()),
                political_party: Some("Independent".to_string()),
            },
        )
        .await?;

        let user2 = crate::models::users::create_user(
            &SignupRequest {
                username: "user2".to_string(),
                password: "pw2".to_string(),
                email: "user2@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: user2.id,
                consented: true,
                ethnicity: Some("Hispanic".to_string()),
                age: Some(30),
                gender: Some("Male".to_string()),
                zipcode: Some("67890".to_string()),
                political_party: Some("Democrat".to_string()),
            },
        )
        .await?;

        let user3 = crate::models::users::create_user(
            &SignupRequest {
                username: "user3".to_string(),
                password: "pw3".to_string(),
                email: "user3@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: user3.id,
                consented: true,
                ethnicity: Some("Asian".to_string()),
                age: Some(45),
                gender: Some("Non-binary".to_string()),
                zipcode: Some("11111".to_string()),
                political_party: Some("Republican".to_string()),
            },
        )
        .await?;

        // Register users to the workflow
        user_participation::create(&pool, &user1.id, &workflow_id).await?;
        user_participation::create(&pool, &user2.id, &workflow_id).await?;
        user_participation::create(&pool, &user3.id, &workflow_id).await?;

        // Generate report
        let report = get_demographic_report(&pool, &workflow_id).await?;

        // Verify total participants
        assert_eq!(report.total_participants, 3, "incorrect total participants");

        // Verify ethnicity breakdown
        assert_eq!(report.categories.len(), 5, "incorrect number of categories");

        let ethnicity = report.categories.get("ethnicity");
        let age = report.categories.get("age");
        let gender = report.categories.get("gender");
        let political_party = report.categories.get("political_party");
        let zipcode = report.categories.get("zipcode");

        assert!(ethnicity.is_some(), "ethnicity category should exist");
        assert!(age.is_some(), "age category should exist");
        assert!(gender.is_some(), "gender category should exist");
        assert!(
            political_party.is_some(),
            "political party category should exist"
        );
        assert!(zipcode.is_some(), "zipcode category should exist");

        let ethnicities = ethnicity.unwrap();
        let ages = age.unwrap();
        let genders = gender.unwrap();
        let political_parties = political_party.unwrap();
        let zipcodes = zipcode.unwrap();

        assert_eq!(ethnicities.len(), 2, "incorrect ethnicity count");
        let asian_count = ethnicities
            .iter()
            .find(|e| e.value == "Asian".to_string())
            .map(|e| e.count)
            .unwrap_or(0);
        assert_eq!(asian_count, 2, "incorrect Asian count");

        let hispanic_count = ethnicities
            .iter()
            .find(|e| e.value == "Hispanic".to_string())
            .map(|e| e.count)
            .unwrap_or(0);
        assert_eq!(hispanic_count, 1, "incorrect Hispanic count");

        // Verify age ranges (user1=25, user2=30, user3=45)
        let age_25_34 = ages
            .iter()
            .find(|a| a.value == "25-34".to_string())
            .map(|a| a.count)
            .unwrap_or(0);
        assert_eq!(age_25_34, 2, "incorrect 25-34 age range count");

        let age_45_54 = ages
            .iter()
            .find(|a| a.value == "45-54".to_string())
            .map(|a| a.count)
            .unwrap_or(0);
        assert_eq!(age_45_54, 1, "incorrect 45-54 age range count");

        // Verify gender breakdown
        assert_eq!(genders.len(), 3, "incorrect gender count");

        // Verify political party breakdown
        assert_eq!(
            political_parties.len(),
            3,
            "incorrect political party count"
        );

        // Verify zipcode counts
        assert_eq!(zipcodes.len(), 3, "should have 3 unique zipcodes");
        assert_eq!(
            // Count of users in zipcode 12345
            zipcodes.iter().find(|dc| dc.value == "12345").iter().len(),
            1,
            "zipcode 12345 should have 1 user"
        );
        assert_eq!(
            zipcodes.iter().find(|dc| dc.value == "67890").iter().len(),
            1,
            "zipcode 67890 should have 1 user"
        );
        assert_eq!(
            zipcodes.iter().find(|dc| dc.value == "11111").iter().len(),
            1,
            "zipcode 11111 should have 1 user"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn demographic_report_should_exclude_non_consented_users(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{
            model_test_helpers::setup_default_app_and_session, user_participation,
        };

        // Setup app and session
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        // Create conversation and workflow via API
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: crate::routes::conversations::dto::ConversationDto =
            serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: crate::routes::workflows::dto::WorkflowDto =
            serde_json::from_value(workflow)?;
        let workflow_id = workflow.id;

        // Add default demographics to the conversation
        add_default_demographics_to_conversation(&pool, conversation.id).await?;

        // Create consented user
        let consented_user = crate::models::users::create_user(
            &SignupRequest {
                username: "consented".to_string(),
                password: "pw".to_string(),
                email: "consented@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: consented_user.id,
                consented: true,
                ethnicity: Some("Asian".to_string()),
                age: Some(25),
                gender: Some("Female".to_string()),
                zipcode: Some("12345".to_string()),
                political_party: Some("Independent".to_string()),
            },
        )
        .await?;

        // Create non-consented user
        let non_consented_user = crate::models::users::create_user(
            &SignupRequest {
                username: "non_consented".to_string(),
                password: "pw".to_string(),
                email: "non_consented@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: non_consented_user.id,
                consented: false,
                ethnicity: Some("Hispanic".to_string()),
                age: Some(30),
                gender: Some("Male".to_string()),
                zipcode: Some("67890".to_string()),
                political_party: Some("Democrat".to_string()),
            },
        )
        .await?;

        // Register both users to workflow
        user_participation::create(&pool, &consented_user.id, &workflow.id).await?;
        user_participation::create(&pool, &non_consented_user.id, &workflow.id).await?;

        // Generate report
        let report = get_demographic_report(&pool, &workflow_id).await?;

        // Both users should count in total
        assert_eq!(report.total_participants, 2, "incorrect total participants");

        let ethnicities = report.categories.get("ethnicity");
        let genders = report.categories.get("gender");
        let zipcodes = report.categories.get("zipcode");

        assert!(ethnicities.is_some(), "ethnicity category should exist");
        assert!(genders.is_some(), "gender category should exist");
        assert!(zipcodes.is_some(), "zipcode category should exist");

        let ethnicities = ethnicities.unwrap();
        let genders = genders.unwrap();
        let zipcodes = zipcodes.unwrap();

        assert_eq!(ethnicities.len(), 1, "should only have one ethnicity");
        assert_eq!(genders.len(), 1, "should only have one gender");
        assert_eq!(zipcodes.len(), 1, "should only have one zipcode");

        // Only consented user should appear in demographics
        assert_eq!(
            ethnicities[0].value,
            "Asian".to_string(),
            "should only show consented user's ethnicity"
        );
        assert_eq!(ethnicities[0].count, 1, "should only count consented user");

        assert_eq!(genders.len(), 1, "should only have one gender");
        assert_eq!(
            genders[0].value,
            "Female".to_string(),
            "should only show consented user's gender"
        );

        // Verify zipcode counts only include consented user
        assert_eq!(zipcodes.len(), 1, "should only have one zipcode");
        assert_eq!(
            zipcodes[0].value,
            "12345".to_string(),
            "should only show consented user's zipcode"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn demographic_report_should_handle_null_values(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{
            model_test_helpers::setup_default_app_and_session, user_participation,
        };

        // Setup app and session
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        // Create conversation and workflow via API
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: crate::routes::conversations::dto::ConversationDto =
            serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: crate::routes::workflows::dto::WorkflowDto =
            serde_json::from_value(workflow)?;
        let workflow_id = workflow.id;

        // Create user with null demographics
        let user = crate::models::users::create_user(
            &SignupRequest {
                username: "user".to_string(),
                password: "pw".to_string(),
                email: "user@example.com".to_string(),
                avatar_url: None,
            },
            &pool,
        )
        .await?;

        create(
            &pool,
            &CreateUserProfile {
                user_id: user.id,
                consented: true,
                ethnicity: None,
                age: None,
                gender: None,
                zipcode: None,
                political_party: None,
            },
        )
        .await?;

        // Register user to workflow
        user_participation::create(&pool, &user.id, &workflow_id).await?;

        // Generate report
        let report = get_demographic_report(&pool, &workflow_id).await?;

        assert_eq!(
            report.categories.len(),
            0,
            "should have no demographics entries for user with null values"
        );

        Ok(())
    }
}
