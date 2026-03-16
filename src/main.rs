use std::io::{self, Write};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use teg::config::Config;
use teg::engine::tree_runner;
use teg::llm::LlmClient;
use teg::types::Tree;

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
    #[arg(long, default_value_t = 0.6)]
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

    /// LLM temperature for generation (0.0-1.0)
    #[arg(long, default_value_t = 1.0)]
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

    /// Clear cache and start fresh
    #[arg(long, default_value_t = false)]
    pub fresh: bool,

    /// Read cached results and display without running
    #[arg(long, default_value_t = false)]
    pub read: bool,
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

    if cli.read {
        match teg::cache::load_tree_state(&topic) {
            Some(cached) => {
                let tree = teg::engine::tree_runner::build_tree(
                    &topic, cached.draw_pool, cached.branches, vec![],
                )?;
                if cli.output == "json" {
                    println!("{}", serde_json::to_string_pretty(&tree.branches)?);
                } else {
                    print_results(&tree);
                }
                return Ok(());
            }
            None => {
                anyhow::bail!("No cached results found for this topic. Run without --read first.");
            }
        }
    }

    if cli.fresh {
        teg::cache::clear_cache(&topic)?;
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

fn print_results(tree: &Tree) {
    let total_nodes: usize = tree.branches.iter().map(|b| b.nodes.len()).sum();

    println!("\n{}", "=".repeat(60));
    println!("RESULTS\n");

    if tree.branches.is_empty() {
        println!("No branches completed.");
        return;
    }

    // Summary table
    println!("Branches: {}  |  Nodes: {}  |  Novel quads: {}  |  Cross-pollinations: {}\n",
        tree.branches.len(), total_nodes, tree.draw_pool.novel.len(), tree.cross_pollinations.len());

    println!("{:<8} {:<10} {:<8} {:<12} {}",
        "Rank", "Traj.", "Best", "Depths", "Root thought experiment");
    println!("{}", "-".repeat(80));

    for (i, branch) in tree.branches.iter().enumerate() {
        let traj = branch.trajectory_score.unwrap_or(0.0);
        let best = branch.nodes.iter()
            .map(|n| n.deutsch_score.overall_score)
            .fold(0.0f64, f64::max);
        let depths = format!("{}/{}", branch.nodes.len(), branch.depth_limit);

        // First line of root thought experiment as preview
        let preview = branch.nodes.first()
            .map(|n| {
                let te = n.thought_experiment.trim();
                // Skip markdown headers
                let text = te.lines()
                    .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
                    .unwrap_or(te.lines().next().unwrap_or(""));
                let text = text.trim();
                if text.len() > 50 {
                    format!("{}...", &text[..50])
                } else {
                    text.to_string()
                }
            })
            .unwrap_or_default();

        println!("#{:<7} {:<10.2} {:<8.2} {:<12} {}",
            i + 1, traj, best, depths, preview);
    }

    // Top 3 detailed view
    println!("\n{}", "=".repeat(60));
    println!("TOP TRAJECTORIES\n");

    for (i, branch_id) in tree.top_trajectories.iter().enumerate() {
        let branch = match tree.branches.iter().find(|b| &b.id == branch_id) {
            Some(b) => b,
            None => continue,
        };

        let is_cross = !branch.parent_branch_ids.is_empty();
        let label = if is_cross { "Cross-pollination" } else { "Trajectory" };

        println!(
            "#{} {} (trajectory score: {:.2})\n",
            i + 1,
            label,
            branch.trajectory_score.unwrap_or(0.0)
        );

        // Show the chain as: title + score + tension for each depth
        for node in &branch.nodes {
            // Extract title (first non-empty line, often a markdown header)
            let title = node.thought_experiment.lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or("(untitled)")
                .trim()
                .trim_start_matches('#')
                .trim();

            print!(
                "  depth {} [{:.2}] {}",
                node.depth, node.deutsch_score.overall_score, title
            );

            if let Some(tension) = &node.unresolved_tension {
                let t = &tension.tension;
                let short = if t.len() > 80 {
                    format!("{}...", &t[..80])
                } else {
                    t.clone()
                };
                println!("\n           -> {short}");
            } else {
                println!();
            }
        }

        println!();
    }

    // Show the single best node
    let best_node = tree.branches.iter()
        .flat_map(|b| b.nodes.iter())
        .max_by(|a, b| a.deutsch_score.overall_score.partial_cmp(&b.deutsch_score.overall_score).unwrap());

    if let Some(node) = best_node {
        println!("{}", "=".repeat(60));
        println!("BEST SINGLE THOUGHT EXPERIMENT (score: {:.2})\n", node.deutsch_score.overall_score);
        println!("{}", node.thought_experiment);
        println!();
    }
}
