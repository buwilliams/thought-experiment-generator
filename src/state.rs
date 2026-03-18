use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::info;

use crate::types::{
    Conjecture, ConjectureMeta, Layer, Problem, ProblemMeta, ProblemSet, ProblemSetMeta, Question,
    StateInfo, Tool, ToolMeta,
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
        Layer::Perspectives => "perspectives",
    })
}

fn problems_dir() -> PathBuf {
    state_dir().join("problems")
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

// --- Tools ---

pub fn load_tools(layer: &Layer) -> Result<Vec<Tool>> {
    let dir = layer_dir(layer);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut tools = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match load_tool_from_json(&path) {
                Ok(tool) => tools.push(tool),
                Err(e) => tracing::warn!("Skipping tool {}: {e}", path.display()),
            }
        }
    }
    tools.sort_by_key(|t| t.meta.rank);
    Ok(tools)
}

fn load_tool_from_json(json_path: &Path) -> Result<Tool> {
    let json_text = std::fs::read_to_string(json_path)?;
    let meta: ToolMeta = serde_json::from_str(&json_text)
        .with_context(|| format!("Invalid JSON in {}", json_path.display()))?;

    let md_path = json_path.with_extension("md");
    let md_text = std::fs::read_to_string(&md_path)
        .with_context(|| format!("Missing .md for {}", json_path.display()))?;

    let (title, summary, full_text) = parse_content_md(&md_text);
    Ok(Tool { meta, title, summary, full_text })
}

pub fn save_tool(tool: &Tool) -> Result<()> {
    let dir = layer_dir(&tool.meta.layer);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join(format!("{}.md", tool.meta.id)),
        format_content_md(&tool.title, &tool.summary, &tool.full_text),
    )?;
    std::fs::write(
        dir.join(format!("{}.json", tool.meta.id)),
        serde_json::to_string_pretty(&tool.meta)?,
    )?;
    Ok(())
}

pub fn delete_tool(id: &str, layer: &Layer) -> Result<()> {
    let dir = layer_dir(layer);
    let md = dir.join(format!("{}.md", id));
    let json = dir.join(format!("{}.json", id));
    if md.exists() { std::fs::remove_file(md)?; }
    if json.exists() { std::fs::remove_file(json)?; }
    Ok(())
}

// --- Problems ---

pub fn load_problems() -> Result<Vec<Problem>> {
    let dir = problems_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut problems = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match load_problem_from_json(&path) {
                Ok(p) => problems.push(p),
                Err(e) => tracing::warn!("Skipping problem {}: {e}", path.display()),
            }
        }
    }
    problems.sort_by_key(|p| p.meta.rank);
    Ok(problems)
}

fn load_problem_from_json(json_path: &Path) -> Result<Problem> {
    let json_text = std::fs::read_to_string(json_path)?;
    let meta: ProblemMeta = serde_json::from_str(&json_text)?;
    let md_path = json_path.with_extension("md");
    let md_text = std::fs::read_to_string(md_path)?;
    let (title, summary, full_text) = parse_content_md(&md_text);
    Ok(Problem { meta, title, summary, full_text })
}

pub fn save_problem(problem: &Problem) -> Result<()> {
    let dir = problems_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join(format!("{}.md", problem.meta.id)),
        format_content_md(&problem.title, &problem.summary, &problem.full_text),
    )?;
    std::fs::write(
        dir.join(format!("{}.json", problem.meta.id)),
        serde_json::to_string_pretty(&problem.meta)?,
    )?;
    Ok(())
}

pub fn problem_exists(id: &str) -> bool {
    problems_dir().join(format!("{}.json", id)).exists()
}

pub fn delete_problem(id: &str) -> Result<()> {
    let dir = problems_dir();
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
    let md_text = std::fs::read_to_string(&md_path)
        .with_context(|| format!("Missing .md for {}", json_path.display()))?;
    let title = md_text
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l[2..].trim().to_string())
        .unwrap_or_default();
    let summary = extract_section(&md_text, "Summary").unwrap_or_default();
    Ok(ProblemSet { meta, title, summary })
}

pub fn save_problemset(ps: &ProblemSet) -> Result<()> {
    let dir = problemsets_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join(format!("{}.md", ps.meta.id)),
        format!("# {}\n\n## Summary\n\n{}\n", ps.title, ps.summary),
    )?;
    std::fs::write(
        dir.join(format!("{}.json", ps.meta.id)),
        serde_json::to_string_pretty(&ps.meta)?,
    )?;
    Ok(())
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
                    "No problem sets found. Create one with:\n  cargo run -- create-problemset --title \"...\""
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

pub fn load_problems_for_set(ps: &ProblemSet) -> Result<Vec<Problem>> {
    let mut problems = vec![];
    for id in &ps.meta.problem_ids {
        let path = problems_dir().join(format!("{}.json", id));
        if !path.exists() {
            tracing::warn!("Problem '{}' in set '{}' not found — skipping", id, ps.meta.id);
            continue;
        }
        match load_problem_from_json(&path) {
            Ok(p) => problems.push(p),
            Err(e) => tracing::warn!("Skipping problem {}: {e}", id),
        }
    }
    Ok(problems)
}

// --- Conjectures ---

fn conjecture_base(run: u32, problem_id: &str, tool_id: &str) -> PathBuf {
    run_dir(run).join(format!("{}-{}", problem_id, tool_id))
}

pub fn conjecture_exists(run: u32, problem_id: &str, tool_id: &str) -> bool {
    conjecture_base(run, problem_id, tool_id)
        .with_extension("json")
        .exists()
}

pub fn save_conjecture(conjecture: &Conjecture) -> Result<()> {
    let base = conjecture_base(
        conjecture.meta.run,
        &conjecture.meta.problem_id,
        &conjecture.meta.tool_id,
    );

    std::fs::write(base.with_extension("json"), serde_json::to_string_pretty(&conjecture.meta)?)?;

    let mut md = format!(
        "# Conjecture: {} × {}\n\n## Conjecture\n\n{}\n\n## Questions\n\n",
        conjecture.meta.problem_id, conjecture.meta.tool_id, conjecture.text
    );
    for (i, q) in conjecture.questions.iter().enumerate() {
        md.push_str(&format!(
            "{}. {} — **{}**\n",
            i + 1,
            q.question,
            if q.answer { "yes" } else { "no" }
        ));
    }
    if !conjecture.meta.candidate_problems.is_empty() {
        md.push_str("\n## Candidate Problems\n\n");
        for cp in &conjecture.meta.candidate_problems {
            md.push_str(&format!("- {} (score: {:.2})\n", cp.text, cp.score));
        }
    }
    std::fs::write(base.with_extension("md"), md)?;
    Ok(())
}

pub fn load_run_conjectures(run: u32) -> Result<Vec<Conjecture>> {
    let dir = run_dir(run);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut conjectures = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let json_text = std::fs::read_to_string(&path)?;
        let meta: ConjectureMeta = match serde_json::from_str(&json_text) {
            Ok(m) => m,
            Err(_) => continue, // skip non-conjecture json
        };
        let md_path = path.with_extension("md");
        let text = if md_path.exists() {
            extract_section(&std::fs::read_to_string(&md_path)?, "Conjecture")
                .unwrap_or_default()
        } else {
            String::new()
        };
        // Reconstruct questions from md if present
        let questions = if md_path.exists() {
            parse_questions_from_md(&std::fs::read_to_string(&md_path)?)
        } else {
            vec![]
        };
        conjectures.push(Conjecture { meta, text, questions });
    }
    Ok(conjectures)
}

fn parse_questions_from_md(md: &str) -> Vec<Question> {
    let section = match extract_section(md, "Questions") {
        Some(s) => s,
        None => return vec![],
    };
    section
        .lines()
        .filter_map(|line| {
            // Format: "N. Question text — **yes/no**"
            let line = line.trim();
            if line.is_empty() { return None; }
            let answer = if line.ends_with("**yes**") {
                true
            } else if line.ends_with("**no**") {
                false
            } else {
                return None;
            };
            // Strip leading "N. " and trailing " — **yes/no**"
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
