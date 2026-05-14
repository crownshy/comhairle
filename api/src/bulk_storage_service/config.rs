use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct BulkStorageServiceConfig {
    pub store_name: String,
}
