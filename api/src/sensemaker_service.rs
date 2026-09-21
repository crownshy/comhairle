use apalis::prelude::*;
use apalis_redis::RedisStorage;
use sensemaker_jobs::{ThinkingSpaceNextQuestionJob, ThinkingSpaceSummaryJob, redis_conn};

pub struct SenseMakerService {
    pub thinking_space_followup_question_generator: RedisStorage<ThinkingSpaceNextQuestionJob>,
    pub thinking_space_summary_generator: RedisStorage<ThinkingSpaceSummaryJob>,
}

impl SenseMakerService {
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
        let mut storage = self.thinking_space_followup_question_generator;
        match storage.push(job).await.unwrap();
    }

    pub async fn request_thinking_space_summary(&mut self, job: ThinkingSpaceSummaryJob) {
        let mut storage = self.thinking_space_summary_generator;
        match storage.push(job).await.unwrap();
    }
}
