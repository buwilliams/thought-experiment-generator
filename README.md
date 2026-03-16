# Thought Experiment Generator

A depth-bounded branching search over explanation space. Given a topic, the system draws structured knowledge fragments from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, and filters them for coherence and explanatory quality using Deutschian criteria. The novel pool compounds discoveries into future draws, making each pass better than the last. The [core argument](docs/llms-as-universal-explainer.md): LLMs can create new knowledge by randomly colliding objects, relationships, and properties, biased by background knowledge, and filtering the results for explanatory quality. Inspired by [Brett Hall's discussion](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) on whether LLMs can create new knowledge, drawing on David Deutsch's epistemology.

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
  --threshold 0.6 --novel-threshold 0.85 \
  --max-calls 500 --max-concurrent 5 \
  --temperature 1.0 --output text
```

### LLM call budget

Each run costs LLM calls. Background init and vocabulary are cached after the first run, so re-runs and resumes are cheaper.

| Goal | Estimated calls |
|---|---|
| One branch to full depth (10), scored | ~100-135 |
| Full run (10 branches, depth 10, cross-pollination) | ~900-1100 |

The default `--max-calls 500` gets you through setup, roots, and ~4-5 full branches. Run the same topic again to resume and spend more budget.

## How It Works

**1. Background init** (2 LLM calls, cached). The LLM generates 50 facts about your topic, then extracts structured knowledge fragments called quads (object, relationship, object, property) from those facts.

**2. Three-pool draw system.** Every draw samples from three pools:
- **Background** -- quads from the topic's facts (what is known)
- **Universal** -- ~50k domain-agnostic vocabulary terms (what is possible)
- **Novel** -- quads earned by high-scoring discoveries during the run (what has been found)

The novel pool starts empty and grows as the tree runs, shifting the ratio from 50/50/0 toward 30/30/40. This is what makes the tree compound its own discoveries.

**3. Atom structure.** Each thought experiment is generated from a 4/3/2 draw: 4 objects, 3 relationships, 2 properties. Half drawn from background/novel, half from universal. Derived empirically from Einstein's journal entries.

**4. Filter stack** (cheap to expensive, per draw):
- Grammar check (no LLM) -- is this valid language?
- Coherence filter (1 LLM call) -- internally consistent and non-trivially related to topic?
- Deutsch scorer (1 LLM call) -- hard to vary, reaches beyond inputs, minimal assumptions, resolves tension?
- Survivor threshold -- score >= 0.6?

**5. Tree search.** 10 root branches, each running to depth 10. At each depth, the system draws, generates, and filters until a survivor is found (up to 100 attempts). Low scores don't trigger pruning. Only depth limit, circularity, or vocabulary exhaustion terminate a branch.

**6. Cross-pollination.** After branches complete, the system checks pairs for complementary unresolved tensions. Complementary pairs merge into new branches that neither could reach alone.

**7. Trajectory scoring.** Each completed branch is scored on cumulative explanatory reach. Results are ranked and displayed.

## Documents

- [Design Document](docs/thought-experiment-generator-design-doc.md) -- Architecture spec, data structures, prompts, filter stack, build order
- [Rust Implementation](docs/rust-implementation.md) -- Module layout, interfaces, concurrency model
