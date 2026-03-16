use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::config::Config;
use crate::engine::{background_init, branch_runner, root_generator};
use crate::llm::LlmClient;
use crate::pipeline::{cross_pollination, trajectory_scorer};
use crate::types::{Branch, CrossPollinationRecord, DrawPool, NormalizedTopic, QuestionType, Tree};
use crate::vocab;

/// Run the full tree search.
pub async fn run_tree(client: Arc<LlmClient>, config: &Config, topic: &str) -> Result<Tree> {
    // Step 8: Initialize background pool
    let bg_result = background_init::initialize_background_pool(&client, topic).await?;

    if !bg_result.resources.is_empty() {
        println!("\nSuggested resources for deeper exploration:");
        for r in &bg_result.resources {
            println!("  - {r}");
        }
        println!();
    }

    // Step 1: Load universal vocabulary
    let vocab_path = std::path::Path::new("data/universal_vocabulary.txt");
    let universal = if vocab_path.exists() {
        vocab::load_universal_vocabulary(vocab_path)?
    } else {
        info!("No vocabulary file found, generating starter vocabulary...");
        let terms = vocab::generate_vocabulary(&client).await?;
        if let Err(e) = std::fs::create_dir_all("data") {
            tracing::warn!("Failed to create data dir: {e}");
        }
        if let Err(e) = std::fs::write(vocab_path, terms.join("\n")) {
            tracing::warn!("Failed to save vocabulary: {e}");
        }
        terms
            .into_iter()
            .map(|t| crate::types::Quad::new_universal(t))
            .collect()
    };

    println!(
        "Draw pools initialized:\n  Background: {} quads\n  Universal:  {} terms\n  Novel:      0 quads\n  Ratio:      {:.0}% background / {:.0}% universal\n",
        bg_result.quads.len(),
        universal.len(),
        config.bg_universal_ratio * 100.0,
        (1.0 - config.bg_universal_ratio) * 100.0,
    );

    // Create shared draw pool
    let pool = Arc::new(RwLock::new(DrawPool::new(
        bg_result.quads,
        universal,
        config.bg_universal_ratio,
    )));

    // Step 9: Generate root branches
    let mut branches = root_generator::generate_root_branches(&client, &pool, topic, config).await?;

    println!(
        "\nRoot generation complete: {} branches\n",
        branches.len()
    );

    // Step 12: Run branches in parallel
    println!(
        "Running tree search (depth limit: {}, branches: {})...\n",
        config.depth_limit,
        branches.len()
    );

    let mut handles = Vec::new();

    for mut branch in branches.drain(..) {
        let client = Arc::clone(&client);
        let pool = Arc::clone(&pool);
        let config = config.clone();

        let handle = tokio::spawn(async move {
            branch_runner::run_branch(&client, &pool, &mut branch, &config).await?;
            Ok::<Branch, anyhow::Error>(branch)
        });

        handles.push(handle);
    }

    let mut completed_branches = Vec::new();
    for handle in handles {
        match handle.await? {
            Ok(branch) => completed_branches.push(branch),
            Err(e) => tracing::warn!("Branch failed: {e}"),
        }
    }

    // Step 11: Check cross-pollination between completed branches
    let mut cross_pollinations = Vec::new();
    let mut new_branches = Vec::new();

    for i in 0..completed_branches.len() {
        for j in (i + 1)..completed_branches.len() {
            if let Ok(Some(result)) = cross_pollination::check_cross_pollination(
                &client,
                topic,
                &completed_branches[i],
                &completed_branches[j],
            )
            .await
            {
                info!(
                    "Cross-pollination detected: {} + {} — {}",
                    completed_branches[i].id,
                    completed_branches[j].id,
                    result.reason
                );

                let mut merged = Branch::new(topic.to_string(), config.depth_limit);
                merged.parent_branch_ids = vec![
                    completed_branches[i].id,
                    completed_branches[j].id,
                ];

                let record = CrossPollinationRecord {
                    branch_a: completed_branches[i].id,
                    branch_b: completed_branches[j].id,
                    new_branch: merged.id,
                    depth_at_merge: completed_branches[i].current_depth,
                };

                let pool_clone = Arc::clone(&pool);
                match branch_runner::run_branch(&client, &pool_clone, &mut merged, &config).await {
                    Ok(()) => {
                        cross_pollinations.push(record);
                        new_branches.push(merged);
                    }
                    Err(e) => tracing::warn!("Cross-pollinated branch failed: {e}"),
                }
            }
        }
    }

    completed_branches.extend(new_branches);

    // Step 10: Score all trajectories
    println!("\nScoring trajectories...\n");

    for branch in &mut completed_branches {
        if branch.nodes.is_empty() {
            continue;
        }
        match trajectory_scorer::score_trajectory(&client, topic, branch).await {
            Ok(score) => {
                branch.trajectory_score = Some(score.trajectory_score);
                info!(
                    "Branch {} trajectory score: {:.2} — {}",
                    branch.id, score.trajectory_score, score.justification
                );
            }
            Err(e) => tracing::warn!("Trajectory scoring failed for {}: {e}", branch.id),
        }
    }

    // Rank by trajectory score
    completed_branches.sort_by(|a, b| {
        b.trajectory_score
            .unwrap_or(0.0)
            .partial_cmp(&a.trajectory_score.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let top_ids: Vec<Uuid> = completed_branches
        .iter()
        .take(3)
        .map(|b| b.id)
        .collect();

    let tree = Tree {
        id: Uuid::new_v4(),
        topic: topic.to_string(),
        topic_normalized: NormalizedTopic {
            core_subject: topic.to_string(),
            core_tension: String::new(),
            question_type: QuestionType::Other,
        },
        draw_pool: pool.read().await.clone(),
        branches: completed_branches,
        cross_pollinations,
        top_trajectories: top_ids,
    };

    Ok(tree)
}
