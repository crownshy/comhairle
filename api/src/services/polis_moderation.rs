//! Polis moderation authorization.
//!
//! Deciding whether a user may moderate walks step to workflow to conversation and
//! then calls `authorize`, which needs the permission cache off [`ComhairleState`].

use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::models;
use crate::models::error::ValidationError;
use crate::models::users::User;
use crate::routes::auth::authorize;
use crate::{ComhairleState, error::ComhairleError};

#[instrument(err(Debug), skip(state))]
pub async fn check_can_moderate(
    state: &Arc<ComhairleState>,
    user: &User,
    workflow_step_id: &Uuid,
) -> Result<(), ComhairleError> {
    let workflow_step = models::workflow_step::get_by_id(&state.db, workflow_step_id).await?;

    let workflow = models::workflow::get_by_id(&state.db, &workflow_step.workflow_id).await?;
    let conversation_id = workflow.conversation_id.ok_or(ValidationError::BadRequest(
        "workflow is not attached to a conversation".into(),
    ))?;
    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;
    let conversation_resource = crate::authz::ConversationResource {
        conversation_id: conversation.id,
        owner_id: conversation.owner_id,
    };
    authorize(
        state,
        user,
        models::permissions::Action::ConversationUpdate,
        &conversation_resource,
    )
    .await?;
    Ok(())
}
