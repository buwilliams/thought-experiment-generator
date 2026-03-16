use anyhow::Result;

use crate::llm::LlmClient;
use crate::llm::prompts::{self, DeutschScorerParams};
use crate::types::{DeutschScore, Node};

/// Score a thought experiment using Deutsch criteria.
pub async fn score_deutsch(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
    accumulated_path: &[Node],
) -> Result<DeutschScore> {
    let params = DeutschScorerParams {
        topic,
        thought_experiment,
        accumulated_path,
    };
    let prompt = prompts::deutsch_scorer(&params);
    client.call(&prompt, 0.1).await
}
