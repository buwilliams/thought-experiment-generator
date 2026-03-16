use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::types::{Branch, DrawPool, Node, Quad};

const CACHE_DIR: &str = "data/cache";

/// Cached background initialization result.
#[derive(Serialize, Deserialize)]
pub struct CachedBackground {
    pub quads: Vec<Quad>,
    pub resources: Vec<String>,
}

/// Full tree state for resume.
#[derive(Serialize, Deserialize)]
pub struct CachedTreeState {
    pub draw_pool: DrawPool,
    pub branches: Vec<Branch>,
    pub phase: TreePhase,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TreePhase {
    /// Background + vocab initialized, root generation not started
    Initialized,
    /// Root branches generated, tree search not started
    RootsGenerated,
    /// Tree search complete, scoring/cross-pollination may remain
    BranchesComplete,
    /// Everything done
    Done,
}

fn topic_hash(topic: &str) -> String {
    let mut hasher = DefaultHasher::new();
    topic.to_lowercase().trim().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn cache_dir(topic: &str) -> PathBuf {
    Path::new(CACHE_DIR).join(topic_hash(topic))
}

pub fn clear_cache(topic: &str) -> Result<()> {
    let dir = cache_dir(topic);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
        info!("Cleared cache for topic");
    }
    Ok(())
}

pub fn load_background(topic: &str) -> Option<CachedBackground> {
    let path = cache_dir(topic).join("background.json");
    let data = std::fs::read_to_string(path).ok()?;
    let cached: CachedBackground = serde_json::from_str(&data).ok()?;
    info!("Loaded cached background ({} quads)", cached.quads.len());
    Some(cached)
}

pub fn save_background(topic: &str, bg: &CachedBackground) -> Result<()> {
    let dir = cache_dir(topic);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("background.json");
    std::fs::write(&path, serde_json::to_string(bg)?)?;
    info!("Cached background ({} quads)", bg.quads.len());
    Ok(())
}

pub fn load_tree_state(topic: &str) -> Option<CachedTreeState> {
    let path = cache_dir(topic).join("tree_state.json");
    let data = std::fs::read_to_string(path).ok()?;
    let cached: CachedTreeState = serde_json::from_str(&data).ok()?;
    info!(
        "Loaded cached tree state (phase={}, {} branches)",
        match &cached.phase {
            TreePhase::Initialized => "initialized",
            TreePhase::RootsGenerated => "roots_generated",
            TreePhase::BranchesComplete => "branches_complete",
            TreePhase::Done => "done",
        },
        cached.branches.len()
    );
    Some(cached)
}

pub fn save_tree_state(topic: &str, state: &CachedTreeState) -> Result<()> {
    let dir = cache_dir(topic);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("tree_state.json");
    std::fs::write(&path, serde_json::to_string(state)?)?;
    info!("Saved tree state ({} branches)", state.branches.len());

    if state.phase == TreePhase::Done {
        save_summary(topic, state)?;
    }

    Ok(())
}

fn node_title(node: &Node) -> String {
    node.thought_experiment
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("(untitled)")
        .trim()
        .trim_start_matches('#')
        .trim()
        .to_string()
}

pub fn save_summary(topic: &str, state: &CachedTreeState) -> Result<()> {
    let dir = cache_dir(topic);
    let total_nodes: usize = state.branches.iter().map(|b| b.nodes.len()).sum();

    let mut out = String::new();
    out.push_str(&format!("# {topic}\n\n"));
    out.push_str(&format!(
        "{} branches, {} thought experiments, {} novel quads\n\n",
        state.branches.len(),
        total_nodes,
        state.draw_pool.novel.len()
    ));

    // Collect all nodes ranked by score
    let mut all_nodes: Vec<(usize, &Node)> = state
        .branches
        .iter()
        .enumerate()
        .flat_map(|(bi, b)| b.nodes.iter().map(move |n| (bi + 1, n)))
        .collect();
    all_nodes.sort_by(|a, b| {
        b.1.deutsch_score.overall_score
            .partial_cmp(&a.1.deutsch_score.overall_score)
            .unwrap()
    });

    out.push_str("| Key | Score | Title |\n");
    out.push_str("|-----|-------|-------|\n");
    for (branch_idx, node) in &all_nodes {
        let key = format!("{}.{}", branch_idx, node.depth);
        out.push_str(&format!(
            "| {} | {:.2} | {} |\n",
            key,
            node.deutsch_score.overall_score,
            node_title(node)
        ));
    }

    // Top trajectories
    let mut branches_ranked: Vec<&Branch> = state.branches.iter().collect();
    branches_ranked.sort_by(|a, b| {
        b.trajectory_score.unwrap_or(0.0)
            .partial_cmp(&a.trajectory_score.unwrap_or(0.0))
            .unwrap()
    });

    out.push_str("\n## Top Trajectories\n\n");
    for (i, branch) in branches_ranked.iter().take(3).enumerate() {
        let traj = branch.trajectory_score.unwrap_or(0.0);
        let chain: Vec<String> = branch.nodes.iter().map(|n| node_title(n)).collect();
        out.push_str(&format!(
            "**#{}** (score {:.2}): {}\n\n",
            i + 1,
            traj,
            chain.join(" -> ")
        ));
    }

    let path = dir.join("summary.md");
    std::fs::write(&path, &out)?;
    info!("Saved summary.md");
    Ok(())
}
