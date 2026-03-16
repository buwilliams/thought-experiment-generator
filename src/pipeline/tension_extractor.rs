use anyhow::Result;

use crate::llm::LlmClient;
use crate::llm::prompts::{self, TensionExtractionParams};
use crate::types::{Node, UnresolvedTension};

/// Extract the unresolved tension from a thought experiment.
pub async fn extract_tension(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
    accumulated_path: &[Node],
    score: f64,
) -> Result<UnresolvedTension> {
    let params = TensionExtractionParams {
        topic,
        thought_experiment,
        accumulated_path,
        score,
    };
    let prompt = prompts::tension_extraction(&params);
    client.call(&prompt, 0.1).await
}
