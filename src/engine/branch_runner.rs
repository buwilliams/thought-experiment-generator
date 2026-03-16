use std::sync::Arc;

use anyhow::Result;
use rand::rngs::StdRng;
use rand::SeedableRng;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::pipeline::{coherence_filter, deutsch_scorer, quad_extractor, te_generator, tension_extractor};
use crate::types::{Branch, BranchStatus, DrawPool, Node};

/// Run a single branch from its current depth to the depth limit.
/// The branch modifies in place. The draw pool is shared for novel pool updates.
pub async fn run_branch(
    client: &LlmClient,
    pool: &Arc<RwLock<DrawPool>>,
    branch: &mut Branch,
    config: &Config,
) -> Result<()> {
    let mut rng = StdRng::from_entropy();

    let start_depth = branch.current_depth + 1;

    for depth in start_depth..=branch.depth_limit {
        let mut draws_attempted: u32 = 0;
        let mut survivor_found = false;

        // Collect accumulated path so far
        let accumulated_path: Vec<Node> = branch.nodes.clone();
        let latest_tension = branch.latest_tension().cloned();

        while draws_attempted < config.draws_per_depth && !survivor_found {
            draws_attempted += 1;

            // Draw from pools
            let draw = {
                let pool_read = pool.read().await;
                pool_read.draw(&config.atom, &mut rng)
            };

            // Generate thought experiment
            let te = match te_generator::generate_thought_experiment(
                client,
                &branch.topic,
                &draw,
                &accumulated_path,
                latest_tension.as_ref(),
            )
            .await
            {
                Ok(te) => te,
                Err(e) => {
                    debug!("TE generation failed: {e}");
                    continue;
                }
            };

            // Grammar pre-filter (no LLM call)
            if !te_generator::grammar_check(&te) {
                debug!("Draw {draws_attempted}: failed grammar check");
                continue;
            }

            // Coherence filter
            let coherence = match coherence_filter::check_coherence(client, &branch.topic, &te).await {
                Ok(c) => c,
                Err(e) => {
                    debug!("Coherence check failed: {e}");
                    continue;
                }
            };

            if !coherence.passes {
                debug!(
                    "Draw {draws_attempted}: failed coherence (consistent={}, relevant={})",
                    coherence.internally_consistent, coherence.topic_relevant
                );
                continue;
            }

            // Deutsch scoring
            let score = match deutsch_scorer::score_deutsch(
                client,
                &branch.topic,
                &te,
                &accumulated_path,
            )
            .await
            {
                Ok(s) => s,
                Err(e) => {
                    debug!("Deutsch scoring failed: {e}");
                    continue;
                }
            };

            if score.overall_score < config.survivor_threshold {
                debug!(
                    "Draw {draws_attempted}: below threshold ({:.2} < {:.2})",
                    score.overall_score, config.survivor_threshold
                );
                continue;
            }

            // Survivor found! Extract tension for next depth.
            let tension = tension_extractor::extract_tension(
                client,
                &branch.topic,
                &te,
                &accumulated_path,
                score.overall_score,
            )
            .await
            .ok();

            let node_id = Uuid::new_v4();
            let mut score_history: Vec<f64> = accumulated_path.iter().map(|n| n.deutsch_score.overall_score).collect();
            score_history.push(score.overall_score);

            let mut path_ids: Vec<Uuid> = accumulated_path.iter().map(|n| n.id).collect();
            path_ids.push(node_id);

            let node = Node {
                id: node_id,
                depth,
                branch_id: branch.id,
                thought_experiment: te.clone(),
                quads_used: draw.quads_used,
                deutsch_score: score.clone(),
                unresolved_tension: tension,
                score_history,
                accumulated_path: path_ids,
            };

            info!(
                "Branch {} depth {}: survivor found (score {:.2}, {} draws)",
                branch.id, depth, score.overall_score, draws_attempted
            );

            branch.nodes.push(node);
            branch.current_depth = depth;

            // Novel pool admission
            if score.overall_score >= config.novel_threshold {
                match quad_extractor::extract_novel_quads(client, &te, node_id).await {
                    Ok(novel_quads) => {
                        let count = novel_quads.len();
                        let mut pool_write = pool.write().await;
                        pool_write.add_novel(novel_quads);
                        pool_write.update_ratio();
                        info!("Added {count} novel quads to pool");
                    }
                    Err(e) => {
                        warn!("Failed to extract novel quads: {e}");
                    }
                }
            }

            survivor_found = true;
        }

        if !survivor_found {
            info!(
                "Branch {} terminated at depth {} (no survivor in {} draws)",
                branch.id, depth, config.draws_per_depth
            );
            branch.status = BranchStatus::Terminated;
            return Ok(());
        }
    }

    info!(
        "Branch {} completed to depth limit {}",
        branch.id, branch.depth_limit
    );
    branch.status = BranchStatus::Terminated;
    Ok(())
}
