use std::sync::Arc;

use apalis::prelude::{Data, MemoryStorage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ComhairleError, ComhairleState};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessKnowledgeBaseJob {
    conversation_id: Uuid,
    knowledge_base_id: String,
    chat_bot_id: String,
}

#[derive(Clone, Debug)]
pub struct JobQueue {
    knowledge_base: MemoryStorage<ProcessKnowledgeBaseJob>,
}

pub async fn handle_knowledge_base_processing(
    job: ProcessKnowledgeBaseJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    todo!();
}
