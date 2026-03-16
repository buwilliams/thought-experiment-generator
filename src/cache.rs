use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::types::{Branch, DrawPool, Quad};

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
    Ok(())
}
