# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

A self-improving epistemic engine. It collides perspective conjectures against problems to generate candidates, evaluates those candidates using Deutschian criteria, and feeds survivors back into its own conjecture pool. The core loop: `conjecture + problem → candidate → criticism → conjecture`. Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY`).

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
cargo run -- read                                                         # read last summary
cargo run -- --fresh run                                                  # reset to seed and run
RUST_LOG=debug cargo run -- run                                           # debug logging
cargo run -- add-conjecture --layer mind --title "Title" --text "..."
cargo run -- add-conjecture --layer perspectives --title "Title" --file path/to/file.md
```

## Architecture

**Seven source modules** (`src/`):

- `runner.rs` — orchestrates four phases: (1+2) concurrent candidate generation + evaluation per (problem, perspective conjecture) pair, (3) rank/promote/review problems, (4) report. Uses `tokio::spawn` bounded by `--max-concurrent`.
- `state.rs` — all filesystem I/O. Reads/writes `.md` + `.json` sidecar pairs for conjectures, problems, and candidates. `ensure_initialized()` copies seed to state on first run. Key functions: `load_conjectures`/`save_conjecture`/`delete_conjecture` for mind/perspectives; `save_candidate`/`load_run_candidates`/`candidate_exists` for ephemeral run output.
- `evaluator.rs` — two-pass evaluation: logical consistency (score < threshold → skip), then hard-to-vary (10 yes/no questions, score = yes_count/10). Also calls `extract_candidate_problems`.
- `promoter.rs` — ranking and promotion logic. `composite(conjecture)` = `score × √(problem_coverage_breadth)`. Finds top/bottom conjectures and problems for promotion/demotion/discard.
- `prompts.rs` — all prompt templates. `format_mind_system` serializes mind conjecture summaries into the system prompt. All prompts use summaries, never full text.
- `llm/` — `LlmClient` wrapping Anthropic and OpenAI APIs. `call_raw` returns plain text; `call::<T>` deserializes JSON.
- `types.rs` — all structs. `Conjecture`/`ConjectureMeta` (mind/perspectives layer, stable), `Candidate`/`CandidateMeta` (ephemeral, generated each run). Both have a `meta` (serialized to `.json`) plus content fields (stored in `.md`). LLM response types: `ConsistencyResponse`, `QuestionsResponse`, `AnswersResponse`, `CandidatesResponse`, `PromoteResponse`, `SummaryResponse`, `DeduplicateResponse`.

**State layout:** `data/state/` with `mind/`, `perspectives/`, `problems/`, `problemsets/`, `runs/NNN/`. Each entity is a `.md` + `.json` sidecar. Seed state in `data/seed/`.

**Problem sets:** A run operates on one problem set (selected by `--problemset <id>` or auto-selected if only one exists). Sets are capped at 10 problems. Problems are stored in `problems/` and referenced by ID in `problemsets/{id}.json`. Conjectures (mind/perspectives) are shared across all sets. After each run: candidate problems are admitted to the set, the mind deduplicates within the set (removes from membership, not the problem files), and cap enforcement drops bottom-ranked (min run_count guard) until ≤ 10.

**Evaluation scoring:** `0.3 × logical_consistency + 0.7 × hard_to_vary`. Conjecture composite: `score × √(problem_coverage.len())`. Problems ranked by mean candidate score (rolling average).

## Design Documents

- `docs/epistemic-engine-spec.md` — authoritative design spec (prompts, data structures, phase descriptions, CLI)
- `docs/journal.md` — intellectual lineage and architecture decisions
