use crate::types::Node;

/// Prompt 1 — Fact Generation (Background Pool Initialization)
pub fn fact_generation(topic: &str) -> String {
    format!(
        r#"You are a knowledgeable research assistant with broad expertise.

Topic: {topic}

Generate the 50 most relevant facts, principles, known results,
and contested claims about this topic. Be specific. Include
edge cases, surprising findings, and unresolved questions.

Also list the 5 best external resources where deeper knowledge
can be found (journals, papers, books, databases).

Format your response as:
FACTS:
1. [fact]
2. [fact]
...

RESOURCES:
1. [resource]
..."#
    )
}

/// Prompt 2 — Quad Extraction
pub fn quad_extraction(facts_text: &str) -> String {
    format!(
        r#"Extract all objects, relationships, and properties from the
following text.

An object is a person, place, thing, or idea.
A relationship is any interaction or structural bond between
two objects — any direction, active or static.
A property is a quality put under pressure by the relationship
between two objects.

Return JSON only. No preamble. No markdown.

Format:
[
  {{
    "object_a": "string",
    "relationship": "string",
    "object_b": "string",
    "property": "string"
  }}
]

Text: {facts_text}"#
    )
}

pub struct TeGenerationParams<'a> {
    pub topic: &'a str,
    pub objects: &'a [String],
    pub relationships: &'a [String],
    pub properties: &'a [String],
    pub accumulated_path: &'a [Node],
    pub unresolved_tension: Option<&'a str>,
}

/// Prompt 3 — Thought Experiment Generation
pub fn te_generation(params: &TeGenerationParams) -> String {
    let objects_str = params.objects.join(", ");
    let rels_str = params.relationships.join(", ");
    let props_str = params.properties.join(", ");
    let path_str = format_path(params.accumulated_path);
    let tension_str = params.unresolved_tension.unwrap_or("None");

    format!(
        r#"You are a creative scientific thinker generating a thought experiment.

Topic: {topic}

Using the following elements, generate a single hypothetical
thought experiment. Place an observer or object in an unusual
relationship with the other elements and ask what would happen
or what would be revealed.

Be specific. Be imaginative. Do not explain known results —
generate a scenario that forces a question.

Elements ({n_obj} objects, {n_rel} relationships, {n_prop} properties):
Objects: {objects_str}
Relationships: {rels_str}
Properties: {props_str}

Prior path context (if any): {path_str}
Unresolved tension (if any): {tension_str}

Generate the thought experiment only. No preamble."#,
        topic = params.topic,
        n_obj = params.objects.len(),
        n_rel = params.relationships.len(),
        n_prop = params.properties.len(),
    )
}

/// Prompt 4 — Coherence Filter
pub fn coherence_filter(topic: &str, thought_experiment: &str) -> String {
    format!(
        r#"Evaluate the following thought experiment against two criteria.

Topic: {topic}
Thought experiment: {thought_experiment}

Answer each question with YES or NO and one sentence explanation.

1. Is this thought experiment internally consistent —
   does it make sense on its own terms?

2. Does this thought experiment illuminate anything
   non-trivial about the topic?

Return JSON only:
{{
  "internally_consistent": true or false,
  "consistent_reason": "string",
  "topic_relevant": true or false,
  "relevant_reason": "string",
  "passes": true or false
}}"#
    )
}

pub struct DeutschScorerParams<'a> {
    pub topic: &'a str,
    pub thought_experiment: &'a str,
    pub accumulated_path: &'a [Node],
}

/// Prompt 5 — Deutsch Scorer
pub fn deutsch_scorer(params: &DeutschScorerParams) -> String {
    let path_str = format_path(params.accumulated_path);

    format!(
        r#"Score the following thought experiment on its explanatory quality.

Topic: {topic}
Thought experiment: {te}
Prior path: {path_str}

Evaluate on these criteria:
- Hard to vary: would changing any element break the insight?
- Reach: does it suggest something beyond its immediate inputs?
- Minimal assumptions: does it avoid unnecessary complexity?
- Tension resolution: does it address something unexplained?

Return JSON only:
{{
  "hard_to_vary": float (0-1),
  "reach": float (0-1),
  "minimal_assumptions": float (0-1),
  "tension_resolution": float (0-1),
  "overall_score": float (0-1),
  "justification": "string"
}}"#,
        topic = params.topic,
        te = params.thought_experiment,
    )
}

pub struct TensionExtractionParams<'a> {
    pub topic: &'a str,
    pub thought_experiment: &'a str,
    pub accumulated_path: &'a [Node],
    pub score: f64,
}

/// Prompt 6 — Tension Extraction
pub fn tension_extraction(params: &TensionExtractionParams) -> String {
    let path_str = format_path(params.accumulated_path);

    format!(
        r#"Read the following thought experiment in context of the
accumulated path.

Topic: {topic}
Accumulated path: {path_str}
Current thought experiment: {te}
Current score: {score}

Identify what remains unexplained, paradoxical, or unresolved
after this thought experiment. This is the tension the next
draw should attempt to discharge.

Be specific. Name the objects and relationships involved.

Return JSON only:
{{
  "tension": "string",
  "objects_involved": ["string"],
  "relationships_involved": ["string"],
  "tension_type": "paradox | gap | contradiction | open_question"
}}"#,
        topic = params.topic,
        te = params.thought_experiment,
        score = params.score,
    )
}

/// Prompt 7 — Trajectory Scorer
pub fn trajectory_scorer(topic: &str, nodes: &[Node]) -> String {
    let node_sequence = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| format!("Node {} (depth {}, score {:.2}):\n{}", n.id, i + 1, n.deutsch_score.overall_score, n.thought_experiment))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    format!(
        r#"Read the following sequence of thought experiments generated
while exploring a topic.

Topic: {topic}
Full branch path (root to current):
{node_sequence}

Score the cumulative explanatory reach of this path as a whole.
Consider: how much closer to understanding the topic is this
path than when it started? Has understanding deepened
progressively or stalled?

Return JSON only:
{{
  "trajectory_score": float (0-1),
  "depth_of_insight": "string",
  "best_node": "node_id (uuid string)",
  "justification": "string"
}}"#
    )
}

/// Prompt 8 — Cross-Pollination Candidate Detection
pub fn cross_pollination_check(topic: &str, tension_a: &str, tension_b: &str) -> String {
    format!(
        r#"Read the unresolved tensions from two branches at the same depth.

Topic: {topic}
Branch A tension: {tension_a}
Branch B tension: {tension_b}

Are these tensions complementary — would combining them into
a single thought experiment likely produce insight that neither
branch could reach alone?

Return JSON only:
{{
  "complementary": true or false,
  "reason": "string",
  "suggested_merge_angle": "string or null"
}}"#
    )
}

fn format_path(nodes: &[Node]) -> String {
    if nodes.is_empty() {
        return "None (this is the first node)".to_string();
    }
    nodes
        .iter()
        .map(|n| {
            format!(
                "[depth {}, score {:.2}] {}",
                n.depth, n.deutsch_score.overall_score, n.thought_experiment
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
