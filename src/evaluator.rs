use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::prompts::PromptTemplates;
use crate::types::{
    AnswersResponse, CandidatesResponse, ConsistencyResponse, ExplanatoryReachResponse,
    Generated, GeneratedMeta, Question, QuestionsResponse, ResistanceToRefutationResponse,
};

// Weights must sum to 1.0
const W_CONSISTENCY: f64 = 0.20;
const W_HARD_TO_VARY: f64 = 0.35;
const W_REACH: f64 = 0.30;
const W_REFUTATION: f64 = 0.15;

pub async fn evaluate(
    client: &LlmClient,
    config: &Config,
    templates: &PromptTemplates,
    mind_system: &str,
    generated_text: &str,
    problem_summary: &str,
    problem_id: &str,
    conjecture_id: &str,
    run: u32,
) -> Result<Generated> {
    // Pass 1: Logical consistency
    let p = templates.logical_consistency_check(mind_system, generated_text);
    let consistency: ConsistencyResponse = client.call(Some(&p.system), &p.user, 0.2).await?;

    if consistency.score < config.consistency_threshold {
        info!(
            "  {}-{}: failed consistency ({:.2}), skipping further evaluation",
            problem_id, conjecture_id, consistency.score
        );
        return Ok(Generated {
            meta: GeneratedMeta {
                problem_id: problem_id.to_string(),
                conjecture_id: conjecture_id.to_string(),
                run,
                logical_consistency: consistency.score,
                hard_to_vary: 0.0,
                explanatory_reach: 0.0,
                resistance_to_refutation: 0.0,
                total: 0.0,
                candidate_problems: vec![],
            },
            text: generated_text.to_string(),
            questions: vec![],
        });
    }

    // Pass 2a: Generate hard-to-vary questions
    let p = templates.generate_questions(mind_system, generated_text, problem_summary);
    let questions_resp: QuestionsResponse = client.call(Some(&p.system), &p.user, 0.3).await?;

    // Pass 2b: Answer questions
    let p = templates.answer_questions(mind_system, generated_text, &questions_resp.questions);
    let answers_resp: AnswersResponse = client.call(Some(&p.system), &p.user, 0.2).await?;

    let questions: Vec<Question> = answers_resp
        .answers
        .into_iter()
        .map(|a| Question { question: a.question, answer: a.answer })
        .collect();

    let yes_count = questions.iter().filter(|q| q.answer).count() as f64;
    let hard_to_vary = if questions.is_empty() {
        0.0
    } else {
        yes_count / questions.len() as f64
    };

    // Pass 3: Explanatory reach
    let p = templates.explanatory_reach(mind_system, generated_text, problem_summary);
    let reach_resp: ExplanatoryReachResponse =
        client.call(Some(&p.system), &p.user, 0.2).await?;

    // Pass 4: Resistance to refutation
    let p = templates.resistance_to_refutation(mind_system, generated_text, problem_summary);
    let refutation_resp: ResistanceToRefutationResponse =
        client.call(Some(&p.system), &p.user, 0.3).await?;

    let total = W_CONSISTENCY * consistency.score
        + W_HARD_TO_VARY * hard_to_vary
        + W_REACH * reach_resp.score
        + W_REFUTATION * refutation_resp.score;

    // Extract candidate problems
    let p = templates.extract_candidate_problems(mind_system, generated_text);
    let candidates_resp: CandidatesResponse = client.call(Some(&p.system), &p.user, 0.3).await?;

    let candidate_problems = candidates_resp
        .candidates
        .into_iter()
        .filter(|c| c.score >= config.problem_admission_threshold)
        .collect();

    info!(
        "  {}-{}: consistency={:.2}, htv={:.2}, reach={:.2}, refutation={:.2}, total={:.2}",
        problem_id, conjecture_id,
        consistency.score, hard_to_vary, reach_resp.score, refutation_resp.score, total
    );

    Ok(Generated {
        meta: GeneratedMeta {
            problem_id: problem_id.to_string(),
            conjecture_id: conjecture_id.to_string(),
            run,
            logical_consistency: consistency.score,
            hard_to_vary,
            explanatory_reach: reach_resp.score,
            resistance_to_refutation: refutation_resp.score,
            total,
            candidate_problems,
        },
        text: generated_text.to_string(),
        questions,
    })
}
