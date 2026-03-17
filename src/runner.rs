use std::sync::Arc;

use anyhow::Result;
use rand::prelude::*;
use rand::rngs::StdRng;
use tracing::info;

use crate::cache;
use crate::config::Config;
use crate::llm::LlmClient;
use crate::prompts;
use crate::types::Critique;

/// Words file path.
const WORDS_ALPHA_PATH: &str = "data/words_alpha.txt";

pub async fn run(client: Arc<LlmClient>, config: &Config, topic: &str) -> Result<()> {
    // Phase 1: Create
    let background = ensure_background(&client, config, topic).await?;
    let words = ensure_words(config, topic)?;
    let generated = ensure_generated(&client, config, topic, &words).await?;

    // Phase 2 + 3: Combine & Criticize (one per experiment, concurrent)
    let mut handles = vec![];
    for i in 1..=config.num_experiments {
        if cache::experiment_exists(topic, i) && cache::critique_exists(topic, i) {
            info!("Experiment {:03}: already complete, skipping", i);
            continue;
        }
        let client = Arc::clone(&client);
        let topic = topic.to_string();
        let bg = background.clone();
        let generated_clone = generated.clone();
        let config = config.clone();
        handles.push(tokio::spawn(async move {
            run_single_experiment(&client, &config, &topic, i, &bg, &generated_clone).await
        }));
    }
    for handle in handles {
        handle.await??;
    }

    // Phase 4: Results
    generate_summary(&client, config, topic).await
}

/// Read existing files and regenerate summary without running experiments.
pub async fn read(client: Arc<LlmClient>, config: &Config, topic: &str) -> Result<()> {
    generate_summary(&client, config, topic).await
}

async fn ensure_background(
    client: &LlmClient,
    config: &Config,
    topic: &str,
) -> Result<Vec<String>> {
    if let Some(lines) = cache::load_background(topic) {
        info!("Loaded background.txt ({} sentences)", lines.len());
        return Ok(lines);
    }
    info!("Generating background sentences...");
    let p = prompts::background_generation(topic, config.pool_size);
    let raw = client.call_raw(Some(p.system), &p.user, config.temperature).await?;
    let lines: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    cache::save_background(topic, &lines)?;
    Ok(lines)
}

fn ensure_words(config: &Config, topic: &str) -> Result<Vec<String>> {
    if let Some(lines) = cache::load_words(topic) {
        info!("Loaded words.txt ({} lines)", lines.len());
        return Ok(lines);
    }
    info!("Generating words.txt...");
    let word_list = load_word_list()?;
    let mut rng = StdRng::from_entropy();
    let mut lines = Vec::with_capacity(config.pool_size);
    for _ in 0..config.pool_size {
        let words: Vec<&str> = (0..config.num_words as usize)
            .map(|_| word_list[rng.gen_range(0..word_list.len())].as_str())
            .collect();
        lines.push(words.join(" "));
    }
    cache::save_words(topic, &lines)?;
    Ok(lines)
}

async fn ensure_generated(
    client: &LlmClient,
    config: &Config,
    topic: &str,
    words: &[String],
) -> Result<Vec<String>> {
    if let Some(lines) = cache::load_generated(topic) {
        info!("Loaded generated.txt ({} sentences)", lines.len());
        return Ok(lines);
    }
    info!("Generating sentences from word pairs...");
    let word_lines = words.join("\n");
    let p = prompts::words_to_sentences(&word_lines);
    let raw = client.call_raw(Some(p.system), &p.user, config.temperature).await?;
    let lines: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    cache::save_generated(topic, &lines)?;
    Ok(lines)
}

async fn run_single_experiment(
    client: &LlmClient,
    config: &Config,
    topic: &str,
    n: u32,
    background: &[String],
    generated: &[String],
) -> Result<()> {
    let mut rng = StdRng::from_entropy();

    // Load or generate TE
    let te = if cache::experiment_exists(topic, n) {
        cache::load_experiment(topic, n)
            .ok_or_else(|| anyhow::anyhow!("Failed to load experiment {:03}", n))?
    } else {
        let bg_pick = sample_lines(background, config.num_background as usize, &mut rng);
        let gen_pick = sample_lines(generated, config.num_generated as usize, &mut rng);
        let all_sentences = format!(
            "{}\n{}",
            bg_pick.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"),
            gen_pick.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
        );
        let p = prompts::te_generation(&all_sentences);
        let text = client.call_raw(Some(p.system), &p.user, config.temperature).await?;
        cache::save_experiment(topic, n, &text)?;
        info!("Experiment {:03}: generated thought experiment", n);
        text
    };

    // Criticize if not already done
    if !cache::critique_exists(topic, n) {
        let p = prompts::criticism(&te);
        let critique: Critique = client.call(Some(p.system), &p.user, 0.2).await?;
        cache::save_critique(topic, n, &critique)?;
        info!(
            "Experiment {:03}: criticized (reach={:.2}, novelty={:.2}, falsifiable={:.2}, total={:.2})",
            n, critique.reach, critique.novelty, critique.falsifiable, critique.total_score()
        );
    }

    Ok(())
}

async fn generate_summary(client: &LlmClient, config: &Config, topic: &str) -> Result<()> {
    // Collect all experiments + critiques
    let mut results: Vec<(u32, String, Critique)> = vec![];
    for i in 1..=config.num_experiments {
        if let (Some(te), Some(critique)) =
            (cache::load_experiment(topic, i), cache::load_critique(topic, i))
        {
            results.push((i, te, critique));
        }
    }

    if results.is_empty() {
        anyhow::bail!("No experiments found. Run without --read first.");
    }

    // Sort by total score descending
    results.sort_by(|a, b| {
        b.2.total_score()
            .partial_cmp(&a.2.total_score())
            .unwrap()
    });

    // Build table
    let mut out = String::new();
    out.push_str(&format!("# {topic}\n\n"));
    out.push_str("| # | Experiment | Reach | Novelty | Falsifiable | Total |\n");
    out.push_str("|---|-----------|-------|---------|-------------|-------|\n");
    for (rank, (n, _, c)) in results.iter().enumerate() {
        let name = format!("{:03}-experiment.txt", n);
        let path = format!("experiments/{}", name);
        out.push_str(&format!(
            "| {} | [{}]({}) | {:.2} | {:.2} | {:.2} | {:.2} |\n",
            rank + 1,
            name,
            path,
            c.reach,
            c.novelty,
            c.falsifiable,
            c.total_score(),
        ));
    }

    // Top 5 summaries
    out.push_str("\n## Top 5\n\n");
    for (rank, (n, te, c)) in results.iter().take(5).enumerate() {
        let critique_json = serde_json::to_string_pretty(c)?;
        let p = prompts::summarize(te, &critique_json);
        let summary = client
            .call_raw(Some(p.system), &p.user, 0.3)
            .await
            .unwrap_or_else(|_| "(summary unavailable)".to_string());
        let name = format!("{:03}-experiment.txt", n);
        let path = format!("experiments/{}", name);
        out.push_str(&format!(
            "**{}. [{}]({})**  \n{}\n\n",
            rank + 1,
            name,
            path,
            summary.trim()
        ));
    }

    cache::save_summary(topic, &out)?;

    // Print to stdout
    println!("{out}");

    Ok(())
}

fn sample_lines<'a>(pool: &'a [String], n: usize, rng: &mut StdRng) -> Vec<&'a String> {
    let n = n.min(pool.len());
    pool.choose_multiple(rng, n).collect()
}

fn load_word_list() -> Result<Vec<String>> {
    let text = std::fs::read_to_string(WORDS_ALPHA_PATH)
        .map_err(|_| anyhow::anyhow!("Could not read {WORDS_ALPHA_PATH}"))?;
    let words: Vec<String> = text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && l.chars().all(|c| c.is_alphabetic()))
        .collect();
    if words.is_empty() {
        anyhow::bail!("{WORDS_ALPHA_PATH} is empty or contains no valid words");
    }
    Ok(words)
}
