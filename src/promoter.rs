use crate::types::{Candidate, Conjecture, HistoryEntry, Problem};

/// Bottom problem by score within a problem set — candidate for removal from the set.
/// Only considers problems with run_count >= min_run_count.
pub fn find_problem_to_remove(problems: &[Problem], min_run_count: u32) -> Option<&Problem> {
    problems
        .iter()
        .filter(|p| p.meta.run_count >= min_run_count)
        .min_by(|a, b| a.meta.score.partial_cmp(&b.meta.score).unwrap())
}

/// Composite score: raw score weighted by square root of problem coverage breadth.
fn composite(c: &Conjecture) -> f64 {
    let breadth = (c.meta.problem_coverage.len() as f64).sqrt().max(1.0);
    c.meta.score * breadth
}

/// Update perspective conjecture scores and coverage from this run's candidates.
pub fn update_conjecture_scores(conjectures: &mut Vec<Conjecture>, candidates: &[Candidate]) {
    for conjecture in conjectures.iter_mut() {
        let mine: Vec<&Candidate> = candidates
            .iter()
            .filter(|c| c.meta.conjecture_id == conjecture.meta.id)
            .collect();

        if mine.is_empty() {
            continue;
        }

        let mean = mine.iter().map(|c| c.meta.total).sum::<f64>() / mine.len() as f64;
        let n = conjecture.meta.run_count as f64;
        conjecture.meta.score = (conjecture.meta.score * n + mean) / (n + 1.0);
        conjecture.meta.run_count += 1;

        for c in &mine {
            if !conjecture.meta.problem_coverage.contains(&c.meta.problem_id) {
                conjecture.meta.problem_coverage.push(c.meta.problem_id.clone());
            }
            conjecture.meta.history.push(HistoryEntry {
                run: c.meta.run,
                score: c.meta.total,
                problem_id: c.meta.problem_id.clone(),
            });
        }
    }
}

/// Update problem scores from this run's candidates.
pub fn update_problem_scores(problems: &mut Vec<Problem>, candidates: &[Candidate]) {
    for problem in problems.iter_mut() {
        let mine: Vec<&Candidate> = candidates
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

/// Top candidate by total score — candidate for promotion to perspectives.
pub fn find_top_candidate(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates
        .iter()
        .max_by(|a, b| a.meta.total.partial_cmp(&b.meta.total).unwrap())
}

/// Top perspective conjecture by composite score — candidate for promotion to mind.
/// Only considers conjectures with run_count >= min_run_count.
pub fn find_conjecture_to_promote(perspectives: &[Conjecture], min_run_count: u32) -> Option<&Conjecture> {
    perspectives
        .iter()
        .filter(|c| c.meta.run_count >= min_run_count)
        .max_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom mind conjecture by composite score — candidate for demotion to perspectives.
/// Only considers conjectures with run_count >= min_run_count.
pub fn find_conjecture_to_demote(mind: &[Conjecture], min_run_count: u32) -> Option<&Conjecture> {
    mind.iter()
        .filter(|c| c.meta.run_count >= min_run_count)
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom perspective conjecture by composite score — candidate for discard.
/// Only considers conjectures with run_count >= min_run_count, excluding the given id.
pub fn find_conjecture_to_discard<'a>(
    perspectives: &'a [Conjecture],
    min_run_count: u32,
    exclude_id: Option<&str>,
) -> Option<&'a Conjecture> {
    perspectives
        .iter()
        .filter(|c| {
            c.meta.run_count >= min_run_count
                && exclude_id.map_or(true, |id| c.meta.id != id)
        })
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}
