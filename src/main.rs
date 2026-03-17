use std::io::{self, Write};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use teg::config::Config;
use teg::llm::LlmClient;

#[derive(Parser)]
#[command(name = "teg", about = "Thought Experiment Generator")]
pub struct Cli {
    /// Topic to explore
    pub topic: Option<String>,

    /// Number of thought experiments to generate
    #[arg(long, default_value_t = 20)]
    pub experiments: u32,

    /// Background sentences to use per thought experiment
    #[arg(long, default_value_t = 2)]
    pub background: u32,

    /// Generated sentences to use per thought experiment
    #[arg(long, default_value_t = 2)]
    pub generated: u32,

    /// Random words per line in words.txt
    #[arg(long, default_value_t = 2)]
    pub words: u32,

    /// Total sentences in the background and generated pools
    #[arg(long, default_value_t = 50)]
    pub pool_size: usize,

    /// LLM temperature for generation (0.0-1.0)
    #[arg(long, default_value_t = 1.0)]
    pub temperature: f64,

    /// LLM provider: "anthropic", "anthropic-token", or "openai"
    #[arg(long, default_value = "anthropic")]
    pub provider: String,

    /// Model name
    #[arg(long, default_value = "claude-sonnet-4-6")]
    pub model: String,

    /// Max concurrent LLM calls
    #[arg(long, default_value_t = 5)]
    pub max_concurrent: usize,

    /// Clear cache and start fresh
    #[arg(long, default_value_t = false)]
    pub fresh: bool,

    /// Read cached results and display summary without running
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
        cli.experiments,
        cli.background,
        cli.generated,
        cli.words,
        cli.pool_size,
        cli.temperature,
        cli.max_concurrent,
    )?;

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

    if cli.fresh {
        teg::cache::clear_cache(&topic)?;
    }

    let client = Arc::new(LlmClient::new(config.llm.clone()));

    if cli.read {
        teg::runner::read(client, &config, &topic).await?;
    } else {
        println!("\n=== Thought Experiment Generator ===");
        println!("Topic: {topic}");
        println!(
            "Config: experiments={}, pool={}, background={}, generated={}, words={}\n",
            config.num_experiments, config.pool_size, config.num_background, config.num_generated, config.num_words
        );
        teg::runner::run(client.clone(), &config, &topic).await?;
        println!("\nLLM calls used: {}", client.calls_made());
    }

    Ok(())
}
