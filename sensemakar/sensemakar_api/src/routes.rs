use std::sync::Arc;

use axum::{Json, extract::State};
use rig::providers::ollama;
use sensemakar::statement_theme_assigner::StatementThemeAssigner;
use sensemakar::theme_extraction::ThemeExtractor;
use sensemakar::wikipoll_describer::WikiPollGroupDescriber;
use sensemakar_types::{
    Statement, Theme, ThemeAssignmentResult, WikiPollData, WikiPollReportResult,
};
use serde::Deserialize;

use crate::error::ApiError;

pub type AppState = Arc<ollama::CompletionModel>;

pub async fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
pub struct PolisReportRequest {
    pub title: String,
    pub statements: Vec<sensemakar_types::WikiPollStatement>,
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
}

pub async fn polis_report(
    State(model): State<AppState>,
    Json(request): Json<PolisReportRequest>,
) -> Result<Json<WikiPollReportResult>, ApiError> {
    let poll_data = WikiPollData {
        title: request.title,
        statements: request.statements,
    };

    let describer = WikiPollGroupDescriber {
        context: request.context,
        additional_instructions: request.additional_instructions,
    };

    let result = describer
        .run_with_model(&poll_data, model.as_ref().clone())
        .await?;

    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct GenerateThemesRequest {
    pub statements: Vec<Statement>,
    #[serde(default)]
    pub existing_themes: Option<Vec<Theme>>,
    #[serde(default = "default_true")]
    pub allow_additional_themes: bool,
    #[serde(default)]
    pub min_themes: Option<u32>,
    #[serde(default)]
    pub max_themes: Option<u32>,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub additional_instructions: Option<String>,
    /// Process statements in batches of this size, folding themes found in
    /// each batch into the next. Omit to run all statements in a single pass.
    #[serde(default)]
    pub batch_size: Option<usize>,
}

fn default_true() -> bool {
    true
}

pub async fn generate_themes(
    State(model): State<AppState>,
    Json(request): Json<GenerateThemesRequest>,
) -> Result<Json<Vec<Theme>>, ApiError> {
    let mut extractor = ThemeExtractor::builder()
        .maybe_existing_themes(request.existing_themes)
        .allow_additional_themes(request.allow_additional_themes)
        .maybe_context(request.context)
        .maybe_additional_instructions(request.additional_instructions)
        .maybe_max_themes(request.max_themes)
        .maybe_min_themes(request.min_themes)
        .build();

    let themes = match request.batch_size {
        Some(batch_size) => {
            extractor
                .run_in_batches_with_model(request.statements, model.as_ref().clone(), batch_size)
                .await?
        }
        None => {
            extractor
                .run_with_model(request.statements, model.as_ref().clone())
                .await?
        }
    };

    Ok(Json(themes))
}

#[derive(Debug, Deserialize)]
pub struct AssignThemesRequest {
    pub statements: Vec<Statement>,
    pub themes: Vec<Theme>,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub additional_instructions: Option<String>,
}

pub async fn assign_themes(
    State(model): State<AppState>,
    Json(request): Json<AssignThemesRequest>,
) -> Result<Json<ThemeAssignmentResult>, ApiError> {
    let assigner = StatementThemeAssigner::builder()
        .maybe_context(request.context)
        .maybe_additional_instructions(request.additional_instructions)
        .themes(request.themes.into())
        .build();

    let result = assigner
        .run_with_model(request.statements, model.as_ref().clone())
        .await?;

    Ok(Json(result))
}
