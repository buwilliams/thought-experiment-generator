use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::info;

use crate::types::{
    Conjecture, ConjectureMeta, Generated, GeneratedMeta, Layer, ProblemSet, ProblemSetMeta,
    Question, StateInfo,
};

const STATE_DIR: &str = "data/state";
const SEED_DIR: &str = "data/seed";

// --- Paths ---

fn state_dir() -> PathBuf {
    PathBuf::from(STATE_DIR)
}

fn layer_dir(layer: &Layer) -> PathBuf {
    state_dir().join(match layer {
        Layer::Mind => "mind",
        Layer::Candidates => "candidates",
    })
}

fn run_dir(run: u32) -> PathBuf {
    state_dir().join("runs").join(format!("{:03}", run))
}

fn state_info_path() -> PathBuf {
    state_dir().join("state.json")
}

// --- Initialization ---

pub fn ensure_initialized() -> Result<()> {
    if state_info_path().exists() {
        return Ok(());
    }
    info!("Initializing state from seed...");
    copy_seed_to_state()?;
    let state = StateInfo {
        run: 0,
        created_at: now_iso8601(),
        last_run_at: now_iso8601(),
    };
    save_state_info(&state)
}

pub fn reset_to_seed() -> Result<()> {
    let dir = state_dir();
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    copy_seed_to_state()?;
    let state = StateInfo {
        run: 0,
        created_at: now_iso8601(),
        last_run_at: now_iso8601(),
    };
    save_state_info(&state)?;
    info!("State reset to seed.");
    Ok(())
}

fn copy_seed_to_state() -> Result<()> {
    copy_dir_all(Path::new(SEED_DIR), &state_dir())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}

// --- State info ---

pub fn load_state_info() -> Result<StateInfo> {
    let text = std::fs::read_to_string(state_info_path())?;
    Ok(serde_json::from_str(&text)?)
}

pub fn save_state_info(state: &StateInfo) -> Result<()> {
    std::fs::create_dir_all(state_dir())?;
    std::fs::write(state_info_path(), serde_json::to_string_pretty(state)?)?;
    Ok(())
}

pub fn increment_run() -> Result<u32> {
    let mut state = load_state_info()?;
    state.run += 1;
    state.last_run_at = now_iso8601();
    let run = state.run;
    save_state_info(&state)?;
    std::fs::create_dir_all(run_dir(run))?;
    Ok(run)
}

// --- Conjectures (mind / candidates layer) ---

pub fn load_conjectures(layer: &Layer) -> Result<Vec<Conjecture>> {
    let dir = layer_dir(layer);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut conjectures = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match load_conjecture_from_json(&path) {
                Ok(c) => conjectures.push(c),
                Err(e) => tracing::warn!("Skipping conjecture {}: {e}", path.display()),
            }
        }
    }
    conjectures.sort_by_key(|c| c.meta.rank);
    Ok(conjectures)
}

fn load_conjecture_from_json(json_path: &Path) -> Result<Conjecture> {
    let json_text = std::fs::read_to_string(json_path)?;
    let meta: ConjectureMeta = serde_json::from_str(&json_text)
        .with_context(|| format!("Invalid JSON in {}", json_path.display()))?;

    let md_path = json_path.with_extension("md");
    let md_text = std::fs::read_to_string(&md_path)
        .with_context(|| format!("Missing .md for {}", json_path.display()))?;

    let (title, summary, full_text) = parse_content_md(&md_text);
    Ok(Conjecture { meta, title, summary, full_text })
}

pub fn save_conjecture(conjecture: &Conjecture) -> Result<()> {
    let dir = layer_dir(&conjecture.meta.layer);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join(format!("{}.md", conjecture.meta.id)),
        format_content_md(&conjecture.title, &conjecture.summary, &conjecture.full_text),
    )?;
    std::fs::write(
        dir.join(format!("{}.json", conjecture.meta.id)),
        serde_json::to_string_pretty(&conjecture.meta)?,
    )?;
    Ok(())
}

pub fn delete_conjecture(id: &str, layer: &Layer) -> Result<()> {
    let dir = layer_dir(layer);
    let md = dir.join(format!("{}.md", id));
    let json = dir.join(format!("{}.json", id));
    if md.exists() { std::fs::remove_file(md)?; }
    if json.exists() { std::fs::remove_file(json)?; }
    Ok(())
}

// --- Problem Sets ---

fn problemsets_dir() -> PathBuf {
    state_dir().join("problemsets")
}

pub fn load_problemsets() -> Result<Vec<ProblemSet>> {
    let dir = problemsets_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut sets = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match load_problemset_from_json(&path) {
                Ok(ps) => sets.push(ps),
                Err(e) => tracing::warn!("Skipping problemset {}: {e}", path.display()),
            }
        }
    }
    sets.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    Ok(sets)
}

pub fn load_problemset(id: &str) -> Result<ProblemSet> {
    let path = problemsets_dir().join(format!("{}.json", id));
    if !path.exists() {
        anyhow::bail!("Problem set '{}' not found.", id);
    }
    load_problemset_from_json(&path)
}

fn load_problemset_from_json(json_path: &Path) -> Result<ProblemSet> {
    let json_text = std::fs::read_to_string(json_path)?;
    let meta: ProblemSetMeta = serde_json::from_str(&json_text)
        .with_context(|| format!("Invalid JSON in {}", json_path.display()))?;
    let md_path = json_path.with_extension("md");
    let content = std::fs::read_to_string(&md_path)
        .with_context(|| format!("Missing .md for {}", json_path.display()))?;
    Ok(ProblemSet { meta, content })
}

pub fn save_problemset(ps: &ProblemSet) -> Result<()> {
    let dir = problemsets_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{}.md", ps.meta.id)), &ps.content)?;
    std::fs::write(
        dir.join(format!("{}.json", ps.meta.id)),
        serde_json::to_string_pretty(&ps.meta)?,
    )?;
    Ok(())
}

pub fn hash_content(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())[..8].to_string()
}

pub fn problemset_exists(id: &str) -> bool {
    problemsets_dir().join(format!("{}.json", id)).exists()
}

pub fn resolve_problemset(id: Option<&str>) -> Result<ProblemSet> {
    match id {
        Some(s) => load_problemset(s),
        None => {
            let sets = load_problemsets()?;
            match sets.len() {
                0 => anyhow::bail!(
                    "No problem sets found. Create one with:\n  cargo run -- create-problemset \"...\""
                ),
                1 => Ok(sets.into_iter().next().unwrap()),
                _ => {
                    let ids: Vec<&str> = sets.iter().map(|s| s.meta.id.as_str()).collect();
                    anyhow::bail!(
                        "Multiple problem sets exist. Specify one with --problemset <id>. Available: {}",
                        ids.join(", ")
                    )
                }
            }
        }
    }
}

// --- Generated outputs (ephemeral, produced each run) ---

fn generated_base(run: u32, problem_id: &str, conjecture_id: &str) -> PathBuf {
    run_dir(run).join(format!("{}-{}", problem_id, conjecture_id))
}

pub fn generated_exists(run: u32, problem_id: &str, conjecture_id: &str) -> bool {
    generated_base(run, problem_id, conjecture_id)
        .with_extension("json")
        .exists()
}

pub fn save_generated(generated: &Generated) -> Result<()> {
    let base = generated_base(
        generated.meta.run,
        &generated.meta.problem_id,
        &generated.meta.conjecture_id,
    );

    std::fs::write(base.with_extension("json"), serde_json::to_string_pretty(&generated.meta)?)?;

    let mut md = format!(
        "# Generated: {} × {}\n\n## Conjecture\n\n{}\n\n## Questions\n\n",
        generated.meta.problem_id, generated.meta.conjecture_id, generated.text
    );
    for (i, q) in generated.questions.iter().enumerate() {
        md.push_str(&format!(
            "{}. {} — **{}**\n",
            i + 1,
            q.question,
            if q.answer { "yes" } else { "no" }
        ));
    }
    if !generated.meta.candidate_problems.is_empty() {
        md.push_str("\n## Candidate Problems\n\n");
        for cp in &generated.meta.candidate_problems {
            md.push_str(&format!("- {} (score: {:.2})\n", cp.text, cp.score));
        }
    }
    std::fs::write(base.with_extension("md"), md)?;
    Ok(())
}

pub fn load_run_generated(run: u32) -> Result<Vec<Generated>> {
    let dir = run_dir(run);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut outputs = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let json_text = std::fs::read_to_string(&path)?;
        let meta: GeneratedMeta = match serde_json::from_str(&json_text) {
            Ok(m) => m,
            Err(_) => continue, // skip non-generated json (e.g. state.json)
        };
        let md_path = path.with_extension("md");
        let md_content = if md_path.exists() {
            std::fs::read_to_string(&md_path)?
        } else {
            String::new()
        };
        let text = extract_between(&md_content, "## Conjecture", "## Questions");
        let questions = parse_questions_from_md(&md_content);
        outputs.push(Generated { meta, text, questions });
    }
    Ok(outputs)
}

fn parse_questions_from_md(md: &str) -> Vec<Question> {
    let section = match extract_section(md, "Questions") {
        Some(s) => s,
        None => return vec![],
    };
    section
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() { return None; }
            let answer = if line.ends_with("**yes**") {
                true
            } else if line.ends_with("**no**") {
                false
            } else {
                return None;
            };
            let text = line
                .trim_start_matches(|c: char| c.is_numeric() || c == '.')
                .trim();
            let text = text
                .trim_end_matches("**yes**")
                .trim_end_matches("**no**")
                .trim_end_matches("—")
                .trim()
                .to_string();
            Some(Question { question: text, answer })
        })
        .collect()
}

// --- Summary ---

pub fn save_run_summary(run: u32, text: &str) -> Result<()> {
    std::fs::create_dir_all(run_dir(run))?;
    std::fs::write(run_dir(run).join("summary.md"), text)?;
    Ok(())
}

pub fn load_last_summary() -> Result<Option<String>> {
    let state = load_state_info()?;
    if state.run == 0 {
        return Ok(None);
    }
    let path = run_dir(state.run).join("summary.md");
    if path.exists() {
        Ok(Some(std::fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

// --- Markdown helpers ---

fn parse_content_md(content: &str) -> (String, String, String) {
    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l[2..].trim().to_string())
        .unwrap_or_default();
    let summary = extract_section(content, "Summary").unwrap_or_default();
    let full_text = extract_section(content, "Full Text").unwrap_or_default();
    (title, summary, full_text)
}

fn extract_between(content: &str, start_header: &str, end_header: &str) -> String {
    let start = match content.find(start_header) {
        Some(i) => i + start_header.len(),
        None => return String::new(),
    };
    let after = &content[start..];
    let end = after.find(end_header).unwrap_or(after.len());
    after[..end].trim().to_string()
}

fn extract_section(content: &str, section: &str) -> Option<String> {
    let header = format!("## {}", section);
    let start = content.find(&header)?;
    let after = &content[start + header.len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    Some(after[..end].trim().to_string())
}

fn format_content_md(title: &str, summary: &str, full_text: &str) -> String {
    format!("# {title}\n\n## Summary\n\n{summary}\n\n## Full Text\n\n{full_text}\n")
}

// --- Utilities ---

pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn now_iso8601() -> String {
    chrono::Utc::now().to_rfc3339()
}
