use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct WebsocketConfig {
    pub redis_pubsub_url: String,
}
