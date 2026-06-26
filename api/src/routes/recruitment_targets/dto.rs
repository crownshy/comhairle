use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::recruitment_target::RecruitmentTarget;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecruitmentTargetDto {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub metric: String,
    pub bucket: String,
    pub target_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<RecruitmentTarget> for RecruitmentTargetDto {
    fn from(target: RecruitmentTarget) -> Self {
        Self {
            id: target.id,
            workflow_id: target.workflow_id,
            metric: target.metric,
            bucket: target.bucket,
            target_count: target.target_count,
            created_at: target.created_at,
            updated_at: target.updated_at,
        }
    }
}
