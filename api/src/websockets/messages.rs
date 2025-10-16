use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WebSocketMessage {
    #[serde(rename = "ping")]
    Ping { timestamp: u64 },

    #[serde(rename = "pong")]
    Pong { timestamp: u64 },

    #[serde(rename = "notification")]
    Notification {
        title: String,
        message: String,
        level: NotificationLevel,
    },

    #[serde(rename = "user_started_workflow_step")]
    UserStartedWorkflowStep { workflow_step_id: Uuid },

    #[serde(rename = "user_finished_workflow_step")]
    UserFinishedWorkflowStep { workflow_step_id: Uuid },

    #[serde(rename = "user_idle")]
    UserIdle { workflow_step_id: Uuid },

    #[serde(rename = "user_joined")]
    UserJoined {
        user_id: Uuid,
        username: Option<String>,
    },

    #[serde(rename = "user_left")]
    UserLeft {
        user_id: Uuid,
        username: Option<String>,
    },

    #[serde(rename = "broadcast")]
    Broadcast {
        message: String,
        from_user: Option<Uuid>,
    },

    #[serde(rename = "error")]
    Error { code: String, message: String },

    #[serde(rename = "custom")]
    Custom {
        event: String,
        data: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}
