use anyhow::Result;

use crate::promoter;
use crate::state;
use crate::types::Layer;

const TOP_CANDIDATES: usize = 5;

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
        let mut run_avgs: Vec<(u32, f64, usize)> = vec![];
        for run in 1..=info.run {
            let outputs = state::load_run_generated(run)?;
            if outputs.is_empty() {
                continue;
            }
            let avg = outputs.iter().map(|o| o.meta.total).sum::<f64>() / outputs.len() as f64;
            run_avgs.push((run, avg, outputs.len()));
        }
        for (run, avg, count) in &run_avgs {
            println!("  Run {:03}  {:.3}  ({} outputs)", run, avg, count);
        }
        if run_avgs.len() >= 2 {
            let first = run_avgs.first().unwrap().1;
            let last = run_avgs.last().unwrap().1;
            let delta = last - first;
            println!(
                "\n  Overall trend: {}{:.3} over {} runs",
                if delta >= 0.0 { "+" } else { "" },
                delta,
                run_avgs.len(),
            );
        }
        println!();
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
