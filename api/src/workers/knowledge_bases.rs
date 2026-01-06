use std::sync::Arc;

use apalis::prelude::Data;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ComhairleError, ComhairleState};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KnowledgeBaseJob {
    job_id: Uuid,
    conversation_id: Uuid,
    knowledge_base_id: String,
    chat_bot_id: String,
}

pub async fn handle_knowledge_base_processing(
    job: KnowledgeBaseJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    todo!();
}
