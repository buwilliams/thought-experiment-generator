use std::io::{self, Write};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use teg::config::Config;
use teg::engine::tree_runner;
use teg::llm::LlmClient;

#[derive(Parser)]
#[command(name = "teg", about = "Thought Experiment Generator — depth-bounded branching search over explanation space")]
pub struct Cli {
    /// Topic to explore
    pub topic: Option<String>,

    /// Depth limit per branch
    #[arg(long, default_value_t = 10)]
    pub depth: u32,

    /// Number of root branches
    #[arg(long, default_value_t = 10)]
    pub branches: u32,

    /// Survivor score threshold
    #[arg(long, default_value_t = 0.7)]
    pub threshold: f64,

    /// Novel pool admission threshold
    #[arg(long, default_value_t = 0.85)]
    pub novel_threshold: f64,

    /// Background/universal ratio
    #[arg(long, default_value_t = 0.5)]
    pub ratio: f64,

    /// Max draws per depth before moving on
    #[arg(long, default_value_t = 100)]
    pub draws: u32,

    /// LLM temperature for generation
    #[arg(long, default_value_t = 1.2)]
    pub temperature: f64,

    /// Objects per thought experiment atom
    #[arg(long, default_value_t = 4)]
    pub objects: u32,

    /// Relationships per thought experiment atom
    #[arg(long, default_value_t = 3)]
    pub relationships: u32,

    /// Properties per thought experiment atom
    #[arg(long, default_value_t = 2)]
    pub properties: u32,

    /// LLM provider: "anthropic", "anthropic-token" (Max subscription), or "openai"
    #[arg(long, default_value = "anthropic")]
    pub provider: String,

    /// Model name
    #[arg(long, default_value = "claude-sonnet-4-6")]
    pub model: String,

    /// Max concurrent LLM calls
    #[arg(long, default_value_t = 5)]
    pub max_concurrent: usize,

    /// Max total LLM calls (budget cap, 0 = unlimited)
    #[arg(long, default_value_t = 500)]
    pub max_calls: u64,

    /// Output format: "text" or "json"
    #[arg(long, default_value = "text")]
    pub output: String,
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
        cli.depth,
        cli.branches,
        cli.threshold,
        cli.novel_threshold,
        cli.ratio,
        cli.draws,
        cli.temperature,
        cli.objects,
        cli.relationships,
        cli.properties,
        cli.max_concurrent,
        if cli.max_calls == 0 { None } else { Some(cli.max_calls) },
    )?;

    // Get topic interactively if not provided
    let topic = match cli.topic {
        Some(t) => t,
        None => {
            print!("What topic would you like to explore? ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    if topic.is_empty() {
        anyhow::bail!("Topic cannot be empty");
    }

    println!("\n=== Thought Experiment Generator ===");
    println!("Topic: {topic}");
    let budget_str = match config.max_calls {
        Some(n) => format!("{n}"),
        None => "unlimited".to_string(),
    };
    println!(
        "Config: depth={}, branches={}, threshold={:.2}, draws/depth={}, max_calls={}\n",
        config.depth_limit, config.num_branches, config.survivor_threshold, config.draws_per_depth, budget_str
    );

    let llm_client = Arc::new(LlmClient::new(config.llm.clone(), config.max_calls));

    let tree = tree_runner::run_tree(Arc::clone(&llm_client), &config, &topic).await?;

    println!("LLM calls used: {}", llm_client.calls_made());

    // Output results
    if cli.output == "json" {
        println!("{}", serde_json::to_string_pretty(&tree.branches)?);
    } else {
        print_results(&tree);
    }

    Ok(())
}

fn print_results(tree: &teg::types::Tree) {
    println!("\n{}", "=".repeat(60));
    println!("=== RESULTS ===\n");

    if tree.branches.is_empty() {
        println!("No branches completed.");
        return;
    }

    for (i, branch_id) in tree.top_trajectories.iter().enumerate() {
        let branch = match tree.branches.iter().find(|b| &b.id == branch_id) {
            Some(b) => b,
            None => continue,
        };

        let is_cross = !branch.parent_branch_ids.is_empty();
        let label = if is_cross {
            "Cross-pollination"
        } else {
            "Trajectory"
        };

        println!(
            "#{} — {} (score: {:.2})",
            i + 1,
            label,
            branch.trajectory_score.unwrap_or(0.0)
        );
        println!("{}", "-".repeat(50));

        for node in &branch.nodes {
            println!(
                "  [depth {}, score {:.2}]",
                node.depth, node.deutsch_score.overall_score
            );
            for line in node.thought_experiment.lines() {
                println!("    {line}");
            }
            if let Some(tension) = &node.unresolved_tension {
                println!("    -> Tension: {}", tension.tension);
            }
            println!();
        }
        println!();
    }

    let total_nodes: usize = tree.branches.iter().map(|b| b.nodes.len()).sum();
    println!("--- Summary ---");
    println!("Total branches: {}", tree.branches.len());
    println!("Total nodes: {total_nodes}");
    println!("Cross-pollinations: {}", tree.cross_pollinations.len());
    println!("Novel pool size: {} quads", tree.draw_pool.novel.len());
}
