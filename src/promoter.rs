use crate::types::{Conjecture, Generated, HistoryEntry, Problem};

/// Bottom problem by score within a problem set — candidate for removal from the set.
/// Only considers problems with run_count >= min_run_count.
pub fn find_problem_to_remove(problems: &[Problem], min_run_count: u32) -> Option<&Problem> {
    problems
        .iter()
        .filter(|p| p.meta.run_count >= min_run_count)
        .min_by(|a, b| a.meta.score.partial_cmp(&b.meta.score).unwrap())
}

/// Composite score: raw score weighted by square root of problem coverage breadth.
pub fn composite(c: &Conjecture) -> f64 {
    let breadth = (c.meta.problem_coverage.len() as f64).sqrt().max(1.0);
    c.meta.score * breadth
}

/// Update candidate conjecture scores and coverage from this run's generated outputs.
pub fn update_conjecture_scores(conjectures: &mut Vec<Conjecture>, generated: &[Generated]) {
    for conjecture in conjectures.iter_mut() {
        let mine: Vec<&Generated> = generated
            .iter()
            .filter(|g| g.meta.conjecture_id == conjecture.meta.id)
            .collect();

        if mine.is_empty() {
            continue;
        }

        let mean = mine.iter().map(|g| g.meta.total).sum::<f64>() / mine.len() as f64;
        let n = conjecture.meta.run_count as f64;
        conjecture.meta.score = (conjecture.meta.score * n + mean) / (n + 1.0);
        conjecture.meta.run_count += 1;

        for g in &mine {
            if !conjecture.meta.problem_coverage.contains(&g.meta.problem_id) {
                conjecture.meta.problem_coverage.push(g.meta.problem_id.clone());
            }
            conjecture.meta.history.push(HistoryEntry {
                run: g.meta.run,
                score: g.meta.total,
                problem_id: g.meta.problem_id.clone(),
            });
        }
    }
}

/// Update problem scores from this run's generated outputs.
pub fn update_problem_scores(problems: &mut Vec<Problem>, generated: &[Generated]) {
    for problem in problems.iter_mut() {
        let mine: Vec<&Generated> = generated
            .iter()
            .filter(|g| g.meta.problem_id == problem.meta.id)
            .collect();

        if mine.is_empty() {
            continue;
        }

        let mean = mine.iter().map(|g| g.meta.total).sum::<f64>() / mine.len() as f64;
        let n = problem.meta.run_count as f64;
        problem.meta.score = (problem.meta.score * n + mean) / (n + 1.0);
        problem.meta.run_count += 1;
    }
}

/// Top generated output by total score — candidate for promotion to candidates layer.
pub fn find_top_generated(generated: &[Generated]) -> Option<&Generated> {
    generated
        .iter()
        .max_by(|a, b| a.meta.total.partial_cmp(&b.meta.total).unwrap())
}

/// Top candidate conjecture by composite score — candidate for promotion to mind.
/// Only considers conjectures with run_count >= min_run_count.
pub fn find_conjecture_to_promote(candidates: &[Conjecture], min_run_count: u32) -> Option<&Conjecture> {
    candidates
        .iter()
        .filter(|c| c.meta.run_count >= min_run_count)
        .max_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom mind conjecture by composite score — candidate for demotion to candidates layer.
/// Only considers conjectures with run_count >= min_run_count.
pub fn find_conjecture_to_demote(mind: &[Conjecture], min_run_count: u32) -> Option<&Conjecture> {
    mind.iter()
        .filter(|c| c.meta.run_count >= min_run_count)
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}

/// Bottom candidate conjecture by composite score — candidate for discard.
/// Only considers conjectures with run_count >= min_run_count, excluding the given id.
pub fn find_conjecture_to_discard<'a>(
    candidates: &'a [Conjecture],
    min_run_count: u32,
    exclude_id: Option<&str>,
) -> Option<&'a Conjecture> {
    candidates
        .iter()
        .filter(|c| {
            c.meta.run_count >= min_run_count
                && exclude_id.map_or(true, |id| c.meta.id != id)
        })
        .min_by(|a, b| composite(a).partial_cmp(&composite(b)).unwrap())
}
