# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

A self-improving epistemic engine. It collides perspective tools against problems to generate conjectures, evaluates those conjectures using Deutschian criteria, and feeds survivors back into its own tool pool. The core loop: `tools + problem → conjecture → criticism → tool`. Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY`).

## Commands

```sh
cargo build                                                         # build
cargo run -- create-problemset "description..."                     # create a problem set (ID = sha256 hash)
cargo run -- add-problem --problemset <id> --text "..."             # add problem to set
cargo run -- remove-problem --problemset <id> --problem-id <id>     # remove from set
cargo run -- list-problemsets                                        # list all sets
cargo run -- run --problemset <id>                                   # run on a set
cargo run -- run                                                     # run (if only one set)
cargo run -- run --problemset <id> --problem "..."                  # add problem then run
cargo run -- read                                                    # read last summary
cargo run -- --fresh run                                             # reset to seed and run
RUST_LOG=debug cargo run -- run                                      # debug logging
cargo run -- add-tool --layer mind --title "Title" --text "..."
cargo run -- add-tool --layer perspectives --title "Title" --file path/to/file.md
```

## Architecture

**Seven source modules** (`src/`):

- `runner.rs` — orchestrates four phases: (1+2) concurrent conjecture generation + evaluation per (problem, tool) pair, (3) rank/promote/review problems, (4) report. Uses `tokio::spawn` bounded by `--max-concurrent`.
- `state.rs` — all filesystem I/O. Reads/writes `.md` + `.json` sidecar pairs for tools, problems, and conjectures. `ensure_initialized()` copies seed to state on first run.
- `evaluator.rs` — two-pass evaluation: logical consistency (score < threshold → skip), then hard-to-vary (10 yes/no questions, score = yes_count/10). Also calls `extract_candidate_problems`.
- `promoter.rs` — ranking and promotion logic. `composite(tool)` = `score × √(problem_coverage_breadth)`. Finds top/bottom tools and problems for promotion/demotion/discard.
- `prompts.rs` — all prompt templates. `format_mind_system` serializes mind tool summaries into the system prompt. All prompts use summaries, never full text.
- `llm/` — `LlmClient` wrapping Anthropic and OpenAI APIs. `call_raw` returns plain text; `call::<T>` deserializes JSON.
- `types.rs` — all structs. `Tool`, `Problem`, `Conjecture` have a `meta` (serialized to `.json`) plus content fields (stored in `.md`). LLM response types: `ConsistencyResponse`, `QuestionsResponse`, `AnswersResponse`, `CandidatesResponse`, `ToolSummaryResponse`, `SummarizeToolResponse`, `DeduplicateResponse`.

**State layout:** `data/state/` with `mind/`, `perspectives/`, `problems/`, `problemsets/`, `runs/NNN/`. Each entity is a `.md` + `.json` sidecar. Seed state in `data/seed/`.

**Problem sets:** A run operates on one problem set (selected by `--problemset <id>` or auto-selected if only one exists). Sets are capped at 10 problems. Problems are stored globally in `problems/` and referenced by ID in `problemsets/{id}.json`. Tools are shared across all sets. After each run: candidates are admitted to the set, the mind deduplicates within the set (removes from membership, not global DB), and cap enforcement drops bottom-ranked (min run_count guard) until ≤ 10. `ProblemSet`/`ProblemSetMeta` in `types.rs`; all set I/O in `state.rs`.

**Evaluation scoring:** `0.3 × logical_consistency + 0.7 × hard_to_vary`. Tool composite: `score × √(problem_coverage.len())`. Problems ranked by mean conjecture score (rolling average).

## Design Documents

- `docs/epistemic-engine-spec.md` — authoritative design spec (prompts, data structures, phase descriptions, CLI)
- `docs/journal.md` — intellectual lineage and architecture decisions
