use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;
use reqwest::StatusCode;
use tokio::sync::{Mutex, OwnedMutexGuard};

use super::{ComhairleDocument, GetQueryParams};

pub(super) type DocumentLists = Cache<Option<GetQueryParams>, (StatusCode, Vec<ComhairleDocument>)>;

#[derive(Debug)]
pub(super) struct DocumentCache {
    knowledge_bases: Cache<String, Arc<Mutex<DocumentLists>>>,
}

impl DocumentCache {
    pub fn new() -> Self {
        Self {
            knowledge_bases: Cache::builder()
                .max_capacity(128)
                .time_to_idle(Duration::from_secs(30 * 60))
                .build(),
        }
    }

    /// Serialize cache fills and mutations per knowledge base so an in-flight
    /// list cannot repopulate invalidated results after a mutation.
    pub async fn lock(&self, knowledge_base_id: &str) -> OwnedMutexGuard<DocumentLists> {
        self.knowledge_bases
            .get_with(knowledge_base_id.to_owned(), async {
                Arc::new(Mutex::new(
                    Cache::builder()
                        .max_capacity(16)
                        .time_to_live(Duration::from_secs(5 * 60))
                        .build(),
                ))
            })
            .await
            .lock_owned()
            .await
    }
}
