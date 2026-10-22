use std::sync::Arc;

use apalis::prelude::Data;
use language_tags::LanguageTag;
use sensemakar::statement_theme_assigner::StatementThemeAssigner;
use sensemakar::theme_extraction::ThemeExtractor;
use sensemakar_types::{Statement as SensemakarStatement, Theme as SensemakarTheme};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;

use super::error::{RecordWorkerError, Result, WorkerServiceError};
use crate::models::job::{self, UpdateJob};
use crate::models::statement::Statement;
use crate::models::theme::NewTheme;
use crate::models::{self};
use crate::{ComhairleState, sensemakar_service};

/// Queued after an admin requests theme generation for a workflow step. The
/// statements have already been harvested from the source tool and persisted;
/// this job only runs the sensemaker model against them and stores the result.
#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeExtractionJob {
    pub job_id: Uuid,
    pub workflow_step_id: Uuid,
    pub theme_collection_id: Uuid,
    pub statement_ids: Vec<Uuid>,
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
}

fn to_sensemakar_statement(statement: &Statement) -> SensemakarStatement {
    SensemakarStatement {
        id: statement.id.to_string(),
        speaker_id: statement.user_id.map(|id| id.to_string()),
        statement_type: None,
        text: statement.content.clone(),
        #[expect(clippy::expect_used, reason = "'en' is a valid language tag")]
        lang: LanguageTag::parse("en").expect("'en' is a valid language tag"),
    }
}

fn to_sensemakar_theme(theme: &models::theme::Theme) -> SensemakarTheme {
    SensemakarTheme {
        id: theme.id.to_string(),
        name: theme.title.clone(),
        description: theme.description.clone(),
    }
}

#[instrument(
    err(Debug),
    skip(job, state),
    fields(job_id = %job.job_id, workflow_step_id = %job.workflow_step_id, theme_collection_id = %job.theme_collection_id)
)]
pub async fn theme_extraction_handler(
    job: ThemeExtractionJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<()> {
    info!("Starting theme extraction job");

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

    let model_config = sensemakar_service::build_model_config(model_cfg)
        .map_err(WorkerServiceError::ExternalServiceFailure)
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;
    let model = sensemakar::model_config::build_model(&model_config);

    let mut statements = Vec::with_capacity(job.statement_ids.len());
    for statement_id in &job.statement_ids {
        let statement = models::statement::get_by_id(&state.db, *statement_id)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))
            .ok_or_record_failure(&job.job_id, &state.db)
            .await?;
        statements.push(statement);
    }

    let sensemakar_statements: Vec<SensemakarStatement> =
        statements.iter().map(to_sensemakar_statement).collect();

    // Clear out last run's unpinned AI themes so regenerating replaces them instead
    // of piling new ones on top; pinned and manually created themes are untouched.
    models::theme::delete_stale_generated_themes(&state.db, job.theme_collection_id)
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    // Themes an organizer pinned (or added manually) are fed to the extractor as
    // "existing themes" so the model skips statements that already fit one of them,
    // rather than inventing a duplicate. The model only sees their name/description
    // (never the id), so these rows are reused as-is for assignment below, with no
    // id reconciliation needed against whatever the model invents for new themes.
    let carry_forward_themes =
        models::theme::list_carry_forward_themes(&state.db, job.theme_collection_id)
            .await
            .map_err(|e| WorkerServiceError::DbError(e.to_string()))
            .ok_or_record_failure(&job.job_id, &state.db)
            .await?;
    let carry_forward_sensemakar_themes: Vec<SensemakarTheme> = carry_forward_themes
        .iter()
        .map(to_sensemakar_theme)
        .collect();

    info!(
        statement_count = sensemakar_statements.len(),
        carry_forward_theme_count = carry_forward_themes.len(),
        "Running theme extraction against sensemaker model"
    );

    let extractor = ThemeExtractor::builder()
        .maybe_context(job.context.clone())
        .maybe_additional_instructions(job.additional_instructions.clone())
        .maybe_existing_themes(
            (!carry_forward_sensemakar_themes.is_empty())
                .then(|| carry_forward_sensemakar_themes.clone()),
        )
        .allow_additional_themes(true)
        .build();

    let extracted_themes: Vec<SensemakarTheme> = extractor
        .run_with_model(sensemakar_statements.clone(), model.clone())
        .await
        .map_err(|e| WorkerServiceError::ExternalServiceFailure(e.to_string()))
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

    // Every theme the extractor returns is new (it was told to skip statements that
    // already fit a carried-forward theme), so each one gets its own row with our
    // own id, rather than trying to make sense of whatever id the model invented.
    let mut assignment_themes = carry_forward_sensemakar_themes;
    for theme in &extracted_themes {
        let created = models::theme::add_theme(
            &state.db,
            job.workflow_step_id,
            job.theme_collection_id,
            &NewTheme {
                title: theme.name.clone(),
                description: theme.description.clone(),
                created_by_ai: true,
            },
        )
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))
        .ok_or_record_failure(&job.job_id, &state.db)
        .await?;

        assignment_themes.push(to_sensemakar_theme(&created));
    }

    if !assignment_themes.is_empty() {
        let assigner = StatementThemeAssigner::builder()
            .maybe_context(job.context.clone())
            .maybe_additional_instructions(job.additional_instructions.clone())
            .themes(assignment_themes.into())
            .build();

        let assignments = assigner
            .run_with_model(sensemakar_statements, model)
            .await
            .map_err(|e| WorkerServiceError::ExternalServiceFailure(e.to_string()))
            .ok_or_record_failure(&job.job_id, &state.db)
            .await?;

        for assignment in assignments.statement_assignments {
            // Every theme id handed to the assigner is one of our own UUIDs, so a
            // parse failure here just means the model assigned its literal
            // "unassigned" sentinel (or something unparsable) - leave that
            // statement's theme_id untouched rather than treating it as an error.
            let Ok(theme_id) = Uuid::parse_str(&assignment.theme_id) else {
                continue;
            };
            let Ok(statement_id) = Uuid::parse_str(&assignment.statement_id) else {
                continue;
            };

            models::statement::assign_theme(&state.db, statement_id, Some(theme_id))
                .await
                .map_err(|e| WorkerServiceError::DbError(e.to_string()))
                .ok_or_record_failure(&job.job_id, &state.db)
                .await?;
        }
    }

    job::complete(&state.db, job.job_id, "Themes generated")
        .await
        .map_err(|e| WorkerServiceError::DbError(e.to_string()))?;

    info!("Theme extraction job completed");

    Ok(())
}
