# Thought Experiment Generator

A depth-bounded branching search over explanation space. Given a topic, the system draws structured knowledge fragments from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, and filters them for coherence and explanatory quality using Deutschian criteria. The novel pool compounds discoveries into future draws, making each pass better than the last. The [core argument](docs/llms-as-universal-explainer.md): LLMs can create new knowledge through informed guessing and iterative filtering.

## Usage

```
cargo run -- "why does light always travel at the same speed"
```

### Authentication

**Anthropic API key:**
```
ANTHROPIC_API_KEY=sk-... cargo run -- "your topic"
```

**Claude Max subscription (session token):**
```
ANTHROPIC_TOKEN=... cargo run -- --provider anthropic-token "your topic"
```

**OpenAI:**
```
OPENAI_API_KEY=sk-... cargo run -- --provider openai --model gpt-4o "your topic"
```

### Options

```
--depth             Depth limit per branch (default: 10)
--branches          Number of root branches (default: 10)
--threshold         Survivor score threshold (default: 0.7)
--novel-threshold   Novel pool admission threshold (default: 0.85)
--draws             Max draws per depth (default: 100)
--temperature       LLM generation temperature (default: 1.2)
--max-concurrent    Parallel LLM calls (default: 5)
--output            "text" or "json" (default: text)
```

## Documents

- [Design Document](docs/thought-experiment-generator-design-doc.md) — Architecture spec, data structures, prompts, filter stack, build order
- [Rust Implementation](docs/rust-implementation.md) — Module layout, interfaces, concurrency model

## Inspiration

- [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) — Brett Hall on whether LLMs can create new knowledge, drawing on David Deutsch's epistemology
