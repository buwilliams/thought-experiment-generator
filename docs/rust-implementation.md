# Rust Implementation Plan

## Thought Experiment Generator

This document translates the [design doc](./thought-experiment-generator-design-doc.md) into a concrete Rust implementation. It follows the 13-step build order from the design doc, maps each component to Rust crates and modules, and specifies the interfaces between them.

---

## Toolchain & Dependencies

| Purpose | Crate | Why |
|---|---|---|
| Async runtime | `tokio` | Parallel branch execution, concurrent LLM calls |
| HTTP client | `reqwest` | LLM API calls (OpenAI-compatible / Anthropic) |
| Serialization | `serde`, `serde_json` | All data structures, LLM prompt/response parsing |
| UUID generation | `uuid` | Quad, node, branch, tree IDs |
| RNG | `rand` | Pool draws, shuffle |
| CLI | `clap` | Argument parsing matching the design doc's CLI params |
| Logging | `tracing`, `tracing-subscriber` | Structured logging for tree progress |
| Error handling | `anyhow`, `thiserror` | Application vs. domain errors |
| Rate limiting | `governor` | LLM API rate limiting |

**Rust edition:** 2024
**MSRV:** 1.85+

---

## Project Layout

```
src/
  main.rs                  # CLI entry point (step 13)
  lib.rs                   # Re-exports, top-level types
  config.rs                # CLI params → Config struct
  types/
    mod.rs
    quad.rs                # Quad struct
    node.rs                # Node struct
    branch.rs              # Branch struct
    tree.rs                # Tree struct
    draw_pool.rs           # DrawPool struct + ratio logic
    scores.rs              # DeutschScore, CoherenceResult, TrajectoryScore
    tension.rs             # UnresolvedTension struct
  llm/
    mod.rs
    client.rs              # LLM API client (Anthropic / OpenAI-compatible)
    prompts.rs             # Prompt templates (Prompts 1-8)
    parser.rs              # JSON response parsing with fallback
  vocab/
    mod.rs                 # Universal vocabulary loader (step 1)
  pipeline/
    mod.rs
    quad_extractor.rs      # Step 2: text → quads
    te_generator.rs        # Step 3: draw → thought experiment
    coherence_filter.rs    # Step 4: coherence check
    deutsch_scorer.rs      # Step 5: Deutsch score
    tension_extractor.rs   # Step 6: tension extraction
    trajectory_scorer.rs   # Step 10: trajectory scoring
    cross_pollination.rs   # Step 11: cross-pollination detection
  engine/
    mod.rs
    branch_runner.rs       # Step 7: single branch loop
    background_init.rs     # Step 8: background pool initialization
    root_generator.rs      # Step 9: root branch generation
    tree_runner.rs          # Step 12: parallel tree orchestration
data/
  universal_vocabulary.txt # 50k terms, one per line (step 1)
```

---

## Data Structures

### `Quad`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadSource {
    Background,
    Universal,
    Novel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quad {
    pub id: Uuid,
    pub object_a: String,
    pub relationship: String,
    pub object_b: String,
    pub property: String,
    pub source: QuadSource,
    /// For novel quads: the node ID that generated them.
    pub provenance: Option<Uuid>,
}
```

### `Node`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub depth: u32,
    pub branch_id: Uuid,
    pub thought_experiment: String,
    pub quads_used: Vec<Uuid>,
    pub deutsch_score: DeutschScore,
    pub unresolved_tension: Option<UnresolvedTension>,
    pub score_history: Vec<f64>,
    pub accumulated_path: Vec<Uuid>,
}
```

### `Branch`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchStatus {
    Active,
    Terminated,
    CrossPollinated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub parent_branch_ids: Vec<Uuid>,
    pub topic: String,
    pub nodes: Vec<Node>,
    pub current_depth: u32,
    pub depth_limit: u32,
    pub status: BranchStatus,
    pub trajectory_score: Option<f64>,
}
```

### `DrawPool`

```rust
#[derive(Debug, Clone)]
pub struct PoolRatio {
    pub background: f64,
    pub universal: f64,
    pub novel: f64,
}

#[derive(Debug, Clone)]
pub struct DrawPool {
    pub background: Vec<Quad>,
    pub universal: Vec<Quad>,
    pub novel: Vec<Quad>,
    pub ratio: PoolRatio,
}

impl DrawPool {
    /// Recalculate ratio based on novel pool size relative to total quads.
    /// Starts at 50/50/0, shifts toward 30/30/40 as novel pool grows.
    pub fn update_ratio(&mut self) { .. }

    /// Sample a 4/3/2 atom (or custom atom size) from the three pools
    /// according to current ratio. Returns (objects, relationships, properties).
    pub fn draw(&self, config: &AtomConfig, rng: &mut impl Rng) -> DrawResult { .. }
}
```

### `DeutschScore`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeutschScore {
    pub hard_to_vary: f64,
    pub reach: f64,
    pub minimal_assumptions: f64,
    pub tension_resolution: f64,
    pub overall_score: f64,
    pub justification: String,
}
```

### `UnresolvedTension`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensionType {
    Paradox,
    Gap,
    Contradiction,
    OpenQuestion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedTension {
    pub tension: String,
    pub objects_involved: Vec<String>,
    pub relationships_involved: Vec<String>,
    pub tension_type: TensionType,
}
```

### `Tree`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedTopic {
    pub core_subject: String,
    pub core_tension: String,
    pub question_type: QuestionType, // Causal, Counterfactual, Mechanistic, Other
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPollinationRecord {
    pub branch_a: Uuid,
    pub branch_b: Uuid,
    pub new_branch: Uuid,
    pub depth_at_merge: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub id: Uuid,
    pub topic: String,
    pub topic_normalized: NormalizedTopic,
    pub draw_pool: DrawPool,
    pub branches: Vec<Branch>,
    pub cross_pollinations: Vec<CrossPollinationRecord>,
    pub top_trajectories: Vec<Uuid>,
}
```

### `Config`

```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub depth_limit: u32,          // --depth, default 10
    pub num_branches: u32,         // --branches, default 10
    pub survivor_threshold: f64,   // --threshold, default 0.7
    pub novel_threshold: f64,      // --novel-threshold, default 0.85
    pub bg_universal_ratio: f64,   // --ratio, default 0.5
    pub draws_per_depth: u32,      // --draws, default 100
    pub temperature: f64,          // --temperature, default 1.2
    pub atom: AtomConfig,
    pub llm: LlmConfig,
}

#[derive(Debug, Clone)]
pub struct AtomConfig {
    pub objects: u32,              // default 4
    pub relationships: u32,        // default 3
    pub properties: u32,           // default 2
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: LlmProvider,     // Anthropic or OpenAI-compatible
    pub api_key: String,           // from env var
    pub model: String,             // e.g. "claude-sonnet-4-6"
    pub max_concurrent: usize,     // max parallel LLM calls
    pub low_temperature: f64,      // for deterministic prompts (0.1)
    pub high_temperature: f64,     // for generative prompts (1.2)
}
```

---

## LLM Client

### `llm/client.rs`

```rust
pub struct LlmClient {
    http: reqwest::Client,
    config: LlmConfig,
    rate_limiter: governor::RateLimiter<..>,
}

impl LlmClient {
    /// Send a prompt and parse a JSON response of type T.
    /// Retries on transient failures (429, 500, 503) with exponential backoff.
    /// Uses low or high temperature based on the prompt type.
    pub async fn call<T: DeserializeOwned>(
        &self,
        prompt: &str,
        temperature: f64,
    ) -> Result<T>;

    /// Raw string response for non-JSON prompts (fact generation).
    pub async fn call_raw(
        &self,
        prompt: &str,
        temperature: f64,
    ) -> Result<String>;
}
```

### `llm/prompts.rs`

Each prompt template from the design doc becomes a function that takes typed parameters and returns a formatted string:

```rust
pub fn fact_generation(topic: &str) -> String { .. }
pub fn quad_extraction(facts_text: &str) -> String { .. }
pub fn te_generation(params: &TeGenerationParams) -> String { .. }
pub fn coherence_filter(topic: &str, thought_experiment: &str) -> String { .. }
pub fn deutsch_scorer(params: &DeutschScorerParams) -> String { .. }
pub fn tension_extraction(params: &TensionExtractionParams) -> String { .. }
pub fn trajectory_scorer(topic: &str, node_sequence: &str) -> String { .. }
pub fn cross_pollination_check(topic: &str, tension_a: &str, tension_b: &str) -> String { .. }
```

### `llm/parser.rs`

LLM JSON responses are unreliable. The parser:

1. Strips markdown fences (```json ... ```) if present
2. Attempts `serde_json::from_str::<T>`
3. On failure: extracts the first `{...}` or `[...]` block and retries
4. On second failure: returns a structured error with the raw response for logging

```rust
pub fn parse_llm_json<T: DeserializeOwned>(raw: &str) -> Result<T>;
```

---

## Build Steps (1-13)

### Step 1 — Universal Vocabulary

**Module:** `vocab/mod.rs`

Ship a `data/universal_vocabulary.txt` file with ~50k terms (one per line). At startup, load into `Vec<String>` and convert to universal quads by generating identity quads:

```rust
/// Load vocabulary and generate universal quads.
/// Each term produces a quad: (term, "is", term, "identity")
/// These serve as raw material — the LLM generation step
/// gives them meaning through collision with other quads.
pub fn load_universal_vocabulary(path: &Path) -> Result<Vec<Quad>>;
```

The vocabulary file is curated once and shipped with the binary (or downloaded on first run). It should span: scientific terms, everyday objects, abstract concepts, actions, and properties.

**Generating the vocabulary:** Use an LLM call at build time or as a one-off script (`scripts/generate_vocab.rs`) to produce the initial 50k-term list across domains. Store as a flat text file.

---

### Step 2 — Quad Extractor

**Module:** `pipeline/quad_extractor.rs`

```rust
pub async fn extract_quads(
    client: &LlmClient,
    text: &str,
) -> Result<Vec<Quad>>;
```

- Uses Prompt 2 (low temperature)
- Parses JSON array of quads
- Assigns `QuadSource::Background` and fresh UUIDs
- Deduplicates on `(object_a, relationship, object_b)` tuple

---

### Step 3 — Single TE Generator

**Module:** `pipeline/te_generator.rs`

```rust
pub struct DrawResult {
    pub objects: Vec<String>,
    pub relationships: Vec<String>,
    pub properties: Vec<String>,
    pub quads_used: Vec<Uuid>,
}

pub async fn generate_thought_experiment(
    client: &LlmClient,
    topic: &str,
    draw: &DrawResult,
    accumulated_path: &[Node],
    unresolved_tension: Option<&UnresolvedTension>,
) -> Result<String>;
```

- Uses Prompt 3 (high temperature)
- Returns raw thought experiment text

---

### Step 4 — Coherence Filter

**Module:** `pipeline/coherence_filter.rs`

```rust
#[derive(Debug, Deserialize)]
pub struct CoherenceResult {
    pub internally_consistent: bool,
    pub consistent_reason: String,
    pub topic_relevant: bool,
    pub relevant_reason: String,
    pub passes: bool,
}

pub async fn check_coherence(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
) -> Result<CoherenceResult>;
```

- Uses Prompt 4 (low temperature)
- Returns pass/fail. A TE must pass both criteria.

---

### Step 5 — Deutsch Scorer

**Module:** `pipeline/deutsch_scorer.rs`

```rust
pub async fn score_deutsch(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
    accumulated_path: &[Node],
) -> Result<DeutschScore>;
```

- Uses Prompt 5 (low temperature)
- Returns the full `DeutschScore` struct

---

### Step 6 — Tension Extractor

**Module:** `pipeline/tension_extractor.rs`

```rust
pub async fn extract_tension(
    client: &LlmClient,
    topic: &str,
    thought_experiment: &str,
    accumulated_path: &[Node],
    score: f64,
) -> Result<UnresolvedTension>;
```

- Uses Prompt 6 (low temperature)

---

### Step 7 — Single Branch Runner

**Module:** `engine/branch_runner.rs`

This is the core loop. It combines steps 3-6 in sequence at each depth.

```rust
pub async fn run_branch(
    client: &LlmClient,
    pool: &Arc<RwLock<DrawPool>>,
    branch: &mut Branch,
    config: &Config,
) -> Result<()>;
```

**Algorithm:**

```
for depth in (current_depth + 1)..=depth_limit:
    draws_attempted = 0
    survivor_found = false

    while draws_attempted < config.draws_per_depth and not survivor_found:
        draw = pool.draw(config.atom, rng)
        te = generate_thought_experiment(draw, path, tension)

        // Filter stack (cheap → expensive)
        if not grammar_check(te):          // simple heuristic, no LLM
            draws_attempted += 1
            continue

        coherence = check_coherence(te)
        if not coherence.passes:
            draws_attempted += 1
            continue

        score = score_deutsch(te, path)
        if score.overall_score < config.survivor_threshold:
            draws_attempted += 1
            continue

        // Survivor found
        tension = extract_tension(te, path, score)
        node = Node { te, score, tension, ... }
        branch.nodes.push(node)

        // Novel pool admission
        if score.overall_score >= config.novel_threshold:
            novel_quads = extract_quads_from_te(te)
            pool.write().add_novel(novel_quads, node.id)
            pool.write().update_ratio()

        survivor_found = true

    if not survivor_found:
        // No survivor after max draws — branch terminates
        branch.status = Terminated
        return
```

**Grammar filter** (no LLM call): Check minimum length, presence of a question mark or conditional structure, absence of pure gibberish (e.g., all words repeated). This is a cheap pre-filter to avoid wasting LLM calls on obviously bad draws.

---

### Step 8 — Background Pool Initializer

**Module:** `engine/background_init.rs`

```rust
pub async fn initialize_background_pool(
    client: &LlmClient,
    topic: &str,
) -> Result<(Vec<Quad>, Vec<String>)>;  // (quads, suggested_resources)
```

- Calls Prompt 1 (high temperature) → raw facts + resources
- Calls `extract_quads` (step 2) on the facts text
- Returns 80-120 background quads + resource list

---

### Step 9 — Root Branch Generator

**Module:** `engine/root_generator.rs`

```rust
pub async fn generate_root_branches(
    client: &LlmClient,
    pool: &Arc<RwLock<DrawPool>>,
    topic: &str,
    config: &Config,
) -> Result<Vec<Branch>>;
```

- Runs the draw → generate → filter → score pipeline repeatedly
- Collects N diverse survivors as root nodes for N branches
- Diversity: reject a root if its quads overlap >50% with any existing root
- Each root becomes depth-0 of a new branch

---

### Step 10 — Trajectory Scorer

**Module:** `pipeline/trajectory_scorer.rs`

```rust
pub async fn score_trajectory(
    client: &LlmClient,
    topic: &str,
    branch: &Branch,
) -> Result<TrajectoryScore>;

#[derive(Debug, Deserialize)]
pub struct TrajectoryScore {
    pub trajectory_score: f64,
    pub depth_of_insight: String,
    pub best_node: Uuid,
    pub justification: String,
}
```

- Uses Prompt 7 (low temperature)
- Called after a branch completes (reaches depth limit or terminates)

---

### Step 11 — Cross-Pollination Detector

**Module:** `pipeline/cross_pollination.rs`

```rust
pub async fn check_cross_pollination(
    client: &LlmClient,
    topic: &str,
    branch_a: &Branch,
    branch_b: &Branch,
) -> Result<CrossPollinationResult>;

#[derive(Debug, Deserialize)]
pub struct CrossPollinationResult {
    pub complementary: bool,
    pub reason: String,
    pub suggested_merge_angle: Option<String>,
}
```

- Uses Prompt 8 (low temperature)
- Compares the latest unresolved tension from each branch
- Called periodically during tree execution (e.g., every 2 depth levels)

**Merge logic:** When complementary, create a new branch whose depth-0 node combines elements from both branches' latest tensions. Reset depth counter. The new branch uses the merge angle as its initial unresolved tension.

---

### Step 12 — Parallel Tree Runner

**Module:** `engine/tree_runner.rs`

This is the orchestrator.

```rust
pub async fn run_tree(
    client: &LlmClient,
    config: &Config,
    topic: &str,
) -> Result<Tree>;
```

**Algorithm:**

```
1. Initialize background pool (step 8)
2. Load universal vocabulary (step 1)
3. Create DrawPool with initial 50/50/0 ratio
4. Generate root branches (step 9)
5. Spawn each branch as a tokio task:
   - Each branch runs independently (step 7)
   - DrawPool is shared via Arc<RwLock<DrawPool>>
     (novel pool writes are infrequent, contention is low)
6. Cross-pollination check loop:
   - Every time any branch completes a depth level,
     check its latest tension against all other active branches
   - If complementary pair found, spawn a new merged branch
7. Wait for all branches to complete or terminate
8. Score all trajectories (step 10)
9. Rank and return top trajectories
```

**Concurrency model:**

- Branches run as `tokio::spawn` tasks
- LLM calls are the bottleneck — use a semaphore (`tokio::sync::Semaphore`) with `config.llm.max_concurrent` permits to cap concurrent API calls
- The `DrawPool` is wrapped in `Arc<RwLock<DrawPool>>` — reads are frequent (every draw), writes are rare (novel pool admission only)
- A `tokio::sync::broadcast` channel notifies the cross-pollination checker when any branch advances a depth level

**Progress reporting:**

- Use `tracing` to emit structured events at each depth advancement
- A subscriber formats these as the CLI output shown in the design doc

---

### Step 13 — CLI Wrapper

**Module:** `main.rs` + `config.rs`

```rust
#[derive(Parser)]
#[command(name = "teg", about = "Thought Experiment Generator")]
struct Cli {
    /// Topic to explore
    topic: Option<String>,

    #[arg(long, default_value_t = 10)]
    depth: u32,

    #[arg(long, default_value_t = 10)]
    branches: u32,

    #[arg(long, default_value_t = 0.7)]
    threshold: f64,

    #[arg(long, default_value_t = 0.85)]
    novel_threshold: f64,

    #[arg(long, default_value_t = 0.5)]
    ratio: f64,

    #[arg(long, default_value_t = 100)]
    draws: u32,

    #[arg(long, default_value_t = 1.2)]
    temperature: f64,

    #[arg(long, default_value_t = 4)]
    objects: u32,

    #[arg(long, default_value_t = 3)]
    relationships: u32,

    #[arg(long, default_value_t = 2)]
    properties: u32,

    /// LLM provider: "anthropic" or "openai"
    #[arg(long, default_value = "anthropic")]
    provider: String,

    /// Model name
    #[arg(long, default_value = "claude-sonnet-4-6")]
    model: String,

    /// Max concurrent LLM calls
    #[arg(long, default_value_t = 5)]
    max_concurrent: usize,

    /// Output format: "text" or "json"
    #[arg(long, default_value = "text")]
    output: String,
}
```

**Interactive flow:**

1. If `topic` is not provided as an arg, prompt interactively
2. Print initialization progress (pool sizes, resource suggestions)
3. Print tree progress (depth, active branches, best score) as branches advance
4. On completion, print ranked trajectories with thought experiment chains
5. `--output json` dumps the full `Tree` struct as JSON for programmatic use

---

## Error Handling Strategy

| Layer | Error type | Handling |
|---|---|---|
| LLM API | Transient (429, 500, 503) | Exponential backoff, 3 retries |
| LLM API | Auth failure (401, 403) | Fail fast with clear message |
| LLM response | Invalid JSON | `parser.rs` fallback extraction; log raw response; skip draw |
| LLM response | Missing fields | `serde` defaults where safe; skip draw otherwise |
| Draw pool | Empty pool (no quads) | Fail with message — requires background init to produce quads |
| Branch | No survivor in max draws | Terminate branch (not an error) |
| File I/O | Missing vocabulary file | Fail fast — vocabulary is required |

---

## Testing Strategy

### Unit Tests

- **`parser.rs`** — Fuzz with malformed JSON, markdown-wrapped JSON, partial responses
- **`draw_pool.rs`** — Ratio calculation, draw distribution over many samples, edge cases (empty novel pool)
- **`vocab/mod.rs`** — Vocabulary loading, deduplication
- **Grammar filter** — Known good/bad inputs

### Integration Tests

- **Single pipeline pass** — Mock LLM client returns canned responses; verify quad extraction → TE generation → coherence → scoring → tension extraction produces a valid `Node`
- **Branch runner** — Mock LLM; verify branch runs to depth limit and terminates correctly
- **Cross-pollination** — Mock LLM; verify complementary detection triggers merge

### LLM Client Mock

```rust
/// Test double for LlmClient that returns canned responses
/// based on prompt content matching.
pub struct MockLlmClient {
    responses: Vec<(String, String)>,  // (prompt_contains, response)
}
```

### End-to-End

- Run against a real LLM API with a simple topic, depth 2, 2 branches
- Assert: tree has branches, nodes have scores, at least one trajectory scored

---

## Performance Considerations

**LLM calls dominate runtime.** With defaults (10 branches, depth 10, 100 draws per depth), worst case is:

```
Root generation:    ~500 draws × 3 LLM calls (generate + coherence + score) = 1,500 calls
Branch execution:   10 branches × 10 depths × ~50 avg draws × 3 calls = 15,000 calls
Tension extraction: ~100 nodes × 1 call = 100 calls
Cross-pollination:  ~45 pairs × 1 call = 45 calls
Trajectory scoring: ~10 branches × 1 call = 10 calls
Total worst case:   ~16,655 LLM calls
```

**Mitigations:**

1. Grammar pre-filter eliminates ~30-50% of draws before any LLM call
2. Coherence filter is cheaper than Deutsch scoring — fail fast
3. Concurrent branch execution means wall-clock time scales with depth, not branches
4. `max_concurrent` caps API calls to avoid rate limits
5. Caching: hash (prompt, temperature) → response for deterministic (low-temp) calls. Same coherence question = same answer. Use an in-memory `HashMap` with optional disk persistence.

**Estimated runtime with 5 concurrent calls, ~500ms per call:**

```
~16,655 calls / 5 concurrent / 2 (grammar filter) ≈ 1,666 seconds ≈ 28 minutes
With caching on deterministic calls: ~20 minutes
```

---

## Future Extensions (Not in Scope)

These are noted for design awareness but are explicitly **not** part of the initial build:

- **Persistence / resume** — Serialize tree state to disk, resume interrupted runs
- **Web UI** — Real-time tree visualization
- **Custom vocabulary** — User-provided domain vocabulary files
- **Multi-topic** — Run multiple topics and cross-pollinate across topic trees
- **Embeddings-based draw** — Use vector similarity instead of random draw to bias toward promising regions
