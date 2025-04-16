use std::collections::HashMap;

use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, Order, OrderedStatement, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tracing::info;
use uuid::Uuid;

use crate::error::ComhairleError;

use super::{
    user_participation::{UserParticipation, UserParticipationIden},
    user_progress::UserProgressIden,
    workflow_step::WorkflowStepIden,
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "workflow")]
#[partially(derive(Deserialize, Debug, JsonSchema))]
pub struct Workflow {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub conversation_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
    #[partially(omit)]
    pub owner_id: Uuid,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [WorkflowIden; 9] = [
    WorkflowIden::Id,
    WorkflowIden::ConversationId,
    WorkflowIden::Name,
    WorkflowIden::Description,
    WorkflowIden::IsPublic,
    WorkflowIden::IsActive,
    WorkflowIden::CreatedAt,
    WorkflowIden::UpdatedAt,
    WorkflowIden::OwnerId,
];

impl PartialWorkflow {
    pub fn to_values(&self) -> Vec<(WorkflowIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.name {
            values.push((WorkflowIden::Name, value.into()))
        };
        if let Some(value) = &self.description {
            values.push((WorkflowIden::Description, value.into()))
        };

        if let Some(value) = self.is_public {
            values.push((WorkflowIden::Description, value.into()))
        };
        if let Some(value) = self.is_active {
            values.push((WorkflowIden::Description, value.into()))
        };
        values
    }
}

/// Get a conversation by ID
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<Workflow, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let conversation = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Workflow".into()))?;

    Ok(conversation)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CreateWorkflow {
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub is_public: bool,
}

impl CreateWorkflow {
    pub fn columns(&self) -> Vec<WorkflowIden> {
        vec![
            WorkflowIden::Name,
            WorkflowIden::Description,
            WorkflowIden::IsActive,
            WorkflowIden::IsPublic,
        ]
    }
    pub fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.name.to_owned().into(),
            self.description.to_owned().into(),
            self.is_active.into(),
            self.is_public.into(),
        ]
    }
}

// TODO ensure this deletes all workflow steps on deletion
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Workflow, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(WorkflowIden::Table)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::ResourceNotFound("Conversation".into()))?;

    Ok(workflow)
}

pub async fn update(
    db: &PgPool,
    id: Uuid,
    update: &PartialWorkflow,
) -> Result<Workflow, ComhairleError> {
    info!("Updating workflow {id} with update {update:#?}");
    let values = update.to_values();

    if values.len() == 0 {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(WorkflowIden::Table)
        .values(values)
        .and_where(Expr::col(WorkflowIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(workflow)
}

pub async fn list(db: &PgPool, conversation_id: Uuid) -> Result<Vec<Workflow>, ComhairleError> {
    let query = Query::select()
        .from(WorkflowIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(WorkflowIden::ConversationId).eq(conversation_id))
        .to_owned();

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let workflows = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_all(db)
        .await?;

    Ok(workflows)
}

#[derive(Serialize, Deserialize, JsonSchema, FromRow)]
pub struct WorkflowStats {
    pub total_users: i32,
    pub users_completed_step: HashMap<Uuid, i32>,
}

/// Calculate stastistics for the workflow
pub async fn stats(db: &PgPool, workflow_id: Uuid) -> Result<WorkflowStats, ComhairleError> {
    let (sql, values) = Query::select()
        .from(WorkflowStepIden::Table)
        .column((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .expr(Expr::cust(
            "COUNT(CASE WHEN user_progress.status = 'done' THEN 1 END)::INT as count",
        ))
        .join(
            sea_query::JoinType::LeftJoin,
            UserProgressIden::Table,
            Expr::col((UserProgressIden::Table, UserProgressIden::WorkflowStepId))
                .equals((WorkflowStepIden::Table, WorkflowStepIden::Id)),
        )
        .and_where(
            Expr::col((WorkflowStepIden::Table, WorkflowStepIden::WorkflowId)).eq(workflow_id),
        )
        .group_by_col((WorkflowStepIden::Table, WorkflowStepIden::Id))
        .group_by_col(WorkflowStepIden::StepOrder)
        .order_by(
            (WorkflowStepIden::Table, WorkflowStepIden::StepOrder),
            Order::Asc,
        )
        .to_owned()
        .build_sqlx(PostgresQueryBuilder);

    let step_stats = sqlx::query_as_with::<_, (Uuid, i32), _>(&sql, values)
        .fetch_all(db)
        .await?;

    let step_stats: HashMap<Uuid, i32> = step_stats.into_iter().collect();

    let (sql, values) = Query::select()
        .from(UserParticipationIden::Table)
        .expr(Expr::cust("COUNT(*)::INT as count"))
        .and_where(Expr::col(UserParticipationIden::WorkflowId).eq(workflow_id))
        .to_owned()
        .build_sqlx(PostgresQueryBuilder);

    let total_users: i32 = sqlx::query_scalar_with(&sql, values).fetch_one(db).await?;

    Ok(WorkflowStats {
        users_completed_step: step_stats,
        total_users,
    })
}

pub async fn create(
    db: &PgPool,
    workflow: &CreateWorkflow,
    conversation_id: Uuid,
    owner_id: Uuid,
) -> Result<Workflow, ComhairleError> {
    let mut columns = workflow.columns();
    let mut values = workflow.values();

    columns.push(WorkflowIden::ConversationId);
    values.push(conversation_id.into());

    columns.push(WorkflowIden::OwnerId);
    values.push(owner_id.into());

    let (sql, values) = Query::insert()
        .into_table(WorkflowIden::Table)
        .columns(columns)
        .values(values)
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let workflow = sqlx::query_as_with::<_, Workflow, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(workflow)
}
