use anyhow::Result;
use tracing::info;

use crate::llm::LlmClient;
use crate::llm::prompts;
use crate::pipeline::quad_extractor;
use crate::types::Quad;

pub struct BackgroundInitResult {
    pub quads: Vec<Quad>,
    pub resources: Vec<String>,
}

/// Initialize the background pool by generating facts about the topic
/// and extracting quads from them.
pub async fn initialize_background_pool(
    client: &LlmClient,
    topic: &str,
) -> Result<BackgroundInitResult> {
    info!("Generating background knowledge for topic: {topic}");

    // Step 1: Generate facts (high temperature, nondeterministic)
    let prompt = prompts::fact_generation(topic);
    let raw_response = client.call_raw(&prompt, 1.0).await?;

    // Parse out facts and resources sections
    let (facts_text, resources) = parse_facts_response(&raw_response);

    info!(
        "Generated {} chars of facts, {} resources",
        facts_text.len(),
        resources.len()
    );

    // Step 2: Extract quads from facts (low temperature, deterministic)
    let quads = quad_extractor::extract_quads(client, &facts_text).await?;

    info!("Extracted {} background quads", quads.len());

    Ok(BackgroundInitResult { quads, resources })
}

fn parse_facts_response(response: &str) -> (String, Vec<String>) {
    let mut facts = String::new();
    let mut resources = Vec::new();
    let mut in_resources = false;

    for line in response.lines() {
        let trimmed = line.trim();
        if trimmed.to_uppercase().starts_with("RESOURCES:") {
            in_resources = true;
            continue;
        }
        if in_resources {
            if !trimmed.is_empty() {
                // Strip leading number and period
                let resource = trimmed
                    .trim_start_matches(|c: char| c.is_ascii_digit() || c == '.')
                    .trim()
                    .to_string();
                if !resource.is_empty() {
                    resources.push(resource);
                }
            }
        } else {
            facts.push_str(line);
            facts.push('\n');
        }
    }

    (facts, resources)
}
