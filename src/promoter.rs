use crate::types::{Conjecture, HistoryEntry, Problem, Tool};

/// Bottom problem by score within a problem set — candidate for removal from the set.
/// Only considers problems with run_count >= min_run_count.
/// Returns None if all problems are below the threshold (e.g. newly added, unscored).
pub fn find_problem_to_remove(problems: &[Problem], min_run_count: u32) -> Option<&Problem> {
    problems
        .iter()
        .filter(|p| p.meta.run_count >= min_run_count)
        .min_by(|a, b| a.meta.score.partial_cmp(&b.meta.score).unwrap())
}

/// Composite score: raw score weighted by square root of problem coverage breadth.
/// Using sqrt prevents tools that happen to run on many problems from dominating
/// over tools that are genuinely excellent on fewer problems.
fn composite(tool: &Tool) -> f64 {
    let breadth = (tool.meta.problem_coverage.len() as f64).sqrt().max(1.0);
    tool.meta.score * breadth
}

/// Update perspective tool scores and coverage from this run's conjectures.
pub fn update_tool_scores(tools: &mut Vec<Tool>, conjectures: &[Conjecture]) {
    for tool in tools.iter_mut() {
        let mine: Vec<&Conjecture> = conjectures
            .iter()
            .filter(|c| c.meta.tool_id == tool.meta.id)
            .collect();

        if mine.is_empty() {
            continue;
        }

        let mean = mine.iter().map(|c| c.meta.total).sum::<f64>() / mine.len() as f64;
        let n = tool.meta.run_count as f64;
        tool.meta.score = (tool.meta.score * n + mean) / (n + 1.0);
        tool.meta.run_count += 1;

        for c in &mine {
            if !tool.meta.problem_coverage.contains(&c.meta.problem_id) {
                tool.meta.problem_coverage.push(c.meta.problem_id.clone());
            }
            tool.meta.history.push(HistoryEntry {
                run: c.meta.run,
                score: c.meta.total,
                problem_id: c.meta.problem_id.clone(),
            });
        }
    }
}

/// Update problem scores from this run's conjectures.
pub fn update_problem_scores(problems: &mut Vec<Problem>, conjectures: &[Conjecture]) {
    for problem in problems.iter_mut() {
        let mine: Vec<&Conjecture> = conjectures
            .iter()
            .filter(|c| c.meta.problem_id == problem.meta.id)
            .collect();

        if mine.is_empty() {
            continue;
        }

        let mean = mine.iter().map(|c| c.meta.total).sum::<f64>() / mine.len() as f64;
        let n = problem.meta.run_count as f64;
        problem.meta.score = (problem.meta.score * n + mean) / (n + 1.0);
        problem.meta.run_count += 1;
    }
}

/// Top conjecture by total score — candidate for promotion to perspectives.
pub fn find_top_conjecture(conjectures: &[Conjecture]) -> Option<&Conjecture> {
    conjectures
        .iter()
        .max_by(|a, b| a.meta.total.partial_cmp(&b.meta.total).unwrap())
}

/// Top perspective tool by composite score — candidate for promotion to mind.
/// Only considers tools with run_count >= min_run_count.
pub fn find_tool_to_promote(perspectives: &[Tool], min_run_count: u32) -> Option<&Tool> {
    perspectives
        .iter()
        .filter(|t| t.meta.run_count >= min_run_count)
        .max_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom mind tool by composite score — candidate for demotion to perspectives.
/// Only considers tools with run_count >= min_run_count.
pub fn find_mind_tool_to_demote(mind: &[Tool], min_run_count: u32) -> Option<&Tool> {
    mind.iter()
        .filter(|t| t.meta.run_count >= min_run_count)
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom perspective tool by composite score — candidate for discard.
/// Only considers tools with run_count >= min_run_count, excluding the given id.
pub fn find_perspective_to_discard<'a>(
    perspectives: &'a [Tool],
    min_run_count: u32,
    exclude_id: Option<&str>,
) -> Option<&'a Tool> {
    perspectives
        .iter()
        .filter(|t| {
            t.meta.run_count >= min_run_count
                && exclude_id.map_or(true, |id| t.meta.id != id)
        })
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}
