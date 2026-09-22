use apalis::prelude::*;
use apalis_redis::RedisStorage;
use sensemakar_jobs::{ThinkingSpaceNextQuestionJob, ThinkingSpaceSummaryJob, redis_conn};

#[derive(Clone)]
pub struct SenseMakarService {
    pub thinking_space_followup_question_generator: RedisStorage<ThinkingSpaceNextQuestionJob>,
    pub thinking_space_summary_generator: RedisStorage<ThinkingSpaceSummaryJob>,
}

impl SenseMakarService {
    pub async fn new() -> Self {
        let conn = redis_conn().await;

        let thinking_space_followup_question_generator: RedisStorage<ThinkingSpaceNextQuestionJob> =
            RedisStorage::new(conn.clone());

        let thinking_space_summary_generator: RedisStorage<ThinkingSpaceSummaryJob> =
            RedisStorage::new(conn.clone());

        Self {
            thinking_space_summary_generator,
            thinking_space_followup_question_generator,
        }
    }

    pub async fn request_thinking_space_questions(&mut self, job: ThinkingSpaceNextQuestionJob) {
        self.thinking_space_followup_question_generator
            .push(job)
            .await
            .unwrap();
    }

    pub async fn request_thinking_space_summary(&mut self, job: ThinkingSpaceSummaryJob) {
        self.thinking_space_summary_generator
            .push(job)
            .await
            .unwrap();
    }
}
