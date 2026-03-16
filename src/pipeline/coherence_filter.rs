use anyhow::Result;

use crate::llm::LlmClient;
use crate::llm::prompts;
use crate::types::CoherenceResult;

/// Check coherence of a thought experiment. Returns pass/fail with reasons.
pub async fn check_coherence(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
) -> Result<CoherenceResult> {
    let prompt = prompts::coherence_filter(topic, thought_experiment);
    client.call(&prompt, 0.1).await
}
