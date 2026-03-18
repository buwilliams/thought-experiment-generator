# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

A self-improving epistemic engine. It collides candidate conjectures against problems to generate outputs, evaluates those outputs using Deutschian criteria, and feeds survivors back into its own conjecture pool. The core loop: `problem + conjecture → generated output → criticism → conjecture`. Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY`).

## Commands

```sh
cargo build                                                              # build
cargo run -- create-problemset "description..."                          # create a problem set (ID = sha256 hash)
cargo run -- add-problem --problemset <id> --text "..."                  # add problem to set
cargo run -- remove-problem --problemset <id> --problem-id <id>          # remove from set
cargo run -- list-problemsets                                             # list all sets
cargo run -- run --problemset <id>                                        # run on a set
cargo run -- run                                                          # run (if only one set)
cargo run -- run --problemset <id> --problem "..."                        # add problem then run
cargo run -- run-all                                                      # run all problem sets sequentially
cargo run -- ask "question"                                               # ask the system; all lenses run concurrently, then consolidated
cargo run -- read                                                         # read last summary
cargo run -- --fresh run                                                  # reset to seed and run
RUST_LOG=debug cargo run -- run                                           # debug logging
cargo run -- add-conjecture --layer mind --title "Title" --text "..."
cargo run -- add-conjecture --layer candidates --title "Title" --file path/to/file.md
```

## Architecture

**Seven source modules** (`src/`):

- `runner.rs` — orchestrates four phases: (1+2) concurrent output generation + evaluation per (problem, candidate conjecture) pair, (3) rank/promote/review problems, (4) report. Uses `tokio::spawn` bounded by `--max-concurrent`.
- `state.rs` — all filesystem I/O. Reads/writes `.md` + `.json` sidecar pairs for conjectures and generated outputs; problemsets store problems inline in their `.json`. `ensure_initialized()` copies seed to state on first run. Key functions: `load_conjectures`/`save_conjecture`/`delete_conjecture` for mind/candidates; `save_generated`/`load_run_generated`/`generated_exists` for ephemeral run output.
- `evaluator.rs` — two-pass evaluation: logical consistency (score < threshold → skip), then hard-to-vary (10 yes/no questions, score = yes_count/10). Also calls `extract_candidate_problems`.
- `promoter.rs` — ranking and promotion logic. `composite(conjecture)` = `score × √(problem_coverage_breadth)`. Finds top/bottom conjectures and problems for promotion/demotion/discard.
- `ask.rs` — `ask()` loads mind + candidates, runs all mind conjectures and the top 3 candidates concurrently as separate lenses via `ask.md`, then consolidates all perspectives into a single best explanation via `ask_consolidate.md`. Prints the synthesis followed by the list of lenses used.
- `prompts.rs` — `PromptTemplates` struct loaded from `data/prompts/*.md` at startup. `load()` reads all template files; each method applies `{{variable}}` substitution and returns a `Prompt { system, user }`. `format_mind_system` builds the system prompt from mind conjecture summaries. All prompts use summaries, never full text.
- `llm/` — `LlmClient` wrapping Anthropic and OpenAI APIs. `call_raw` returns plain text; `call::<T>` deserializes JSON.
- `types.rs` — all structs. `Conjecture`/`ConjectureMeta` (mind/candidates layer, stable), `Generated`/`GeneratedMeta` (ephemeral, produced each run). Both have a `meta` (serialized to `.json`) plus content fields (stored in `.md`). LLM response types: `ConsistencyResponse`, `QuestionsResponse`, `AnswersResponse`, `CandidatesResponse`, `PromoteResponse`, `SummaryResponse`, `DeduplicateResponse`.

**Prompt templates:** `data/prompts/` contains one `.md` file per prompt (e.g. `generate_questions.md`). Each file has `## System` and `## User` sections with `{{variable}}` placeholders. Edit these to tune LLM behavior without touching Rust code. `mind_system.md` has `## Header`, `## Item`, and `## Empty` sections used to build the system prompt from mind conjectures.

**State layout:** `data/state/` with `mind/`, `candidates/`, `evaluations/`, `problemsets/`, `runs/NNN/`. Conjectures are `.md` + `.json` sidecar pairs. Problemsets are also `.md` + `.json` pairs, with problems embedded directly in the `.json`. Seed state in `data/seed/`.

**Problem sets:** A run operates on one problem set (selected by `--problemset <id>` or auto-selected if only one exists). Sets are capped at 10 problems. Problems are embedded directly in `problemsets/{id}.json` — no separate `problems/` directory. Conjectures (mind/candidates) are shared across all sets. After each run: candidate problems are admitted, deduplication removes near-duplicates, and cap enforcement drops bottom-ranked (min run_count guard) until ≤ 10.

**Evaluations:** Scoring criteria loaded from `data/state/evaluations/` at the start of Phase 2. Seed: Logical Consistency (weight 0.3), Hard to Vary (weight 0.7). Combined score = normalized weighted sum. The system never writes to `evaluations/` — users manage this bucket manually. Conjecture composite: `score × √(problem_coverage.len())`. Problems ranked by mean generated output score (rolling average).

## Design Documents

- `docs/epistemic-engine-spec.md` — authoritative design spec (prompts, data structures, phase descriptions, CLI)
- `docs/journal.md` — intellectual lineage and architecture decisions
