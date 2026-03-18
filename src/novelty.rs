use std::sync::Arc;

use anyhow::Result;
use futures::future::join_all;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::prompts::PromptTemplates;
use crate::state;
use crate::types::{Conjecture, Layer, NoveltyResponse};

pub async fn check(
    client: Arc<LlmClient>,
    _config: &Config,
    templates: &PromptTemplates,
) -> Result<()> {
    state::ensure_initialized()?;

    let mind = state::load_conjectures(&Layer::Mind)?;
    let candidates = state::load_conjectures(&Layer::Candidates)?;
    let mind_system = templates.format_mind_system(&mind);

    let all: Vec<&Conjecture> = mind.iter().chain(candidates.iter()).collect();
    if all.is_empty() {
        anyhow::bail!("No conjectures to check.");
    }

    println!("Checking {} conjectures for novelty...\n", all.len());

    let futures: Vec<_> = all
        .iter()
        .map(|c| {
            let client = Arc::clone(&client);
            let p = templates.novelty_check(&mind_system, &c.title, &c.summary);
            let title = c.title.clone();
            let layer = c.meta.layer.clone();
            async move {
                let resp: NoveltyResponse = client.call(Some(&p.system), &p.user, 0.3).await?;
                Ok::<(String, Layer, NoveltyResponse), anyhow::Error>((title, layer, resp))
            }
        })
        .collect();

    let mut entries: Vec<(String, Layer, NoveltyResponse)> = join_all(futures)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // Sort by novelty score descending
    entries.sort_by(|a, b| b.2.novelty_score.partial_cmp(&a.2.novelty_score).unwrap());

    for (title, layer, resp) in &entries {
        let layer_tag = match layer {
            Layer::Mind => "[mind]",
            Layer::Candidates => "[cand]",
        };
        let novel_tag = if resp.is_novel { "NOVEL" } else { "KNOWN " };
        println!(
            "{} {} {:.2}  {}",
            layer_tag, novel_tag, resp.novelty_score, title
        );
        if let Some(ref analog) = resp.closest_analog {
            println!("         Closest: {}", analog);
        }
        println!("         {}\n", resp.explanation);
    }

    Ok(())
}
