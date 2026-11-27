use std::sync::Arc;

use apalis::prelude::Data;
use serde::{Deserialize, Serialize};

use super::error::{Result, WorkerServiceError};
use crate::ComhairleState;
use crate::models::region_area_pipeline::lifecycle;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RegionAreaPipelineTick;

pub async fn sync_due_region_area_pipelines(
    _tick: RegionAreaPipelineTick,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    let pipeline_ids = lifecycle::due_pipeline_ids(&state.db)
        .await
        .map_err(|error| WorkerServiceError::DbError(error.to_string()))?;
    let mut first_error = None;
    for pipeline_id in pipeline_ids {
        if let Err(error) = lifecycle::run_due(
            &state.db,
            pipeline_id,
            state.bulk_storage_service.as_deref(),
        )
        .await
        {
            tracing::error!(%pipeline_id, %error, "Region Area Pipeline run failed");
            if first_error.is_none() {
                first_error = Some(error.to_string());
            }
        }
    }
    if let Some(error) = first_error {
        return Err(WorkerServiceError::JobFailure(error));
    }
    Ok(())
}
