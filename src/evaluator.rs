use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::llm::LlmClient;
use crate::prompts;
use crate::types::{
    AnswersResponse, Candidate, CandidateMeta, CandidatesResponse, ConsistencyResponse, Question,
    QuestionsResponse,
};

pub async fn evaluate(
    client: &LlmClient,
    config: &Config,
    mind_system: &str,
    candidate_text: &str,
    problem_summary: &str,
    problem_id: &str,
    conjecture_id: &str,
    run: u32,
) -> Result<Candidate> {
    // Pass 1: Logical consistency
    let p = prompts::logical_consistency_check(mind_system, candidate_text);
    let consistency: ConsistencyResponse = client.call(Some(&p.system), &p.user, 0.2).await?;

    if consistency.score < config.consistency_threshold {
        info!(
            "  {}-{}: failed consistency ({:.2}), skipping hard-to-vary",
            problem_id, conjecture_id, consistency.score
        );
        return Ok(Candidate {
            meta: CandidateMeta {
                problem_id: problem_id.to_string(),
                conjecture_id: conjecture_id.to_string(),
                run,
                logical_consistency: consistency.score,
                hard_to_vary: 0.0,
                total: 0.0,
                candidate_problems: vec![],
            },
            text: candidate_text.to_string(),
            questions: vec![],
        });
    }

    // Pass 2a: Generate hard-to-vary questions
    let p = prompts::generate_questions(mind_system, candidate_text, problem_summary);
    let questions_resp: QuestionsResponse = client.call(Some(&p.system), &p.user, 0.3).await?;

    // Pass 2b: Answer questions
    let p = prompts::answer_questions(mind_system, candidate_text, &questions_resp.questions);
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

    let total = 0.3 * consistency.score + 0.7 * hard_to_vary;

    // Extract candidate problems
    let p = prompts::extract_candidate_problems(mind_system, candidate_text);
    let candidates_resp: CandidatesResponse = client.call(Some(&p.system), &p.user, 0.3).await?;

    let candidate_problems = candidates_resp
        .candidates
        .into_iter()
        .filter(|c| c.score >= config.problem_admission_threshold)
        .collect();

    info!(
        "  {}-{}: consistency={:.2}, hard_to_vary={:.2}, total={:.2}",
        problem_id, conjecture_id, consistency.score, hard_to_vary, total
    );

    Ok(Candidate {
        meta: CandidateMeta {
            problem_id: problem_id.to_string(),
            conjecture_id: conjecture_id.to_string(),
            run,
            logical_consistency: consistency.score,
            hard_to_vary,
            total,
            candidate_problems,
        },
        text: candidate_text.to_string(),
        questions,
    })
}
