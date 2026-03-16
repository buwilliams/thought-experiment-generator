# Thought Experiment Generator

A depth-bounded branching search over explanation space. Given a topic, the system draws structured knowledge fragments from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, and filters them for coherence and explanatory quality using Deutschian criteria. The novel pool compounds discoveries into future draws, making each pass better than the last. The [core argument](docs/llms-as-universal-explainer.md): LLMs can create new knowledge by randomly colliding objects, relationships, and properties, biased by background knowledge, and filtering the results for explanatory quality.

## Usage

```sh
# Default: uses claude-sonnet-4-6, requires ANTHROPIC_API_KEY env var
cargo run -- "your topic"

# Resume a previous run (cached automatically by topic)
cargo run -- "your topic"

# Start fresh, clearing cached background + tree state
cargo run -- --fresh "your topic"

# Use a different model
cargo run -- --model claude-haiku-4-5-20251001 "your topic"   # cheaper/faster
cargo run -- --model claude-opus-4-6 "your topic"             # highest quality

# OpenAI
OPENAI_API_KEY=sk-... cargo run -- --provider openai --model gpt-4o "your topic"

# Options (all optional, defaults shown)
cargo run -- "your topic" \
  --depth 10 --branches 10 --draws 100 \
  --threshold 0.7 --novel-threshold 0.85 \
  --max-calls 500 --max-concurrent 5 \
  --temperature 1.0 --output text
```

## Documents

- [Design Document](docs/thought-experiment-generator-design-doc.md) — Architecture spec, data structures, prompts, filter stack, build order
- [Rust Implementation](docs/rust-implementation.md) — Module layout, interfaces, concurrency model

## Inspiration

- [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) — Brett Hall on whether LLMs can create new knowledge, drawing on David Deutsch's epistemology
