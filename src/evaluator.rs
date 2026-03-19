use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::prompts::PromptTemplates;
use crate::types::{ComparativeEvalResponse, Generated, GeneratedMeta};

// Weights must sum to 1.0
const W_CONSISTENCY: f64 = 0.20;
const W_HARD_TO_VARY: f64 = 0.35;
const W_REACH: f64 = 0.30;
const W_REFUTATION: f64 = 0.15;

/// Evaluate all outputs for one lens in a single comparative call.
/// problem_outputs: Vec<(problem_id, problem_summary, output_text)>
/// Returns one Generated per output, with scores set to zero for consistency failures.
pub async fn evaluate_comparative(
    client: &LlmClient,
    config: &Config,
    templates: &PromptTemplates,
    mind_system: &str,
    lens_id: &str,
    lens_summary: &str,
    problem_outputs: &[(String, String, String)],
    run: u32,
) -> Result<Vec<Generated>> {
    let p = templates.comparative_evaluate(mind_system, lens_summary, problem_outputs);
    let resp: ComparativeEvalResponse = client.call(Some(&p.system), &p.user, 0.3).await?;

    let mut results = vec![];

    for entry in resp.evaluations {
        let text = problem_outputs
            .iter()
            .find(|(id, _, _)| id == &entry.problem_id)
            .map(|(_, _, t)| t.clone())
            .unwrap_or_default();

        if entry.consistency < config.consistency_threshold {
            info!(
                "  {}-{}: failed consistency ({:.2}), zeroing scores",
                entry.problem_id, lens_id, entry.consistency
            );
            results.push(Generated {
                meta: GeneratedMeta {
                    problem_id: entry.problem_id,
                    conjecture_id: lens_id.to_string(),
                    run,
                    logical_consistency: entry.consistency,
                    hard_to_vary: 0.0,
                    explanatory_reach: 0.0,
                    resistance_to_refutation: 0.0,
                    total: 0.0,
                    candidate_problems: vec![],
                },
                text,
                questions: vec![],
            });
            continue;
        }

        let total = W_CONSISTENCY * entry.consistency
            + W_HARD_TO_VARY * entry.hard_to_vary
            + W_REACH * entry.explanatory_reach
            + W_REFUTATION * entry.resistance_to_refutation;

        let candidate_problems = entry
            .candidate_problems
            .into_iter()
            .filter(|cp| cp.score >= config.problem_admission_threshold)
            .collect();

        info!(
            "  {}-{}: consistency={:.2}, htv={:.2}, reach={:.2}, refutation={:.2}, total={:.2}",
            entry.problem_id, lens_id,
            entry.consistency, entry.hard_to_vary,
            entry.explanatory_reach, entry.resistance_to_refutation, total
        );

        results.push(Generated {
            meta: GeneratedMeta {
                problem_id: entry.problem_id,
                conjecture_id: lens_id.to_string(),
                run,
                logical_consistency: entry.consistency,
                hard_to_vary: entry.hard_to_vary,
                explanatory_reach: entry.explanatory_reach,
                resistance_to_refutation: entry.resistance_to_refutation,
                total,
                candidate_problems,
            },
            text,
            questions: vec![],
        });
    }

    Ok(results)
}
