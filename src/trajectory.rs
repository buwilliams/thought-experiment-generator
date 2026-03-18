use anyhow::Result;

use crate::state;
use crate::types::Layer;

pub fn report() -> Result<()> {
    state::ensure_initialized()?;

    let info = state::load_state_info()?;
    if info.run == 0 {
        println!("No runs yet.");
        return Ok(());
    }

    // --- Run score trend ---
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

    // --- Per-conjecture score history ---
    let mind = state::load_conjectures(&Layer::Mind)?;
    let candidates = state::load_conjectures(&Layer::Candidates)?;

    let has_history = mind.iter().chain(candidates.iter()).any(|c| !c.meta.history.is_empty());
    if !has_history {
        return Ok(());
    }

    println!("\n## Conjecture Score History\n");

    for c in mind.iter().chain(candidates.iter()) {
        if c.meta.history.is_empty() {
            continue;
        }
        let layer_tag = match c.meta.layer {
            Layer::Mind => "[mind]",
            Layer::Candidates => "[cand]",
        };
        let scores: Vec<String> = c.meta.history.iter().map(|h| format!("{:.2}", h.score)).collect();
        // Trend arrow
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

    Ok(())
}
