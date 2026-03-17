# Thought Experiment Generator

Can LLMs create new knowledge? This system tries to find out. Give it a topic and it generates thought experiments by colliding real background knowledge with sentences built from random words, then scores each one for explanatory quality using the principles of fallibilism. The result is a ranked set of thought experiments that neither you nor the LLM started with.

The core idea: take random background knowledge sentences and random generated sentences (built from random words) and use both as inspiration to explore new knowledge. Inspired by [Brett Hall's discussion](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) on whether LLMs can create new knowledge, drawing on David Deutsch's epistemology.

## Usage

```sh
# Default: uses claude-sonnet-4-6, requires ANTHROPIC_API_KEY env var
cargo run -- "your topic"

# Resume a previous run (cached automatically by topic)
cargo run -- "your topic"

# Read cached results and display summary without running
cargo run -- --read "your topic"

# Start fresh, clearing the cache
cargo run -- --fresh "your topic"

# Use a different model
cargo run -- --model claude-haiku-4-5-20251001 "your topic"   # cheaper/faster
cargo run -- --model claude-opus-4-6 "your topic"             # highest quality

# OpenAI
OPENAI_API_KEY=sk-... cargo run -- --provider openai --model gpt-4o "your topic"

# Claude Max subscription (session token auth)
cargo run -- --provider anthropic-token "your topic"

# All options (defaults shown)
cargo run -- "your topic" \
  --experiments 20 \
  --pool-size 50 \
  --background 3 \
  --generated 2 \
  --words 5 \
  --temperature 1.0 \
  --max-concurrent 5
```

### Options

| Flag | Default | Description |
|---|---|---|
| `--experiments` | 20 | Number of thought experiments to generate |
| `--pool-size` | 50 | Total sentences in the background and generated pools |
| `--background` | 3 | Background sentences to use per thought experiment |
| `--generated` | 2 | Generated sentences to use per thought experiment |
| `--words` | 5 | Random words per line in words.txt |
| `--temperature` | 1.0 | LLM temperature for generation (0.0–1.0) |
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--fresh` | false | Clear cache and start over |
| `--read` | false | Show cached summary without running |

### LLM call budget

Background and generated pools are cached after first run. Each subsequent run (same topic, same pool size) skips the Create phase entirely.

| Phase | Calls |
|---|---|
| Create: background pool | 1 |
| Create: words → sentences | 1 |
| Combine: one thought experiment | 1 |
| Criticize: one thought experiment | 1 |
| Results: top 5 summaries | 5 |
| **Total for default run (20 experiments, first run)** | **~47** |
| **Total for re-run (pools cached)** | **~45** |

## How It Works

**1. Create background sentences.** The LLM generates a pool of sentences of fundamental knowledge about your topic (default: 50). These are cached in `data/cache/[hash]/background.txt`.

**2. Create generated sentences.** Random words are drawn from a large word list and grouped into lines (default: 5 words per line, 50 lines). The LLM converts each line into a sentence as though it were fundamental knowledge, using those words. These nonsense-seeded sentences force the system into territory that pure topic knowledge would never reach. Cached in `words.txt` and `generated.txt`.

**3. Combine into thought experiments.** For each experiment, the system picks a random sample of background sentences and generated sentences (default: 3 background, 2 generated), then asks the LLM to rewrite the collection as a single thought experiment in 500 words or fewer. Experiments must be wild, imaginative, unintuitive, or combine ideas in novel ways. Each is saved as `NNN-experiment.txt`.

**4. Criticize using fallibilism.** Each thought experiment is scored on three criteria using the principles of fallibilism:
- **Reach** — does the thought experiment break beyond the existing corpus of human knowledge into new territory?
- **Novelty** — does it contribute new understanding?
- **Falsifiable** — is it testable or disprovable?

Each dimension is scored 0.0–1.0. Scores are saved as `NNN-experiment-criticize.json`.

**5. Rank and summarize.** Results are sorted by total score (reach + novelty + falsifiable, max 3.0). The top 5 are summarized in 20 words each. Everything is written to `summary.txt`.

## Output Files

All output lives in `data/cache/[topic-hash]/`:

```
background.txt                   — sentence pool about the topic
words.txt                        — random word pairs
generated.txt                    — sentences generated from word pairs
001-experiment.txt               — thought experiment #1
001-experiment-criticize.json    — scores for experiment #1
...
summary.txt                      — ranked table + top 5 summaries
```

## Documents

- [Design Document](docs/thought-experiment-generator-design-doc.md) — Architecture, prompts, data structures
- [LLMs as Universal Explainer](docs/llms-as-universal-explainer.md) — Motivation and theoretical argument
