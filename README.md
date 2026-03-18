# Thought Experiment Generator

A self-improving epistemic engine. It collides perspective conjectures against problems to generate candidates, evaluates those candidates for explanatory quality, and feeds the survivors back into its own conjecture pool. Over many runs, the system steers toward whatever generates genuine insight.

The core loop:

```
conjecture + problem → candidate → criticism → conjecture
```

Inspired by David Deutsch's epistemology — knowledge grows through conjecture and criticism, not random search. The system narrows the search space by compounding success: only candidates that survive criticism become conjectures, and only conjectures that generate high-scoring candidates survive long enough to be promoted.

Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY` for OpenAI).

## Usage

```sh
# Create a problem set (ID is a hash of the content)
cargo run -- create-problemset "LLMs and epistemology: exploring whether LLMs generate genuine knowledge"
cargo run -- create-problemset --file my-problemset.md
cat my-problemset.md | cargo run -- create-problemset

# Add problems to a set (cap: 10 per set)
cargo run -- add-problem --problemset llms-and-knowledge --text "Can LLMs create new knowledge?"

# Remove a problem from a set
cargo run -- remove-problem --problemset llms-and-knowledge --problem-id "can-llms-create-new-knowledge"

# List all problem sets and their contents
cargo run -- list-problemsets

# Run on a problem set
cargo run -- run --problemset llms-and-knowledge
cargo run -- run                               # works if only one set exists
cargo run -- run --problemset llms-and-knowledge --problem "new problem text"

# Read last run summary without running
cargo run -- read

# Reset state to seed
cargo run -- --fresh run

# Add a conjecture
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --text "Full text of the conjecture"
cargo run -- add-conjecture --layer perspectives --title "Conjecture Title" --file path/to/conjecture.md
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

**Phase 1 — Generate Candidates.** Each (problem, perspective conjecture) pair produces one candidate. The mind's conjecture summaries form the system prompt. The perspective conjecture's summary and problem summary are combined in the user prompt. Pairs run concurrently. Already-generated candidates are skipped (resumable runs).

**Phase 2 — Evaluate Candidates.** Two-pass evaluation:
- *Pass 1 — Logical Consistency:* Score 0.0–1.0. Below threshold (default 0.3), candidate is skipped.
- *Pass 2 — Hard to Vary:* The mind generates 10 yes/no questions probing whether each part of the candidate is load-bearing. Score = yes_count / 10. Combined score: `0.3 × consistency + 0.7 × hard_to_vary`.
- *Candidate Problems:* The mind also identifies unresolved tensions and open questions raised by each candidate. Candidates scoring above threshold are admitted to the active problem set.

**Phase 3 — Rank and Promote.**
- Conjecture scores update as rolling averages weighted by run_count. Composite score = `score × √(problem_coverage_breadth)`.
- Problem scores update as rolling averages of mean candidate score across all conjectures applied.
- *Problem review:* The mind receives all problem summaries in the set and removes duplicates and subsumed problems (removed from set membership, not global). The bottom-ranked problem with sufficient run history is also dropped from the set, enforcing the 10-problem cap.
- *Conjecture promotion:* Top perspective conjecture (by composite, min run_count) → mind. Top candidate → summarized into a new perspective conjecture.
- *Conjecture demotion:* Bottom mind conjecture → perspectives. Bottom perspective conjecture → discarded.

**Phase 4 — Report.** Ranked candidate table, top 5 summaries, all changes to conjectures and problems.

## Conceptual Hierarchy

All layers contain the same atom: a **Conjecture** — a unit of explanatory knowledge or perspective, held fallibly. The layers differ only in trust and stability, not in kind.

```
Mind          — most trusted conjectures, slowest to change
  ↑ promote / ↓ demote
Perspectives  — active conjectures under test, medium stability
  ↑ promote / ↓ demote
Candidates    — new conjectures generated each run, scored and ephemeral
```

Conjectures are shared across all problem sets — the mind and perspectives are not scoped to any one set.

**Problem Sets** are named, scoped collections of problems (cap: 10). Problems only exist within sets. A run operates on one problem set, discovers candidate problems, and adds them to the set. Deduplication and cap enforcement keep the set focused.

## State Layout

```
data/state/
  state.json                     — current run number
  mind/                          — mind conjectures (.md + .json each)
  perspectives/                  — perspective conjectures (.md + .json each)
  problems/                      — problems (.md + .json each)
  problemsets/                   — problem set index (.md + .json each)
  runs/
    001/
      {problem}-{conjecture}.md    — candidate content + questions
      {problem}-{conjecture}.json  — scores + candidate problems
      summary.md                   — ranked results + changes
```

Seed state lives in `data/seed/`. `--fresh` resets `data/state/` from seed.

## Seed State

**Mind conjectures:** Deutschian Epistemology, Ontology, Systems Thinking (Donella Meadows)

**Perspective conjectures:** Thought Experiments, Mathematical Formalism, Counterfactual Reasoning, Extreme Cases, Historical Genesis

## Documents

- [Epistemic Engine Spec](docs/epistemic-engine-spec.md) — Full design specification
- [Journal](docs/journal.md) — Intellectual lineage and architecture decisions
