use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use sensemakar::statement_classifier::StatementTypeClassifier;
use sensemakar::statement_theme_assigner::StatementThemeAssigner;
use sensemakar::theme_extraction::ThemeExtractor;
use sensemakar::theme_merger::ThemeMerger;
use sensemakar::wikipoll_describer::WikiPollGroupDescriber;
use sensemakar_evals::harness::evaluate_case;
use sensemakar_evals::judge::{JudgeVerdict, LlmAsJudge};
use sensemakar_evals::model_select::{ModelSelection, Provider};
use sensemakar_evals::report::{self, DEFAULT_SCORE_THRESHOLD};
use sensemakar_evals::rubric::RubricBlueprint;
use sensemakar_types::{Statement, Theme, WikiPollData};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Parser)]
#[command(
    name = "sensemakar_evals",
    about = "Rubric evals and LLM-as-judge spot-checks for the sensemakar tasks"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum ProviderArg {
    #[default]
    Ollama,
    OpenaiCompatible,
    Bedrock,
}

impl From<ProviderArg> for Provider {
    fn from(value: ProviderArg) -> Self {
        match value {
            ProviderArg::Ollama => Provider::Ollama,
            ProviderArg::OpenaiCompatible => Provider::OpenAiCompatible,
            ProviderArg::Bedrock => Provider::Bedrock,
        }
    }
}

/// Flags for the model under test - unprefixed, since `judge` and `report`
/// never need a second model alongside it.
#[derive(Args, Clone)]
struct ModelArgs {
    #[arg(long, value_enum, default_value_t = ProviderArg::Ollama)]
    provider: ProviderArg,
    #[arg(long, default_value = "qwen3-long")]
    model: String,
    #[arg(long, default_value = "http://localhost:11434")]
    base_url: String,
    #[arg(long)]
    api_key: Option<String>,
    #[arg(long)]
    region: Option<String>,
}

impl ModelArgs {
    fn selection(&self) -> ModelSelection {
        ModelSelection {
            provider: self.provider.into(),
            model: self.model.clone(),
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone().unwrap_or_default(),
            region: self.region.clone(),
        }
    }
}

/// Flags for the judge model, always `--judge-*` prefixed so `run` can take
/// both a model-under-test and a judge at once without a name clash.
#[derive(Args, Clone)]
struct JudgeArgs {
    #[arg(long = "judge-provider", value_enum, default_value_t = ProviderArg::Ollama)]
    provider: ProviderArg,
    #[arg(long = "judge-model", default_value = "qwen3-long")]
    model: String,
    #[arg(long = "judge-base-url", default_value = "http://localhost:11434")]
    base_url: String,
    #[arg(long = "judge-api-key")]
    api_key: Option<String>,
    #[arg(long = "judge-region")]
    region: Option<String>,
}

impl JudgeArgs {
    fn build(&self) -> LlmAsJudge {
        let selection = ModelSelection {
            provider: self.provider.into(),
            model: self.model.clone(),
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone().unwrap_or_default(),
            region: self.region.clone(),
        };
        LlmAsJudge::new(selection.build())
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum Task {
    ThemeExtraction,
    ThemeMerge,
    ClassifyStatements,
    AssignThemes,
    WikipollDescribe,
}

#[derive(Subcommand)]
enum Command {
    /// Regenerate a task's output from a local input file, then judge it
    /// against a rubric case. Needs the model under test and the judge.
    Run {
        #[arg(long, value_enum)]
        task: Task,
        /// Path to the input fixture (JSON matching the task's input type).
        #[arg(short, long)]
        input: PathBuf,
        /// Required for assign-themes: the themes to assign statements to.
        #[arg(long)]
        themes: Option<PathBuf>,
        #[arg(long)]
        rubric: PathBuf,
        #[arg(long)]
        case: String,
        #[arg(long, default_value_t = DEFAULT_SCORE_THRESHOLD)]
        threshold: f64,
        /// Where to record this run's verdict. Defaults to this crate's own
        /// target/sensemakar-evals-results, same as the test suite, so a
        /// `report` run afterwards picks it up without extra flags.
        #[arg(long)]
        results_dir: Option<PathBuf>,
        #[command(flatten)]
        model_args: ModelArgs,
        #[command(flatten)]
        judge_args: JudgeArgs,
    },
    /// Judge output that has already been produced (e.g. from a real
    /// production export run through `sensemakar_cli`) against a rubric
    /// case. Never calls the model under test - reads local files only.
    Judge {
        /// The original input given to the task, for the judge's context.
        #[arg(short, long)]
        input: PathBuf,
        /// The task's already-produced output to grade.
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        rubric: PathBuf,
        #[arg(long)]
        case: String,
        #[arg(long, default_value_t = DEFAULT_SCORE_THRESHOLD)]
        threshold: f64,
        /// Where to record this run's verdict. Defaults to this crate's own
        /// target/sensemakar-evals-results, same as the test suite, so a
        /// `report` run afterwards picks it up without extra flags.
        #[arg(long)]
        results_dir: Option<PathBuf>,
        #[command(flatten)]
        judge_args: JudgeArgs,
    },
    /// Render the HTML report from every recorded eval result (test suite
    /// runs and CLI spot-checks alike).
    Report {
        /// Defaults to this crate's own target/sensemakar-evals-results.
        #[arg(long)]
        results_dir: Option<PathBuf>,
        #[arg(long, default_value = "sensemakar-evals-report.html")]
        out: PathBuf,
    },
}

fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("failed to parse {} as JSON", path.display()))
}

fn print_verdict(verdict: &JudgeVerdict) {
    for point in &verdict.points {
        let mark = if point.passed { "PASS" } else { "FAIL" };
        println!("[{mark}] {}", point.criterion);
        println!("      {}", point.rationale);
    }
}

fn print_score(score: f64, threshold: f64) {
    let mark = if score >= threshold { "PASS" } else { "FAIL" };
    println!(
        "\n{mark}: score {:.0}% (threshold {:.0}%)",
        score * 100.0,
        threshold * 100.0
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            task,
            input,
            themes,
            rubric,
            case,
            threshold,
            results_dir,
            model_args,
            judge_args,
        } => {
            let blueprint = RubricBlueprint::load(&rubric)
                .with_context(|| format!("failed to load rubric {}", rubric.display()))?;
            let rubric_case = blueprint
                .case(&case)
                .with_context(|| format!("no case '{case}' in {}", rubric.display()))?;

            let model = model_args.selection().build();
            let judge = judge_args.build();

            let verdict = match task {
                Task::ThemeExtraction => {
                    let statements: Vec<Statement> = load_json(&input)?;
                    let extractor = ThemeExtractor::builder().allow_additional_themes(true).build();
                    let output = extractor
                        .run_with_model(statements.clone(), model)
                        .await
                        .context("theme extraction failed")?;
                    evaluate_case(&judge, &blueprint.description, &statements, &output, rubric_case).await?
                }
                Task::ThemeMerge => {
                    let candidates: Vec<Theme> = load_json(&input)?;
                    let merger = ThemeMerger::builder().build();
                    let output = merger
                        .merge_with_model(candidates.clone(), model)
                        .await
                        .context("theme merge failed")?;
                    evaluate_case(&judge, &blueprint.description, &candidates, &output, rubric_case).await?
                }
                Task::ClassifyStatements => {
                    let statements: Vec<Statement> = load_json(&input)?;
                    let classifier = StatementTypeClassifier::builder().build();
                    let output = classifier
                        .run_with_model(statements.clone(), model)
                        .await
                        .context("statement classification failed")?;
                    evaluate_case(&judge, &blueprint.description, &statements, &output, rubric_case).await?
                }
                Task::AssignThemes => {
                    let statements: Vec<Statement> = load_json(&input)?;
                    let themes_path = themes.context("--themes is required for assign-themes")?;
                    let themes: Vec<Theme> = load_json(&themes_path)?;
                    let assigner = StatementThemeAssigner::builder().themes(themes.into()).build();
                    let output = assigner
                        .run_with_model(statements.clone(), model)
                        .await
                        .context("theme assignment failed")?;
                    evaluate_case(&judge, &blueprint.description, &statements, &output, rubric_case).await?
                }
                Task::WikipollDescribe => {
                    let poll_data: WikiPollData = load_json(&input)?;
                    let describer = WikiPollGroupDescriber::builder().build();
                    let output = describer
                        .run_with_model(&poll_data, model)
                        .await
                        .context("wikipoll description failed")?;
                    evaluate_case(&judge, &blueprint.description, &poll_data, &output, rubric_case).await?
                }
            };

            print_verdict(&verdict);
            let results_dir = results_dir.unwrap_or_else(report::default_results_dir);
            let result = report::record_verdict(
                &results_dir,
                &blueprint.task,
                &blueprint.description,
                &rubric_case.id,
                "spot-check",
                &verdict,
                threshold,
            )?;
            print_score(result.score, result.threshold);
            Ok(())
        }

        Command::Judge {
            input,
            output,
            rubric,
            case,
            threshold,
            results_dir,
            judge_args,
        } => {
            let blueprint = RubricBlueprint::load(&rubric)
                .with_context(|| format!("failed to load rubric {}", rubric.display()))?;
            let rubric_case = blueprint
                .case(&case)
                .with_context(|| format!("no case '{case}' in {}", rubric.display()))?;

            let input_value: Value = load_json(&input)?;
            let output_value: Value = load_json(&output)?;

            let judge = judge_args.build();
            let verdict = evaluate_case(
                &judge,
                &blueprint.description,
                &input_value,
                &output_value,
                rubric_case,
            )
            .await
            .context("judge call failed")?;

            print_verdict(&verdict);
            let results_dir = results_dir.unwrap_or_else(report::default_results_dir);
            let result = report::record_verdict(
                &results_dir,
                &blueprint.task,
                &blueprint.description,
                &rubric_case.id,
                "spot-check",
                &verdict,
                threshold,
            )?;
            print_score(result.score, result.threshold);
            Ok(())
        }

        Command::Report { results_dir, out } => {
            let results_dir = results_dir.unwrap_or_else(report::default_results_dir);
            let tasks = report::load_results(&results_dir).with_context(|| {
                format!("failed to load eval results from {}", results_dir.display())
            })?;
            let html = report::render_html(&tasks)?;
            std::fs::write(&out, html)
                .with_context(|| format!("failed to write report to {}", out.display()))?;
            println!("report written to {}", out.display());
            Ok(())
        }
    }
}
