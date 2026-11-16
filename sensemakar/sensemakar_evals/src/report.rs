use std::fs;
use std::path::{Path, PathBuf};

use askama::Template;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::judge::{JudgeVerdict, PointVerdict};

/// Score floor used by the test suite and reused as the CLI's default, so
/// "did it pass" means the same thing whether it came from `cargo test` or
/// a spot-check run.
pub const DEFAULT_SCORE_THRESHOLD: f64 = 0.75;

/// Where results get recorded/read by default. Anchored to this crate's own
/// directory via `CARGO_MANIFEST_DIR` (baked in at compile time) rather
/// than a bare relative path - `cargo test` always runs test binaries with
/// their CWD set to the package root, but `cargo run`/a built binary uses
/// whatever directory it's invoked from, so a relative default would have
/// the suite and the CLI silently disagree about where results live
/// depending on how each was run.
pub fn default_results_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/sensemakar-evals-results")
}

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("failed to create results dir {path}: {source}")]
    CreateDir { path: PathBuf, source: std::io::Error },
    #[error("failed to write result {path}: {source}")]
    Write { path: PathBuf, source: std::io::Error },
    #[error("failed to serialize eval result {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to read results dir {path}: {source}")]
    ReadDir { path: PathBuf, source: std::io::Error },
    #[error("failed to read result file {path}: {source}")]
    Read { path: PathBuf, source: std::io::Error },
    #[error("failed to parse result file {path}: {source}")]
    Parse { path: PathBuf, source: serde_json::Error },
    #[error("no eval results found in {0} - run the test suite or a spot-check first")]
    Empty(PathBuf),
    #[error("template error {0}")]
    TemplateError(#[from] askama::Error),
}

/// One case's recorded verdict, written to disk as a side effect of running
/// it (from a test or from the CLI's `run`/`judge` subcommands) so the
/// `report` subcommand can pick it back up later, independently of however
/// it was produced.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvalResult {
    pub task: String,
    pub task_description: String,
    pub case_id: String,
    /// "suite" for a `cargo test` run, "spot-check" for the CLI.
    pub source: String,
    pub score: f64,
    pub threshold: f64,
    pub points: Vec<PointVerdict>,
}

impl EvalResult {
    pub fn passed(&self) -> bool {
        self.score >= self.threshold
    }
}

pub fn record_result(results_dir: &Path, result: &EvalResult) -> Result<(), ReportError> {
    fs::create_dir_all(results_dir).map_err(|source| ReportError::CreateDir {
        path: results_dir.to_path_buf(),
        source,
    })?;

    let path = results_dir.join(format!("{}__{}.json", result.task, result.case_id));
    let json = serde_json::to_string_pretty(result)?;
    fs::write(&path, json).map_err(|source| ReportError::Write { path, source })
}

/// Convenience wrapper: builds an `EvalResult` from a fresh `JudgeVerdict`,
/// records it, and hands back the result so the caller can assert on
/// `.passed()` or print the score.
pub fn record_verdict(
    results_dir: &Path,
    task: &str,
    task_description: &str,
    case_id: &str,
    source: &str,
    verdict: &JudgeVerdict,
    threshold: f64,
) -> Result<EvalResult, ReportError> {
    let result = EvalResult {
        task: task.to_string(),
        task_description: task_description.to_string(),
        case_id: case_id.to_string(),
        source: source.to_string(),
        score: verdict.score(),
        threshold,
        points: verdict.points.clone(),
    };
    record_result(results_dir, &result)?;
    Ok(result)
}

#[derive(Debug)]
pub struct CaseReport {
    pub case_id: String,
    pub source: String,
    pub score_pct: u32,
    pub passed: bool,
    pub points: Vec<PointVerdict>,
}

#[derive(Debug)]
pub struct TaskReport {
    pub task: String,
    pub description: String,
    pub pass_count: usize,
    pub case_count: usize,
    pub average_score_pct: u32,
    pub cases: Vec<CaseReport>,
}

/// Scans every recorded result in `results_dir` and groups them by task, in
/// preparation for rendering the HTML report.
pub fn load_results(results_dir: &Path) -> Result<Vec<TaskReport>, ReportError> {
    let entries = fs::read_dir(results_dir).map_err(|source| ReportError::ReadDir {
        path: results_dir.to_path_buf(),
        source,
    })?;

    let mut results: Vec<EvalResult> = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| ReportError::ReadDir {
            path: results_dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let text = fs::read_to_string(&path).map_err(|source| ReportError::Read {
            path: path.clone(),
            source,
        })?;
        let result: EvalResult = serde_json::from_str(&text)
            .map_err(|source| ReportError::Parse { path: path.clone(), source })?;
        results.push(result);
    }

    if results.is_empty() {
        return Err(ReportError::Empty(results_dir.to_path_buf()));
    }

    results.sort_by(|a, b| a.task.cmp(&b.task).then(a.case_id.cmp(&b.case_id)));

    let mut tasks: Vec<TaskReport> = Vec::new();
    for result in results {
        let case = CaseReport {
            case_id: result.case_id,
            source: result.source,
            score_pct: (result.score * 100.0).round() as u32,
            passed: result.score >= result.threshold,
            points: result.points,
        };

        match tasks.iter_mut().find(|task| task.task == result.task) {
            Some(task_report) => task_report.cases.push(case),
            None => tasks.push(TaskReport {
                task: result.task,
                description: result.task_description,
                pass_count: 0,
                case_count: 0,
                average_score_pct: 0,
                cases: vec![case],
            }),
        }
    }

    for task_report in &mut tasks {
        task_report.case_count = task_report.cases.len();
        task_report.pass_count = task_report.cases.iter().filter(|case| case.passed).count();
        let total: u32 = task_report.cases.iter().map(|case| case.score_pct).sum();
        task_report.average_score_pct = total / task_report.case_count as u32;
    }

    Ok(tasks)
}

#[derive(Template)]
#[template(path = "report/report.html")]
struct ReportTemplate<'a> {
    tasks: &'a [TaskReport],
}

pub fn render_html(tasks: &[TaskReport]) -> Result<String, ReportError> {
    Ok(ReportTemplate { tasks }.render()?)
}
