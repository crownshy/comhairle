pub mod process_documents;
pub mod process_video_call_transcriptions;

use std::sync::Arc;

use apalis::prelude::{MemoryStorage, StepRequest};
use apalis_redis::RedisStorage;
use tokio::sync::Mutex;

use crate::workers::process_documents::DocumentJob;

#[derive(Clone, Debug)]
pub struct JobQueues {
    pub process_documents: Arc<Mutex<MemoryStorage<DocumentJob>>>,
    pub process_transcriptions: Arc<Mutex<RedisStorage<StepRequest<Vec<u8>>>>>,
}
