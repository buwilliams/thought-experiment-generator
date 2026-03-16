use anyhow::Result;
use serde::Deserialize;

use crate::llm::LlmClient;
use crate::llm::prompts;
use crate::types::Branch;

#[derive(Debug, Clone, Deserialize)]
pub struct CrossPollinationResult {
    pub complementary: bool,
    pub reason: String,
    pub suggested_merge_angle: Option<String>,
}

/// Check if two branches have complementary tensions that could be merged.
pub async fn check_cross_pollination(
    client: &LlmClient,
    topic: &str,
    branch_a: &Branch,
    branch_b: &Branch,
) -> Result<Option<CrossPollinationResult>> {
    let tension_a = match branch_a.latest_tension() {
        Some(t) => &t.tension,
        None => return Ok(None),
    };
    let tension_b = match branch_b.latest_tension() {
        Some(t) => &t.tension,
        None => return Ok(None),
    };

    let prompt = prompts::cross_pollination_check(topic, tension_a, tension_b);
    let result: CrossPollinationResult = client.call(&prompt, 0.1).await?;

    if result.complementary {
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
