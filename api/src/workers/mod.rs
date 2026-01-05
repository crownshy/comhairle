pub mod knowledge_base_jobs;

use std::sync::Arc;

use apalis::prelude::{MemoryStorage, Monitor, WorkerBuilder, WorkerFactoryFn};

use crate::{
    workers::knowledge_base_jobs::{handle_knowledge_base_processing, ProcessKnowledgeBaseJob},
    ComhairleState,
};

#[derive(Clone, Debug)]
pub struct JobQueue {
    knowledge_base: MemoryStorage<ProcessKnowledgeBaseJob>,
}

pub async fn setup_workers(state: Arc<ComhairleState>) {
    let storage_knowledge_base: MemoryStorage<ProcessKnowledgeBaseJob> = MemoryStorage::new();
    let knowledge_base_worker = WorkerBuilder::new("process_knowledge_base_job")
        .data(state.clone())
        .backend(storage_knowledge_base.clone())
        .build_fn(handle_knowledge_base_processing);

    Monitor::new()
        .register(knowledge_base_worker)
        .run()
        .await
        .unwrap()
}

