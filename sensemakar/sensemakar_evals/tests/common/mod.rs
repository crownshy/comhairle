use std::env;

use rig_core::DynModel;
use rig_core::operation::Completion;
use sensemakar_evals::judge::{JudgeVerdict, LlmAsJudge};
use sensemakar_evals::model_select::{ModelSelection, Provider};
use sensemakar_evals::report::{self, DEFAULT_SCORE_THRESHOLD};

/// Builds a `DynModel` from `{prefix}_PROVIDER`/`_MODEL`/`_BASE_URL`/
/// `_API_KEY`/`_REGION` env vars, falling back to local Ollama
/// `qwen3-long` when unset - the same default `sensemakar_core`'s own
/// tests use (see `sensemakar::test_helpers::test_model`), so this suite
/// runs out of the box with nothing configured.
fn model_from_env(prefix: &str) -> DynModel<Completion> {
    let provider = env::var(format!("{prefix}_PROVIDER"))
        .ok()
        .and_then(|value| Provider::parse(&value))
        .unwrap_or(Provider::Ollama);
    let model = env::var(format!("{prefix}_MODEL")).unwrap_or_else(|_| "qwen3-long".to_string());
    let base_url =
        env::var(format!("{prefix}_BASE_URL")).unwrap_or_else(|_| "http://localhost:11434".to_string());
    let api_key = env::var(format!("{prefix}_API_KEY")).unwrap_or_default();
    let region = env::var(format!("{prefix}_REGION")).ok();

    ModelSelection {
        provider,
        model,
        base_url,
        api_key,
        region,
    }
    .build()
}

/// The model under test - override with `SENSEMAKAR_EVAL_MODEL_*`.
pub fn candidate_model() -> DynModel<Completion> {
    model_from_env("SENSEMAKAR_EVAL_MODEL")
}

/// The judge model - override with `SENSEMAKAR_JUDGE_MODEL_*`, e.g. to
/// point it at a stronger Bedrock model in CI.
pub fn judge() -> LlmAsJudge {
    LlmAsJudge::new(model_from_env("SENSEMAKAR_JUDGE_MODEL"))
}

/// Records the verdict to disk (for the `report` subcommand to pick up
/// later) and asserts it cleared the score floor.
pub fn record_and_assert(task: &str, task_description: &str, case_id: &str, verdict: &JudgeVerdict) {
    let result = report::record_verdict(
        &report::default_results_dir(),
        task,
        task_description,
        case_id,
        "suite",
        verdict,
        DEFAULT_SCORE_THRESHOLD,
    )
    .expect("failed to record eval result");

    assert!(
        result.passed(),
        "{task}/{case_id} scored {:.2} (threshold {:.2}):\n{:#?}",
        result.score,
        result.threshold,
        result.points
    );
}
