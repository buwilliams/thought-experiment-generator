use std::io::{self, IsTerminal, Read};
use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use teg::config::Config;
use teg::llm::LlmClient;

#[derive(Parser)]
#[command(name = "teg", about = "Epistemic Engine — self-improving thought experiment generator")]
pub struct Cli {
    /// LLM provider: "anthropic", "anthropic-token", or "openai"
    #[arg(long, global = true, default_value = "anthropic")]
    pub provider: String,

    /// Model name
    #[arg(long, global = true, default_value = "claude-sonnet-4-6")]
    pub model: String,

    /// Max concurrent LLM calls
    #[arg(long, global = true, default_value_t = 5)]
    pub max_concurrent: usize,

    /// Minimum logical consistency score (0.0–1.0) to proceed to hard-to-vary evaluation
    #[arg(long, global = true, default_value_t = 0.3)]
    pub consistency_threshold: f64,

    /// Minimum score for candidate problems to be admitted to the problem database
    #[arg(long, global = true, default_value_t = 0.6)]
    pub problem_admission_threshold: f64,

    /// Minimum run count before a tool is eligible for promotion or demotion
    #[arg(long, global = true, default_value_t = 3)]
    pub min_run_count: u32,

    /// LLM temperature for generation (0.0–1.0)
    #[arg(long, global = true, default_value_t = 1.0)]
    pub temperature: f64,

    /// Reset state to seed before running
    #[arg(long, global = true, default_value_t = false)]
    pub fresh: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run one full cycle across all problems and perspective tools
    Run {
        /// Add a new problem before running
        #[arg(long)]
        problem: Option<String>,
    },

    /// Display the last run summary without running
    Read,

    /// Add a new tool to the mind or perspectives layer
    AddTool {
        /// Target layer: "mind" or "perspectives"
        #[arg(long)]
        layer: String,

        /// Human-readable title (used to derive the file slug)
        #[arg(long)]
        title: String,

        /// Inline full text of the tool
        #[arg(long)]
        text: Option<String>,

        /// Path to a file whose contents are the full text
        #[arg(long)]
        file: Option<std::path::PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = Config::new(
        &cli.provider,
        &cli.model,
        cli.max_concurrent,
        cli.consistency_threshold,
        cli.problem_admission_threshold,
        cli.min_run_count,
        cli.temperature,
    )?;

    if cli.fresh {
        teg::state::reset_to_seed()?;
    }

    let client = Arc::new(LlmClient::new(config.llm.clone()));

    match cli.command {
        Command::Run { problem } => {
            teg::runner::run(client, &config, problem.as_deref()).await?;
        }

        Command::Read => {
            teg::runner::read(&config).await?;
        }

        Command::AddTool { layer, title, text, file } => {
            let layer: teg::types::Layer = layer.parse()?;

            let full_text = if let Some(t) = text {
                t
            } else if let Some(path) = file {
                std::fs::read_to_string(&path)
                    .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", path.display()))?
            } else if !io::stdin().is_terminal() {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input)?;
                input.trim().to_string()
            } else {
                anyhow::bail!("Provide tool text via --text, --file, or stdin.");
            };

            if full_text.is_empty() {
                anyhow::bail!("Tool text cannot be empty.");
            }

            teg::state::ensure_initialized()?;
            let mind = teg::state::load_tools(&teg::types::Layer::Mind)?;
            let mind_system = teg::prompts::format_mind_system(&mind);

            let p = teg::prompts::summarize_tool(&mind_system, &title, &full_text);
            let resp: teg::types::SummarizeToolResponse =
                client.call(Some(&p.system), &p.user, 0.3).await?;

            let id = teg::state::slugify(&title);
            let count = teg::state::load_tools(&layer)?.len();

            let tool = teg::types::Tool {
                meta: teg::types::ToolMeta {
                    id: id.clone(),
                    layer: layer.clone(),
                    score: 0.0,
                    rank: count as u32 + 1,
                    run_count: 0,
                    problem_coverage: vec![],
                    created_at: teg::state::now_iso8601(),
                    promoted_from: None,
                    history: vec![],
                },
                title: title.clone(),
                summary: resp.summary.clone(),
                full_text,
            };

            teg::state::save_tool(&tool)?;
            println!("Added '{}' to {}.", title, layer);
            println!("Summary: {}", resp.summary);
        }
    }

    Ok(())
}
