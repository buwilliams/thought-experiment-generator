use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::evaluator;
use crate::llm::LlmClient;
use crate::promoter;
use crate::prompts::PromptTemplates;
use crate::state;
use crate::types::{
    Conjecture, ConjectureMeta, DeduplicateResponse, Generated, Layer, Problem, ProblemMeta,
    ProblemSet, ProblemSource, PromoteResponse, PROBLEMSET_MAX_SIZE,
};

pub async fn run(
    client: Arc<LlmClient>,
    config: &Config,
    problemset_id: Option<&str>,
    new_problem: Option<&str>,
) -> Result<()> {
    let templates = Arc::new(PromptTemplates::load()?);
    state::ensure_initialized()?;

    let mut problemset = state::resolve_problemset(problemset_id)?;

    // Add user-supplied problem before loading the set
    if let Some(text) = new_problem {
        add_user_problem_to_set(text, &mut problemset)?;
    }

    let mind = state::load_conjectures(&Layer::Mind)?;
    let candidates = state::load_conjectures(&Layer::Candidates)?;

    if problemset.meta.problems.is_empty() {
        anyhow::bail!(
            "Problem set '{}' has no problems. Add one with:\n  cargo run -- add-problem --problemset {} --text \"...\"",
            problemset.meta.id,
            problemset.meta.id,
        );
    }
    if candidates.is_empty() {
        anyhow::bail!("No candidate conjectures found in data/state/candidates/.");
    }

    let run = state::increment_run()?;
    println!("\n=== Epistemic Engine — Run {:03} ===", run);
    let ps_display = problemset.content.lines().next().unwrap_or("").trim();
    println!("Problem set:           {} — {}", problemset.meta.id, ps_display);
    let total_lenses = mind.len() + candidates.len();
    println!("Mind conjectures:      {}", mind.len());
    println!("Candidate conjectures: {}", candidates.len());
    println!("Problems:              {}", problemset.meta.problems.len());
    println!("Pairs to process:      {}\n", problemset.meta.problems.len() * total_lenses);

    let mind_system = templates.format_mind_system(&mind);

    // Phase 1 + 2: For each lens, generate all problem outputs concurrently then
    // evaluate them comparatively in one call. One task per lens.
    let mut handles = vec![];
    for lens in candidates.iter().chain(mind.iter()) {
        let all_exist = problemset.meta.problems.iter()
            .all(|p| state::generated_exists(run, &p.meta.id, &lens.meta.id));
        if all_exist {
            info!("Resuming: all outputs exist for lens {}", lens.meta.id);
            continue;
        }

        let client = Arc::clone(&client);
        let templates = Arc::clone(&templates);
        let config = config.clone();
        let mind_system = mind_system.clone();
        let lens_id = lens.meta.id.clone();
        let lens_summary = lens.summary.clone();
        let problemset_context = problemset.content.clone();
        let problems = problemset.meta.problems.clone();

        handles.push(tokio::spawn(async move {
            // Generate all problems for this lens concurrently.
            let mut gen_handles = vec![];
            for problem in &problems {
                let client = Arc::clone(&client);
                let templates = Arc::clone(&templates);
                let mind_system = mind_system.clone();
                let lens_summary = lens_summary.clone();
                let problemset_context = problemset_context.clone();
                let problem_id = problem.meta.id.clone();
                let problem_summary = problem.summary.clone();
                let config = config.clone();
                let lens_id_log = lens_id.clone();

                gen_handles.push(tokio::spawn(async move {
                    let p = templates.generate_output(
                        &mind_system, &lens_summary, &problemset_context, &problem_summary,
                    );
                    let text = client.call_raw(Some(&p.system), &p.user, config.temperature).await?;
                    info!("Generated: {}-{}", problem_id, lens_id_log);
                    anyhow::Ok((problem_id, problem_summary, text))
                }));
            }

            let raw_outputs: Vec<(String, String, String)> = {
                let mut out = vec![];
                for h in gen_handles {
                    match h.await? {
                        Ok(entry) => out.push(entry),
                        Err(e) => tracing::warn!("Generation failed: {e:#}"),
                    }
                }
                out
            };

            if raw_outputs.is_empty() {
                return anyhow::Ok(());
            }

            // Comparative evaluation: one call covers all problems for this lens.
            let generated_list = evaluator::evaluate_comparative(
                &client, &config, &templates, &mind_system,
                &lens_id, &lens_summary, &raw_outputs, run,
            ).await?;

            for g in &generated_list {
                state::save_generated(g)?;
            }

            anyhow::Ok(())
        }));
    }

    for handle in handles {
        if let Err(e) = handle.await? {
            tracing::warn!("Lens evaluation failed: {e:#}");
        }
    }

    let generated = state::load_run_generated(run)?;
    info!("Phase 1+2 complete. {} generated outputs.", generated.len());

    // Phase 3: Rank and promote
    let mut candidates_mut = candidates.clone();
    let mut mind_mut = mind.clone();

    promoter::update_conjecture_scores(&mut candidates_mut, &generated);
    promoter::update_mind_scores(&mut mind_mut, &generated);
    promoter::update_problem_scores(&mut problemset.meta.problems, &generated);

    for conjecture in &candidates_mut {
        state::save_conjecture(conjecture)?;
    }
    for conjecture in &mind_mut {
        state::save_conjecture(conjecture)?;
    }

    let mind_ids: std::collections::HashSet<String> =
        mind.iter().map(|c| c.meta.id.clone()).collect();

    // Admit candidate problems into the problem set
    admit_candidates_to_set(&generated, &mut problemset, config.problem_admission_threshold)?;

    // Problem review: deduplicate within the set
    let removed_ids = run_problem_deduplication(&client, &templates, &mind_system, &problemset.meta.problems).await;
    problemset.meta.problems.retain(|p| !removed_ids.contains(&p.meta.id));
    info!("Deduplication removed {} problems from set", removed_ids.len());

    problemset.meta.run_count += 1;
    state::save_problemset(&problemset)?;

    let problems_removed = removed_ids.len();

    // Promote top generated output → candidates layer
    let promoted_generated_summary =
        promote_top_generated(&client, &templates, &mind_system, &generated, &mind_ids).await;

    // Promote top candidate conjecture → mind
    let (promoted_candidate_name, promoted_candidate_id) =
        promote_top_candidate(&candidates_mut, config.min_run_count)?;

    // Demote bottom mind conjecture → candidates (requires 2× min_run_count)
    let demoted_mind_name = demote_bottom_mind(&mind_mut, config.min_run_count)?;

    // Discard bottom candidate conjecture (excluding what was just promoted)
    let discarded_name =
        discard_bottom_candidate(&candidates_mut, config.min_run_count, promoted_candidate_id.as_deref())?;

    // Phase 4: Report
    generate_report(
        &client, &templates, run, &generated,
        promoted_generated_summary.as_deref(),
        promoted_candidate_name.as_deref(),
        demoted_mind_name.as_deref(),
        discarded_name.as_deref(),
        problems_removed,
    ).await?;

    println!("\nLLM calls used: {}", client.calls_made());
    Ok(())
}

pub async fn run_all(client: Arc<LlmClient>, config: &Config) -> Result<()> {
    state::ensure_initialized()?;
    let sets = state::load_problemsets()?;
    if sets.is_empty() {
        anyhow::bail!("No problem sets found. Create one with:\n  cargo run -- create-problemset \"...\"");
    }
    let pending: Vec<_> = sets.iter()
        .filter(|ps| ps.meta.problems.iter().any(|p| p.meta.run_count == 0))
        .collect();
    if pending.is_empty() {
        println!("All {} problem set(s) are fully processed. Use --fresh to reset and re-run all.", sets.len());
        return Ok(());
    }
    println!("Running {} of {} problem set(s) ({} fully processed)...", pending.len(), sets.len(), sets.len() - pending.len());
    for ps in &pending {
        let display = ps.content.lines().next().unwrap_or("").trim();
        println!("\n=== Problem set: {} — {} ===", ps.meta.id, display);
        run(Arc::clone(&client), config, Some(&ps.meta.id), None).await?;
    }

    println!("\n=== Review ===");
    let templates = PromptTemplates::load()?;
    let writer = crate::review::report()?;
    crate::review::assess(client, config, &templates, writer).await?;

    Ok(())
}

pub async fn read(_: &Config) -> Result<()> {
    state::ensure_initialized()?;
    match state::load_last_summary()? {
        Some(summary) => println!("{summary}"),
        None => println!("No runs yet. Run: cargo run -- run"),
    }
    Ok(())
}

// --- Helpers ---

fn add_user_problem_to_set(text: &str, problemset: &mut ProblemSet) -> Result<()> {
    if problemset.meta.problems.len() >= PROBLEMSET_MAX_SIZE {
        anyhow::bail!(
            "Problem set '{}' is at capacity ({} problems). Remove a problem first with:\n  cargo run -- remove-problem --problemset {} --problem-id <id>",
            problemset.meta.id,
            PROBLEMSET_MAX_SIZE,
            problemset.meta.id,
        );
    }
    let id = state::slugify(&text.chars().take(60).collect::<String>());
    if problemset.meta.problems.iter().any(|p| p.meta.id == id) {
        info!("Problem already in set: {}", id);
        return Ok(());
    }
    let rank = problemset.meta.problems.len() as u32 + 1;
    let problem = Problem {
        meta: ProblemMeta {
            id: id.clone(),
            source: ProblemSource::User,
            score: 0.0,
            rank,
            run_count: 0,
            created_at: state::now_iso8601(),
        },
        title: text.chars().take(80).collect(),
        summary: text.chars().take(200).collect(),
        full_text: text.to_string(),
    };
    problemset.meta.problems.push(problem);
    state::save_problemset(problemset)?;
    info!("Added problem to set: {}", id);
    Ok(())
}

fn admit_candidates_to_set(
    generated: &[Generated],
    problemset: &mut ProblemSet,
    admission_threshold: f64,
) -> Result<()> {
    const MAX_ADMISSIONS_PER_RUN: usize = 3;

    // Collect all qualifying candidates across all outputs, sorted by score descending.
    let mut pool: Vec<(f64, String)> = generated
        .iter()
        .flat_map(|o| o.meta.candidate_problems.iter())
        .filter(|cp| cp.score >= admission_threshold)
        .map(|cp| (cp.score, cp.text.clone()))
        .collect();
    pool.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut rank_base = problemset.meta.problems.len() as u32;
    let mut in_set: std::collections::HashSet<String> =
        problemset.meta.problems.iter().map(|p| p.meta.id.clone()).collect();
    let mut admitted = 0;

    for (score, text) in &pool {
        if admitted >= MAX_ADMISSIONS_PER_RUN {
            break;
        }
        if problemset.meta.problems.len() >= PROBLEMSET_MAX_SIZE {
            info!("Problem set at capacity ({}), skipping further admissions this run", PROBLEMSET_MAX_SIZE);
            break;
        }
        let id = state::slugify(&text.chars().take(60).collect::<String>());
        if in_set.contains(&id) {
            continue;
        }
        rank_base += 1;
        let problem = Problem {
            meta: ProblemMeta {
                id: id.clone(),
                source: ProblemSource::System,
                score: 0.0,
                rank: rank_base,
                run_count: 0,
                created_at: state::now_iso8601(),
            },
            title: text.chars().take(80).collect(),
            summary: text.chars().take(200).collect(),
            full_text: text.clone(),
        };
        info!("Created candidate problem (score {score:.2}): {}", id);
        problemset.meta.problems.push(problem);
        in_set.insert(id);
        admitted += 1;
    }
    Ok(())
}

async fn run_problem_deduplication(
    client: &LlmClient,
    templates: &PromptTemplates,
    mind_system: &str,
    problems: &[Problem],
) -> Vec<String> {
    if problems.len() < 2 {
        return vec![];
    }
    let summaries: Vec<(String, String)> = problems
        .iter()
        .map(|p| (p.meta.id.clone(), p.summary.clone()))
        .collect();
    let p = templates.deduplicate_problems(mind_system, &summaries);
    match client
        .call::<DeduplicateResponse>(Some(&p.system), &p.user, 0.3)
        .await
    {
        Ok(resp) => {
            if resp.remove.len() >= problems.len() {
                tracing::warn!("Deduplication would remove all problems — skipping");
                return vec![];
            }
            resp.remove
        }
        Err(e) => {
            tracing::warn!("Problem deduplication failed: {e}");
            vec![]
        }
    }
}

async fn promote_top_generated(
    client: &LlmClient,
    templates: &PromptTemplates,
    mind_system: &str,
    generated: &[Generated],
    mind_ids: &std::collections::HashSet<String>,
) -> Option<String> {
    let top = promoter::find_top_generated(generated, mind_ids)?;
    let p = templates.promote_generated(mind_system, &top.text, top.meta.total);
    match client
        .call::<PromoteResponse>(Some(&p.system), &p.user, 0.3)
        .await
    {
        Ok(resp) => {
            let count = state::load_conjectures(&Layer::Candidates)
                .map(|c| c.len())
                .unwrap_or(0);
            let id = state::slugify(&resp.title);
            let new_conjecture = Conjecture {
                meta: ConjectureMeta {
                    id: id.clone(),
                    layer: Layer::Candidates,
                    score: top.meta.total,
                    rank: count as u32 + 1,
                    run_count: 1,
                    problem_coverage: vec![top.meta.problem_id.clone()],
                    created_at: state::now_iso8601(),
                    promoted_from: None,
                    history: vec![],
                },
                title: resp.title.clone(),
                summary: resp.summary.clone(),
                full_text: resp.full_text,
            };
            if let Err(e) = state::save_conjecture(&new_conjecture) {
                tracing::warn!("Failed to save promoted generated output: {e}");
                return None;
            }
            info!("Promoted generated output to candidates: {}", id);
            Some(resp.summary)
        }
        Err(e) => {
            tracing::warn!("Failed to promote generated output: {e}");
            None
        }
    }
}

fn promote_top_candidate(
    candidates: &[Conjecture],
    min_run_count: u32,
) -> Result<(Option<String>, Option<String>)> {
    match promoter::find_conjecture_to_promote(candidates, min_run_count) {
        Some(conjecture) => {
            let id = conjecture.meta.id.clone();
            let name = conjecture.title.clone();
            let mut promoted = conjecture.clone();
            let mind_count = state::load_conjectures(&Layer::Mind)?.len();
            promoted.meta.layer = Layer::Mind;
            promoted.meta.rank = mind_count as u32 + 1;
            state::delete_conjecture(&id, &Layer::Candidates)?;
            state::save_conjecture(&promoted)?;
            info!("Promoted candidate conjecture to mind: {}", id);
            Ok((Some(name), Some(id)))
        }
        None => Ok((None, None)),
    }
}

fn demote_bottom_mind(mind: &[Conjecture], min_run_count: u32) -> Result<Option<String>> {
    match promoter::find_conjecture_to_demote(mind, min_run_count) {
        Some(conjecture) => {
            let id = conjecture.meta.id.clone();
            let name = conjecture.title.clone();
            let mut demoted = conjecture.clone();
            let cand_count = state::load_conjectures(&Layer::Candidates)?.len();
            demoted.meta.layer = Layer::Candidates;
            demoted.meta.rank = cand_count as u32 + 1;
            state::delete_conjecture(&id, &Layer::Mind)?;
            state::save_conjecture(&demoted)?;
            info!("Demoted mind conjecture to candidates: {}", id);
            Ok(Some(name))
        }
        None => Ok(None),
    }
}

fn discard_bottom_candidate(
    candidates: &[Conjecture],
    min_run_count: u32,
    exclude_id: Option<&str>,
) -> Result<Option<String>> {
    match promoter::find_conjecture_to_discard(candidates, min_run_count, exclude_id) {
        Some(conjecture) => {
            let id = conjecture.meta.id.clone();
            let name = conjecture.title.clone();
            state::delete_conjecture(&id, &Layer::Candidates)?;
            info!("Discarded candidate conjecture: {}", id);
            Ok(Some(name))
        }
        None => Ok(None),
    }
}

async fn generate_report(
    client: &LlmClient,
    templates: &PromptTemplates,
    run: u32,
    generated: &[Generated],
    promoted_generated: Option<&str>,
    promoted_candidate: Option<&str>,
    demoted_mind: Option<&str>,
    discarded: Option<&str>,
    problems_removed: usize,
) -> Result<()> {
    let mut sorted = generated.to_vec();
    sorted.sort_by(|a, b| b.meta.total.partial_cmp(&a.meta.total).unwrap());

    let mut out = format!("# Run {:03} Results\n\n", run);

    out.push_str("| Rank | Problem | Conjecture | Consist. | HtV | Reach | Refut. | Total |\n");
    out.push_str("|------|---------|------------|----------|-----|-------|--------|-------|\n");
    for (rank, g) in sorted.iter().enumerate() {
        let output_link = format!("{}-{}.md", g.meta.problem_id, g.meta.conjecture_id);
        let conjecture_link = format!("../../candidates/{}.md", g.meta.conjecture_id);
        out.push_str(&format!(
            "| [{}]({}) | {} | [{}]({}) | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
            rank + 1,
            output_link,
            g.meta.problem_id,
            g.meta.conjecture_id,
            conjecture_link,
            g.meta.logical_consistency,
            g.meta.hard_to_vary,
            g.meta.explanatory_reach,
            g.meta.resistance_to_refutation,
            g.meta.total,
        ));
    }

    out.push_str("\n## Top 5\n\n");
    for (rank, g) in sorted.iter().take(5).enumerate() {
        let p = templates.summarize_generated(&g.text, g.meta.total);
        let summary = client
            .call_raw(Some(&p.system), &p.user, 0.3)
            .await
            .unwrap_or_else(|_| "(summary unavailable)".to_string());
        let output_link = format!("{}-{}.md", g.meta.problem_id, g.meta.conjecture_id);
        let conjecture_link = format!("../../candidates/{}.md", g.meta.conjecture_id);
        out.push_str(&format!(
            "**{}. {} × [{}]({})** ([output]({output_link})) (total: {:.2})  \n{}\n\n",
            rank + 1,
            g.meta.problem_id,
            g.meta.conjecture_id,
            conjecture_link,
            g.meta.total,
            summary.trim(),
        ));
    }

    out.push_str("## Changes\n\n");
    let any_change = promoted_generated.is_some()
        || promoted_candidate.is_some()
        || demoted_mind.is_some()
        || discarded.is_some()
        || problems_removed > 0;
    if let Some(s) = promoted_generated {
        out.push_str(&format!("- Promoted generated output → candidates: \"{s}\"\n"));
    }
    if let Some(s) = promoted_candidate {
        out.push_str(&format!("- Promoted candidate conjecture → mind: \"{s}\"\n"));
    }
    if let Some(s) = demoted_mind {
        out.push_str(&format!("- Demoted mind conjecture → candidates: \"{s}\"\n"));
    }
    if let Some(s) = discarded {
        out.push_str(&format!("- Discarded candidate conjecture: \"{s}\"\n"));
    }
    if problems_removed > 0 {
        out.push_str(&format!(
            "- Removed {problems_removed} problem(s) from set (deduplication/cap enforcement).\n"
        ));
    }
    if !any_change {
        out.push_str("- No promotions or demotions this run (insufficient run counts).\n");
    }

    state::save_run_summary(run, &out)?;
    println!("{out}");
    Ok(())
}
