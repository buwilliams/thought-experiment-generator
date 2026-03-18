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

# Run on all problem sets sequentially
cargo run -- run-all

# Ask the system a question — answered by running all mind conjectures + top candidates
# concurrently as separate lenses, then consolidating into a single best explanation
cargo run -- ask "What causes institutional decay?"
cargo run -- ask --file question.md
cat question.md | cargo run -- ask

# Check whether conjectures are novel or restatements of known theories
cargo run -- novelty-check

# Show score trajectory across all runs and per-conjecture score history
cargo run -- trajectory

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

The system maintains a **mind** — a small set of **conjectures** that represent its most trusted explanatory lenses. A conjecture is any unit of explanatory knowledge held fallibly: a perspective, a framework, a way of seeing. The mind's conjectures serve as context for every operation — they form the system prompt for all LLM calls.

Alongside the mind, the system holds a pool of **candidate conjectures**: active ideas under test, less trusted than the mind but ready to be promoted into it. You also provide a **problem set** — a scoped collection of up to 10 questions you want the system to explore. Each run operates on one problem set.

**Phase 1 — Generate.** Each (problem, candidate conjecture) pair is collided: the mind provides context, the candidate provides the lens, and the LLM produces a **generated output** — a structured thought experiment exploring what happens when that lens meets that problem. All pairs run concurrently. Already-generated outputs are skipped, so runs are resumable.

**Phase 2 — Evaluate.** Each output is scored against a set of **evaluation criteria** loaded from `data/state/evaluations/`. The two seed criteria are *Logical Consistency* (is the output internally self-consistent? outputs below threshold are discarded) and *Hard to Vary* (the mind generates 10 yes/no questions probing whether each part of the output is load-bearing; score = yes_count / 10). Combined score is a normalized weighted sum. Evaluation criteria are manually managed — the system never modifies them. You extend or refine them by editing `data/state/evaluations/` directly. The mind also scans each output for unresolved tensions worth pursuing as new problems; those above a score threshold are admitted to the active problem set.

**Phase 3 — Rank and Promote.** Scores feed back into the conjecture pool. The top-ranked candidate conjecture (scored by `score × √problem_coverage`, with a minimum run history) is promoted into the mind. The top-ranked generated output is summarized and added to candidates. The bottom mind conjecture is demoted to candidates; the bottom candidate is discarded. Problems are deduplicated and the lowest-ranked (with sufficient run history) is dropped if the set exceeds 10. Over many runs, what survives is whatever produces the highest-quality explanations.

**Phase 4 — Report.** A ranked output table, top 5 summaries, and a full log of promotions, demotions, and problem changes is written to `data/state/runs/NNN/summary.md`.

## State Layout

```
data/state/
  state.json                     — current run number
  mind/                          — mind conjectures (.md + .json each)
  candidates/                    — candidate conjectures (.md + .json each)
  evaluations/                   — evaluation criteria (.md + .json each) [manually managed]
  problemsets/                   — problem sets (.md scope + .json with embedded problems)
  runs/
    001/
      {problem}-{conjecture}.md    — generated output content + questions
      {problem}-{conjecture}.json  — scores + candidate problems
      summary.md                   — ranked results + changes
```

Seed state lives in `data/seed/`. `--fresh` resets `data/state/` from seed. The seed is runnable out of the box — `cargo run -- --fresh run` starts a full cycle with no setup. The seed problem set uses the static ID `default`; problem sets you create via `create-problemset` get an ID equal to the first 8 characters of the sha256 of their content, printed on creation and shown by `list-problemsets` as `[id]`.

## Documents

- [Epistemic Engine Spec](docs/epistemic-engine-spec.md) — Full design specification
- [Journal](docs/journal.md) — Intellectual lineage and architecture decisions
