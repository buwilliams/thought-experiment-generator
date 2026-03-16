use anyhow::Result;

use crate::llm::LlmClient;
use crate::llm::prompts::{self, TeGenerationParams};
use crate::types::{DrawResult, Node, UnresolvedTension};

/// Generate a thought experiment from a draw result.
pub async fn generate_thought_experiment(
    client: &LlmClient,
    topic: &str,
    draw: &DrawResult,
    accumulated_path: &[Node],
    unresolved_tension: Option<&UnresolvedTension>,
    temperature: f64,
) -> Result<String> {
    let tension_text = unresolved_tension.map(|t| t.tension.as_str());

    let params = TeGenerationParams {
        topic,
        objects: &draw.objects,
        relationships: &draw.relationships,
        properties: &draw.properties,
        accumulated_path,
        unresolved_tension: tension_text,
    };

    let prompt = prompts::te_generation(&params);
    let te = client.call_raw(&prompt, temperature).await?;
    Ok(te.trim().to_string())
}

/// Cheap grammar pre-filter — no LLM call.
/// Returns true if the thought experiment passes basic checks.
pub fn grammar_check(te: &str) -> bool {
    // Must be at least 50 characters
    if te.len() < 50 {
        return false;
    }

    // Must contain at least one question mark or conditional word
    let has_question = te.contains('?');
    let has_conditional = ["what if", "suppose", "imagine", "consider", "would"]
        .iter()
        .any(|w| te.to_lowercase().contains(w));

    if !has_question && !has_conditional {
        return false;
    }

    // Reject if all words are the same (gibberish detector)
    let words: Vec<&str> = te.split_whitespace().collect();
    if words.len() < 10 {
        return false;
    }
    let unique: std::collections::HashSet<&str> = words.iter().copied().collect();
    if unique.len() < words.len() / 3 {
        return false;
    }

    true
}
