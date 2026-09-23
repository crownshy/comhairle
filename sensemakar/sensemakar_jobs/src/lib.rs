use sensemakar_types::thinking_space::QuestionChain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ThinkingSpaceNextQuestionJob {
    pub job_id: Uuid,
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
    pub reading_age_target: Option<String>,
    pub topic: String,
    pub question_chain: QuestionChain,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThinkingSpaceSummaryJob {
    pub job_id: Uuid,
    pub context: Option<String>,
    pub additional_instructions: Option<String>,
    pub reading_age_target: Option<String>,
    pub topic: String,
    pub question_chain: QuestionChain,
}

pub async fn redis_conn() -> apalis_redis::ConnectionManager {
    let url = std::env::var("SENSEMAKAR_REDIS_URL").expect("REDIS_URL not set");
    apalis_redis::connect(url)
        .await
        .expect("could not connect to redis")
}
