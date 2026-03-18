# Thought Experiment Generator

A self-improving epistemic engine. It collides candidate conjectures against problems to generate outputs, evaluates those outputs for explanatory quality, and feeds the survivors back into its own conjecture pool. Over many runs, the system steers toward whatever generates genuine insight.

The core loop:

```
problem + conjecture → generated output → criticism → conjecture
```

Inspired by David Deutsch's epistemology — knowledge grows through conjecture and criticism, not random search. The system narrows the search space by compounding success: only generated outputs that survive criticism become conjectures, and only conjectures that generate high-scoring outputs survive long enough to be promoted.

Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY` for OpenAI).

## Why Architecture, Not Model

All progress — mathematical theorems, physical theories, philosophical arguments — exists in abstract space. It is not stored in neurons; it travels through them. A brain does not think in spikes; a mind thinks in ideas. The neuron is the substrate, not the thought.

The same distinction applies here. An LLM predicts tokens — it is the substrate. This system operates one level up: in the space where conjectures are formed, evaluated against problems, and promoted based on explanatory power. The LLM executes; the architecture reasons.

This is why we are not building a better model. The bottleneck is not substrate quality but structure. Early signs of this pattern are visible in purpose-built agent systems — research agents, scientific automation pipelines — that use LLMs as an execution layer while organizing reasoning at a higher level. This system is the generalization of that pattern: a domain-agnostic architecture for how knowledge grows through conjecture and criticism, operating in the abstract space where explanations live.

## Usage

```sh
# Create a problem set (ID is first 8 chars of sha256 of content — printed on creation)
cargo run -- create-problemset "LLMs and epistemology: exploring whether LLMs generate genuine knowledge"
cargo run -- create-problemset --file my-problemset.md
cat my-problemset.md | cargo run -- create-problemset

# List all problem sets to find the hash ID
cargo run -- list-problemsets

# Add problems to a set (cap: 10 per set)
cargo run -- add-problem --problemset a1b2c3d4 --text "Can LLMs create new knowledge?"

# Remove a problem from a set
cargo run -- remove-problem --problemset a1b2c3d4 --problem-id "can-llms-create-new-knowledge"

# Run on a problem set
cargo run -- run --problemset a1b2c3d4
cargo run -- run                               # works if only one set exists
cargo run -- run --problemset a1b2c3d4 --problem "new problem text"

# Read last run summary without running
cargo run -- read

# Reset state to seed and run immediately (seed is runnable out of the box)
cargo run -- --fresh run

# Add a conjecture
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --text "Full text of the conjecture"
cargo run -- add-conjecture --layer candidates --title "Conjecture Title" --file path/to/conjecture.md
cat my-conjecture.md | cargo run -- add-conjecture --layer mind --title "Conjecture Title"
```

### Options

| Flag | Default | Description |
|---|---|---|
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--consistency-threshold` | 0.3 | Minimum logical consistency score to proceed to Pass 2 |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter the active set |
| `--min-run-count` | 3 | Minimum runs before a conjecture or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--temperature` | 0.9 | LLM temperature |
| `--fresh` | false | Reset state to seed |

## How It Works

**Phase 1 — Generate Outputs.** Each (problem, candidate conjecture) pair produces one generated output. The mind's conjecture summaries form the system prompt. The candidate conjecture's summary and problem summary are combined in the user prompt. Pairs run concurrently. Already-generated outputs are skipped (resumable runs).

**Phase 2 — Evaluate Outputs.** Each generated output is scored by all active evaluation criteria (loaded from `data/state/evaluations/`). The seed criteria are:
- *Logical Consistency:* Is the output internally self-consistent? Score 0.0–1.0. Below threshold (default 0.3), output is skipped.
- *Hard to Vary:* The mind generates 10 yes/no questions probing whether each part of the output is load-bearing. Score = yes_count / 10.

Combined score is a weighted sum of all active evaluation scores. Evaluation criteria are manually managed — the system never adds or removes them automatically. Users can add criteria by editing `data/state/evaluations/` directly.

- *Candidate Problems:* The mind also identifies unresolved tensions and open questions raised by each output. Candidates scoring above threshold are admitted to the active problem set.

**Phase 3 — Rank and Promote.**
- Conjecture scores update as rolling averages weighted by run_count. Composite score = `score × √(problem_coverage_breadth)`.
- Problem scores update as rolling averages of mean output score across all conjectures applied.
- *Problem review:* The mind receives all problem summaries in the set and removes duplicates and subsumed problems (removed from set membership, not global). The bottom-ranked problem with sufficient run history is also dropped from the set, enforcing the 10-problem cap.
- *Conjecture promotion:* Top candidate conjecture (by composite, min run_count) → mind. Top generated output → summarized into a new candidate conjecture.
- *Conjecture demotion:* Bottom mind conjecture → candidates. Bottom candidate conjecture → discarded.

**Phase 4 — Report.** Ranked output table, top 5 summaries, all changes to conjectures and problems.

## Conceptual Hierarchy

All layers contain the same atom: a **Conjecture** — a unit of explanatory knowledge or perspective, held fallibly. The layers differ only in trust and stability, not in kind.

```
Mind        — most trusted conjectures, slowest to change
  ↑ promote / ↓ demote
Candidates  — active conjectures under test, medium stability
  ↑ promote / ↓ demote
Generated   — outputs produced each run, scored and ephemeral
```

Conjectures are shared across all problem sets — the mind and candidates are not scoped to any one set.

**Problem Sets** are scoped collections of problems (cap: 10), identified by a hash of their content. Problems only exist within sets. A run operates on one problem set, discovers candidate problems, and adds them to the set. Deduplication and cap enforcement keep the set focused.

**Evaluations** are a stable, manually-governed set of scoring criteria applied in Phase 2. They sit outside the promotion hierarchy — the system never adds, removes, or reorders them. Users extend or refine evaluations by editing `data/state/evaluations/` directly. Each evaluation defines a scoring criterion and a weight; the combined output score is the weighted sum across all active evaluations.

## State Layout

```
data/state/
  state.json                     — current run number
  mind/                          — mind conjectures (.md + .json each)
  candidates/                    — candidate conjectures (.md + .json each)
  evaluations/                   — evaluation criteria (.md + .json each) [manually managed]
  problems/                      — problems (.md + .json each)
  problemsets/                   — problem set index (.md + .json each)
  runs/
    001/
      {problem}-{conjecture}.md    — generated output content + questions
      {problem}-{conjecture}.json  — scores + candidate problems
      summary.md                   — ranked results + changes
```

Seed state lives in `data/seed/`. `--fresh` resets `data/state/` from seed.

## Seed State

The seed is runnable out of the box — `cargo run -- --fresh run` starts a full cycle immediately.

**Problem set:** ID `default`, scoped to "Can LLMs create knowledge?" with three starter problems:
- Can LLMs create new knowledge?
- Does architecture matter more than model scale?
- What makes an explanation hard to vary?

**Mind conjectures:** Deutschian Epistemology, Ontology, Systems Thinking (Donella Meadows)

**Candidate conjectures:** Thought Experiments, Mathematical Formalism, Counterfactual Reasoning, Extreme Cases, Historical Genesis

**Evaluations:** Logical Consistency (weight 0.3), Hard to Vary (weight 0.7)

The seed problem set uses the static ID `default`. Problem sets you create via `create-problemset` get an ID that is the first 8 characters of the sha256 of their content — printed when the set is created and shown in `list-problemsets` output as `[id]`.

## Documents

- [Epistemic Engine Spec](docs/epistemic-engine-spec.md) — Full design specification
- [Journal](docs/journal.md) — Intellectual lineage and architecture decisions
