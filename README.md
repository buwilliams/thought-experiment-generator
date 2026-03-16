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

Each run costs LLM calls. Background init (2 calls) and vocabulary (3 calls) are cached after the first run, so re-runs and resumes are cheaper.

| Goal | Estimated calls |
|---|---|
| One branch to full depth (10), scored | ~100-135 |
| Full run (10 branches, depth 10, cross-pollination) | ~900-1100 |

The default `--max-calls 500` gets you through setup, roots, and ~4-5 full branches. Run the same topic again to resume and spend more budget.

## How It Works

**1. Learn about the topic.** The LLM generates 50 facts about your topic and extracts structured knowledge fragments called quads: `(object, relationship, object, property)`. For example, a topic about budgeting might produce `("50/30/20 rule", "was popularized by", "Elizabeth Warren", "credibility")`. These form the **background pool**. The system also loads a cross-domain vocabulary of objects, relationships, and properties that have nothing to do with your topic, like `("entropy", "resists", "order", "stability")`. These form the **universal pool**.

**2. Generate a thought experiment.** The system randomly draws 4 objects, 3 relationships, and 2 properties from both pools, half from each. It sends these elements plus the topic to the LLM, which writes a thought experiment that collides them into a novel scenario. The randomness from the universal pool forces surprising combinations that pure topic knowledge would never produce.

**3. Filter for quality.** Each thought experiment passes through a series of checks. First, a grammar check (no LLM call). Then the LLM judges coherence: is it internally consistent and non-trivially related to the topic? Then the LLM scores it on Deutsch criteria: is it hard to vary, does it reach beyond its inputs, does it use minimal assumptions, does it resolve a tension? If the score is below 0.6, it's discarded and a new draw is attempted (up to 100 times).

**4. Extract what's unresolved.** When a thought experiment survives, the LLM identifies what remains unexplained or paradoxical. This unresolved tension becomes the starting point for the next thought experiment in the chain.

**5. Deepen the chain.** Steps 2-4 repeat, but now the LLM sees the full chain so far and the latest unresolved tension. Each new thought experiment builds on what came before, the way Einstein's train-and-lightning-bolts built on his earlier frozen-light-wave scenario. A single chain running 10 levels deep is called a **branch**.

**6. Run many branches in parallel.** The system starts 10 branches from different root thought experiments and runs them all to depth 10. High-scoring discoveries from any branch get added to a **novel pool**, which feeds into future draws across all branches. This is what makes the system compound its own discoveries.

**7. Cross-pollinate.** After branches complete, the system checks pairs for complementary unresolved tensions. If two branches are stuck on tensions that would resolve each other, they merge into a new branch that neither could reach alone.

**8. Score and rank.** Each completed branch is scored on cumulative explanatory reach across the full chain. Results are ranked and displayed.

## Documents

- [Design Document](docs/thought-experiment-generator-design-doc.md) -- Architecture spec, data structures, prompts, filter stack, build order
- [Rust Implementation](docs/rust-implementation.md) -- Module layout, interfaces, concurrency model
