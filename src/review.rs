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

pub fn report() -> Result<()> {
    state::ensure_initialized()?;

    let info = state::load_state_info()?;
    let mind = state::load_conjectures(&Layer::Mind)?;
    let mut candidates = state::load_conjectures(&Layer::Candidates)?;
    let problemsets = state::load_problemsets()?;

    println!("=== System Review — Run {:03} ===\n", info.run);

    // --- Mind ---
    println!("## Mind ({} conjecture{})\n", mind.len(), if mind.len() == 1 { "" } else { "s" });
    if mind.is_empty() {
        println!("  (empty)\n");
    } else {
        for c in &mind {
            println!(
                "  {:<40} score: {:.2}  runs: {:>3}  coverage: {} problem{}",
                truncate(&c.title, 40),
                c.meta.score,
                c.meta.run_count,
                c.meta.problem_coverage.len(),
                if c.meta.problem_coverage.len() == 1 { "" } else { "s" },
            );
        }
        println!();
    }

    // --- Top candidates ---
    candidates.sort_by(|a, b| {
        promoter::composite(b)
            .partial_cmp(&promoter::composite(a))
            .unwrap()
    });
    println!(
        "## Candidates — Top {} by Composite (of {})\n",
        TOP_CANDIDATES.min(candidates.len()),
        candidates.len()
    );
    if candidates.is_empty() {
        println!("  (empty)\n");
    } else {
        for c in candidates.iter().take(TOP_CANDIDATES) {
            println!(
                "  {:<40} composite: {:.2}  score: {:.2}  runs: {:>3}",
                truncate(&c.title, 40),
                promoter::composite(c),
                c.meta.score,
                c.meta.run_count,
            );
        }
        println!();
    }

    // --- Score trajectory ---
    if info.run > 0 {
        println!("## Score Trajectory\n");
        println!("{:<6} {:>10} {:>10}", "Run", "Avg Score", "Outputs");
        println!("{}", "-".repeat(30));
        let mut run_avgs: Vec<(u32, f64, usize)> = vec![];
        for run in 1..=info.run {
            let outputs = state::load_run_generated(run)?;
            if outputs.is_empty() {
                continue;
            }
            let avg = outputs.iter().map(|o| o.meta.total).sum::<f64>() / outputs.len() as f64;
            run_avgs.push((run, avg, outputs.len()));
            println!("{:<6} {:>10.3} {:>10}", run, avg, outputs.len());
        }
        if run_avgs.len() >= 2 {
            let first = run_avgs.first().unwrap().1;
            let last = run_avgs.last().unwrap().1;
            let delta = last - first;
            println!(
                "\nOverall trend: {}{:.3} from run {} to run {}",
                if delta >= 0.0 { "+" } else { "" },
                delta,
                run_avgs.first().unwrap().0,
                run_avgs.last().unwrap().0,
            );
        }
        println!();

        // --- Per-conjecture score history ---
        let has_history = mind.iter().chain(candidates.iter()).any(|c| !c.meta.history.is_empty());
        if has_history {
            println!("## Conjecture Score History\n");
            for c in mind.iter().chain(candidates.iter()) {
                if c.meta.history.is_empty() {
                    continue;
                }
                let layer_tag = match c.meta.layer {
                    Layer::Mind => "[mind]",
                    Layer::Candidates => "[cand]",
                };
                let scores: Vec<String> = c.meta.history.iter().map(|h| format!("{:.2}", h.score)).collect();
                let trend = if c.meta.history.len() >= 2 {
                    let first = c.meta.history.first().unwrap().score;
                    let last = c.meta.history.last().unwrap().score;
                    if last > first + 0.05 { " ↑" } else if last < first - 0.05 { " ↓" } else { " →" }
                } else {
                    ""
                };
                println!("{} {}{} (runs: {})", layer_tag, c.title, trend, c.meta.run_count);
                println!("    {}", scores.join(" → "));
            }
            println!();
        }
    }

    // --- Problem sets ---
    println!("## Problem Sets ({} set{})\n", problemsets.len(), if problemsets.len() == 1 { "" } else { "s" });
    if problemsets.is_empty() {
        println!("  (none)\n");
    } else {
        for ps in &problemsets {
            let title = ps.content.lines().next().unwrap_or("").trim().trim_start_matches('#').trim();
            println!(
                "  [{}] {} — {} problem{}, {} run{}",
                ps.meta.id,
                truncate(title, 50),
                ps.meta.problems.len(),
                if ps.meta.problems.len() == 1 { "" } else { "s" },
                ps.meta.run_count,
                if ps.meta.run_count == 1 { "" } else { "s" },
            );
            let mut problems = ps.meta.problems.clone();
            problems.sort_by(|a, b| b.meta.score.partial_cmp(&a.meta.score).unwrap());
            for p in &problems {
                println!(
                    "    • {:<50} score: {:.2}  runs: {}",
                    truncate(&p.meta.id, 50),
                    p.meta.score,
                    p.meta.run_count,
                );
            }
        }
        println!();
    }

    // --- Last run changes ---
    if let Some(summary) = state::load_last_summary()? {
        if let Some(changes) = extract_changes_section(&summary) {
            println!("## Last Run Changes\n");
            for line in changes.lines() {
                println!("  {}", line);
            }
            println!();
        }
    }

    Ok(())
}

pub async fn assess(
    client: Arc<LlmClient>,
    config: &Config,
    templates: &PromptTemplates,
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

    // Full mind text for assessment
    let mind_full = mind
        .iter()
        .map(|c| format!("### {}\n\n{}", c.title, c.full_text))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    // Top outputs from most recent run
    let top_outputs = if info.run > 0 {
        let mut outputs = state::load_run_generated(info.run)?;
        outputs.sort_by(|a, b| b.meta.total.partial_cmp(&a.meta.total).unwrap());
        outputs
            .into_iter()
            .take(TOP_OUTPUTS_FOR_ASSESS)
            .map(|o| {
                format!(
                    "### {} × {} (total: {:.2})\n\n{}",
                    o.meta.problem_id,
                    o.meta.conjecture_id,
                    o.meta.total,
                    o.text.trim(),
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    } else {
        "(no runs yet)".to_string()
    };

    // Score trajectory string
    let trajectory = if info.run > 0 {
        let mut lines = vec![];
        for run in 1..=info.run {
            let outputs = state::load_run_generated(run)?;
            if outputs.is_empty() {
                continue;
            }
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
        println!("## Novelty Check\n");
        let novelty_futures: Vec<_> = all
            .iter()
            .map(|c| {
                let client = Arc::clone(&client);
                let p = templates.novelty_check(&mind_system, &c.title, &c.summary);
                let title = c.title.clone();
                let layer = c.meta.layer.clone();
                async move {
                    let resp: NoveltyResponse = client.call(Some(&p.system), &p.user, 0.3).await?;
                    Ok::<(String, Layer, NoveltyResponse), anyhow::Error>((title, layer, resp))
                }
            })
            .collect();
        let mut novelty_entries: Vec<(String, Layer, NoveltyResponse)> = join_all(novelty_futures)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();
        novelty_entries.sort_by(|a, b| b.2.novelty_score.partial_cmp(&a.2.novelty_score).unwrap());
        for (title, layer, resp) in &novelty_entries {
            let layer_tag = match layer {
                Layer::Mind => "[mind]",
                Layer::Candidates => "[cand]",
            };
            let novel_tag = if resp.is_novel { "NOVEL" } else { "KNOWN " };
            println!("{} {} {:.2}  {}", layer_tag, novel_tag, resp.novelty_score, title);
            if let Some(ref analog) = resp.closest_analog {
                println!("         Closest: {}", analog);
            }
            println!("         {}\n", resp.explanation);
        }
    }

    // --- Self-assessment ---
    println!("## Self-Assessment\n");
    let p = templates.review_assess(&mind_system, &mind_full, &top_outputs, &trajectory);
    let assessment = client.call_raw(Some(&p.system), &p.user, config.temperature).await?;
    println!("{}", assessment);

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    }
}

fn extract_changes_section(summary: &str) -> Option<String> {
    let header = "## Changes";
    let start = summary.find(header)?;
    let after = &summary[start + header.len()..];
    let end = after.find("\n## ").unwrap_or(after.len());
    let text = after[..end].trim();
    if text.is_empty() { None } else { Some(text.to_string()) }
}
