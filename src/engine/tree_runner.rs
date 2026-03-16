use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::cache::{self, CachedBackground, CachedTreeState, TreePhase};
use crate::config::Config;
use crate::engine::{background_init, branch_runner, root_generator};
use crate::llm::LlmClient;
use crate::pipeline::{cross_pollination, trajectory_scorer};
use crate::types::{Branch, CrossPollinationRecord, DrawPool, NormalizedTopic, QuestionType, Tree};
use crate::vocab;

/// Run the full tree search, resuming from cache if available.
pub async fn run_tree(client: Arc<LlmClient>, config: &Config, topic: &str) -> Result<Tree> {
    // Try to resume from cached tree state
    if let Some(cached) = cache::load_tree_state(topic) {
        if cached.phase == TreePhase::Done {
            println!("Tree already complete (cached). Use --fresh to re-run.");
            return build_tree(topic, cached.draw_pool, cached.branches, vec![]);
        }
        return resume_tree(client, config, topic, cached).await;
    }

    // Fresh run: initialize background + vocabulary
    let (bg_quads, resources) = load_or_init_background(&client, topic).await?;

    if !resources.is_empty() {
        println!("\nSuggested resources for deeper exploration:");
        for r in &resources {
            println!("  - {r}");
        }
        println!();
    }

    let universal = load_or_generate_vocab(&client).await?;

    println!(
        "Draw pools initialized:\n  Background: {} quads\n  Universal:  {} terms\n  Novel:      0 quads\n  Ratio:      {:.0}% background / {:.0}% universal\n",
        bg_quads.len(),
        universal.len(),
        config.bg_universal_ratio * 100.0,
        (1.0 - config.bg_universal_ratio) * 100.0,
    );

    let pool = DrawPool::new(bg_quads, universal, config.bg_universal_ratio);

    // Save initial state
    cache::save_tree_state(topic, &CachedTreeState {
        draw_pool: pool.clone(),
        branches: vec![],
        phase: TreePhase::Initialized,
    })?;

    run_from_phase(client, config, topic, pool, vec![], TreePhase::Initialized).await
}

async fn resume_tree(
    client: Arc<LlmClient>,
    config: &Config,
    topic: &str,
    cached: CachedTreeState,
) -> Result<Tree> {
    println!("Resuming from cached state ({} branches)...\n", cached.branches.len());
    run_from_phase(client, config, topic, cached.draw_pool, cached.branches, cached.phase).await
}

async fn run_from_phase(
    client: Arc<LlmClient>,
    config: &Config,
    topic: &str,
    pool: DrawPool,
    mut branches: Vec<Branch>,
    phase: TreePhase,
) -> Result<Tree> {
    let pool = Arc::new(RwLock::new(pool));

    // Step 9: Generate root branches (if not done yet)
    if phase == TreePhase::Initialized {
        branches = root_generator::generate_root_branches(&client, &pool, topic, config).await?;

        println!("\nRoot generation complete: {} branches\n", branches.len());

        // Save after root generation
        cache::save_tree_state(topic, &CachedTreeState {
            draw_pool: pool.read().await.clone(),
            branches: branches.clone(),
            phase: TreePhase::RootsGenerated,
        })?;

        if client.budget_exhausted() {
            println!("Budget exhausted after root generation. Resume to continue.");
            return build_tree(topic, pool.read().await.clone(), branches, vec![]);
        }
    }

    // Step 12: Run branches in parallel
    let active_branches: Vec<Branch> = branches.iter()
        .filter(|b| b.current_depth < b.depth_limit && b.status == crate::types::BranchStatus::Active)
        .cloned()
        .collect();
    let completed_branches: Vec<Branch> = branches.iter()
        .filter(|b| b.current_depth >= b.depth_limit || b.status != crate::types::BranchStatus::Active)
        .cloned()
        .collect();

    if !active_branches.is_empty() {
        println!(
            "Running tree search (depth limit: {}, active branches: {})...\n",
            config.depth_limit,
            active_branches.len()
        );

        let mut handles = Vec::new();

        for mut branch in active_branches {
            let client = Arc::clone(&client);
            let pool = Arc::clone(&pool);
            let config = config.clone();

            let handle = tokio::spawn(async move {
                branch_runner::run_branch(&client, &pool, &mut branch, &config).await?;
                Ok::<Branch, anyhow::Error>(branch)
            });

            handles.push(handle);
        }

        let mut all_branches = completed_branches;
        for handle in handles {
            match handle.await? {
                Ok(branch) => all_branches.push(branch),
                Err(e) => tracing::warn!("Branch failed: {e}"),
            }
        }
        branches = all_branches;

        // Save after branch execution
        cache::save_tree_state(topic, &CachedTreeState {
            draw_pool: pool.read().await.clone(),
            branches: branches.clone(),
            phase: TreePhase::BranchesComplete,
        })?;

        if client.budget_exhausted() {
            println!("Budget exhausted after branch execution. Resume to continue.");
            return build_tree(topic, pool.read().await.clone(), branches, vec![]);
        }
    }

    // Step 11: Check cross-pollination between completed branches
    let mut cross_pollinations = Vec::new();
    let mut new_branches = Vec::new();

    for i in 0..branches.len() {
        if client.budget_exhausted() {
            break;
        }
        for j in (i + 1)..branches.len() {
            if client.budget_exhausted() {
                break;
            }
            // Only compare branches at similar depth (design doc constraint)
            if branches[i].current_depth != branches[j].current_depth {
                continue;
            }
            if let Ok(Some(result)) = cross_pollination::check_cross_pollination(
                &client,
                topic,
                &branches[i],
                &branches[j],
            )
            .await
            {
                info!(
                    "Cross-pollination detected: {} + {} — {}",
                    branches[i].id, branches[j].id, result.reason
                );

                let mut merged = Branch::new(topic.to_string(), config.depth_limit);
                merged.parent_branch_ids = vec![branches[i].id, branches[j].id];

                let record = CrossPollinationRecord {
                    branch_a: branches[i].id,
                    branch_b: branches[j].id,
                    new_branch: merged.id,
                    depth_at_merge: branches[i].current_depth,
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

    branches.extend(new_branches);

    // Step 10: Score all trajectories
    if !client.budget_exhausted() {
        println!("\nScoring trajectories...\n");

        for branch in &mut branches {
            if branch.nodes.is_empty() || client.budget_exhausted() {
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
    }

    // Save final state
    cache::save_tree_state(topic, &CachedTreeState {
        draw_pool: pool.read().await.clone(),
        branches: branches.clone(),
        phase: TreePhase::Done,
    })?;

    build_tree(topic, pool.read().await.clone(), branches, cross_pollinations)
}

fn build_tree(
    topic: &str,
    draw_pool: DrawPool,
    mut branches: Vec<Branch>,
    cross_pollinations: Vec<CrossPollinationRecord>,
) -> Result<Tree> {
    // Rank by trajectory score
    branches.sort_by(|a, b| {
        b.trajectory_score
            .unwrap_or(0.0)
            .partial_cmp(&a.trajectory_score.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let top_ids: Vec<Uuid> = branches.iter().take(3).map(|b| b.id).collect();

    Ok(Tree {
        id: Uuid::new_v4(),
        topic: topic.to_string(),
        topic_normalized: NormalizedTopic {
            core_subject: topic.to_string(),
            core_tension: String::new(),
            question_type: QuestionType::Other,
        },
        draw_pool,
        branches,
        cross_pollinations,
        top_trajectories: top_ids,
    })
}

async fn load_or_init_background(
    client: &LlmClient,
    topic: &str,
) -> Result<(Vec<crate::types::Quad>, Vec<String>)> {
    if let Some(cached) = cache::load_background(topic) {
        return Ok((cached.quads, cached.resources));
    }

    let bg_result = background_init::initialize_background_pool(client, topic).await?;

    cache::save_background(topic, &CachedBackground {
        quads: bg_result.quads.clone(),
        resources: bg_result.resources.clone(),
    })?;

    Ok((bg_result.quads, bg_result.resources))
}

async fn load_or_generate_vocab(client: &LlmClient) -> Result<Vec<crate::types::Quad>> {
    let vocab_path = std::path::Path::new("data/universal_vocabulary.txt");
    if vocab_path.exists() {
        return vocab::load_universal_vocabulary(vocab_path);
    }

    info!("No vocabulary file found, generating starter vocabulary...");
    let terms = vocab::generate_vocabulary(client).await?;
    if let Err(e) = std::fs::create_dir_all("data") {
        tracing::warn!("Failed to create data dir: {e}");
    }
    if let Err(e) = std::fs::write(vocab_path, terms.join("\n")) {
        tracing::warn!("Failed to save vocabulary: {e}");
    }
    Ok(terms
        .into_iter()
        .map(|t| crate::types::Quad::new_universal(t))
        .collect())
}
