use std::sync::Arc;

use apalis::prelude::Data;
use sensemakar::{model_config, wikipoll_describer::WikiPollGroupDescriber};
use sensemakar_jobs::WikiPollSummaryJob;
use tracing::{info, instrument};

use crate::{
    ComhairleState,
    models::{
        self,
        job::{self, UpdateJob},
        wikipoll_conversation_summary::CreateWikiPollConversationSummary,
    },
};

use super::error::{RecordWorkerError, Result, WorkerServiceError};

/// Runs the Polis "describer" analysis for a workflow step and stores the
/// result. The `poll_data` sent to the model is stored alongside the result so
/// the summary row is reproducible after the fact.
#[instrument(
    err(Debug),
    skip(job, state),
    fields(job_id = %job.job_id, workflow_step_id = %job.workflow_step_id)
)]
pub async fn wiki_poll_summary_handler(
    job: WikiPollSummaryJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Starting Polis summary job");

    let update_job = UpdateJob {
        status: Some("running".to_string()),
        ..Default::default()
    };
    let _ = job::update(&state.db, &job.job_id, update_job)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    let model_cfg = state
        .config
        .sensemaker_model
        .as_ref()
        .ok_or(WorkerServiceError::NoSensemakerModelConfigured)
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    let model = model_config::build_model(&model_config::ModelConfig {
        base_url: model_cfg.base_url.clone(),
        api_key: model_cfg.api_key.clone(),
        model: model_cfg.model.clone(),
    });

    let describer = WikiPollGroupDescriber {
        context: job.context.clone(),
        additional_instructions: job.additional_instructions.clone(),
    };

    let poll_data_value = serde_json::to_value(&job.poll_data)
        .map_err(WorkerServiceError::SerdeJsonError)
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    info!(
        statement_count = job.poll_data.statements.len(),
        "Running Polis describer against sensemaker model"
    );

    let result = describer
        .run_with_model(&job.poll_data, model)
        .await
        .map_err(|e| WorkerServiceError::ExternalServiceFailure(e.to_string()))
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    let result_value = serde_json::to_value(&result)
        .map_err(WorkerServiceError::SerdeJsonError)
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    models::wikipoll_conversation_summary::create(
        &state.db,
        &CreateWikiPollConversationSummary {
            workflow_step_id: job.workflow_step_id,
            job_id: Some(job.job_id),
            poll_data: poll_data_value,
            result: result_value,
        },
    )
    .await
    .map_err(|e| WorkerServiceError::DbError(e.to_string()))
    .ok_or_record_failure(&job.job_id, &state.db)
    .await?;

    job::complete(&state.db, job.job_id, "Polis summary generated")
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    info!("Polis summary job completed");

    Ok(())
}
