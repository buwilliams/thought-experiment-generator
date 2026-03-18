use std::io::{self, IsTerminal, Read};
use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use teg::config::Config;
use teg::llm::LlmClient;
use teg::types::{
    Layer, ProblemMeta, ProblemSet, ProblemSetMeta, ProblemSource, PROBLEMSET_MAX_SIZE,
};

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

    /// Minimum score for candidate problems to be admitted to the problem set
    #[arg(long, global = true, default_value_t = 0.6)]
    pub problem_admission_threshold: f64,

    /// Minimum run count before a tool or problem is eligible for promotion or demotion
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
    /// Run one full cycle on a problem set
    Run {
        /// Problem set ID to run on (uses the only set if omitted)
        #[arg(long)]
        problemset: Option<String>,

        /// Add a new problem to the problem set before running
        #[arg(long)]
        problem: Option<String>,
    },

    /// Display the last run summary without running
    Read,

    /// Create a new problem set
    CreateProblemset {
        /// Human-readable title (used to derive the ID slug)
        #[arg(long)]
        title: String,

        /// Short description of the set's theme
        #[arg(long)]
        description: Option<String>,
    },

    /// Add a problem to a problem set
    AddProblem {
        /// Problem set ID
        #[arg(long)]
        problemset: String,

        /// Problem text
        #[arg(long)]
        text: String,
    },

    /// Remove a problem from a problem set (problem remains in global DB)
    RemoveProblem {
        /// Problem set ID
        #[arg(long)]
        problemset: String,

        /// Problem ID to remove
        #[arg(long)]
        problem_id: String,
    },

    /// List all problem sets and their contents
    ListProblemsets,

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
        Command::Run { problemset, problem } => {
            teg::runner::run(client, &config, problemset.as_deref(), problem.as_deref()).await?;
        }

        Command::Read => {
            teg::runner::read(&config).await?;
        }

        Command::CreateProblemset { title, description } => {
            teg::state::ensure_initialized()?;
            let id = teg::state::slugify(&title);
            if teg::state::problemset_exists(&id) {
                anyhow::bail!("Problem set '{}' already exists.", id);
            }
            let ps = ProblemSet {
                meta: ProblemSetMeta {
                    id: id.clone(),
                    problem_ids: vec![],
                    run_count: 0,
                    created_at: teg::state::now_iso8601(),
                },
                title: title.clone(),
                summary: description.unwrap_or_default(),
            };
            teg::state::save_problemset(&ps)?;
            println!("Created problem set '{}': {}", id, title);
        }

        Command::AddProblem { problemset, text } => {
            teg::state::ensure_initialized()?;
            let mut ps = teg::state::load_problemset(&problemset)?;
            if ps.meta.problem_ids.len() >= PROBLEMSET_MAX_SIZE {
                anyhow::bail!(
                    "Problem set '{}' is at capacity ({} problems). Remove a problem first with:\n  cargo run -- remove-problem --problemset {} --problem-id <id>",
                    ps.meta.id, PROBLEMSET_MAX_SIZE, ps.meta.id
                );
            }
            let id = teg::state::slugify(&text.chars().take(60).collect::<String>());
            if ps.meta.problem_ids.contains(&id) {
                println!("Problem already in set '{}'.", ps.meta.id);
                return Ok(());
            }
            if !teg::state::problem_exists(&id) {
                let count = teg::state::load_problems()?.len();
                let problem = teg::types::Problem {
                    meta: ProblemMeta {
                        id: id.clone(),
                        source: ProblemSource::User,
                        score: 0.0,
                        rank: count as u32 + 1,
                        run_count: 0,
                        created_at: teg::state::now_iso8601(),
                    },
                    title: text.chars().take(80).collect(),
                    summary: text.chars().take(200).collect(),
                    full_text: text.clone(),
                };
                teg::state::save_problem(&problem)?;
            }
            ps.meta.problem_ids.push(id.clone());
            teg::state::save_problemset(&ps)?;
            println!("Added problem to '{}': {}", ps.meta.id, &text[..text.len().min(60)]);
        }

        Command::RemoveProblem { problemset, problem_id } => {
            teg::state::ensure_initialized()?;
            let mut ps = teg::state::load_problemset(&problemset)?;
            if !ps.meta.problem_ids.contains(&problem_id) {
                anyhow::bail!("Problem '{}' not found in set '{}'.", problem_id, ps.meta.id);
            }
            ps.meta.problem_ids.retain(|id| id != &problem_id);
            teg::state::save_problemset(&ps)?;
            println!("Removed '{}' from set '{}'.", problem_id, ps.meta.id);
            println!("(Problem remains in global DB — it can be added to another set.)");
        }

        Command::ListProblemsets => {
            teg::state::ensure_initialized()?;
            let sets = teg::state::load_problemsets()?;
            if sets.is_empty() {
                println!("No problem sets found. Create one with:");
                println!("  cargo run -- create-problemset --title \"...\"");
                return Ok(());
            }
            for ps in &sets {
                println!(
                    "[{}] {} — {} problem(s), {} run(s)",
                    ps.meta.id,
                    ps.title,
                    ps.meta.problem_ids.len(),
                    ps.meta.run_count,
                );
                for id in &ps.meta.problem_ids {
                    println!("  • {}", id);
                }
            }
        }

        Command::AddTool { layer, title, text, file } => {
            let layer: Layer = layer.parse()?;

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
            let mind = teg::state::load_tools(&Layer::Mind)?;
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
