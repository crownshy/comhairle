use serde::Serialize;

use crate::judge::{JudgeError, JudgeVerdict, LlmAsJudge};
use crate::rubric::RubricCase;

/// Renders a task's input and output to JSON and hands them to the judge
/// along with the case's rubric criteria. The one piece of glue shared by
/// every task's test and by the CLI - small enough that it doesn't need a
/// task-dispatch trait on top of it.
pub async fn evaluate_case<I: Serialize, O: Serialize>(
    judge: &LlmAsJudge,
    task_description: &str,
    input: &I,
    output: &O,
    case: &RubricCase,
) -> Result<JudgeVerdict, JudgeError> {
    let input_rendered = serde_json::to_string_pretty(input)?;
    let output_rendered = serde_json::to_string_pretty(output)?;
    judge
        .evaluate(task_description, &input_rendered, &output_rendered, case)
        .await
}
