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
