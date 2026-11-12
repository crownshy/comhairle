use askama::Template;
use bon::Builder;
use rig_agent::completion::StructuredOutputError;
use rig_agent::extractor::ExtractorBuilder;
use rig_core::DynModel;
use rig_core::operation::Completion;
use schemars::JsonSchema;
use sensemakar_types::thinking_space::{FollowUpQuestion, QuestionChain};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Builder, Template)]
#[template(path = "thinking_space/generate_question_followup.md")]
pub struct FollowUpQuestionGenerator {
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
    pub reading_age_target: Option<String>,
    pub topic: String,
}

#[derive(Builder, Template)]
#[template(path = "thinking_space/summarize_interview.md")]
pub struct SummaryGenerator {
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
    pub reading_age_target: Option<String>,
    pub topic: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct FollowUpQuestions {
    /// A list of the follow up questions
    questions: Vec<FollowUpQuestion>,
}

#[derive(Error, Debug)]
pub enum ThinkingSpaceError {
    #[error("Template Error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error {0}")]
    RigExtractorError(#[from] Box<StructuredOutputError>),
    #[error("Statement serialization error {0}")]
    QuestionChainSeializationError(#[from] serde_json::Error),
    #[error("summary generation produced no usable output after {attempts} attempts")]
    SuspiciousSummary { attempts: usize },
    #[error("participant's answers did not contain enough content to summarize")]
    InsufficientAnswers,
}

impl FollowUpQuestionGenerator {
    pub async fn run_with_model(
        &self,
        question_chain: QuestionChain,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<Vec<FollowUpQuestion>, ThinkingSpaceError> {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<FollowUpQuestions>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();
        let question_list = serde_json::to_string(&question_chain)?;
        let follow_ups = extractor
            .extract(question_list)
            .await
            .map_err(Box::new)?
            .output;

        Ok(follow_ups.questions)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
/// Summarizes an interview with a participant
pub struct InterviewSummary {
    /// The summary of the interview in the participants voice.
    pub summary: String,
    /// A summary of any areas in which it seems like the
    /// participant has uncertainty or conflicting ideas.
    pub areas_of_uncertainty: String,
}

/// Below this many words, `summary` reads as a stub rather than a real
/// personal statement (the extractor's "even with default values" preamble
/// can make a model submit the literal schema type name, "string", instead
/// of refusing to answer).
const MIN_SUMMARY_WORDS: usize = 8;

/// Told to the model (see `summarize_interview.md`'s `<QualityCheck>` block)
/// as the value to submit instead of fabricating a summary when the
/// participant's answers are too sparse or incoherent to summarize honestly.
const INSUFFICIENT_INPUT_SENTINEL: &str = "UNABLE_TO_SUMMARIZE";

impl InterviewSummary {
    /// The model deliberately declined rather than fabricating a summary.
    /// Distinct from `is_suspicious`: this is a stable, expected response to
    /// the input given, so retrying the same question_chain won't help.
    fn is_insufficient_input(&self) -> bool {
        self.summary
            .trim()
            .eq_ignore_ascii_case(INSUFFICIENT_INPUT_SENTINEL)
    }

    fn is_suspicious(&self) -> bool {
        is_placeholder(&self.summary)
            || is_placeholder(&self.areas_of_uncertainty)
            || self.summary.split_whitespace().count() < MIN_SUMMARY_WORDS
    }
}

fn is_placeholder(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case("string")
}

const MAX_SUMMARY_ATTEMPTS: usize = 3;

impl SummaryGenerator {
    pub async fn run_with_model(
        &self,
        question_chain: QuestionChain,
        model: impl Into<DynModel<Completion>>,
    ) -> Result<InterviewSummary, ThinkingSpaceError> {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<InterviewSummary>::new(model)
            .append_preamble(&template)
            .retries(2)
            .build();
        let question_list = serde_json::to_string(&question_chain)?;

        for attempt in 1..=MAX_SUMMARY_ATTEMPTS {
            let summary = extractor
                .extract(question_list.clone())
                .await
                .map_err(Box::new)?
                .output;

            if summary.is_insufficient_input() {
                return Err(ThinkingSpaceError::InsufficientAnswers);
            }

            if !summary.is_suspicious() {
                return Ok(summary);
            }

            tracing::warn!(attempt, "thinking space summary looked suspicious, retrying");
        }

        Err(ThinkingSpaceError::SuspiciousSummary {
            attempts: MAX_SUMMARY_ATTEMPTS,
        })
    }
}

#[cfg(test)]
mod tests {
    use askama::Template;
    use sensemakar_types::thinking_space::{Answer, Question, QuestionChain, QuestionWithAnswer};

    use crate::{
        test_helpers::test_model,
        thinking_space::{
            FollowUpQuestionGenerator, FollowUpQuestions, InterviewSummary, SummaryGenerator,
        },
    };

    #[test]
    fn regression_insufficient_input_sentinel_is_detected() {
        let summary = InterviewSummary {
            summary: "UNABLE_TO_SUMMARIZE".into(),
            areas_of_uncertainty: "UNABLE_TO_SUMMARIZE".into(),
        };

        assert!(summary.is_insufficient_input());
    }

    #[test]
    fn regression_insufficient_input_sentinel_is_not_mistaken_for_real_content() {
        let summary = InterviewSummary {
            summary: "I think this policy would really help my community access resources."
                .into(),
            areas_of_uncertainty:
                "I am unsure whether older residents would take advantage of it.".into(),
        };

        assert!(!summary.is_insufficient_input());
    }

    #[test]
    fn regression_suspicious_summary_detects_placeholder_string() {
        let summary = InterviewSummary {
            summary: "string".into(),
            areas_of_uncertainty: "Some genuine uncertainty text here".into(),
        };

        assert!(summary.is_suspicious());
    }

    #[test]
    fn regression_suspicious_summary_detects_too_short_summary() {
        let summary = InterviewSummary {
            summary: "Fine.".into(),
            areas_of_uncertainty: "None noted.".into(),
        };

        assert!(summary.is_suspicious());
    }

    #[test]
    fn regression_summary_with_real_content_is_not_suspicious() {
        let summary = InterviewSummary {
            summary: "I think this policy would really help my community access resources."
                .into(),
            areas_of_uncertainty:
                "I am unsure whether older residents would take advantage of it.".into(),
        };

        assert!(!summary.is_suspicious());
    }

    #[test]
    fn regression_follow_up_prompt_example_matches_output_type() {
        let generator = FollowUpQuestionGenerator::builder()
            .topic("Community participation".into())
            .build();
        let prompt = generator.render().unwrap();
        let example = prompt
            .split_once("```json\n")
            .unwrap()
            .1
            .split_once("\n```")
            .unwrap()
            .0;
        let output: FollowUpQuestions = serde_json::from_str(example).unwrap();

        assert_eq!(output.questions.len(), 1);
        let question = &output.questions[0];
        assert!(!question.question.is_empty());
        assert_eq!(question.question_type, "Adaptive Questions");
        assert!(prompt.contains("exactly 5 question objects"));
        assert!(prompt.contains("Do not return an array of strings"));
        assert!(prompt.contains("submit function arguments"));
    }

    #[test]
    fn regression_follow_up_output_rejects_bare_question_strings() {
        let result = serde_json::from_value::<FollowUpQuestions>(serde_json::json!({
            "questions": ["Why does flexible timing matter to you?"]
        }));

        assert!(result.is_err());
    }

    #[test]
    fn regression_follow_up_output_preserves_question_metadata() {
        let questions = (0..5)
            .map(|index| {
                serde_json::json!({
                    "question": format!("Question {index}?"),
                    "question_type": "Adaptive Questions"
                })
            })
            .collect::<Vec<_>>();
        let output: FollowUpQuestions = serde_json::from_value(serde_json::json!({
            "questions": questions
        }))
        .unwrap();

        assert_eq!(output.questions.len(), 5);
        for (index, question) in output.questions.iter().enumerate() {
            assert_eq!(question.question, format!("Question {index}?"));
            assert_eq!(question.question_type, "Adaptive Questions");
        }
    }

    #[test]
    fn regression_follow_up_output_requires_question_and_type() {
        for question in [
            serde_json::json!({"question": "Why does flexible timing matter to you?"}),
            serde_json::json!({"question_type": "Adaptive Questions"}),
        ] {
            let result = serde_json::from_value::<FollowUpQuestions>(serde_json::json!({
                "questions": [question]
            }));
            assert!(result.is_err());
        }
    }

    #[test]
    fn regression_follow_up_schema_matches_requested_contract() {
        let schema = serde_json::to_value(schemars::schema_for!(FollowUpQuestions)).unwrap();

        assert_eq!(schema["required"], serde_json::json!(["questions"]));
        assert_eq!(
            schema["properties"],
            serde_json::json!({
                "questions": {
                    "description": "A list of the follow up questions",
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "question": {
                                "description": "the text of the generated question",
                                "type": "string"
                            },
                            "question_type": {
                                "description": "The type of the question as defined in the prompt",
                                "type": "string"
                            }
                        },
                        "required": ["question_type", "question"]
                    }
                }
            })
        );
    }

    #[tokio::test]
    async fn test_question_followup() {
        let model = test_model();
        let topic = "A conversation about social proscribing in scotland";
        let question_chain =
            QuestionWithAnswer {
                question: Question {text:"How might social proscribing effect your community?".into(), asking_reasion: "We /* wan */t to understand the lived experience of people and how this policy might improve their own lives and that of their communities".into(), ai_generated: false },
                answer: Answer { text: "I think it would be a great benefit for the community. There are many resources in the areas surrounding this neighborhood but none are financially accesiable to people who live here. This would give them access".into()  },
            };

        let generator = FollowUpQuestionGenerator::builder()
            .topic(topic.into())
            .reading_age_target("9-11 year old".into())
            .build();
        let result = generator
            .run_with_model(
                QuestionChain {
                    base_interaction: question_chain,
                    follow_ups: vec![],
                },
                model,
            )
            .await;
        assert!(result.is_ok(), "model should run");
        result.unwrap();
    }

    #[tokio::test]
    async fn test_summary() {
        let model = test_model();
        let topic = "A conversation about social proscribing in scotland";
        let question_chain = QuestionChain{
            base_interaction: QuestionWithAnswer {
                question: Question {text:"How might social proscribing effect your community?".into(), asking_reasion: "We /* wan */t to understand the lived experience of people and how this policy might improve their own lives and that of their communities".into(), ai_generated: false },
                answer: Answer { text: "I think it would be a great benefit for the community. There are many resources in the areas surrounding this neighborhood but none are financially accesiable to people who live here. This would give them access".into()  },
            },
            follow_ups: vec![
                QuestionWithAnswer{
                    question: Question {text:"Can you say more about what resources might be made available".into(), asking_reasion: "We want to deepend the users response".into(), ai_generated: true  },
                    answer: Answer { text: "There is a sports club that has swimming and tennis courts but it's really expensive. Lots of folks I know have expressed being stressed out and being able to exercise regularly would help. Having access to that would be wonderful".into()  },
                },
                QuestionWithAnswer{
                    question: Question {text:"Do you think that people would take advantage of those resources if offered?".into(), asking_reasion: "We want to understand how effective the policy might be ".into(), ai_generated: true  },
                    answer: Answer { text: "I am not really sure. I know younger people would but I am unsure if older people might feel like it's just not for them, that they would be out of place and not welcome at a membership club".into()  },
                },
                QuestionWithAnswer{
                    question: Question {text:"DDo you think young people would be happy at being proscribed a social remedy over a medical one".into(), asking_reasion: "We want to understand how different demographics might respond to the policy ".into(), ai_generated: true  },
                    answer: Answer { text: "No all the young people I know would be very against it ".into()  },
                },

            ]};

        let generator = SummaryGenerator::builder()
            .topic(topic.into())
            .reading_age_target("9-11 year old".into())
            .build();

        let result = generator.run_with_model(question_chain, model).await;
        assert!(result.is_ok(), "model should run");
        result.unwrap();
    }
}
