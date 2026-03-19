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

This system explores a complementary direction to scaling: rather than improving the substrate, it organizes reasoning at a higher level. Purpose-built agent systems — research agents, scientific automation pipelines, reasoning models — show that structure compounds what substrate alone cannot. This system is the generalization of that pattern: a domain-agnostic architecture for how knowledge grows through conjecture and criticism, operating in the abstract space where explanations live.

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

# Run on all problem sets that have not yet been run (skips already-processed sets)
cargo run -- run-all
cargo run -- --fresh run-all                    # reset to seed, then run all

# Ask the system a question — answered by running all mind conjectures + top candidates
# concurrently as separate lenses, then consolidating into a single best explanation
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
| `--consistency-threshold` | 0.3 | Minimum logical consistency score; outputs below threshold are zeroed |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter the active set |
| `--min-run-count` | 3 | Minimum runs before a conjecture or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--temperature` | 0.9 | LLM temperature |
| `--fresh` | false | Reset state to seed |

## How It Works

The system maintains a **mind** — a small set of **conjectures** that represent its most trusted explanatory lenses. A conjecture is any unit of explanatory knowledge held fallibly: a perspective, a framework, a way of seeing. The mind's conjectures serve as context for every operation — they form the system prompt for all LLM calls.

Alongside the mind, the system holds a pool of **candidate conjectures**: active ideas under test, less trusted than the mind but ready to be promoted into it. You also provide a **problem set** — a scoped collection of up to 10 questions you want the system to explore. Each run operates on one problem set.

**Phase 1+2 — Generate and Evaluate.** The system iterates over all **lenses** — every candidate conjecture plus every mind conjecture. For each lens, all problem outputs are generated concurrently, then evaluated in a single comparative call that scores all four dimensions at once. The LLM sees all outputs for a lens together, producing calibrated scores across the batch rather than in isolation. Already-generated outputs are skipped, so runs are resumable. Mind conjectures participate as lenses alongside candidates — they are tested directly, not just used as background context.

Evaluation scores four dimensions under a fallibilist frame — the goal is to find falsifying instances, not confirm:

- **Logical Consistency** (weight 0.20) — gate: outputs below threshold have remaining scores zeroed
- **Hard to Vary** (weight 0.35) — are the output's claims load-bearing? Substitution tests on named elements; generic language that could describe almost anything scores near 0
- **Explanatory Reach** (weight 0.30) — does the output commit to specific claims in adjacent cases that could independently fail? Generic assertions of breadth score near 0
- **Resistance to Refutation** (weight 0.15) — attempt to construct a specific falsifying case; vagueness scores near 0 (not robust — empty)

Combined score = `0.20 × consistency + 0.35 × htv + 0.30 × reach + 0.15 × refutation`. The same call also extracts candidate problems: unresolved tensions specific enough that a subsequent conjecture could be wrong about them.

**Phase 3 — Rank and Promote.** Scores feed back into the conjecture pool. The mind operates with built-in resistance — not rigidity: composite score = `score × √(coverage) × 1.5` for mind conjectures vs `× 1.0` for candidates; new runs update mind scores at half weight (inertia); mind conjectures require twice the run history before demotion is eligible. The top-ranked candidate (by composite) is promoted to mind; the top-ranked generated output becomes a new candidate, with a slight preference for outputs generated by mind lenses. The bottom mind conjecture is demoted; the bottom candidate is discarded. Candidate problems are admitted if the set has capacity (cap enforced at admission — new problems blocked when at 10, not by ejecting existing ones), then deduplicated. Over many runs, what survives is whatever produces the highest-quality explanations.

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
      {problem}-{conjecture}.md    — generated output content
      {problem}-{conjecture}.json  — scores + candidate problems
      summary.md                   — ranked results + changes
  review.md                      — latest full review (markdown with links)
```

Seed state lives in `data/seed/`. `--fresh` resets `data/state/` from seed. The seed is runnable out of the box — `cargo run -- --fresh run` starts a full cycle with no setup. The seed problem set uses the static ID `default`; problem sets you create via `create-problemset` get an ID equal to the first 8 characters of the sha256 of their content, printed on creation and shown by `list-problemsets` as `[id]`.

## Documents

- [Epistemic Engine Spec](docs/epistemic-engine-spec.md) — Full design specification
- [Journal](docs/journal.md) — Intellectual lineage and architecture decisions
