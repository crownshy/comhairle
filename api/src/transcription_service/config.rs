use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TranscriptionServiceConfig {
    AmazonTranscribe(AmazonTranscribeConfig),
}

#[derive(Clone, Debug, Deserialize)]
pub struct AmazonTranscribeConfig {
    pub amazon_transcribe_api_key: String,
}
