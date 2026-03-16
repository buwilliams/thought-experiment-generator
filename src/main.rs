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

    /// Show full details for a specific node (e.g. --show 3.5 for branch 3, depth 5)
    #[arg(long)]
    pub show: Option<String>,
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

    if cli.read || cli.show.is_some() {
        let cached = teg::cache::load_tree_state(&topic)
            .ok_or_else(|| anyhow::anyhow!("No cached results found for this topic. Run without --read first."))?;
        let tree = teg::engine::tree_runner::build_tree(
            &topic, cached.draw_pool, cached.branches, vec![],
        )?;

        if let Some(key) = &cli.show {
            print_node_detail(&tree, key)?;
        } else if cli.output == "json" {
            println!("{}", serde_json::to_string_pretty(&tree.branches)?);
        } else {
            print_results(&tree);
        }
        return Ok(());
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

fn node_title(node: &teg::types::Node) -> String {
    node.thought_experiment
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("(untitled)")
        .trim()
        .trim_start_matches('#')
        .trim()
        .to_string()
}

fn print_results(tree: &Tree) {
    if tree.branches.is_empty() {
        println!("\nNo branches completed.");
        return;
    }

    // Collect all nodes with their branch index
    let mut all_nodes: Vec<(usize, &teg::types::Node)> = tree
        .branches
        .iter()
        .enumerate()
        .flat_map(|(bi, b)| b.nodes.iter().map(move |n| (bi + 1, n)))
        .collect();

    all_nodes.sort_by(|a, b| {
        b.1.deutsch_score
            .overall_score
            .partial_cmp(&a.1.deutsch_score.overall_score)
            .unwrap()
    });

    let total_nodes: usize = tree.branches.iter().map(|b| b.nodes.len()).sum();
    println!(
        "\n{} branches, {} thought experiments, {} novel quads\n",
        tree.branches.len(),
        total_nodes,
        tree.draw_pool.novel.len()
    );

    println!("{:<8} {:<8} {}", "Key", "Score", "Title");
    println!("{}", "-".repeat(70));

    for (branch_idx, node) in &all_nodes {
        let key = format!("{}.{}", branch_idx, node.depth);
        let title = node_title(node);
        println!(
            "{:<8} {:<8.2} {}",
            key, node.deutsch_score.overall_score, title
        );
    }

    println!("\nUse --show <key> to see full details, e.g. --show 1.5");
}

fn print_node_detail(tree: &Tree, key: &str) -> anyhow::Result<()> {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid key format. Use <branch>.<depth>, e.g. 3.5");
    }

    let branch_idx: usize = parts[0]
        .parse::<usize>()
        .map_err(|_| anyhow::anyhow!("Invalid branch number"))?;
    let depth: u32 = parts[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid depth number"))?;

    if branch_idx == 0 || branch_idx > tree.branches.len() {
        anyhow::bail!(
            "Branch {} not found. Valid range: 1-{}",
            branch_idx,
            tree.branches.len()
        );
    }

    let branch = &tree.branches[branch_idx - 1];
    let node = branch
        .nodes
        .iter()
        .find(|n| n.depth == depth)
        .ok_or_else(|| anyhow::anyhow!("Depth {} not found in branch {}", depth, branch_idx))?;

    println!("\n[{}.{}] Score: {:.2}\n", branch_idx, depth, node.deutsch_score.overall_score);
    println!("{}\n", node.thought_experiment);

    let ds = &node.deutsch_score;
    println!("Deutsch score breakdown:");
    println!("  Hard to vary:        {:.2}", ds.hard_to_vary);
    println!("  Reach:               {:.2}", ds.reach);
    println!("  Minimal assumptions: {:.2}", ds.minimal_assumptions);
    println!("  Tension resolution:  {:.2}", ds.tension_resolution);
    println!("  Overall:             {:.2}", ds.overall_score);
    println!("  Justification: {}", ds.justification);

    if let Some(tension) = &node.unresolved_tension {
        println!("\nUnresolved tension: {}", tension.tension);
    }

    Ok(())
}
