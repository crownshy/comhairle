mod statement_data_loader;
mod theme_data_loader;
mod wiki_poll_data_loader;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use language_tags::LanguageTag;
use rig::client::CompletionClient;
use rig::providers::ollama;
use sensemakar::statement_theme_assigner::StatementThemeAssigner;
use sensemakar::theme_extraction::ThemeExtractor;
use sensemakar::wikipoll_describer::WikiPollGroupDescriber;
use sensemakar_types::{Statement, WikiPollData};

/// Shape of the `--input` file for tasks that work over generic statements.
#[derive(Clone, Copy, ValueEnum)]
enum InputFormat {
    /// A `Statement` list: `id`, `text`, optional `speaker_id`/`lang`.
    Statement,
    /// A WikiPoll/Polis statement list: `statement` text plus vote counts
    /// (same shape accepted by `polis-report`). Votes are ignored; each
    /// statement's position in the file is used as its id.
    Wikipoll,
}

fn load_input_statements(
    path: &std::path::Path,
    format: InputFormat,
    lang: &LanguageTag,
) -> Result<Vec<Statement>> {
    match format {
        InputFormat::Statement => statement_data_loader::load_statements(path, lang)
            .with_context(|| format!("failed to load statements from {}", path.display())),
        InputFormat::Wikipoll => {
            let statements = wiki_poll_data_loader::load_statements(path)
                .with_context(|| format!("failed to load statements from {}", path.display()))?;
            Ok(wiki_poll_data_loader::to_statements(statements, lang))
        }
    }
}

#[derive(Parser)]
#[command(name = "sensemakar", about = "CLI for the sensemakar deliberation analysis tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Flags shared by every subcommand for talking to the completion model.
#[derive(Args)]
struct ModelArgs {
    /// Ollama model name to use.
    #[arg(long, default_value = "qwen3-long")]
    model: String,

    /// Base URL of the Ollama server.
    #[arg(long, default_value = "http://localhost:11434")]
    base_url: String,

    /// API key for the Ollama server, if required.
    #[arg(long)]
    api_key: Option<String>,
}

impl ModelArgs {
    fn build_model(&self) -> Result<ollama::CompletionModel> {
        let client = ollama::Client::builder()
            .base_url(&self.base_url)
            .api_key(self.api_key.as_deref().unwrap_or(""))
            .build()
            .context("failed to build ollama client")?;
        Ok(client.completion_model(&self.model))
    }
}

#[derive(Subcommand)]
enum Command {
    /// Run the Polis group-description report over a set of statements.
    PolisReport {
        /// Path to the input data file (.csv or .json).
        #[arg(short, long)]
        input: PathBuf,

        /// Title of the poll/conversation, used as context for the model.
        #[arg(short, long)]
        title: String,

        /// Optional free-text context to prime the model with.
        #[arg(long)]
        context: Option<String>,

        /// Optional additional instructions for the model.
        #[arg(long)]
        additional_instructions: Option<String>,

        /// Where to write the resulting JSON report. Defaults to stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,

        #[command(flatten)]
        model_args: ModelArgs,
    },

    /// Extract themes from a set of statements.
    GenerateThemes {
        /// Path to the input statements file (.csv or .json).
        #[arg(short, long)]
        input: PathBuf,

        /// Shape of the input file.
        #[arg(long, value_enum, default_value_t = InputFormat::Statement)]
        format: InputFormat,

        /// Path to a file of pre-existing themes (.csv or .json) to extend
        /// rather than start from scratch.
        #[arg(long)]
        existing_themes: Option<PathBuf>,

        /// Allow the model to propose themes beyond the existing set.
        #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
        allow_additional_themes: bool,

        /// Minimum number of themes to extract.
        #[arg(long)]
        min_themes: Option<u32>,

        /// Maximum number of themes to extract.
        #[arg(long)]
        max_themes: Option<u32>,

        /// Optional free-text context to prime the model with.
        #[arg(long)]
        context: Option<String>,

        /// Optional additional instructions for the model.
        #[arg(long)]
        additional_instructions: Option<String>,

        /// Process statements in batches of this size, folding themes found
        /// in each batch into the next. Defaults to running all statements
        /// in a single pass.
        #[arg(long)]
        batch_size: Option<usize>,

        /// BCP-47 language tag to use for CSV rows without a 'lang' column.
        #[arg(long, default_value = "en")]
        lang: String,

        /// Where to write the resulting JSON theme list. Defaults to stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,

        #[command(flatten)]
        model_args: ModelArgs,
    },

    /// Assign each statement in a set to one of a given list of themes.
    AssignThemes {
        /// Path to the input statements file (.csv or .json).
        #[arg(short, long)]
        input: PathBuf,

        /// Shape of the input file.
        #[arg(long, value_enum, default_value_t = InputFormat::Statement)]
        format: InputFormat,

        /// Path to the themes file (.csv or .json) to assign statements to.
        #[arg(short, long)]
        themes: PathBuf,

        /// Optional free-text context to prime the model with.
        #[arg(long)]
        context: Option<String>,

        /// Optional additional instructions for the model.
        #[arg(long)]
        additional_instructions: Option<String>,

        /// BCP-47 language tag to use for CSV rows without a 'lang' column.
        #[arg(long, default_value = "en")]
        lang: String,

        /// Where to write the resulting JSON theme assignments. Defaults to stdout.
        #[arg(short, long)]
        output: Option<PathBuf>,

        #[command(flatten)]
        model_args: ModelArgs,
    },
}

fn write_output<T: serde::Serialize>(value: &T, output: Option<PathBuf>) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    match output {
        Some(path) => {
            std::fs::write(&path, json)
                .with_context(|| format!("failed to write output to {}", path.display()))?;
            println!("output written to {}", path.display());
        }
        None => println!("{json}"),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::PolisReport {
            input,
            title,
            context,
            additional_instructions,
            output,
            model_args,
        } => {
            let statements = wiki_poll_data_loader::load_statements(&input)
                .with_context(|| format!("failed to load statements from {}", input.display()))?;
            println!("loaded {} statements", statements.len());

            let poll_data = WikiPollData { title, statements };
            let completion_model = model_args.build_model()?;

            let describer = WikiPollGroupDescriber {
                context,
                additional_instructions,
            };

            let result = describer
                .run_with_model(&poll_data, completion_model)
                .await
                .context("failed to generate polis report")?;

            write_output(&result, output)
        }

        Command::GenerateThemes {
            input,
            format,
            existing_themes,
            allow_additional_themes,
            min_themes,
            max_themes,
            context,
            additional_instructions,
            batch_size,
            lang,
            output,
            model_args,
        } => {
            let lang = LanguageTag::parse(&lang).with_context(|| format!("invalid language tag '{lang}'"))?;
            let statements = load_input_statements(&input, format, &lang)?;
            println!("loaded {} statements", statements.len());

            let existing_themes = existing_themes
                .map(|path| theme_data_loader::load_themes(&path))
                .transpose()
                .context("failed to load existing themes")?;

            let completion_model = model_args.build_model()?;

            let mut extractor = ThemeExtractor::builder()
                .maybe_existing_themes(existing_themes)
                .allow_additional_themes(allow_additional_themes)
                .maybe_context(context)
                .maybe_additional_instructions(additional_instructions)
                .maybe_max_themes(max_themes)
                .maybe_min_themes(min_themes)
                .build();

            let themes = match batch_size {
                Some(batch_size) => {
                    extractor
                        .run_in_batches_with_model(statements, completion_model, batch_size)
                        .await
                }
                None => extractor.run_with_model(statements, completion_model).await,
            }
            .context("failed to generate themes")?;

            write_output(&themes, output)
        }

        Command::AssignThemes {
            input,
            format,
            themes,
            context,
            additional_instructions,
            lang,
            output,
            model_args,
        } => {
            let lang = LanguageTag::parse(&lang).with_context(|| format!("invalid language tag '{lang}'"))?;
            let statements = load_input_statements(&input, format, &lang)?;
            println!("loaded {} statements", statements.len());

            let themes = theme_data_loader::load_themes(&themes)
                .with_context(|| format!("failed to load themes from {}", themes.display()))?;
            println!("loaded {} themes", themes.len());

            let completion_model = model_args.build_model()?;

            let assigner = StatementThemeAssigner::builder()
                .maybe_context(context)
                .maybe_additional_instructions(additional_instructions)
                .themes(themes.into())
                .build();

            let result = assigner
                .run_with_model(statements, completion_model)
                .await
                .context("failed to assign themes")?;

            write_output(&result, output)
        }
    }
}
