use std::collections::HashSet;

use anyhow::Result;
use serde::Deserialize;

use crate::llm::LlmClient;
use crate::llm::prompts;
use crate::types::Quad;

#[derive(Debug, Deserialize)]
struct RawQuad {
    object_a: String,
    relationship: String,
    object_b: String,
    property: String,
}

/// Extract quads from a block of text using the LLM.
pub async fn extract_quads(client: &LlmClient, text: &str) -> Result<Vec<Quad>> {
    let prompt = prompts::quad_extraction(text);
    let raw_quads: Vec<RawQuad> = client.call(&prompt, 0.1).await?;

    let mut seen = HashSet::new();
    let quads = raw_quads
        .into_iter()
        .filter(|rq| {
            let key = (
                rq.object_a.to_lowercase(),
                rq.relationship.to_lowercase(),
                rq.object_b.to_lowercase(),
            );
            seen.insert(key)
        })
        .map(|rq| {
            Quad::new_background(rq.object_a, rq.relationship, rq.object_b, rq.property)
        })
        .collect();

    Ok(quads)
}

/// Extract novel quads from a thought experiment text.
/// These are added to the novel pool with provenance.
pub async fn extract_novel_quads(
    client: &LlmClient,
    text: &str,
    provenance: uuid::Uuid,
) -> Result<Vec<Quad>> {
    let prompt = prompts::quad_extraction(text);
    let raw_quads: Vec<RawQuad> = client.call(&prompt, 0.1).await?;

    let quads = raw_quads
        .into_iter()
        .map(|rq| Quad::new_novel(rq.object_a, rq.relationship, rq.object_b, rq.property, provenance))
        .collect();

    Ok(quads)
}
