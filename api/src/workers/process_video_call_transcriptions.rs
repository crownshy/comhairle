use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoCallJob {
    event_id: Uuid,
}
