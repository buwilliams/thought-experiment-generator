use std::path::Path;

use anyhow::{Context, Result};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::types::Quad;

/// Load three vocabulary files from the data directory and combine them
/// into universal quads by randomly pairing objects, relationships, and properties.
pub fn load_universal_vocabulary(data_dir: &Path) -> Result<Vec<Quad>> {
    let objects_path = data_dir.join("vocab_objects.txt");
    let relationships_path = data_dir.join("vocab_relationships.txt");
    let properties_path = data_dir.join("vocab_properties.txt");

    let objects = load_lines(&objects_path)?;
    let relationships = load_lines(&relationships_path)?;
    let properties = load_lines(&properties_path)?;

    tracing::info!(
        "Loaded vocabulary: {} objects, {} relationships, {} properties",
        objects.len(),
        relationships.len(),
        properties.len()
    );

    let quads = build_universal_quads(&objects, &relationships, &properties);

    tracing::info!("Built {} universal quads", quads.len());
    Ok(quads)
}

/// Build universal quads by randomly combining objects, relationships, and properties.
/// Each object gets paired with a random other object, relationship, and property.
fn build_universal_quads(
    objects: &[String],
    relationships: &[String],
    properties: &[String],
) -> Vec<Quad> {
    let mut rng = StdRng::seed_from_u64(42); // deterministic for reproducibility
    let mut quads = Vec::with_capacity(objects.len());

    for obj in objects {
        let other = &objects[rng.r#gen::<usize>() % objects.len()];
        let rel = &relationships[rng.r#gen::<usize>() % relationships.len()];
        let prop = &properties[rng.r#gen::<usize>() % properties.len()];

        quads.push(Quad::new_universal_rich(
            obj.clone(),
            rel.clone(),
            other.clone(),
            prop.clone(),
        ));
    }

    quads
}

fn load_lines(path: &Path) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read vocabulary file: {}", path.display()))?;

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    if lines.is_empty() {
        anyhow::bail!("Vocabulary file is empty: {}", path.display());
    }

    Ok(lines)
}

/// Generate three vocabulary files using an LLM.
pub async fn generate_vocabulary(client: &crate::llm::LlmClient) -> Result<()> {
    std::fs::create_dir_all("data")?;

    // Generate objects
    let prompt = r#"Generate a list of 1000 diverse nouns and noun phrases spanning:
- Scientific concepts (photon, enzyme, black hole, prime number)
- Everyday objects (bicycle, kitchen table, umbrella, clock)
- Abstract concepts (justice, entropy, symmetry, nostalgia)
- People and roles (observer, child, merchant, scientist)
- Places and systems (marketplace, ecosystem, border, network)

Output one term per line. No numbering. No categories. Just terms."#;

    tracing::info!("Generating objects vocabulary...");
    let raw = client.call_raw(prompt, 1.0).await?;
    let objects: Vec<&str> = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    std::fs::write("data/vocab_objects.txt", objects.join("\n"))?;
    tracing::info!("Generated {} objects", objects.len());

    // Generate relationships
    let prompt = r#"Generate a list of 300 diverse relationships, interactions, and verbs that connect two things:
- Physical (collides with, contains, orbits, dissolves)
- Causal (causes, prevents, enables, accelerates)
- Structural (is part of, depends on, mirrors, opposes)
- Transformative (becomes, erodes, amplifies, distorts)
- Social (teaches, deceives, competes with, inherits from)
- Temporal (precedes, outlasts, interrupts, triggers)

Output one relationship per line. No numbering. No categories. Just relationships."#;

    tracing::info!("Generating relationships vocabulary...");
    let raw = client.call_raw(prompt, 1.0).await?;
    let relationships: Vec<&str> = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    std::fs::write("data/vocab_relationships.txt", relationships.join("\n"))?;
    tracing::info!("Generated {} relationships", relationships.len());

    // Generate properties
    let prompt = r#"Generate a list of 200 diverse properties and qualities that can be put under pressure:
- Physical (rigidity, temperature, density, velocity)
- Abstract (fairness, complexity, coherence, ambiguity)
- Behavioral (resilience, fragility, adaptability, inertia)
- Epistemic (certainty, falsifiability, predictability, transparency)
- Relational (trust, dependency, autonomy, reciprocity)

Output one property per line. No numbering. No categories. Just properties."#;

    tracing::info!("Generating properties vocabulary...");
    let raw = client.call_raw(prompt, 1.0).await?;
    let properties: Vec<&str> = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    std::fs::write("data/vocab_properties.txt", properties.join("\n"))?;
    tracing::info!("Generated {} properties", properties.len());

    Ok(())
}
