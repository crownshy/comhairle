use askama::Template;
use bon::Builder;
use rig::{
    completion::CompletionModel,
    extractor::{ExtractionError, ExtractorBuilder},
};
use schemars::JsonSchema;
use sensemakar_types::thinking_space::{Question, QuestionChain};
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
/// The list of followupquestions
pub struct FollowUpQuestions {
    questions: Vec<Question>,
}
#[derive(Error, Debug)]
pub enum ThinkingSpaceError {
    #[error("model error (0)")]
    RigError(#[from] rig::ProviderResponseError),
    #[error("Template Error ()")]
    TemplateError(#[from] askama::Error),
    #[error("extractor error (0)")]
    RigExtractorError(#[from] ExtractionError),
    #[error("Statement serialization error(0)")]
    QuestionChainSeializationError(#[from] serde_json::Error),
}

impl FollowUpQuestionGenerator {
    pub async fn run_with_model<M>(
        &self,
        question_chain: QuestionChain,
        model: M,
    ) -> Result<Vec<Question>, ThinkingSpaceError>
    where
        M: CompletionModel + 'static,
    {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<FollowUpQuestions>::new(model)
            .preamble(&template)
            .build();
        let question_list = serde_json::to_string(&question_chain)?;
        let follow_ups: FollowUpQuestions = extractor.extract(question_list).await?;

        Ok(follow_ups.questions)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug,Clone)]
/// Summarizes an interview with a participant
pub struct InterviewSummary {
    /// The summary of the interview in the participants voice.
    pub summary: String,
    /// A summary of any areas in which it seems like the
    /// participant has uncertainty or conflicting ideas.
    pub areas_of_uncertainty: String,
}

impl SummaryGenerator {
    pub async fn run_with_model<M>(
        &self,
        question_chain: QuestionChain,
        model: M,
    ) -> Result<InterviewSummary, ThinkingSpaceError>
    where
        M: CompletionModel + 'static,
    {
        let template = self.render()?;
        let extractor = ExtractorBuilder::<InterviewSummary>::new(model)
            .preamble(&template)
            .build();
        let question_list = serde_json::to_string(&question_chain)?;
        let summary: InterviewSummary = extractor.extract(question_list).await?;

        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use sensemakar_types::thinking_space::{Answer, Question, QuestionChain, QuestionWithAnswer};

use crate::{test_helpers::test_model, thinking_space::{FollowUpQuestionGenerator, SummaryGenerator}};


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
                    base_interaction:question_chain, 
                    follow_ups: vec![] 
                },
                model,
            )
            .await;
        assert!(result.is_ok(), "model should run");
        let result = result.unwrap();
        println!("{result:#?}");
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

        let result = generator
            .run_with_model(
                question_chain,
                model,
            )
            .await;
        assert!(result.is_ok(), "model should run");
        let result = result.unwrap();
        println!("{result:#?}");
    }
}
