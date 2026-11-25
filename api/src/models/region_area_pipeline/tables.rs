use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Condition, Expr, JoinType, Query, SelectStatement, enum_def};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::PipelineDefinition;
use super::reconcile::ReconciliationCounts;
use crate::models::region::RegionIden;
use crate::models::region_area::RegionAreaIden;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    sqlx::Type,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PipelineStatus {
    Draft,
    Active,
    Staged,
    Deactivated,
}

impl PipelineStatus {
    pub fn is_active(self) -> bool {
        matches!(self, Self::Active | Self::Staged)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    sqlx::Type,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RunStatus {
    Unscheduled,
    Waiting,
    Running,
    RetryPending,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    sqlx::Type,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeatureStatus {
    Live,
    Retired,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    sqlx::Type,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SnapshotKind {
    Preview,
    Successful,
}

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[enum_def(table_name = "region_area_pipeline")]
pub struct Pipeline {
    pub id: Uuid,
    #[sqlx(json)]
    pub draft_definition: PipelineDefinition,
    #[sqlx(json(nullable))]
    pub active_definition: Option<PipelineDefinition>,
    pub status: PipelineStatus,
    pub generation: i64,
    pub run_status: RunStatus,
    pub next_run_at: Option<DateTime<Utc>>,
    pub lease_id: Option<Uuid>,
    pub lease_expires_at: Option<DateTime<Utc>>,
    pub run_started_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[enum_def(table_name = "region_area_pipeline_feature")]
pub struct Feature {
    pub pipeline_id: Uuid,
    pub external_id: String,
    pub region_area_id: Uuid,
    pub region_id: Uuid,
    pub managed_tags: Vec<String>,
    pub manual_tags: Vec<String>,
    pub status: FeatureStatus,
    pub retired_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[enum_def(table_name = "region_area_pipeline_snapshot")]
pub struct Snapshot {
    pub pipeline_id: Uuid,
    pub kind: SnapshotKind,
    #[sqlx(json)]
    pub definition: PipelineDefinition,
    pub snapshot: Vec<u8>,
    pub preview_generation: Option<i64>,
    pub state_hash: Option<String>,
    #[sqlx(json)]
    pub summary: ReconciliationCounts,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, JsonSchema)]
#[enum_def(table_name = "region_region_area")]
pub struct AreaLink {
    pub region_id: Uuid,
    pub region_area_id: Uuid,
}

impl Pipeline {
    pub fn select() -> SelectStatement {
        Query::select()
            .column(sea_query::Asterisk)
            .from(PipelineIden::Table)
            .to_owned()
    }

    pub fn active_condition() -> Condition {
        Condition::all().add(Expr::col(PipelineIden::Status).is_in([
            PipelineStatus::Active.as_ref(),
            PipelineStatus::Staged.as_ref(),
        ]))
    }

    pub fn due_condition(now: DateTime<Utc>) -> Condition {
        Self::active_condition()
            .add(Expr::col(PipelineIden::NextRunAt).lte(now))
            .add(
                Condition::any()
                    .add(Expr::col(PipelineIden::RunStatus).ne(RunStatus::Running.as_ref()))
                    .add(Expr::col(PipelineIden::LeaseExpiresAt).lte(now)),
            )
    }
}

impl Feature {
    pub fn owned_entities(pipeline_id: Uuid) -> SelectStatement {
        Query::select()
            .from(FeatureIden::Table)
            .join(
                JoinType::InnerJoin,
                RegionAreaIden::Table,
                Expr::col((RegionAreaIden::Table, RegionAreaIden::Id))
                    .equals((FeatureIden::Table, FeatureIden::RegionAreaId)),
            )
            .join(
                JoinType::InnerJoin,
                RegionIden::Table,
                Expr::col((RegionIden::Table, RegionIden::Id))
                    .equals((FeatureIden::Table, FeatureIden::RegionId)),
            )
            .and_where(Expr::col((FeatureIden::Table, FeatureIden::PipelineId)).eq(pipeline_id))
            .to_owned()
    }
}
