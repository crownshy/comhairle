pub mod knowledge_bases;

use std::sync::Arc;

use apalis::prelude::{MemoryStorage, Monitor, WorkerBuilder, WorkerFactoryFn};

use crate::{
    workers::knowledge_bases::{handle_knowledge_base_processing, KnowledgeBaseJob},
    ComhairleState,
};

#[derive(Clone, Debug)]
pub struct JobQueues {
    pub knowledge_bases: MemoryStorage<KnowledgeBaseJob>,
}

pub async fn setup_workers(state: Arc<ComhairleState>) {
    let knowledge_base_worker = WorkerBuilder::new("process_knowledge_base_job")
        .data(state.clone())
        .backend(state.jobs.knowledge_bases.clone())
        .build_fn(handle_knowledge_base_processing);

    Monitor::new()
        .register(knowledge_base_worker)
        .run()
        .await
        .unwrap()
}
