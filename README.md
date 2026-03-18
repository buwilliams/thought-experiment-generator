# Thought Experiment Generator

A self-improving epistemic engine. It collides perspective tools against problems to generate conjectures, evaluates those conjectures for explanatory quality, and feeds the survivors back into its own tool pool. Over many runs, the system steers toward whatever generates genuine insight.

The core loop:

```
tools + problem → conjecture → criticism → tool
```

Inspired by David Deutsch's epistemology — knowledge grows through conjecture and criticism, not random search. The system narrows the search space by compounding success: only conjectures that survive criticism become tools, and only tools that generate high-scoring conjectures survive long enough to be promoted.

Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY` for OpenAI).

## Usage

```sh
# Create a problem set
cargo run -- create-problemset --title "LLMs and Knowledge"
cargo run -- create-problemset --title "LLMs and Knowledge" --description "A short description"

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

# Add a tool
cargo run -- add-tool --layer mind --title "Tool Title" --text "Full text of the tool"
cargo run -- add-tool --layer perspectives --title "Tool Title" --file path/to/tool.md
cat my-tool.md | cargo run -- add-tool --layer mind --title "Tool Title"
```

### Options

| Flag | Default | Description |
|---|---|---|
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--consistency-threshold` | 0.3 | Minimum logical consistency score to proceed to Pass 2 |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter the active set |
| `--min-run-count` | 3 | Minimum runs before a tool or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--temperature` | 0.9 | LLM temperature |
| `--fresh` | false | Reset state to seed |

## How It Works

**Phase 1 — Generate Conjectures.** Each (problem, perspective tool) pair produces one conjecture. The mind's tool summaries form the system prompt. The perspective tool's summary and problem summary are combined in the user prompt. Pairs run concurrently. Already-generated conjectures are skipped (resumable runs).

**Phase 2 — Evaluate Conjectures.** Two-pass evaluation:
- *Pass 1 — Logical Consistency:* Score 0.0–1.0. Below threshold (default 0.3), conjecture is skipped.
- *Pass 2 — Hard to Vary:* The mind generates 10 yes/no questions probing whether each part of the conjecture is load-bearing. Score = yes_count / 10. Combined score: `0.3 × consistency + 0.7 × hard_to_vary`.
- *Candidate Problems:* The mind also identifies unresolved tensions and open questions raised by each conjecture. Candidates scoring above threshold are admitted to the active problem set.

**Phase 3 — Rank and Promote.**
- Tool scores update as rolling averages weighted by run_count. Composite score = `score × √(problem_coverage_breadth)`.
- Problem scores update as rolling averages of mean conjecture score across all tools applied.
- *Problem review:* The mind receives all problem summaries in the set and removes duplicates and subsumed problems (removed from set membership, not the global store). The bottom-ranked problem with sufficient run history is also dropped from the set, enforcing the 10-problem cap.
- *Tool promotion:* Top perspective tool (by composite, min run_count) → mind. Top conjecture → summarized into a new perspective tool.
- *Tool demotion:* Bottom mind tool → perspectives. Bottom perspective tool → discarded.

**Phase 4 — Report.** Ranked conjecture table, top 5 summaries, all changes to tools and problems.

## Conceptual Hierarchy

All layers contain the same atom: a **Tool** — a unit of perspective expressed as plain text. Tools are shared across all problem sets.

```
Mind          — most trusted, slowest to change
  ↑ promote / ↓ demote
Perspectives  — active tool database, medium stability
  ↑ promote / ↓ demote
Conjectures   — generated each run, ephemeral
```

**Problem Sets** are named collections of related problems, capped at 10. A run operates on one problem set. Problems are stored globally and referenced by ID — a problem can belong to multiple sets. Each run discovers candidate problems and adds them to the active set, then deduplication and cap enforcement keep the set focused.

## State Layout

```
data/state/
  state.json             — current run number
  mind/                  — mind tools (.md + .json each)
  perspectives/          — perspective tools (.md + .json each)
  problems/              — global problem store (.md + .json each)
  problemsets/           — problem set index (.md + .json each)
  runs/
    001/
      {problem}-{tool}.md    — conjecture content + questions
      {problem}-{tool}.json  — scores + candidate problems
      summary.md             — ranked results + changes
```

Seed state lives in `data/seed/`. `--fresh` resets `data/state/` from seed.

## Seed State

**Mind tools:** Deutschian Epistemology, Ontology, Systems Thinking (Donella Meadows)

**Perspective tools:** Thought Experiments, Mathematical Formalism, Counterfactual Reasoning, Extreme Cases, Historical Genesis

## Documents

- [Epistemic Engine Spec](docs/epistemic-engine-spec.md) — Full design specification
- [Journal](docs/journal.md) — Intellectual lineage and architecture decisions
