//! Workflow orchestration.
//!
//! Launching a workflow walks its steps and launches each one, which reaches into
//! the tool services. The persistence half stays in [`crate::models::workflow`].

use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::models::{conversation, workflow, workflow_step};
use crate::{ComhairleState, error::ComhairleError};

#[instrument(err(Debug), skip(state))]
pub async fn launch(state: &Arc<ComhairleState>, workflow_id: &Uuid) -> Result<(), ComhairleError> {
    let steps = workflow_step::list(&state.db, workflow_id).await?;
    for step in steps {
        crate::services::workflow_step::launch(state, &step.id).await?;
    }

    Ok(())
}

/// Take a conversation live by launching every workflow hanging off it.
#[instrument(err(Debug), skip(state))]
pub async fn launch_conversation(
    state: &Arc<ComhairleState>,
    conversation_id: Uuid,
) -> Result<conversation::Conversation, ComhairleError> {
    let db = &state.db;
    let workflows = workflow::list(db, conversation_id, None).await?;
    for wf in workflows {
        launch(state, &wf.id).await?;
    }

    conversation::update(
        db,
        &conversation_id,
        &conversation::PartialConversation {
            is_live: Some(true),
            ..Default::default()
        },
    )
    .await?;

    conversation::get_by_id(db, &conversation_id)
        .await
        .map_err(Into::into)
}
