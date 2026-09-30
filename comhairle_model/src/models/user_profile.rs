use crate::models::error::ModelError;
use crate::models::{
    SqlxResultExt,
    demographics::{self, ValueBuckets},
};
use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, types::Json};
use std::collections::HashMap;
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
pub async fn create(db: &PgPool, profile: &CreateUserProfile) -> Result<UserProfile, ModelError> {
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

#[instrument(err(Debug), skip(db))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<UserProfile, ModelError> {
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
pub async fn get_by_user_id(db: &PgPool, user_id: &Uuid) -> Result<UserProfile, ModelError> {
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
) -> Result<UserProfile, ModelError> {
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
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<UserProfile, ModelError> {
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
) -> Result<Vec<demographics::DemographicsResponse>, ModelError> {
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
    pub display_name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfileExport {
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub demographics: Json<HashMap<String, UserProfileDemographicsExport>>,
}

#[instrument(err(Debug), skip(db))]
pub async fn get_demographics_for_export(
    db: &PgPool,
    conversation_id: &Uuid,
) -> Result<Vec<UserProfileExport>, ModelError> {
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
        INNER JOIN conversation_demographics cd ON cd.conversation_id = w.conversation_id
        INNER JOIN demographics_question dq ON dq.slug = cd.question_slug
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

#[instrument(err(Debug), skip(db))]
pub async fn get_demographic_report(
    db: &PgPool,
    workflow_id: &Uuid,
) -> Result<DemographicReport, ModelError> {
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
        display_name: Option<String>,
        bucket_config: Option<sqlx::types::Json<ValueBuckets>>,
        value: Option<String>,
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
            let bucket_value = row
                .value
                .as_deref()
                .map(|value| demographics::resolve_category_bucket(value, &bucket_config.0))
                .unwrap_or_default();
            let category_counts = categories
                .entry(row.category_name.clone())
                .or_insert_with(Vec::new);

            if let Some(category_count) =
                category_counts.iter_mut().find(|c| c.value == bucket_value)
            {
                category_count.count += row.count;
            } else {
                category_counts.push(DemographicCount {
                    display_name: row.display_name.unwrap_or(row.category_name.clone()),
                    value: bucket_value,
                    count: row.count,
                });
            }
        } else {
            categories
                .entry(row.category_name.clone())
                .or_insert_with(Vec::new)
                .push(DemographicCount {
                    display_name: row.display_name.unwrap_or(row.category_name.clone()),
                    value: row.value.unwrap_or_default(),
                    count: row.count,
                });
        };
    }

    Ok(DemographicReport {
        total_participants,
        categories,
    })
}
