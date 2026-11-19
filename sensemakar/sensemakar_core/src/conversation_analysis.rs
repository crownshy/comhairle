use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_types::Transcript;
use sensemakar_types::conversation_analysis::ConversationAnalysis;
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "conversation_analysis/analyze_transcript.md")]
pub struct ConversationAnalyzer {
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
}

#[derive(Error, Debug)]
pub enum ConversationAnalyzerError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("Transcript serialization error {0}")]
    TranscriptSerializationError(#[from] serde_json::Error),
}

impl ConversationAnalyzer {
    pub async fn run_with_model(
        &self,
        transcript: &Transcript,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<ConversationAnalysis, ConversationAnalyzerError> {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<ConversationAnalysis>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();

        let transcript_json = serde_json::to_string(transcript)?;

        let result = extractor
            .extract(transcript_json)
            .await
            .map_err(Box::new)?
            .output;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use jiff::Timestamp;
    use sensemakar_types::{Transcript, Utterance};

    use crate::{conversation_analysis::ConversationAnalyzer, test_helpers::test_model};

    fn test_transcript() -> Transcript {
        Transcript {
            id: "t1".into(),
            started_at: Timestamp::now(),
            parts: vec![
                Utterance {
                    speaker_id: "alice".into(),
                    text: "I think we should ban cars from the town centre entirely.".into(),
                    started_offset: 0,
                    ended_offset: 5,
                },
                Utterance {
                    speaker_id: "bob".into(),
                    text: "That would hurt local shops who rely on deliveries.".into(),
                    started_offset: 5,
                    ended_offset: 10,
                },
                Utterance {
                    speaker_id: "alice".into(),
                    text: "I hadn't thought about deliveries. Maybe we just need a permit scheme for delivery vehicles instead of a full ban.".into(),
                    started_offset: 10,
                    ended_offset: 16,
                },
            ],
        }
    }

    #[tokio::test]
    async fn test_conversation_analysis() {
        let analyzer = ConversationAnalyzer::builder()
            .context("A public meeting about traffic policy in a town centre".into())
            .build();

        let transcript = test_transcript();
        let model = test_model();

        let result = analyzer.run_with_model(&transcript, model).await;
        let result = result.unwrap();

        assert!(!result.summary.is_empty(), "expected a non-empty summary");
    }
}
