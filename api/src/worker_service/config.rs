use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WorkerConfig {
    pub redis_url: String,
}
