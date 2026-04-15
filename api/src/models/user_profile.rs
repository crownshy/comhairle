use crate::error::ComhairleError;
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
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
    #[partially(transparent)]
    pub ethnicity: Option<String>,
    #[partially(transparent)]
    pub age: Option<i32>,
    #[partially(transparent)]
    pub gender: Option<String>,
    #[partially(transparent)]
    pub zipcode: Option<String>,
    #[partially(transparent)]
    pub political_party: Option<String>,
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
    pub political_party: Option<String>,
}

const DEFAULT_COLUMNS: [UserProfileIden; 10] = [
    UserProfileIden::Id,
    UserProfileIden::UserId,
    UserProfileIden::Consented,
    UserProfileIden::Ethnicity,
    UserProfileIden::Age,
    UserProfileIden::Gender,
    UserProfileIden::Zipcode,
    UserProfileIden::PoliticalParty,
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
        if self.political_party.is_some() {
            columns.push(UserProfileIden::PoliticalParty);
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
        if let Some(ref political_party) = self.political_party {
            values.push(political_party.clone().into());
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
    if let Some(value) = &update.political_party {
        query = query
            .value(UserProfileIden::PoliticalParty, value)
            .to_owned();
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

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DemographicCategory {
    pub category: String,
    pub value: Option<String>,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DemographicReport {
    pub total_participants: i64,
    pub ethnicity: Vec<DemographicCategory>,
    pub age_ranges: Vec<DemographicCategory>,
    pub gender: Vec<DemographicCategory>,
    pub political_party: Vec<DemographicCategory>,
    pub zipcode_counts: HashMap<String, i64>,
}

/// Generate a demographic report for users participating in a workflow
pub async fn get_demographic_report(
    db: &PgPool,
    workflow_id: &Uuid,
) -> Result<DemographicReport, ComhairleError> {
    // Get total participants
    let total_query = r#"
        SELECT COUNT(DISTINCT up.user_id)::BIGINT as count
        FROM user_participation up
        WHERE up.workflow_id = $1
    "#;
    let total_participants: i64 = sqlx::query_scalar(total_query)
        .bind(workflow_id)
        .fetch_one(db)
        .await?;

    // Get ethnicity breakdown (only consented users)
    let ethnicity_query = r#"
        SELECT
            'ethnicity'::TEXT as category,
            prof.ethnicity as value,
            COUNT(*)::BIGINT as count
        FROM user_participation up
        INNER JOIN user_profile prof ON prof.user_id = up.user_id
        WHERE up.workflow_id = $1 AND prof.consented = true
        GROUP BY prof.ethnicity
        ORDER BY count DESC
    "#;
    let ethnicity: Vec<DemographicCategory> = sqlx::query_as(ethnicity_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    // Get age ranges breakdown (only consented users)
    let age_query = r#"
        WITH age_categories AS (
            SELECT
                'age_range'::TEXT as category,
                CASE
                    WHEN prof.age IS NULL THEN NULL
                    WHEN prof.age < 18 THEN 'Under 18'
                    WHEN prof.age >= 18 AND prof.age < 25 THEN '18-24'
                    WHEN prof.age >= 25 AND prof.age < 35 THEN '25-34'
                    WHEN prof.age >= 35 AND prof.age < 45 THEN '35-44'
                    WHEN prof.age >= 45 AND prof.age < 55 THEN '45-54'
                    WHEN prof.age >= 55 AND prof.age < 65 THEN '55-64'
                    ELSE '65+'
                END as value
            FROM user_participation up
            INNER JOIN user_profile prof ON prof.user_id = up.user_id
            WHERE up.workflow_id = $1 AND prof.consented = true
        )
        SELECT
            category,
            value,
            COUNT(*)::BIGINT as count
        FROM age_categories
        GROUP BY category, value
        ORDER BY
            CASE value
                WHEN 'Under 18' THEN 1
                WHEN '18-24' THEN 2
                WHEN '25-34' THEN 3
                WHEN '35-44' THEN 4
                WHEN '45-54' THEN 5
                WHEN '55-64' THEN 6
                WHEN '65+' THEN 7
                ELSE 8
            END
    "#;
    let age_ranges: Vec<DemographicCategory> = sqlx::query_as(age_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    // Get gender breakdown (only consented users)
    let gender_query = r#"
        SELECT
            'gender'::TEXT as category,
            prof.gender as value,
            COUNT(*)::BIGINT as count
        FROM user_participation up
        INNER JOIN user_profile prof ON prof.user_id = up.user_id
        WHERE up.workflow_id = $1 AND prof.consented = true
        GROUP BY prof.gender
        ORDER BY count DESC
    "#;
    let gender: Vec<DemographicCategory> = sqlx::query_as(gender_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    // Get political party breakdown (only consented users)
    let political_party_query = r#"
        SELECT
            'political_party'::TEXT as category,
            prof.political_party as value,
            COUNT(*)::BIGINT as count
        FROM user_participation up
        INNER JOIN user_profile prof ON prof.user_id = up.user_id
        WHERE up.workflow_id = $1 AND prof.consented = true
        GROUP BY prof.political_party
        ORDER BY count DESC
    "#;
    let political_party: Vec<DemographicCategory> = sqlx::query_as(political_party_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    // Get zipcode counts (only non-null zipcodes from consented users)
    let zipcode_query = r#"
        SELECT
            prof.zipcode as zipcode,
            COUNT(*)::BIGINT as count
        FROM user_participation up
        INNER JOIN user_profile prof ON prof.user_id = up.user_id
        WHERE up.workflow_id = $1 AND prof.consented = true AND prof.zipcode IS NOT NULL
        GROUP BY prof.zipcode
        ORDER BY count DESC
    "#;

    #[derive(FromRow)]
    struct ZipcodeCount {
        zipcode: String,
        count: i64,
    }

    let zipcode_results: Vec<ZipcodeCount> = sqlx::query_as(zipcode_query)
        .bind(workflow_id)
        .fetch_all(db)
        .await?;

    let zipcode_counts: HashMap<String, i64> = zipcode_results
        .into_iter()
        .map(|z| (z.zipcode, z.count))
        .collect();

    Ok(DemographicReport {
        total_participants,
        ethnicity,
        age_ranges,
        gender,
        political_party,
        zipcode_counts,
    })
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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

    #[sqlx::test]
    async fn should_generate_demographic_report_for_workflow(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{model_test_helpers::setup_default_app_and_session, user_participation};

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

        // Create users with different demographics
        let user1 = crate::models::users::create_user(
            &SignupRequest {
                username: "user1".to_string(),
                password: "pw1".to_string(),
                email: "user1@example.com".to_string(),
                avatar_url: None,
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
        assert_eq!(
            report.total_participants, 3,
            "incorrect total participants"
        );

        // Verify ethnicity breakdown
        assert_eq!(report.ethnicity.len(), 2, "incorrect ethnicity count");
        let asian_count = report
            .ethnicity
            .iter()
            .find(|e| e.value == Some("Asian".to_string()))
            .map(|e| e.count)
            .unwrap_or(0);
        assert_eq!(asian_count, 2, "incorrect Asian count");

        let hispanic_count = report
            .ethnicity
            .iter()
            .find(|e| e.value == Some("Hispanic".to_string()))
            .map(|e| e.count)
            .unwrap_or(0);
        assert_eq!(hispanic_count, 1, "incorrect Hispanic count");

        // Verify age ranges (user1=25, user2=30, user3=45)
        let age_25_34 = report
            .age_ranges
            .iter()
            .find(|a| a.value == Some("25-34".to_string()))
            .map(|a| a.count)
            .unwrap_or(0);
        assert_eq!(age_25_34, 2, "incorrect 25-34 age range count");

        let age_45_54 = report
            .age_ranges
            .iter()
            .find(|a| a.value == Some("45-54".to_string()))
            .map(|a| a.count)
            .unwrap_or(0);
        assert_eq!(age_45_54, 1, "incorrect 45-54 age range count");

        // Verify gender breakdown
        assert_eq!(report.gender.len(), 3, "incorrect gender count");

        // Verify political party breakdown
        assert_eq!(
            report.political_party.len(),
            3,
            "incorrect political party count"
        );

        // Verify zipcode counts
        assert_eq!(
            report.zipcode_counts.len(),
            3,
            "should have 3 unique zipcodes"
        );
        assert_eq!(
            report.zipcode_counts.get("12345"),
            Some(&1),
            "zipcode 12345 should have 1 user"
        );
        assert_eq!(
            report.zipcode_counts.get("67890"),
            Some(&1),
            "zipcode 67890 should have 1 user"
        );
        assert_eq!(
            report.zipcode_counts.get("11111"),
            Some(&1),
            "zipcode 11111 should have 1 user"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn demographic_report_should_exclude_non_consented_users(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{model_test_helpers::setup_default_app_and_session, user_participation};

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

        // Create consented user
        let consented_user = crate::models::users::create_user(
            &SignupRequest {
                username: "consented".to_string(),
                password: "pw".to_string(),
                email: "consented@example.com".to_string(),
                avatar_url: None,
                ..Default::default()
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
                ..Default::default()
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
        assert_eq!(
            report.total_participants, 2,
            "incorrect total participants"
        );

        // Only consented user should appear in demographics
        assert_eq!(report.ethnicity.len(), 1, "should only have one ethnicity");
        assert_eq!(
            report.ethnicity[0].value,
            Some("Asian".to_string()),
            "should only show consented user's ethnicity"
        );
        assert_eq!(
            report.ethnicity[0].count, 1,
            "should only count consented user"
        );

        assert_eq!(report.gender.len(), 1, "should only have one gender");
        assert_eq!(
            report.gender[0].value,
            Some("Female".to_string()),
            "should only show consented user's gender"
        );

        // Verify zipcode counts only include consented user
        assert_eq!(
            report.zipcode_counts.len(),
            1,
            "should only have one zipcode"
        );
        assert_eq!(
            report.zipcode_counts.get("12345"),
            Some(&1),
            "should only show consented user's zipcode"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn demographic_report_should_handle_null_values(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        use crate::models::{model_test_helpers::setup_default_app_and_session, user_participation};

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
                ..Default::default()
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

        // Should show None for null values
        assert_eq!(report.ethnicity.len(), 1, "should have one ethnicity entry");
        assert_eq!(
            report.ethnicity[0].value,
            None,
            "should show None for null ethnicity"
        );

        assert_eq!(report.age_ranges.len(), 1, "should have one age range entry");
        assert_eq!(
            report.age_ranges[0].value,
            None,
            "should show None for null age"
        );

        assert_eq!(report.gender.len(), 1, "should have one gender entry");
        assert_eq!(
            report.gender[0].value,
            None,
            "should show None for null gender"
        );

        // Verify zipcode counts (user has null zipcode, so map should be empty)
        assert_eq!(
            report.zipcode_counts.len(),
            0,
            "should have no zipcodes for user with null zipcode"
        );

        Ok(())
    }
}
