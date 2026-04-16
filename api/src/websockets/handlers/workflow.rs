use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

use crate::{
    error::ComhairleError,
    websockets::{messages::WebSocketMessage, WebSocketConnection, WebSocketMessageHandler},
    ComhairleState,
};

/// Handler for workflow-related WebSocket messages.
///
/// This handler tracks user progress through workflows in real-time, including:
/// - When users start/finish workflow steps
/// - When users become idle on a step
/// - Custom workflow events via the `workflow:` event prefix
///
/// # Message Types
///
/// ## Handled Message Types
/// - `UserStartedWorkflowStep { workflow_step_id }` - User began a workflow step
/// - `UserFinishedWorkflowStep { workflow_step_id }` - User completed a workflow step
/// - `UserIdle { workflow_step_id }` - User is idle/inactive on a workflow step
/// - `Custom { event: "workflow:*", ... }` - Custom workflow events
///
/// # Use Cases
///
/// - Track user progress through onboarding flows
/// - Monitor activity on collaborative workflows
/// - Trigger actions when users complete steps
/// - Send reminders when users are idle
/// - Broadcast step completion to other participants
///
/// # Example
///
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use uuid::Uuid;
/// # use comhairle::ComhairleState;
/// use comhairle::websockets::messages::WebSocketMessage;
///
/// # async fn example(state: Arc<ComhairleState>, step_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
/// // In your workflow step component/handler:
/// let message = WebSocketMessage::UserStartedWorkflowStep {
///     workflow_step_id: step_id,
/// };
/// state.websockets.broadcast_to_all(&message).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Database Integration
///
/// This handler can be extended to:
/// - Record step start/completion times in `user_workflow_progress` table
/// - Update workflow completion status
/// - Track analytics/metrics
/// - Trigger workflow-specific business logic
///
/// # Registration
///
/// This handler is automatically registered in `websockets::setup::register_handlers()`.
pub struct WorkflowMessageHandler;

impl WorkflowMessageHandler {
    /// Create a new workflow message handler.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl WebSocketMessageHandler for WorkflowMessageHandler {
    fn domain(&self) -> &str {
        "workflow"
    }

    async fn handle_message(
        &self,
        message: &WebSocketMessage,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        match message {
            WebSocketMessage::UserStartedWorkflowStep { workflow_step_id } => {
                self.handle_workflow_step_started(workflow_step_id, connection, state)
                    .await
            }
            WebSocketMessage::UserFinishedWorkflowStep { workflow_step_id } => {
                self.handle_workflow_step_finished(workflow_step_id, connection, state)
                    .await
            }
            WebSocketMessage::UserIdle { workflow_step_id } => {
                self.handle_user_idle(workflow_step_id, connection, state)
                    .await
            }
            WebSocketMessage::Custom { event, data } if event.starts_with("workflow:") => {
                self.handle_custom_workflow_event(event, data, connection, state)
                    .await
            }
            _ => Ok(()),
        }
    }
}

impl WorkflowMessageHandler {
    async fn handle_workflow_step_started(
        &self,
        workflow_step_id: &uuid::Uuid,
        connection: &WebSocketConnection,
        state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        info!(
            "User {} started workflow step {}",
            connection.user.id, workflow_step_id
        );

        // Example: Query database to get workflow step details
        // let _step = sqlx::query!(
        //     "SELECT id, workflow_id FROM workflow_steps WHERE id = $1",
        //     workflow_step_id
        // )
        // .fetch_optional(&state.db)
        // .await?;

        // Example: Update user progress in database
        // sqlx::query!(
        //     "INSERT INTO user_workflow_progress (user_id, workflow_step_id, started_at)
        //      VALUES ($1, $2, NOW())
        //      ON CONFLICT (user_id, workflow_step_id) DO UPDATE SET started_at = NOW()",
        //     connection.user.id,
        //     workflow_step_id
        // )
        // .execute(&state.db)
        // .await?;

        // Example: Broadcast to other users in the same workflow
        // let broadcast_msg = WebSocketMessage::Custom {
        //     event: "workflow:user_joined_step".to_string(),
        //     data: serde_json::json!({
        //         "user_id": connection.user.id,
        //         "username": connection.user.username,
        //         "workflow_step_id": workflow_step_id,
        //     }),
        // };
        // state.websockets.broadcast_to_all(&broadcast_msg).await?;

        Ok(())
    }

    async fn handle_workflow_step_finished(
        &self,
        workflow_step_id: &uuid::Uuid,
        connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        info!(
            "User {} finished workflow step {}",
            connection.user.id, workflow_step_id
        );

        // Example: Update completion status in database
        // sqlx::query!(
        //     "UPDATE user_workflow_progress
        //      SET completed_at = NOW()
        //      WHERE user_id = $1 AND workflow_step_id = $2",
        //     connection.user.id,
        //     workflow_step_id
        // )
        // .execute(&state.db)
        // .await?;

        // Example: Send acknowledgment back to user
        let response = WebSocketMessage::Custom {
            event: "workflow:step_completed".to_string(),
            data: serde_json::json!({
                "workflow_step_id": workflow_step_id,
                "completed": true,
            }),
        };
        connection.send_message(&response).await?;

        Ok(())
    }

    async fn handle_user_idle(
        &self,
        workflow_step_id: &uuid::Uuid,
        connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        info!(
            "User {} is idle on workflow step {}",
            connection.user.id, workflow_step_id
        );

        // Example: Track idle time for analytics
        // You could store this in database or send to analytics service

        Ok(())
    }

    async fn handle_custom_workflow_event(
        &self,
        event: &str,
        data: &serde_json::Value,
        connection: &WebSocketConnection,
        _state: &Arc<ComhairleState>,
    ) -> Result<(), ComhairleError> {
        info!(
            "Custom workflow event '{}' from user {}: {:?}",
            event, connection.user.id, data
        );

        // Handle custom workflow events here
        // Example: "workflow:save_progress", "workflow:request_help", etc.

        Ok(())
    }
}
