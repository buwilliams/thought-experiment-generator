use std::path::Path;

use anyhow::{Context, Result};

use crate::types::Quad;

/// Load universal vocabulary from a text file (one term per line)
/// and convert each term into a universal quad.
pub fn load_universal_vocabulary(path: &Path) -> Result<Vec<Quad>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read vocabulary file: {}", path.display()))?;

    let quads: Vec<Quad> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|term| Quad::new_universal(term.to_string()))
        .collect();

    if quads.is_empty() {
        anyhow::bail!("Vocabulary file is empty: {}", path.display());
    }

    tracing::info!("Loaded {} universal vocabulary terms", quads.len());
    Ok(quads)
}

/// Generate a starter vocabulary file using an LLM.
pub async fn generate_vocabulary(client: &crate::llm::LlmClient) -> Result<Vec<String>> {
    let prompt = r#"Generate a list of 1000 diverse terms spanning these categories:
- Scientific concepts (physics, biology, chemistry, math)
- Everyday objects and actions
- Abstract concepts (justice, time, entropy, symmetry)
- Relationships and processes (causes, prevents, transforms, contains)
- Properties and qualities (rigid, volatile, recursive, emergent)

Output one term per line. No numbering. No categories. Just terms."#;

    let raw = client.call_raw(prompt, 1.0).await?;
    let terms: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    Ok(terms)
}
