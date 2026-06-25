use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, OnConflict, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "recruitment_target")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct RecruitmentTarget {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub workflow_id: Uuid,
    pub metric: String,
    pub bucket: String,
    pub target_count: i32,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateRecruitmentTarget {
    pub metric: String,
    pub bucket: String,
    pub target_count: i32,
}

const DEFAULT_COLUMNS: [RecruitmentTargetIden; 7] = [
    RecruitmentTargetIden::Id,
    RecruitmentTargetIden::WorkflowId,
    RecruitmentTargetIden::Metric,
    RecruitmentTargetIden::Bucket,
    RecruitmentTargetIden::TargetCount,
    RecruitmentTargetIden::CreatedAt,
    RecruitmentTargetIden::UpdatedAt,
];

pub async fn create(
    db: &PgPool,
    workflow_id: &Uuid,
    create_request: &CreateRecruitmentTarget,
) -> Result<RecruitmentTarget, ComhairleError> {
    let (sql, values) = Query::insert()
        .into_table(RecruitmentTargetIden::Table)
        .columns([
            RecruitmentTargetIden::WorkflowId,
            RecruitmentTargetIden::Metric,
            RecruitmentTargetIden::Bucket,
            RecruitmentTargetIden::TargetCount,
        ])
        .values([
            (*workflow_id).into(),
            create_request.metric.clone().into(),
            create_request.bucket.clone().into(),
            create_request.target_count.into(),
        ])
        .unwrap()
        .on_conflict(
            OnConflict::columns([
                RecruitmentTargetIden::WorkflowId,
                RecruitmentTargetIden::Metric,
                RecruitmentTargetIden::Bucket,
            ])
            .update_columns([
                RecruitmentTargetIden::TargetCount,
                RecruitmentTargetIden::UpdatedAt,
            ])
            .to_owned(),
        )
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(target)
}

pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<RecruitmentTarget, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("RecruitmentTarget".into()))?;

    Ok(target)
}

pub async fn list_for_workflow(
    db: &PgPool,
    workflow_id: &Uuid,
) -> Result<Vec<RecruitmentTarget>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::WorkflowId).eq(*workflow_id))
        .order_by(RecruitmentTargetIden::Metric, sea_query::Order::Asc)
        .order_by(RecruitmentTargetIden::Bucket, sea_query::Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let targets = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(targets)
}

pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_request: &PartialRecruitmentTarget,
) -> Result<RecruitmentTarget, ComhairleError> {
    let mut query = Query::update()
        .table(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .to_owned();

    let mut has_updates = false;

    if let Some(value) = &update_request.metric {
        query = query
            .value(RecruitmentTargetIden::Metric, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = &update_request.bucket {
        query = query
            .value(RecruitmentTargetIden::Bucket, value.clone())
            .to_owned();
        has_updates = true;
    }
    if let Some(value) = update_request.target_count {
        query = query
            .value(RecruitmentTargetIden::TargetCount, value)
            .to_owned();
        has_updates = true;
    }

    if !has_updates {
        return get_by_id(db, id).await;
    }

    query = query
        .value(RecruitmentTargetIden::UpdatedAt, Utc::now())
        .to_owned();

    let (sql, values) = query
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("RecruitmentTarget".into()))?;

    Ok(target)
}

pub async fn delete(db: &PgPool, id: &Uuid) -> Result<RecruitmentTarget, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(RecruitmentTargetIden::Table)
        .and_where(Expr::col(RecruitmentTargetIden::Id).eq(*id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let target = sqlx::query_as_with::<_, RecruitmentTarget, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("RecruitmentTarget".into()))?;

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::model_test_helpers::setup_default_app_and_session;
    use crate::routes::{conversations::dto::ConversationDto, workflows::dto::WorkflowDto};
    use sqlx::PgPool;
    use std::error::Error;

    async fn make_workflow(pool: &PgPool) -> Result<Uuid, Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(pool).await?;
        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(conversation)?;
        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation.id.to_string())
            .await?;
        let workflow: WorkflowDto = serde_json::from_value(workflow)?;
        Ok(workflow.id)
    }

    #[sqlx::test]
    async fn should_create_a_recruitment_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 30,
            },
        )
        .await?;

        assert_eq!(target.workflow_id, workflow_id);
        assert_eq!(target.metric, "age");
        assert_eq!(target.bucket, "18-24");
        assert_eq!(target.target_count, 30);

        Ok(())
    }

    #[sqlx::test]
    async fn should_upsert_when_same_metric_bucket(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        let first = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 30,
            },
        )
        .await?;

        let second = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 45,
            },
        )
        .await?;

        assert_eq!(first.id, second.id, "should be the same row");
        assert_eq!(second.target_count, 45, "target count should be updated");

        let all = list_for_workflow(&pool, &workflow_id).await?;
        assert_eq!(all.len(), 1, "should only have one row");

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_targets_for_workflow(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;

        create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;
        create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "gender".into(),
                bucket: "female".into(),
                target_count: 50,
            },
        )
        .await?;

        let targets = list_for_workflow(&pool, &workflow_id).await?;

        assert_eq!(targets.len(), 2);
        Ok(())
    }

    #[sqlx::test]
    async fn should_update_a_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        let updated = update(
            &pool,
            &target.id,
            &PartialRecruitmentTarget {
                target_count: Some(20),
                ..PartialRecruitmentTarget::default()
            },
        )
        .await?;

        assert_eq!(updated.target_count, 20);
        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_a_target(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        delete(&pool, &target.id).await?;

        let result = get_by_id(&pool, &target.id).await;
        assert!(result.is_err(), "target should be gone");
        Ok(())
    }

    #[sqlx::test]
    async fn should_cascade_delete_when_workflow_deleted(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let workflow_id = make_workflow(&pool).await?;
        let target = create(
            &pool,
            &workflow_id,
            &CreateRecruitmentTarget {
                metric: "age".into(),
                bucket: "18-24".into(),
                target_count: 10,
            },
        )
        .await?;

        crate::models::workflow::delete(&pool, &workflow_id).await?;

        let result = get_by_id(&pool, &target.id).await;
        assert!(result.is_err(), "target should cascade-delete with workflow");
        Ok(())
    }
}
