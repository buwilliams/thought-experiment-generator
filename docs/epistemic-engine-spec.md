# Epistemic Engine — Design Spec

## What This Is

The core loop is:

```
conjecture + problem → candidate
candidate + criticism → conjecture (if it survives)
```

This loop is the mechanism for narrowing the search space. Novel theories exist in a vast possibility space — the challenge is not coverage but steering. Random search has no memory; each draw is as blind as the last. Here, only candidates that survive criticism become conjectures, and only conjectures that generate high-scoring candidates survive long enough to be promoted. The conjecture pool progressively biases toward whatever is generative. The search space is not narrowed by exclusion — it is steered by accumulated success.

The loop is self-referential: the same conjecture pool that generates candidates also criticizes them. The mind is conjectures. Perspectives are conjectures. The distinction between layers is stability and trust, not ontology.

This is a thinking engine, not a search engine. The goal is not to find a pre-existing answer but to generate structured candidates that illuminate a problem in ways neither the conjectures nor the problem alone could produce.

## The Level Distinction

Progress in physics happens in abstract space, not in the brain tissue of physicists. Einstein's relativity exists there — it is a fact about abstract space, not a fact about Einstein's neurons. The same is true of mathematical theorems, logical arguments, and explanations of any kind. Neurons are the substrate. Ideas are what travel through them. Humans do not think in neurons; they think in conjectures, analogies, and explanations.

The same distinction applies here. An LLM predicts tokens — it is the substrate. This system operates one level up: in the space where conjectures are formed, tested against problems, and promoted based on explanatory power. The LLM executes; the architecture reasons. Progress happens at the architecture level, for the same reason that human intellectual progress happens not at the neuron level but at the level of ideas.

This system explores a complementary direction to scaling: organizing reasoning at a higher level rather than improving the substrate. Purpose-built agent systems (research agents, scientific automation pipelines, reasoning models) show that structure compounds what substrate alone cannot. This is the generalization of that pattern: a domain-agnostic architecture for how explanations grow through conjecture and criticism.

---

## Conceptual Hierarchy

All layers contain the same atom: a **Conjecture** — a unit of explanatory knowledge or perspective, held fallibly. The layers differ only in trust and stability, not in kind. This follows directly from Deutschian fallibilism: there is no bedrock of certain knowledge, only conjectures held with varying degrees of confidence.

```
Mind        — most trusted conjectures, slowest to change
  ↑ promote / ↓ demote
Candidates  — active conjectures under test, medium stability
  ↑ promote / ↓ demote
Generated   — outputs produced each run, scored and ephemeral
```

Conjectures (mind and candidates) are shared across all problem sets.

**Problem Sets** are scoped collections of problems (cap: 10). Problems only exist within sets — there is no global problem pool. Problem sets are not in the promotion hierarchy but are scored and pruned by the system each run.

**Evaluations** are a stable, separately-governed set of scoring criteria. They sit outside the promotion hierarchy entirely — the system never modifies them. Users extend or refine evaluations manually by editing `data/state/evaluations/`. These files are for human reference and documentation; evaluation weights are hardcoded in `evaluator.rs` (0.20 consistency, 0.35 htv, 0.30 reach, 0.15 refutation).

---

## Data Structures

The "database" is the filesystem. All state is stored as markdown files in subdirectories. No JSON databases. This keeps state human-readable, diffable, and inspectable without tooling.

Each entity has two files: a `.md` for human-readable content and a `.json` sidecar for machine-readable metadata. Prompts read from JSON (summaries, scores). Humans read `.md`.

**Conjectures (mind and candidates) are shared across all problem sets.** Problem sets are scoped collections — problems only exist within sets.

### Conjecture content — `{layer}/{id}.md`

```markdown
# {title}

## Summary

One or two sentences. This is what goes into prompts.

## Full Text

Full text of the conjecture — as long as needed.
```

### Conjecture metadata — `{layer}/{id}.json`

```json
{
  "id": "slug",
  "layer": "mind | candidates",
  "score": 0.0,
  "rank": 1,
  "run_count": 0,
  "problem_coverage": ["problem-id"],
  "created_at": "iso8601",
  "promoted_from": "id | null",
  "history": [{ "run": 1, "score": 0.0, "problem_id": "slug" }]
}
```

### Evaluation content — `evaluations/{id}.md`

```markdown
# {name}

## Summary

One or two sentences describing what this criterion measures. Used in evaluation prompts.

## Full Text

Full description of the criterion — what it tests, how the LLM should score it, and why it matters.
```

### Evaluation metadata — `evaluations/{id}.json`

```json
{
  "id": "slug",
  "name": "Human Name",
  "weight": 0.3,
  "created_at": "iso8601"
}
```

Evaluations are manually managed for reference. The system never reads or writes to `evaluations/` — weights are hardcoded in `evaluator.rs`.

---

### Problem set content — `problemsets/{id}.md`

Raw text content describing the set's scope and theme. No sections — just the content as provided.

### Problem set metadata — `problemsets/{id}.json`

```json
{
  "id": "8-char-sha256",
  "problems": [
    {
      "meta": { "id": "slug", "source": "user | system", "score": 0.0, "rank": 1, "run_count": 0, "created_at": "iso8601" },
      "title": "...",
      "summary": "One sentence. Used in prompts.",
      "full_text": "Full text of the problem."
    }
  ],
  "run_count": 0,
  "created_at": "iso8601"
}
```

The ID is the first 8 characters of `sha256(content)`. A problem set is capped at **10 problems**. Problems are embedded directly in the problemset JSON — there is no separate `problems/` directory. Conjectures (mind/perspectives) are shared across all sets.

### Generated output content — `runs/NNN/{problem_id}-{conjecture_id}.md`

```markdown
# Generated: {problem_id} × {conjecture_id}

## Output

Full generated output text.
```

### Generated output metadata — `runs/NNN/{problem_id}-{conjecture_id}.json`

```json
{
  "problem_id": "slug",
  "conjecture_id": "slug",
  "run": 1,
  "logical_consistency": 0.0,
  "hard_to_vary": 0.0,
  "explanatory_reach": 0.0,
  "resistance_to_refutation": 0.0,
  "total": 0.0,
  "candidate_problems": [{ "text": "...", "score": 0.0 }]
}
```

---

## Persistent State

All state lives in `data/state/`:

```
data/state/
  state.json             — current run number and global metadata
  mind/
    {id}.md              — conjecture content (summary + full text)
    {id}.json            — conjecture metadata (score, rank, run_count, history)
  candidates/
    {id}.md
    {id}.json
  evaluations/           — evaluation criteria [manually managed, never modified by system]
    {id}.md              — criterion description
    {id}.json            — criterion metadata (name, weight)
  problemsets/
    {id}.md              — set scope (raw text)
    {id}.json            — set metadata + embedded problems (score, rank, run_count per problem)
  runs/
    NNN/
      {problem}-{conjecture}.md    — generated output content
      {problem}-{conjecture}.json  — scores + candidate problems
      summary.md                   — ranked results + promoted/demoted conjectures
  review.md                      — latest full review output (markdown with relative links)
```

`data/state/state.json`:
```json
{
  "run": 3,
  "created_at": "iso8601",
  "last_run_at": "iso8601"
}
```

The run number increments at the start of each run. `runs/NNN/` is created using the new run number before any candidates are generated.

The seed state is checked in as `data/seed/`:

```
data/seed/
  mind/                  — starting mind conjectures (.md + .json each)
  candidates/            — starting candidate conjectures (.md + .json each)
  evaluations/           — starting evaluation criteria (.md + .json each)
  problemsets/           — starting problem sets (.md + .json each, problems embedded in .json)
```

The seed is runnable out of the box. The default problem set uses the static ID `default`. Problem sets created via `create-problemset` get an ID equal to the first 8 characters of `sha256(content)`, printed on creation and shown by `list-problemsets` as `[id]`.

On first run, seed state is copied to `data/state/`. `--fresh` resets `data/state/` from seed.

---

## The Run

A single run processes all problems against all **lenses** — every candidate conjecture plus every mind conjecture — generating one output per (problem × lens) pair.

### Phase 1+2 — Generate and Evaluate Outputs

The system iterates over all **lenses** — every candidate conjecture ∪ every mind conjecture. Mind conjectures participate as lenses alongside candidates: they are tested directly, not just used as background context.

**For each lens (one concurrent task per lens):**

1. Generate all problem outputs concurrently:
   - Build context: mind conjecture summaries (system prompt) + lens summary + problem set context + problem summary
   - Call LLM → generated output text (one call per problem, all in parallel)
2. Evaluate all outputs for this lens in a single comparative call:
   - The LLM receives all outputs together and scores them calibrated against each other
   - Returns scores for all four dimensions plus candidate problems for each output

All lens tasks run concurrently up to `--max-concurrent`. Already-generated outputs are skipped per (problem, lens) pair, so runs are resumable.

**Evaluation** uses a fallibilist frame: seek to falsify, not confirm. Genericness is a failure mode — a conjecture too vague to commit to specific claims scores low, not because it is wrong but because it cannot be wrong. Combined score = `0.20 × consistency + 0.35 × htv + 0.30 × reach + 0.15 × refutation`.

**Logical Consistency** (weight 0.20)
- Is the output internally self-consistent? Does it contradict itself or rely on incompatible premises?
- Note: consistency is necessary but not virtuous. A vague output that commits to nothing cannot contradict itself — that earns no credit.
- If score < threshold (default 0.3), all remaining scores are zeroed.

**Hard to Vary** (weight 0.35)
- Are the output's claims load-bearing? Apply substitution tests on specific named elements: if you replaced a component with its negation or a generic alternative, would the conclusion change?
- Components that survive substitution are decorative, not load-bearing. Generic language that could describe almost anything scores near 0.

**Explanatory Reach** (weight 0.30)
- Does the output commit to specific claims about cases the problem did not raise — claims specific enough that they could be false in those adjacent cases?
- Generic assertions of broad applicability do not count. Near 0 for outputs that only make local claims or merely assert breadth without committing to anything.

**Resistance to Refutation** (weight 0.15)
- Attempt to construct a specific falsifying case. Score near 0 in either of two cases: (a) a specific falsifying case was found, or (b) the output never committed to anything specific enough to be falsifiable.
- Vagueness is not robustness — it is emptiness. Score near 1 only if specific claims survived a genuine falsification attempt.

**Candidate Problem Extraction**
- The same comparative call also extracts candidate problems from each output: unresolved tensions or unexplored implications specific enough that a subsequent conjecture could be wrong about them.
- Generic open questions score near 0. Score each candidate 0.0–1.0. Threshold 0.6 for admission.

### Phase 3 — Rank and Promote

**Output ranking:** sort all generated outputs by combined score descending.

**Score updates:**
- Candidate conjectures: rolling average updated at full weight — `(old×n + new) / (n+1)`.
- Mind conjectures: updated at half weight (inertia) — `(old×n + new×0.5) / (n+0.5)`. New evidence moves mind scores less; the mind requires sustained evidence to shift, not a single run.

**Composite score:** `score × √(problem_coverage_breadth) × layer_weight`, where `layer_weight = 1.5` for mind conjectures and `1.0` for candidates. Mind conjectures carry more weight in all ranking comparisons — ask, promotion selection, demotion selection.

**Problem ranking:** for each problem, compute mean output score across all lenses this run. Update `score` as rolling average.

**Admit candidate problems:** candidate problems extracted during Phase 2 are evaluated for admission to the current problem set (admission threshold 0.6). The top 3 qualifying candidates per run are admitted, embedded directly in the problemset JSON. Cap is enforced at admission: if the set is already at 10 problems, new system-generated problems are not admitted. Problems are never ejected to make room — low score may mean genuinely hard, not unimportant.

**Problem review (one LLM call per run, scoped to the current set):**
- The mind receives summaries of all problems in the set (id + summary) and identifies any that are exact duplicates of or fully subsumed by another problem in the set. Identified problems are removed from the set's embedded problem list.

**Promotion (one per run, skipped if no eligible conjectures):**
- Top-ranked candidate conjecture (by composite, minimum `run_count` of 3) → promoted to mind.
- Top-ranked generated output → promoted to candidates. Outputs generated using a mind conjecture as lens receive a 1.1× bonus in this selection — a slight tilt toward mind-lens outputs without penalizing unrelated conjectures.

**Demotion (one per run, skipped if no eligible conjectures):**
- Bottom-ranked mind conjecture (by composite, minimum `run_count` of **6** — twice the candidate threshold) → demoted to candidates. Mind conjectures need twice the run history before demotion is eligible.
- Bottom-ranked candidate conjecture (by composite, minimum `run_count` of 3, excluding the just-promoted output) → discarded.

Promotion and demotion are skipped entirely if fewer than one conjecture meets the threshold. This protects early runs where signal is thin.

**Resistance model summary:** five layers of asymmetry protect the mind from noise without making it rigid: (1) composite ×1.5 multiplier in all rankings; (2) score inertia (0.5 vote weight) slows score drift; (3) 2× demotion run_count guard; (4) 1.1× promotion bonus for mind-lens outputs; (5) live testing in Phase 1 keeps the mind accountable.

### Phase 4 — Report

Write `data/state/runs/NNN/summary.md`:
- Ranked output table (problem, conjecture, score)
- Top 5 outputs with 20-word summaries
- Promoted candidate conjecture (→ mind)
- Promoted generated output (→ candidates)
- Demoted conjectures
- Problems removed (deduplicated + bottom-ranked discard)

---

## Prompts

All prompt templates live in `data/prompts/*.md`. Each file has `## System` and `## User` sections with `{{variable}}` placeholders. Edit these to tune LLM behavior without touching Rust code.

### generate_output(mind_system, lens_summary, problemset_context, problem_summary)
```
System: [mind conjectures as a structured set of principles and perspectives]

User: Apply [lens_summary] to [problem_summary]. Generate a structured thought experiment
exploring what is illuminated when this lens meets this problem. 500 words or fewer.
Problem set context: [problemset_context]
```

### comparative_evaluate(mind_system, lens_summary, formatted_outputs)

One call per lens, evaluates all problem outputs together for calibrated scoring across four dimensions plus candidate problem extraction.

```
System: [mind conjectures]

User: Evaluate the following outputs. They were all generated by the same lens applied
to different problems. Score comparatively — 0.8 on one output means the same as 0.8 on another.

Lens: [lens_summary]

Score each output on:
- consistency: internal self-consistency (necessary but not virtuous — vague commits-to-nothing earns no credit)
- hard_to_vary: substitution tests on named elements; generic language scores near 0
- explanatory_reach: specific claims in adjacent cases that could independently fail; asserted breadth scores near 0
- resistance_to_refutation: near 0 for trivially falsified OR unfalsifiable-because-vague

Also extract candidate_problems: specific unresolved tensions a subsequent conjecture could be wrong about.

Return JSON: {"evaluations": [{"problem_id": "slug", "consistency": 0.0, "hard_to_vary": 0.0,
  "explanatory_reach": 0.0, "resistance_to_refutation": 0.0,
  "candidate_problems": [{"text": "...", "score": 0.0}]}]}
```

### promote_generated(mind_system, generated, score)

Promotes a generated output into a reusable candidate conjecture. Preserves specific claims exactly — does not generalize into empty universal language.

```
Return JSON: { "title": "...", "summary": "...", "full_text": "..." }
```

### conjecture_summary(mind_system, title, full_text)
```
Return JSON: { "summary": "..." }
```

### summarize_generated(generated, score)
```
Return JSON: { "summary": "..." }
# 20-word summary of what the output claims or illuminates
```

### deduplicate_problems(mind_system, formatted_problems)
```
Return JSON: { "remove": ["id1", "id2"] }
# Only exact duplicates or fully subsumed problems. When in doubt, keep.
```

### novelty_check(mind_system, title, conjecture)
```
Return JSON: { "is_novel": true, "novelty_score": 0.0, "closest_analog": "...", "explanation": "..." }
```

### ask(mind_system, conjecture_summary, question)
One call per lens. Each lens answers the question from its perspective.

### ask_consolidate(mind_system, question, perspectives)
Consolidates all lens answers into a single best explanation.

### review_assess(mind_system, mind_full, top_outputs, trajectory)
Adversarial self-assessment of mind quality and output quality.

---

## Seed Mind

The seed mind is defined by the files in `data/seed/mind/`. Each conjecture has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/mind/`.

Current seed mind conjectures:

- **`deutschian-epistemology`** — Fallibilism and the growth of knowledge through conjecture and criticism. Abstract objects are real; progress in abstract space (mathematics, epistemology) is as genuine as progress in physics and requires no physical grounding.
- **`ontology`** — The philosophical study of what exists. Root axiom: knowledge and information are fundamental (held fallibly). Properties are informational distinctions; mechanisms are processes by which information is preserved, altered, or lost.
- **`systems-thinking`** — Donella Meadows' framework: behavior is produced by structure, not isolated causes. Stocks, flows, feedback loops, delays, and leverage points. The system's real goal is revealed by its behavior, not its stated purpose.

---

## Seed Candidates

The seed candidates are defined by the files in `data/seed/candidates/`. Each conjecture has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/candidates/`.

Current seed candidate conjectures:

- **`thought-experiments`** — Construct a hypothetical scenario, isolate the key variable, stipulate it as real, and follow the logic strictly to wherever it leads. Strangeness is information.
- **`mathematical-formalism`** — Translate the problem into mathematical structure (variables, invariants, transformations). Let the structure reveal what is preserved, what varies, and what constraints are non-negotiable.
- **`counterfactual-reasoning`** — Systematically vary assumptions: remove one, invert one, replace one with its opposite. What breaks is load-bearing. What survives is incidental.
- **`extreme-cases`** — Push to limits: infinity, zero, maximum, minimum, phase transitions. Boundary behavior reveals the structure hidden in the comfortable middle range.
- **`historical-genesis`** — Ask how the problem came to exist, what it replaced, and what it inherited without examination. Origin stories make invisible constraints visible.

---

## Seed Problems

The seed problem set (`default`) contains three starter problems:

- **`can-llms-create-new-knowledge`** — Can a system that predicts tokens generate explanations that constitute genuine knowledge, or does it only rearrange what it has seen?
- **`does-architecture-matter-more-than-model-scale`** — Is the bottleneck for AI epistemic progress in the substrate (model size, compute) or in the structure above it (how conjectures are formed, tested, and promoted)?
- **`what-makes-an-explanation-hard-to-vary`** — What structural properties distinguish an explanation whose parts are all load-bearing from one where details can be swapped freely without damaging the account?

---

## Seed Evaluations

The seed evaluations are defined by the files in `data/seed/evaluations/`. On first run, these are copied to `data/state/evaluations/`. They are never modified by the system — users manage them directly for reference.

Evaluation weights are hardcoded in `evaluator.rs`, not read from these files:

| Dimension | Weight |
|---|---|
| Logical Consistency | 0.20 |
| Hard to Vary | 0.35 |
| Explanatory Reach | 0.30 |
| Resistance to Refutation | 0.15 |

---

## CLI

```sh
# Create a problem set (content drives the ID via sha256 hash)
cargo run -- create-problemset "LLMs and epistemology: exploring whether LLMs generate genuine knowledge"
cargo run -- create-problemset --file my-problemset.md
cat my-problemset.md | cargo run -- create-problemset

# Add problems to a set
cargo run -- add-problem --problemset a1b2c3d4 --text "Can LLMs create new knowledge?"

# Remove a problem from a set
cargo run -- remove-problem --problemset a1b2c3d4 --problem-id "can-llms-create-new-knowledge"

# List all problem sets
cargo run -- list-problemsets

# Run on a problem set
cargo run -- run --problemset a1b2c3d4
cargo run -- run                              # works if only one set exists

# Run with a new problem added before running
cargo run -- run --problemset a1b2c3d4 --problem "your problem text"

# Run on all problem sets that have not yet been run (run_count == 0)
cargo run -- run-all
cargo run -- --fresh run-all                    # reset to seed, then run all

# Ask the system a question
# Runs all mind conjectures + top 3 candidates concurrently as separate lenses,
# then consolidates all perspectives into a single best explanation
cargo run -- ask "What causes institutional decay?"
cargo run -- ask --file question.md
cat question.md | cargo run -- ask

# Full system review: mind, candidates, score trajectory, conjecture score history,
# problem sets, novelty check, and adversarial LLM self-assessment
# Output saved to data/state/review.md (markdown with links) and printed to stdout (plain text)
cargo run -- review

# Read last run summary without running
cargo run -- read

# Reset scores, run counts, and history — keeps conjectures and problem sets (prompts for confirmation)
cargo run -- reset

# Reset state to seed and run
cargo run -- --fresh run

# Add a new conjecture by inline text
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --text "Full text of the conjecture"
cargo run -- add-conjecture --layer candidates --title "Conjecture Title" --text "Full text of the conjecture"

# Add a new conjecture from a file or stdin
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --file path/to/conjecture.md
cat my-conjecture.md | cargo run -- add-conjecture --layer candidates --title "Conjecture Title"
```

The `add-conjecture` command:
1. Accepts the full text of the conjecture via `--text`, `--file`, or stdin
2. Calls the LLM using the current mind as system prompt to generate a `summary` (1-2 sentences) from the full text
3. Derives a slug from `--title` (lowercased, spaces to hyphens)
4. Writes `{layer}/{slug}.md` and `{layer}/{slug}.json` to `data/state/`
5. Prints the generated summary for review

### Options

| Flag | Default | Description |
|---|---|---|
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--consistency-threshold` | 0.3 | Minimum logical consistency score; outputs below threshold are zeroed |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter the set |
| `--min-run-count` | 3 | Minimum runs before a conjecture or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--temperature` | 0.9 | LLM temperature |
| `--fresh` | false | Reset state to seed |

**`add-conjecture` options:**

| Flag | Required | Description |
|---|---|---|
| `--layer` | yes | `mind` or `candidates` |
| `--title` | yes | Human-readable title, used to derive the file slug |
| `--text` | no | Inline full text of the conjecture |
| `--file` | no | Path to a file whose contents are the full text |
| stdin | no | Piped content used as full text if `--text` and `--file` are omitted |

---

## LLM Call Budget (per run, default settings)

Phase 1+2 is structured per-lens: all problem outputs for a lens are generated concurrently, then evaluated in one comparative call.

| Phase | Calls | Notes |
|---|---|---|
| Output generation | N × M | N problems × M lenses, all concurrent |
| Comparative evaluation | M | 1 call per lens, covers all N problems |
| Problem deduplication | 1 | 1 per run |
| Promote generated output | 1 | 1 per run |
| Top 5 summaries | 5 | |
| **Default (5 problems × 10 lenses)** | | **~57** |

Generated outputs are not cached between runs — they are regenerated each run because the mind and candidate states may have changed. Already-generated outputs for a lens are skipped (resumable).

---

## Module Structure

```
src/
  main.rs         — CLI parsing, dispatch
  runner.rs       — orchestrates phases 1–4; per-lens generation + comparative evaluation
  state.rs        — load/save mind, candidates, problems, generated outputs, runs
  prompts.rs      — all prompt templates (loaded from data/prompts/*.md)
  evaluator.rs    — comparative evaluation: one call per lens covers all four dimensions
  promoter.rs     — ranking, promotion, demotion logic; resistance model
  review.rs       — data report + adversarial self-assessment; writes data/state/review.md
  ask.rs          — multi-lens question answering with consolidation
  types.rs        — Conjecture, Generated, Problem, ProblemSet, LLM response structs
  llm/            — LlmClient (Anthropic + OpenAI)
```

---

## Resolved Decisions

1. **Context efficiency:** Prompts always use summaries, never full text. Mind conjectures serialize as their `summary` fields into the system prompt. This keeps context lean regardless of database size.
2. **All problems × all lenses per run.** No sampling. Lens = candidates ∪ mind.
3. **Promotion preserves specificity:** `promote_conjecture.md` explicitly instructs against generalization — preserve specific claims exactly. Breadth is tested by running the candidate against new problems, not written in upfront.
4. **Problem cap at admission:** The 10-problem cap is enforced when new system-generated problems arrive. If at capacity, new problems are blocked. Problems are only removed by deduplication or manually. Low score ≠ unimportant.
5. **Comparative evaluation:** All outputs for a lens are evaluated in one call for calibrated scoring. Four dimensions plus candidate problem extraction in a single pass.
6. **Evaluation weights hardcoded:** `evaluator.rs` owns the weights directly. `data/state/evaluations/` is for human reference only.

## Open Questions

1. **Lens selection:** Currently all conjectures apply to all problems every run. A future improvement would have the mind select which conjecture to apply to which problem — but that adds complexity and requires signal that doesn't exist yet.
2. **Invalid problem detection:** A problem is invalid if it is self-defeating (the question is its own answer, or progress is impossible in principle). Hard problems are not invalid. Needs a detection mechanism distinct from deduplication.
