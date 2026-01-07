pub mod knowledge_bases;

use std::sync::Arc;

use apalis::prelude::MemoryStorage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::workers::knowledge_bases::KnowledgeBaseJob;

#[derive(Clone, Debug)]
pub struct JobQueues {
    pub knowledge_bases: Arc<Mutex<MemoryStorage<KnowledgeBaseJob>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JobMetadata {
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub completion_message: Option<String>,
    pub status: String,
}

impl Default for JobMetadata {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            finished_at: None,
            error: None,
            completion_message: None,
            status: "pending".to_string(),
        }
    }
}
