use axum::{Json, http::StatusCode, response::IntoResponse};
use sensemakar::{
    statement_theme_assigner::StatementThemeAssignerError, theme_extraction::ThemeExtractorError,
    wikipoll_describer::WikiPollGroupDescriberError,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("polis report failed: {0}")]
    PolisReport(#[from] WikiPollGroupDescriberError),

    #[error("theme extraction failed: {0}")]
    ThemeExtraction(#[from] ThemeExtractorError),

    #[error("theme assignment failed: {0}")]
    ThemeAssignment(#[from] StatementThemeAssignerError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = %self, "request failed");
        let body = Json(json!({ "error": self.to_string() }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
