use std::{ops::Deref, sync::Arc};

use apalis::prelude::*;
use apalis_redis::RedisStorage;
use rig_core::DynModel;
use rig_core::operation::Completion;
use rig_core::providers::ollama::Ollama;
use sensemakar::{
    thinking_space::{
        FollowUpQuestionGenerator, InterviewSummary, SummaryGenerator, ThinkingSpaceError,
    },
    wikipoll_describer::{WikiPollGroupDescriber, WikiPollGroupDescriberError},
};
use sensemakar_jobs::{
    ThinkingSpaceNextQuestionJob, ThinkingSpaceSummaryJob, WikiPollSummaryJob, redis_conn,
};
use sensemakar_types::{WikiPollReportResult, thinking_space::Question};

async fn handle_thinking_space_next_question_generator(
    job: ThinkingSpaceNextQuestionJob,
    model: Data<DynModel<Completion>>,
) -> Result<Vec<Question>, ThinkingSpaceError> {
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

async fn handle_thinking_space_summary_generator(
    job: ThinkingSpaceSummaryJob,
    model: Data<DynModel<Completion>>,
) -> Result<InterviewSummary, ThinkingSpaceError> {
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

async fn handle_wiki_poll_summary_generator(
    job: WikiPollSummaryJob,
    model: Data<DynModel<Completion>>,
) -> Result<WikiPollReportResult, WikiPollGroupDescriberError> {
    let extractor = WikiPollGroupDescriber {
        context: job.context,
        additional_instructions: job.additional_instructions,
    };

    let result = extractor
        .run_with_model(&job.poll_data, model.deref().clone())
        .await?;

    Ok(result)
}

#[tokio::main]
async fn main() {
    let conn = redis_conn().await;
    let ts_next_q_storage: RedisStorage<ThinkingSpaceNextQuestionJob> =
        RedisStorage::new(conn.clone());
    let ts_summary_storage: RedisStorage<ThinkingSpaceSummaryJob> = RedisStorage::new(conn.clone());
    let wiki_poll_summary_storage: RedisStorage<WikiPollSummaryJob> =
        RedisStorage::new(conn.clone());

    let model = Arc::new(Ollama::new().completion("qwen3-long").erase());

    let ts_next_q_worker = WorkerBuilder::new("thinking_space_next_question_generator")
        .concurrency(4)
        .data(model.clone())
        .backend(ts_next_q_storage)
        .build_fn(handle_thinking_space_next_question_generator);

    let ts_summary_worker = WorkerBuilder::new("thinking_space_summary_generator")
        .concurrency(4)
        .data(model.clone())
        .backend(ts_summary_storage)
        .build_fn(handle_thinking_space_summary_generator);

    let wiki_poll_summary_worker = WorkerBuilder::new("wiki_poll_summary_generator")
        .concurrency(4)
        .data(model)
        .backend(wiki_poll_summary_storage)
        .build_fn(handle_wiki_poll_summary_generator);

    Monitor::new()
        .register(ts_next_q_worker)
        .register(ts_summary_worker)
        .register(wiki_poll_summary_worker)
        .run()
        .await
        .unwrap();
}
