use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use rand::rngs::StdRng;
use rand::SeedableRng;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::pipeline::{coherence_filter, deutsch_scorer, te_generator, tension_extractor};
use crate::types::{Branch, DrawPool, Node};

/// Generate N diverse root branches by finding diverse survivors.
pub async fn generate_root_branches(
    client: &LlmClient,
    pool: &Arc<RwLock<DrawPool>>,
    topic: &str,
    config: &Config,
) -> Result<Vec<Branch>> {
    let mut rng = StdRng::from_entropy();
    let mut branches = Vec::new();
    let mut all_used_quads: Vec<HashSet<Uuid>> = Vec::new();
    let mut total_draws: u32 = 0;
    let max_total_draws = config.draws_per_depth * config.num_branches * 5; // generous limit

    info!("Generating {} root branches...", config.num_branches);

    while branches.len() < config.num_branches as usize && total_draws < max_total_draws && !client.budget_exhausted() {
        total_draws += 1;

        let draw = {
            let pool_read = pool.read().await;
            pool_read.draw(&config.atom, &mut rng)
        };

        // Generate TE
        let te = match te_generator::generate_thought_experiment(
            client, topic, &draw, &[], None, config.temperature,
        )
        .await
        {
            Ok(te) => te,
            Err(e) => {
                warn!("Draw {total_draws}: TE generation failed: {e}");
                continue;
            }
        };

        // Grammar check
        if !te_generator::grammar_check(&te) {
            info!("Draw {total_draws}: failed grammar check");
            continue;
        }

        // Coherence
        let coherence = match coherence_filter::check_coherence(client, topic, &te).await {
            Ok(c) => c,
            Err(e) => {
                warn!("Draw {total_draws}: coherence check error: {e}");
                continue;
            }
        };
        if !coherence.passes {
            info!(
                "Draw {total_draws}: failed coherence (consistent={}, relevant={}): {} / {}",
                coherence.internally_consistent, coherence.topic_relevant,
                coherence.consistent_reason, coherence.relevant_reason
            );
            continue;
        }

        // Deutsch score
        let score = match deutsch_scorer::score_deutsch(client, topic, &te, &[]).await {
            Ok(s) => s,
            Err(e) => {
                warn!("Draw {total_draws}: Deutsch scoring error: {e}");
                continue;
            }
        };
        if score.overall_score < config.survivor_threshold {
            info!(
                "Draw {total_draws}: below threshold ({:.2} < {:.2}): {}",
                score.overall_score, config.survivor_threshold, score.justification
            );
            continue;
        }

        // Diversity check: reject if >50% quad overlap with any existing root
        let draw_quad_set: HashSet<Uuid> = draw.quads_used.iter().copied().collect();
        let is_diverse = all_used_quads.iter().all(|existing| {
            let overlap = draw_quad_set.intersection(existing).count();
            let max_size = draw_quad_set.len().max(existing.len());
            max_size == 0 || (overlap as f64 / max_size as f64) < 0.5
        });

        if !is_diverse {
            debug!("Root candidate rejected: too similar to existing branch");
            continue;
        }

        // Extract tension for this root
        let tension = tension_extractor::extract_tension(
            client, topic, &te, &[], score.overall_score,
        )
        .await
        .ok();

        let node_id = Uuid::new_v4();
        let mut branch = Branch::new(topic.to_string(), config.depth_limit);

        let node = Node {
            id: node_id,
            depth: 1,
            branch_id: branch.id,
            thought_experiment: te,
            quads_used: draw.quads_used.clone(),
            deutsch_score: score.clone(),
            unresolved_tension: tension,
            score_history: vec![score.overall_score],
            accumulated_path: vec![node_id],
        };

        info!(
            "Root branch {} found after {} draws (score {:.2})",
            branches.len() + 1,
            total_draws,
            score.overall_score
        );

        branch.nodes.push(node);
        branch.current_depth = 1;
        all_used_quads.push(draw_quad_set);
        branches.push(branch);
    }

    info!(
        "Root generation complete: {} branches, {} total draws",
        branches.len(),
        total_draws
    );

    Ok(branches)
}
