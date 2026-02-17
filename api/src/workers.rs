pub mod process_documents;

use std::sync::Arc;

use apalis::prelude::MemoryStorage;
use tokio::sync::Mutex;

use crate::workers::process_documents::DocumentJob;

#[derive(Clone, Debug)]
pub struct JobQueues {
    pub process_documents: Arc<Mutex<MemoryStorage<DocumentJob>>>,
}
