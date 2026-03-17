use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::types::Critique;

const CACHE_DIR: &str = "data/cache";

fn topic_hash(topic: &str) -> String {
    let mut hasher = DefaultHasher::new();
    topic.to_lowercase().trim().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn cache_dir(topic: &str) -> PathBuf {
    Path::new(CACHE_DIR).join(topic_hash(topic))
}

pub fn clear_cache(topic: &str) -> Result<()> {
    let dir = cache_dir(topic);
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
        tracing::info!("Cleared cache for topic");
    }
    Ok(())
}

fn ensure_dir(topic: &str) -> Result<PathBuf> {
    let dir = cache_dir(topic);
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

// --- background.txt ---

pub fn background_path(topic: &str) -> PathBuf {
    cache_dir(topic).join("background.txt")
}

pub fn load_background(topic: &str) -> Option<Vec<String>> {
    load_lines(&background_path(topic))
}

pub fn save_background(topic: &str, lines: &[String]) -> Result<()> {
    let dir = ensure_dir(topic)?;
    std::fs::write(dir.join("background.txt"), lines.join("\n"))?;
    tracing::info!("Saved background.txt ({} sentences)", lines.len());
    Ok(())
}

// --- words.txt ---

pub fn words_path(topic: &str) -> PathBuf {
    cache_dir(topic).join("words.txt")
}

pub fn load_words(topic: &str) -> Option<Vec<String>> {
    load_lines(&words_path(topic))
}

pub fn save_words(topic: &str, lines: &[String]) -> Result<()> {
    let dir = ensure_dir(topic)?;
    std::fs::write(dir.join("words.txt"), lines.join("\n"))?;
    tracing::info!("Saved words.txt ({} lines)", lines.len());
    Ok(())
}

// --- generated.txt ---

pub fn generated_path(topic: &str) -> PathBuf {
    cache_dir(topic).join("generated.txt")
}

pub fn load_generated(topic: &str) -> Option<Vec<String>> {
    load_lines(&generated_path(topic))
}

pub fn save_generated(topic: &str, lines: &[String]) -> Result<()> {
    let dir = ensure_dir(topic)?;
    std::fs::write(dir.join("generated.txt"), lines.join("\n"))?;
    tracing::info!("Saved generated.txt ({} sentences)", lines.len());
    Ok(())
}

// --- NNN-experiment.txt ---

pub fn experiment_path(topic: &str, n: u32) -> PathBuf {
    cache_dir(topic).join(format!("{:03}-experiment.txt", n))
}

pub fn experiment_exists(topic: &str, n: u32) -> bool {
    experiment_path(topic, n).exists()
}

pub fn load_experiment(topic: &str, n: u32) -> Option<String> {
    std::fs::read_to_string(experiment_path(topic, n)).ok()
}

pub fn save_experiment(topic: &str, n: u32, text: &str) -> Result<()> {
    let dir = ensure_dir(topic)?;
    std::fs::write(dir.join(format!("{:03}-experiment.txt", n)), text)?;
    Ok(())
}

// --- NNN-experiment-criticize.json ---

pub fn critique_path(topic: &str, n: u32) -> PathBuf {
    cache_dir(topic).join(format!("{:03}-experiment-criticize.json", n))
}

pub fn critique_exists(topic: &str, n: u32) -> bool {
    critique_path(topic, n).exists()
}

pub fn load_critique(topic: &str, n: u32) -> Option<Critique> {
    let text = std::fs::read_to_string(critique_path(topic, n)).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn save_critique(topic: &str, n: u32, critique: &Critique) -> Result<()> {
    let dir = ensure_dir(topic)?;
    let json = serde_json::to_string_pretty(critique)?;
    std::fs::write(dir.join(format!("{:03}-experiment-criticize.json", n)), json)?;
    Ok(())
}

// --- summary.txt ---

pub fn summary_path(topic: &str) -> PathBuf {
    cache_dir(topic).join("summary.txt")
}

pub fn save_summary(topic: &str, text: &str) -> Result<()> {
    let dir = ensure_dir(topic)?;
    std::fs::write(dir.join("summary.txt"), text)?;
    tracing::info!("Saved summary.txt");
    Ok(())
}

// --- helpers ---

fn load_lines(path: &Path) -> Option<Vec<String>> {
    let text = std::fs::read_to_string(path).ok()?;
    let lines: Vec<String> = text
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.is_empty() { None } else { Some(lines) }
}
