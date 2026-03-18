use std::sync::Arc;

use anyhow::Result;
use futures::future::join_all;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::promoter;
use crate::prompts::PromptTemplates;
use crate::state;
use crate::types::{Conjecture, Layer};

const TOP_CANDIDATES: usize = 3;

pub async fn ask(
    client: Arc<LlmClient>,
    config: &Config,
    templates: &PromptTemplates,
    question: &str,
) -> Result<()> {
    state::ensure_initialized()?;

    let mind = state::load_conjectures(&Layer::Mind)?;
    let mut candidates = state::load_conjectures(&Layer::Candidates)?;

    if mind.is_empty() && candidates.is_empty() {
        anyhow::bail!("No conjectures found. Run first to build up the conjecture pool.");
    }

    let mind_system = templates.format_mind_system(&mind);

    // Sort candidates by composite score descending, take top N
    candidates.sort_by(|a, b| {
        promoter::composite(b)
            .partial_cmp(&promoter::composite(a))
            .unwrap()
    });
    candidates.truncate(TOP_CANDIDATES);

    // Gather all lenses: all mind conjectures + top candidates
    let lenses: Vec<&Conjecture> = mind.iter().chain(candidates.iter()).collect();

    if lenses.is_empty() {
        anyhow::bail!("No conjectures available.");
    }

    // Run each lens concurrently
    let futures: Vec<_> = lenses
        .iter()
        .map(|c| {
            let client = Arc::clone(&client);
            let p = templates.ask(&mind_system, &c.summary, question);
            let title = c.title.clone();
            let temp = config.temperature;
            async move {
                let answer = client.call_raw(Some(&p.system), &p.user, temp).await?;
                Ok::<(String, String), anyhow::Error>((title, answer))
            }
        })
        .collect();

    let results: Vec<Result<(String, String)>> = join_all(futures).await;

    // Format perspectives block
    let mut perspectives = String::new();
    let mut lens_labels: Vec<String> = vec![];
    for result in results {
        match result {
            Ok((title, answer)) => {
                perspectives.push_str(&format!("=== {} ===\n{}\n\n", title, answer));
                lens_labels.push(title);
            }
            Err(e) => {
                eprintln!("Warning: perspective failed: {e}");
            }
        }
    }

    if perspectives.is_empty() {
        anyhow::bail!("All perspective calls failed.");
    }

    // Consolidate
    let p = templates.ask_consolidate(&mind_system, question, perspectives.trim_end());
    let synthesis = client
        .call_raw(Some(&p.system), &p.user, config.temperature)
        .await?;

    println!("{}\n", synthesis);
    println!("---");
    println!("Lenses used: {}", lens_labels.join(", "));

    Ok(())
}
