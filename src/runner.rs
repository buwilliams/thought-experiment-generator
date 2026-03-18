use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::evaluator;
use crate::llm::LlmClient;
use crate::promoter;
use crate::prompts;
use crate::state;
use crate::types::{
    Conjecture, Layer, Problem, ProblemMeta, ProblemSource, Tool, ToolMeta, ToolSummaryResponse,
};

pub async fn run(client: Arc<LlmClient>, config: &Config, new_problem: Option<&str>) -> Result<()> {
    state::ensure_initialized()?;

    if let Some(text) = new_problem {
        add_user_problem(text)?;
    }

    let mind = state::load_tools(&Layer::Mind)?;
    let perspectives = state::load_tools(&Layer::Perspectives)?;
    let problems = state::load_problems()?;

    if problems.is_empty() {
        anyhow::bail!(
            "No problems found. Add one with: cargo run -- run --problem \"your problem\""
        );
    }
    if perspectives.is_empty() {
        anyhow::bail!("No perspective tools found in data/state/perspectives/.");
    }

    let run = state::increment_run()?;
    println!("\n=== Epistemic Engine — Run {:03} ===", run);
    println!("Mind tools: {}", mind.len());
    println!("Perspective tools: {}", perspectives.len());
    println!("Problems: {}", problems.len());
    println!("Pairs to process: {}\n", problems.len() * perspectives.len());

    let mind_system = prompts::format_mind_system(&mind);

    // Phase 1 + 2: Generate and evaluate all (problem, tool) pairs concurrently
    let mut handles = vec![];
    for problem in &problems {
        for tool in &perspectives {
            if state::conjecture_exists(run, &problem.meta.id, &tool.meta.id) {
                info!("Resuming: skipping existing conjecture {}-{}", problem.meta.id, tool.meta.id);
                continue;
            }
            let client = Arc::clone(&client);
            let config = config.clone();
            let mind_system = mind_system.clone();
            let tool_summary = tool.summary.clone();
            let problem_summary = problem.summary.clone();
            let problem_id = problem.meta.id.clone();
            let tool_id = tool.meta.id.clone();

            handles.push(tokio::spawn(async move {
                let p = prompts::conjecture_generation(&mind_system, &tool_summary, &problem_summary);
                let text = client.call_raw(Some(&p.system), &p.user, config.temperature).await?;

                info!("Generated: {}-{}", problem_id, tool_id);

                let conjecture = evaluator::evaluate(
                    &client, &config, &mind_system,
                    &text, &problem_summary,
                    &problem_id, &tool_id, run,
                ).await?;

                state::save_conjecture(&conjecture)?;
                anyhow::Ok(conjecture)
            }));
        }
    }

    for handle in handles {
        if let Err(e) = handle.await? {
            tracing::warn!("Conjecture failed: {e}");
        }
    }

    // Load all conjectures for this run (includes resumed ones)
    let conjectures = state::load_run_conjectures(run)?;
    info!("Phase 1+2 complete. {} conjectures.", conjectures.len());

    // Phase 3: Rank and promote
    let mut perspectives_mut = perspectives.clone();
    let mut problems_mut = problems.clone();

    promoter::update_tool_scores(&mut perspectives_mut, &conjectures);
    promoter::update_problem_scores(&mut problems_mut, &conjectures);

    for tool in &perspectives_mut {
        state::save_tool(tool)?;
    }
    for problem in &problems_mut {
        state::save_problem(problem)?;
    }

    admit_candidate_problems(&conjectures, &problems_mut)?;

    // Promote top conjecture → perspectives
    let promoted_conjecture_summary = promote_top_conjecture(
        &client, config, &mind_system, &conjectures, run,
    ).await;

    // Promote top perspective tool → mind (track its id to exclude from discard)
    let (promoted_tool_name, promoted_tool_id) =
        promote_top_tool(&perspectives_mut, config.min_run_count)?;

    // Demote bottom mind tool → perspectives
    let demoted_mind_name = demote_bottom_mind(&mind, config.min_run_count)?;

    // Discard bottom perspective tool (excluding what was just promoted)
    let discarded_name =
        discard_bottom_perspective(&perspectives_mut, config.min_run_count, promoted_tool_id.as_deref())?;

    // Phase 4: Report
    generate_report(
        &client, run, &conjectures,
        promoted_conjecture_summary.as_deref(),
        promoted_tool_name.as_deref(),
        demoted_mind_name.as_deref(),
        discarded_name.as_deref(),
    ).await?;

    println!("\nLLM calls used: {}", client.calls_made());
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

fn add_user_problem(text: &str) -> Result<()> {
    let id = state::slugify(&text.chars().take(60).collect::<String>());
    if state::problem_exists(&id) {
        info!("Problem already exists: {}", id);
        return Ok(());
    }
    let count = state::load_problems()?.len();
    let problem = Problem {
        meta: ProblemMeta {
            id: id.clone(),
            source: ProblemSource::User,
            score: 0.0,
            rank: count as u32 + 1,
            run_count: 0,
            created_at: state::now_iso8601(),
        },
        title: text.chars().take(80).collect(),
        summary: text.chars().take(200).collect(),
        full_text: text.to_string(),
    };
    state::save_problem(&problem)?;
    info!("Added problem: {}", id);
    Ok(())
}

fn admit_candidate_problems(conjectures: &[Conjecture], existing: &[Problem]) -> Result<()> {
    let existing_ids: std::collections::HashSet<&str> =
        existing.iter().map(|p| p.meta.id.as_str()).collect();
    let mut rank_offset = existing.len() as u32;

    for conjecture in conjectures {
        for candidate in &conjecture.meta.candidate_problems {
            let id = state::slugify(&candidate.text.chars().take(60).collect::<String>());
            if existing_ids.contains(id.as_str()) || state::problem_exists(&id) {
                continue;
            }
            rank_offset += 1;
            let problem = Problem {
                meta: ProblemMeta {
                    id: id.clone(),
                    source: ProblemSource::System,
                    score: 0.0,
                    rank: rank_offset,
                    run_count: 0,
                    created_at: state::now_iso8601(),
                },
                title: candidate.text.chars().take(80).collect(),
                summary: candidate.text.chars().take(200).collect(),
                full_text: candidate.text.clone(),
            };
            state::save_problem(&problem)?;
            info!("Admitted candidate problem: {}", id);
        }
    }
    Ok(())
}

async fn promote_top_conjecture(
    client: &LlmClient,
    _config: &Config,
    mind_system: &str,
    conjectures: &[Conjecture],
    run: u32,
) -> Option<String> {
    let top = promoter::find_top_conjecture(conjectures)?;
    let p = prompts::summarize_for_tool(mind_system, &top.text, top.meta.total);
    match client
        .call::<ToolSummaryResponse>(Some(&p.system), &p.user, 0.3)
        .await
    {
        Ok(resp) => {
            let count = state::load_tools(&Layer::Perspectives)
                .map(|t| t.len())
                .unwrap_or(0);
            let new_tool = Tool {
                meta: ToolMeta {
                    id: format!("discovered-{:03}", run),
                    layer: Layer::Perspectives,
                    score: top.meta.total,
                    rank: count as u32 + 1,
                    run_count: 1,
                    problem_coverage: vec![top.meta.problem_id.clone()],
                    created_at: state::now_iso8601(),
                    promoted_from: None,
                    history: vec![],
                },
                title: format!("Discovered: Run {:03}", run),
                summary: resp.summary.clone(),
                full_text: resp.full_text,
            };
            if let Err(e) = state::save_tool(&new_tool) {
                tracing::warn!("Failed to save promoted conjecture: {e}");
                return None;
            }
            info!("Promoted conjecture to perspectives: discovered-{:03}", run);
            Some(resp.summary)
        }
        Err(e) => {
            tracing::warn!("Failed to summarize conjecture for promotion: {e}");
            None
        }
    }
}

fn promote_top_tool(
    perspectives: &[Tool],
    min_run_count: u32,
) -> Result<(Option<String>, Option<String>)> {
    match promoter::find_tool_to_promote(perspectives, min_run_count) {
        Some(tool) => {
            let id = tool.meta.id.clone();
            let name = tool.title.clone();
            let mut promoted = tool.clone();
            let mind_count = state::load_tools(&Layer::Mind)?.len();
            promoted.meta.layer = Layer::Mind;
            promoted.meta.rank = mind_count as u32 + 1;
            state::delete_tool(&id, &Layer::Perspectives)?;
            state::save_tool(&promoted)?;
            info!("Promoted tool to mind: {}", id);
            Ok((Some(name), Some(id)))
        }
        None => Ok((None, None)),
    }
}

fn demote_bottom_mind(mind: &[Tool], min_run_count: u32) -> Result<Option<String>> {
    match promoter::find_mind_tool_to_demote(mind, min_run_count) {
        Some(tool) => {
            let id = tool.meta.id.clone();
            let name = tool.title.clone();
            let mut demoted = tool.clone();
            let persp_count = state::load_tools(&Layer::Perspectives)?.len();
            demoted.meta.layer = Layer::Perspectives;
            demoted.meta.rank = persp_count as u32 + 1;
            state::delete_tool(&id, &Layer::Mind)?;
            state::save_tool(&demoted)?;
            info!("Demoted mind tool to perspectives: {}", id);
            Ok(Some(name))
        }
        None => Ok(None),
    }
}

fn discard_bottom_perspective(
    perspectives: &[Tool],
    min_run_count: u32,
    exclude_id: Option<&str>,
) -> Result<Option<String>> {
    match promoter::find_perspective_to_discard(perspectives, min_run_count, exclude_id) {
        Some(tool) => {
            let id = tool.meta.id.clone();
            let name = tool.title.clone();
            state::delete_tool(&id, &Layer::Perspectives)?;
            info!("Discarded perspective tool: {}", id);
            Ok(Some(name))
        }
        None => Ok(None),
    }
}

async fn generate_report(
    client: &LlmClient,
    run: u32,
    conjectures: &[Conjecture],
    promoted_conjecture: Option<&str>,
    promoted_tool: Option<&str>,
    demoted_mind: Option<&str>,
    discarded: Option<&str>,
) -> Result<()> {
    let mut sorted = conjectures.to_vec();
    sorted.sort_by(|a, b| b.meta.total.partial_cmp(&a.meta.total).unwrap());

    let mut out = format!("# Run {:03} Results\n\n", run);

    out.push_str("| Rank | Problem | Tool | Consistency | Hard to Vary | Total |\n");
    out.push_str("|------|---------|------|-------------|--------------|-------|\n");
    for (rank, c) in sorted.iter().enumerate() {
        out.push_str(&format!(
            "| {} | {} | {} | {:.2} | {:.2} | {:.2} |\n",
            rank + 1,
            c.meta.problem_id,
            c.meta.tool_id,
            c.meta.logical_consistency,
            c.meta.hard_to_vary,
            c.meta.total,
        ));
    }

    out.push_str("\n## Top 5\n\n");
    for (rank, c) in sorted.iter().take(5).enumerate() {
        let p = prompts::summarize_conjecture(&c.text, c.meta.total);
        let summary = client
            .call_raw(Some(&p.system), &p.user, 0.3)
            .await
            .unwrap_or_else(|_| "(summary unavailable)".to_string());
        out.push_str(&format!(
            "**{}. {} × {}** (total: {:.2})  \n{}\n\n",
            rank + 1,
            c.meta.problem_id,
            c.meta.tool_id,
            c.meta.total,
            summary.trim(),
        ));
    }

    out.push_str("## Changes\n\n");
    let any_change =
        promoted_conjecture.is_some() || promoted_tool.is_some() || demoted_mind.is_some() || discarded.is_some();
    if let Some(s) = promoted_conjecture {
        out.push_str(&format!("- Promoted conjecture → perspectives: \"{s}\"\n"));
    }
    if let Some(s) = promoted_tool {
        out.push_str(&format!("- Promoted tool → mind: \"{s}\"\n"));
    }
    if let Some(s) = demoted_mind {
        out.push_str(&format!("- Demoted mind tool → perspectives: \"{s}\"\n"));
    }
    if let Some(s) = discarded {
        out.push_str(&format!("- Discarded perspective tool: \"{s}\"\n"));
    }
    if !any_change {
        out.push_str("- No promotions or demotions this run (insufficient run counts).\n");
    }

    state::save_run_summary(run, &out)?;
    println!("{out}");
    Ok(())
}
