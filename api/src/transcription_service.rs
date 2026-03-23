use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use error::{Result, TranscriptionServiceError};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;

#[cfg(test)]
use mockall::{automock, predicate::*};

pub mod amazon_transcriber;
pub mod config;
pub mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEvent {
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
    pub speaker_id: Option<String>,
    pub is_pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub events: Vec<TranscriptEvent>,
    pub start_time: DateTime<Utc>,
    pub has_speaker_ids: bool,
}

impl Transcription {
    pub fn new() -> Self {
        Self {
            events: vec![],
            start_time: Utc::now(),
            has_speaker_ids: false,
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transcriber: Sync + Send {
    async fn transcribe(&self, audio: &Vec<u8>) -> Result<String>;
    async fn transcribe_live(
        &self,
        _input_stream: Receiver<Bytes>,
    ) -> Result<Receiver<Transcription>> {
        Err(TranscriptionServiceError::StreamingProcessingUnsupported)
    }
    fn model_detects_speakers(&self) -> bool;
    fn supports_streaming(&self) -> bool;
}

#[cfg(test)]
impl MockTranscriber {
    pub fn base() -> MockTranscriber {
        let transcriber = MockTranscriber::new();

        transcriber
    }
}
