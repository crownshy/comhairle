use std::sync::Arc;

use apalis::prelude::Data;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::ComhairleError,
    workers::{JobMetadata, JobQueues},
    ComhairleState,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct KnowledgeBaseJob {
    pub job_id: Uuid,
    pub conversation_id: Uuid,
    pub knowledge_base_id: String,
    pub chat_bot_id: String,
    pub metadata: JobMetadata,
}

pub async fn handle_knowledge_base_processing(
    job: KnowledgeBaseJob,
    state: Data<Arc<ComhairleState>>,
) -> Result<(), ComhairleError> {
    println!();
    println!("    >>>>    Start job execution");
    println!();

    todo!();
}
