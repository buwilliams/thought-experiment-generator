use std::sync::Arc;

use anyhow::Result;
use futures::future::join_all;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::promoter;
use crate::prompts::PromptTemplates;
use crate::state;
use crate::types::{Conjecture, Layer, NoveltyResponse};

const TOP_CANDIDATES: usize = 5;
const TOP_OUTPUTS_FOR_ASSESS: usize = 3;
const REVIEW_PATH: &str = "data/state/review.md";

/// Builds markdown and plain-text outputs simultaneously.
/// Markdown gets relative links to state files; plain text is clean for stdout.
pub struct ReviewWriter {
    md: String,
    plain: String,
}

impl ReviewWriter {
    fn new() -> Self {
        Self { md: String::new(), plain: String::new() }
    }

    /// Write different text to each output.
    fn line(&mut self, md: &str, plain: &str) {
        self.md.push_str(md);
        self.md.push('\n');
        self.plain.push_str(plain);
        self.plain.push('\n');
    }

    /// Write identical text to both outputs.
    fn both(&mut self, text: &str) {
        self.line(text, text);
    }

    /// Section heading: `## Heading` in markdown, plain text in stdout.
    fn section(&mut self, heading: &str) {
        self.md.push_str(&format!("## {}\n", heading));
        self.plain.push_str(&format!("{}\n{}\n", heading, "-".repeat(heading.len())));
    }

    fn blank(&mut self) {
        self.md.push('\n');
        self.plain.push('\n');
    }
}

/// Produce a markdown link for the md output; just the label for plain text.
fn md_link(label: &str, path: &str) -> (String, String) {
    (format!("[{}]({})", label, path), label.to_string())
}

pub fn report() -> Result<ReviewWriter> {
    state::ensure_initialized()?;

    let info = state::load_state_info()?;
    let mind = state::load_conjectures(&Layer::Mind)?;
    let mut candidates = state::load_conjectures(&Layer::Candidates)?;
    let problemsets = state::load_problemsets()?;

    let mut w = ReviewWriter::new();

    w.line(
        &format!("# System Review — Run {:03}\n", info.run),
        &format!("System Review — Run {:03}\n{}\n", info.run, "=".repeat(30)),
    );

    // --- Mind ---
    w.section(&format!("Mind ({} conjecture{})", mind.len(), plurals(mind.len())));
    w.blank();
    if mind.is_empty() {
        w.both("  (empty)");
    } else {
        for c in &mind {
            let path = format!("mind/{}.md", c.meta.id);
            let (md_name, plain_name) = md_link(&c.title, &path);
            let stats = format!(
                "score: {:.2}  runs: {:>3}  coverage: {} problem{}",
                c.meta.score,
                c.meta.run_count,
                c.meta.problem_coverage.len(),
                plurals(c.meta.problem_coverage.len()),
            );
            w.line(
                &format!("- {} — {}", md_name, stats),
                &format!("  {:<40} {}", truncate(&plain_name, 40), stats),
            );
        }
    }
    w.blank();

    // --- Top candidates ---
    candidates.sort_by(|a, b| promoter::composite(b).partial_cmp(&promoter::composite(a)).unwrap());
    w.section(&format!(
        "Candidates — Top {} by Composite (of {})",
        TOP_CANDIDATES.min(candidates.len()),
        candidates.len()
    ));
    w.blank();
    if candidates.is_empty() {
        w.both("  (empty)");
    } else {
        for c in candidates.iter().take(TOP_CANDIDATES) {
            let path = format!("candidates/{}.md", c.meta.id);
            let (md_name, plain_name) = md_link(&c.title, &path);
            let stats = format!(
                "composite: {:.2}  score: {:.2}  runs: {:>3}",
                promoter::composite(c),
                c.meta.score,
                c.meta.run_count,
            );
            w.line(
                &format!("- {} — {}", md_name, stats),
                &format!("  {:<40} {}", truncate(&plain_name, 40), stats),
            );
        }
    }
    w.blank();

    // --- Score trajectory ---
    if info.run > 0 {
        w.section("Score Trajectory");
        w.blank();
        w.both(&format!("{:<6} {:>10} {:>10}", "Run", "Avg Score", "Outputs"));
        w.both(&"-".repeat(30));

        let mut run_avgs: Vec<(u32, f64, usize)> = vec![];
        for run in 1..=info.run {
            let outputs = state::load_run_generated(run)?;
            if outputs.is_empty() {
                continue;
            }
            let avg = outputs.iter().map(|o| o.meta.total).sum::<f64>() / outputs.len() as f64;
            run_avgs.push((run, avg, outputs.len()));

            let summary_path = format!("runs/{:03}/summary.md", run);
            let run_label = format!("{:03}", run);
            let (md_run, plain_run) = md_link(&run_label, &summary_path);
            w.line(
                &format!("{:<6} {:>10.3} {:>10}", md_run, avg, outputs.len()),
                &format!("{:<6} {:>10.3} {:>10}", plain_run, avg, outputs.len()),
            );
        }

        if run_avgs.len() >= 2 {
            let first = run_avgs.first().unwrap().1;
            let last = run_avgs.last().unwrap().1;
            let delta = last - first;
            w.blank();
            w.both(&format!(
                "Overall trend: {}{:.3} from run {} to run {}",
                if delta >= 0.0 { "+" } else { "" },
                delta,
                run_avgs.first().unwrap().0,
                run_avgs.last().unwrap().0,
            ));
        }
        w.blank();

        // --- Per-conjecture score history ---
        let has_history = mind.iter().chain(candidates.iter()).any(|c| !c.meta.history.is_empty());
        if has_history {
            w.section("Conjecture Score History");
            w.blank();
            for c in mind.iter().chain(candidates.iter()) {
                if c.meta.history.is_empty() {
                    continue;
                }
                let (layer_dir, layer_tag) = match c.meta.layer {
                    Layer::Mind => ("mind", "[mind]"),
                    Layer::Candidates => ("candidates", "[cand]"),
                };
                let path = format!("{}/{}.md", layer_dir, c.meta.id);
                let (md_name, plain_name) = md_link(&c.title, &path);
                let scores: Vec<String> = c.meta.history.iter().map(|h| format!("{:.2}", h.score)).collect();
                let trend = if c.meta.history.len() >= 2 {
                    let first = c.meta.history.first().unwrap().score;
                    let last = c.meta.history.last().unwrap().score;
                    if last > first + 0.05 { " ↑" } else if last < first - 0.05 { " ↓" } else { " →" }
                } else {
                    ""
                };
                let score_str = scores.join(" → ");
                w.line(
                    &format!("**{}** {}{} (runs: {})\n    {}", layer_tag, md_name, trend, c.meta.run_count, score_str),
                    &format!("{} {}{} (runs: {})\n    {}", layer_tag, plain_name, trend, c.meta.run_count, score_str),
                );
            }
            w.blank();
        }
    }

    // --- Problem sets ---
    w.section(&format!("Problem Sets ({} set{})", problemsets.len(), plurals(problemsets.len())));
    w.blank();
    if problemsets.is_empty() {
        w.both("  (none)");
    } else {
        for ps in &problemsets {
            let title = ps.content.lines().next().unwrap_or("").trim().trim_start_matches('#').trim();
            let ps_path = format!("problemsets/{}.md", ps.meta.id);
            let (md_ps, plain_ps) = md_link(&ps.meta.id, &ps_path);
            let header = format!(
                "{} — {} — {} problem{}, {} run{}",
                md_ps,
                truncate(title, 50),
                ps.meta.problems.len(),
                plurals(ps.meta.problems.len()),
                ps.meta.run_count,
                plural(ps.meta.run_count),
            );
            let plain_header = format!(
                "  [{}] {} — {} problem{}, {} run{}",
                plain_ps,
                truncate(title, 50),
                ps.meta.problems.len(),
                plurals(ps.meta.problems.len()),
                ps.meta.run_count,
                plural(ps.meta.run_count),
            );
            w.line(&format!("- {}", header), &plain_header);
            let mut problems = ps.meta.problems.clone();
            problems.sort_by(|a, b| b.meta.score.partial_cmp(&a.meta.score).unwrap());
            for p in &problems {
                w.line(
                    &format!("  - {} — score: {:.2}  runs: {}", p.meta.id, p.meta.score, p.meta.run_count),
                    &format!("    • {:<50} score: {:.2}  runs: {}", truncate(&p.meta.id, 50), p.meta.score, p.meta.run_count),
                );
            }
        }
    }
    w.blank();

    // --- Last run changes ---
    if let Some(summary) = state::load_last_summary()? {
        if let Some(changes) = extract_changes_section(&summary) {
            let summary_path = format!("runs/{:03}/summary.md", info.run);
            w.section("Last Run Changes");
            w.line(
                &format!("\n_See [last run summary]({}) for full details._\n", summary_path),
                "",
            );
            for line in changes.lines() {
                w.both(line);
            }
            w.blank();
        }
    }

    Ok(w)
}

pub async fn assess(
    client: Arc<LlmClient>,
    config: &Config,
    templates: &PromptTemplates,
    mut w: ReviewWriter,
) -> Result<()> {
    state::ensure_initialized()?;

    let info = state::load_state_info()?;
    let mind = state::load_conjectures(&Layer::Mind)?;
    let mut candidates_for_assess = state::load_conjectures(&Layer::Candidates)?;
    candidates_for_assess.sort_by(|a, b| promoter::composite(b).partial_cmp(&promoter::composite(a)).unwrap());
    candidates_for_assess.truncate(TOP_CANDIDATES);

    if mind.is_empty() {
        anyhow::bail!("No mind conjectures yet. Run first.");
    }

    let mind_system = templates.format_mind_system(&mind);

    let mind_full = mind
        .iter()
        .map(|c| format!("### {}\n\n{}", c.title, c.full_text))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    let top_outputs = if info.run > 0 {
        let mut outputs = state::load_run_generated(info.run)?;
        outputs.sort_by(|a, b| b.meta.total.partial_cmp(&a.meta.total).unwrap());
        outputs
            .into_iter()
            .take(TOP_OUTPUTS_FOR_ASSESS)
            .map(|o| format!(
                "### {} × {} (total: {:.2})\n\n{}",
                o.meta.problem_id, o.meta.conjecture_id, o.meta.total, o.text.trim(),
            ))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    } else {
        "(no runs yet)".to_string()
    };

    let trajectory = if info.run > 0 {
        let mut lines = vec![];
        for run in 1..=info.run {
            let outputs = state::load_run_generated(run)?;
            if outputs.is_empty() { continue; }
            let avg = outputs.iter().map(|o| o.meta.total).sum::<f64>() / outputs.len() as f64;
            lines.push(format!("Run {:03}: {:.3} ({} outputs)", run, avg, outputs.len()));
        }
        lines.join("\n")
    } else {
        "(no runs yet)".to_string()
    };

    // --- Novelty check ---
    let all: Vec<&Conjecture> = mind.iter().chain(candidates_for_assess.iter()).collect();
    if !all.is_empty() {
        eprintln!("Running novelty check ({} conjectures)...", all.len());
        let novelty_futures: Vec<_> = all
            .iter()
            .map(|c| {
                let client = Arc::clone(&client);
                let p = templates.novelty_check(&mind_system, &c.title, &c.summary);
                let title = c.title.clone();
                let id = c.meta.id.clone();
                let layer = c.meta.layer.clone();
                async move {
                    let resp: NoveltyResponse = client.call(Some(&p.system), &p.user, 0.3).await?;
                    Ok::<(String, String, Layer, NoveltyResponse), anyhow::Error>((title, id, layer, resp))
                }
            })
            .collect();
        let mut novelty_entries: Vec<(String, String, Layer, NoveltyResponse)> = join_all(novelty_futures)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();
        novelty_entries.sort_by(|a, b| b.3.novelty_score.partial_cmp(&a.3.novelty_score).unwrap());

        w.section("Novelty Check");
        w.blank();
        for (title, id, layer, resp) in &novelty_entries {
            let layer_dir = match layer { Layer::Mind => "mind", Layer::Candidates => "candidates" };
            let layer_tag = match layer { Layer::Mind => "[mind]", Layer::Candidates => "[cand]" };
            let path = format!("{}/{}.md", layer_dir, id);
            let (md_name, plain_name) = md_link(title, &path);
            let novel_tag = if resp.is_novel { "NOVEL" } else { "KNOWN " };
            w.line(
                &format!("{} {} {:.2}  {}", layer_tag, novel_tag, resp.novelty_score, md_name),
                &format!("{} {} {:.2}  {}", layer_tag, novel_tag, resp.novelty_score, plain_name),
            );
            if let Some(ref analog) = resp.closest_analog {
                w.both(&format!("         Closest: {}", analog));
            }
            w.both(&format!("         {}", resp.explanation));
            w.blank();
        }
    }

    // --- Self-assessment ---
    eprintln!("Running self-assessment...");
    w.section("Self-Assessment");
    w.blank();
    let p = templates.review_assess(&mind_system, &mind_full, &top_outputs, &trajectory);
    let assessment = client.call_raw(Some(&p.system), &p.user, config.temperature).await?;
    w.both(&assessment);
    w.blank();

    // --- Save and print ---
    std::fs::write(REVIEW_PATH, &w.md)?;
    print!("{}", w.plain);

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    }
}

fn plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn plurals(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn extract_changes_section(summary: &str) -> Option<String> {
    let header = "## Changes";
    let start = summary.find(header)?;
    let after = &summary[start + header.len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    let text = after[..end].trim();
    if text.is_empty() { None } else { Some(text.to_string()) }
}
