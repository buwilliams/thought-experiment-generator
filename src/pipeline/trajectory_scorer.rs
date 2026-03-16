use anyhow::Result;

use crate::llm::LlmClient;
use crate::llm::prompts;
use crate::types::{Branch, TrajectoryScore};

/// Score the full trajectory of a branch.
pub async fn score_trajectory(
    client: &LlmClient,
    topic: &str,
    branch: &Branch,
) -> Result<TrajectoryScore> {
    let prompt = prompts::trajectory_scorer(topic, &branch.nodes);
    client.call(&prompt, 0.1).await
}
