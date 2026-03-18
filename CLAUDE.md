# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

The Thought Experiment Generator (`teg`) is a Rust CLI tool that explores a topic by colliding topic-derived background sentences with LLM-generated strange sentences (seeded from random words), then scores each combination for explanatory novelty using Deutschian criteria (reach, novelty, falsifiability). Requires `ANTHROPIC_API_KEY` (or `OPENAI_API_KEY` for OpenAI).

## Commands

```sh
cargo build                          # build
cargo run -- "your topic"            # run (prompts for topic if omitted)
cargo run -- --read "your topic"     # read cached summary without running
cargo run -- --fresh "your topic"    # clear cache and start fresh
RUST_LOG=debug cargo run -- "topic"  # enable debug logging
```

All output is cached in `data/cache/[topic-hash]/`. Re-running the same topic resumes where it left off.

## Architecture

**Five source modules** (`src/`):

- `runner.rs` — orchestrates the four phases: Create (background + words + generated pools) → Combine (pick sentences, call LLM) → Criticize (score each TE) → Results (rank + summarize). Combine/Criticize run concurrently via `tokio::spawn`, bounded by `--max-concurrent`.
- `cache.rs` — all file I/O. Derives a cache dir from `sha256(topic)`, stores `background.txt`, `words.txt`, `generated.txt`, `experiments/NNN-experiment.txt`, `experiments/NNN-experiment-criticize.json`, `summary.md`. Cache hits skip LLM calls.
- `prompts.rs` — prompt templates for each LLM call: `background_generation`, `words_to_sentences`, `te_generation`, `criticism`, `summarize`.
- `llm/` — `LlmClient` wrapping Anthropic and OpenAI APIs. `call_raw` returns plain text; `call` deserializes JSON into typed structs. Tracks call count via `AtomicUsize`.
- `types.rs` — `Critique` struct (reach, novelty, falsifiable, each 0.0–1.0, plus explanations). `total_score()` sums all three.
- `config.rs` — validated config from CLI args (pool sizes, temperature, provider/model).

**Data flow per experiment:** random sample from background pool + generated pool → `te_generation` prompt → raw text saved → `criticism` prompt → `Critique` JSON saved.

**Word list:** `data/words_alpha.txt` (~17k curated English words). Used to generate random word groups that seed the "generated" sentence pool.

## Design Documents

The `docs/` folder contains the original design essays (motivations, theoretical arguments). The actual implementation diverges from the design doc — it uses a simpler two-pool flat structure rather than the three-pool tree described there.
