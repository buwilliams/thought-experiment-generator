use std::sync::Arc;

use anyhow::Result;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::promoter;
use crate::prompts::PromptTemplates;
use crate::state;
use crate::types::{Conjecture, Layer};

pub async fn ask(
    client: Arc<LlmClient>,
    config: &Config,
    templates: &PromptTemplates,
    question: &str,
) -> Result<()> {
    state::ensure_initialized()?;

    let mind = state::load_conjectures(&Layer::Mind)?;
    let candidates = state::load_conjectures(&Layer::Candidates)?;

    if mind.is_empty() && candidates.is_empty() {
        anyhow::bail!("No conjectures found. Run first to build up the conjecture pool.");
    }

    let mind_system = templates.format_mind_system(&mind);

    // Find top conjecture by composite score across mind + candidates combined
    let all: Vec<&Conjecture> = mind.iter().chain(candidates.iter()).collect();
    let top = all
        .iter()
        .max_by(|a, b| {
            promoter::composite(a)
                .partial_cmp(&promoter::composite(b))
                .unwrap()
        })
        .ok_or_else(|| anyhow::anyhow!("No conjectures available"))?;

    let p = templates.ask(&mind_system, &top.summary, question);
    let answer = client.call_raw(Some(&p.system), &p.user, config.temperature).await?;

    println!("{}\n", answer);
    println!("---");
    println!("Lens: {} (composite: {:.2})", top.title, promoter::composite(top));

    Ok(())
}
