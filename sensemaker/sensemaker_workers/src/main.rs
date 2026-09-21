use std::{ops::Deref, sync::Arc};

use apalis::prelude::*;
use apalis_redis::RedisStorage;
use rig::{client::CompletionClient, completion::CompletionModel, providers::ollama};
use sensemaker::thinking_space::{
    FollowUpQuestionGenerator, InterviewSummary, SummaryGenerator, ThinkingSpaceError,
};
use sensemaker_jobs::{ThinkingSpaceNextQuestionJob, ThinkingSpaceSummaryJob, redis_conn};
use sensemaker_types::thinking_space::Question;

async fn handle_thinking_space_next_question_generator<M>(
    job: ThinkingSpaceNextQuestionJob,
    model: Data<M>,
) -> Result<Vec<Question>, ThinkingSpaceError>
where
    M: CompletionModel + Clone + 'static,
{
    let extractor = FollowUpQuestionGenerator {
        context: job.context,
        additional_instructions: job.additional_instructions,
        topic: job.topic,
        reading_age_target: job.reading_age_target,
    };

    let result = extractor
        .run_with_model(job.question_chain, model.deref().clone())
        .await?;

    Ok(result)
}

async fn handle_thinking_space_summary_generator<M>(
    job: ThinkingSpaceSummaryJob,
    model: Data<M>,
) -> Result<InterviewSummary, ThinkingSpaceError>
where
    M: CompletionModel + Clone + 'static,
{
    let extractor = SummaryGenerator {
        context: job.context,
        additional_instructions: job.additional_instructions,
        reading_age_target: job.reading_age_target,
        topic: job.topic,
    };

    let result = extractor
        .run_with_model(job.question_chain, model.deref().clone())
        .await?;

    Ok(result)
}

#[tokio::main]
async fn main() {
    let conn = redis_conn().await;
    let ts_next_q_storage: RedisStorage<ThinkingSpaceNextQuestionJob> =
        RedisStorage::new(conn.clone());
    let ts_summary_storage: RedisStorage<ThinkingSpaceSummaryJob> = RedisStorage::new(conn.clone());

    let client = ollama::Client::builder()
        .base_url("http://localhost:11434")
        .api_key("AAAAC3NzaC1lZDI1NTE5AAAAIK1z4ddn4+0iDtXx9RTiqMVi+gSLhh2Q6qcXsd109H6S")
        .build()
        .unwrap();

    let model = Arc::new(client.completion_model("qwen3-long"));

    let ts_next_q_worker = WorkerBuilder::new("thinking_space_next_question_generator")
        .concurrency(4)
        .data(model.clone())
        .backend(ts_next_q_storage)
        .build_fn(handle_thinking_space_next_question_generator::<ollama::CompletionModel>);

    let ts_summary_worker = WorkerBuilder::new("thinking_space_summary_generator")
        .concurrency(4)
        .data(model)
        .backend(ts_summary_storage)
        .build_fn(handle_thinking_space_summary_generator::<ollama::CompletionModel>);

    Monitor::new()
        .register(ts_next_q_worker)
        .register(ts_summary_worker)
        .run()
        .await
        .unwrap();
}
