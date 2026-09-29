//! Failures about workflows, workflow steps, and their tool configuration.

use thiserror::Error;

use super::{DataError, DomainError, ErrorKind};

#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Workflow Step has wrong type expected {0}")]
    WorkflowStepHasWrongType(String),

    #[error("User is already participating in workflow: {0}")]
    UserAlreadyParticipatingInWorkflow(String),

    #[error("Tool config error: {0}")]
    ToolConfigError(String),

    #[error("Preview tool and live tool config dont match type")]
    ToolConfigMismatch,

    #[error("Failed to generate stats for Workflow {0}")]
    WorkflowStatsAggregationError(sqlx::Error),

    /// A persistence failure hit while serving a workflow request.
    #[error(transparent)]
    Data(#[from] DataError),
}

impl DomainError for WorkflowError {
    fn kind(&self) -> ErrorKind {
        match self {
            WorkflowError::Data(err) => err.kind(),
            WorkflowError::WorkflowStepHasWrongType(_) => ErrorKind::Unprocessable,
            WorkflowError::UserAlreadyParticipatingInWorkflow(_) => ErrorKind::Conflict,
            WorkflowError::ToolConfigError(_)
            | WorkflowError::ToolConfigMismatch
            | WorkflowError::WorkflowStatsAggregationError(_) => ErrorKind::Internal,
        }
    }
}
