pub mod documents;

use std::sync::Arc;

use apalis::prelude::MemoryStorage;
use tokio::sync::Mutex;

use crate::workers::documents::DocumentJob;

#[derive(Clone, Debug)]
pub struct JobQueues {
    pub documents: Arc<Mutex<MemoryStorage<DocumentJob>>>,
}
