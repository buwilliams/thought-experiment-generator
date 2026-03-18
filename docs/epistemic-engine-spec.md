# Epistemic Engine — Design Spec

## What This Is

The core loop is:

```
tools + problem → conjecture
conjecture + criticism → tool (if it survives)
```

This loop is the mechanism for narrowing the search space. Novel theories exist in a vast possibility space — the challenge is not coverage but steering. Random search has no memory; each draw is as blind as the last. Here, only conjectures that survive criticism become tools, and only tools that generate high-scoring conjectures survive long enough to be promoted. The tool pool progressively biases toward whatever is generative. The search space is not narrowed by exclusion — it is steered by accumulated success.

The loop is self-referential: the same tool pool that generates conjectures also criticizes them. The mind is tools. Perspectives are tools. The distinction between layers is stability and trust, not ontology.

This is a thinking engine, not a search engine. The goal is not to find a pre-existing answer but to generate structured conjectures that illuminate a problem in ways neither the tools nor the problem alone could produce.

---

## Conceptual Hierarchy

All layers contain the same atom: a **Tool** — a unit of perspective or knowledge expressed as a plain text statement or framework. The layers differ only in trust and stability.

```
Mind          — most trusted, slowest to change
  ↑ promote / ↓ demote
Perspectives  — active tool database, medium stability
  ↑ promote / ↓ demote
Conjectures   — generated each run, ephemeral
```

**Problems** are a separate ranked database. They are not in the promotion hierarchy but are scored by the system and can be discovered during evaluation.

---

## Data Structures

The "database" is the filesystem. All state is stored as markdown files in subdirectories. No JSON databases. This keeps state human-readable, diffable, and inspectable without tooling.

Each entity has two files: a `.md` for human-readable content and a `.json` sidecar for machine-readable metadata. Prompts read from JSON (summaries, scores). Humans read `.md`.

**Tools are shared across all problem sets.** The mind and perspective layers are global. Problem sets are scoped views into the shared problem store.

### Tool content — `{layer}/{id}.md`

```markdown
# {title}

## Summary

One or two sentences. This is what goes into prompts.

## Full Text

Full text of the tool — as long as needed.
```

### Tool metadata — `{layer}/{id}.json`

```json
{
  "id": "slug",
  "layer": "mind | perspectives",
  "score": 0.0,
  "rank": 1,
  "run_count": 0,
  "problem_coverage": ["problem-id"],
  "created_at": "iso8601",
  "promoted_from": "id | null",
  "history": [{ "run": 1, "score": 0.0, "problem_id": "slug" }]
}
```

### Problem content — `problems/{id}.md`

```markdown
# {title}

## Summary

One sentence. Used in prompts.

## Full Text

Full text of the problem.
```

### Problem metadata — `problems/{id}.json`

```json
{
  "id": "slug",
  "source": "user | system",
  "score": 0.0,
  "rank": 1,
  "run_count": 0,
  "created_at": "iso8601"
}
```

### Problem set content — `problemsets/{id}.md`

Raw text content describing the set's scope and theme. No sections — just the content as provided.

### Problem set metadata — `problemsets/{id}.json`

```json
{
  "id": "8-char-sha256",
  "problem_ids": ["slug-1", "slug-2"],
  "run_count": 0,
  "created_at": "iso8601"
}
```

The ID is the first 8 characters of `sha256(content)`. A problem set is capped at **10 problems**. Problems are stored globally in `problems/` and referenced by ID in the set. A problem can belong to multiple sets. Tools are shared across all sets.

### Conjecture content — `runs/NNN/{problem_id}-{tool_id}.md`

```markdown
# Conjecture: {problem_title} × {tool_title}

## Conjecture

Full conjecture text.

## Questions

1. Question text — **yes**
2. Question text — **no**

## Candidate Problems

- Candidate problem text
```

### Conjecture metadata — `runs/NNN/{problem_id}-{tool_id}.json`

```json
{
  "problem_id": "slug",
  "tool_id": "slug",
  "run": 1,
  "logical_consistency": 0.0,
  "hard_to_vary": 0.0,
  "total": 0.0,
  "candidate_problems": [{ "text": "...", "score": 0.0 }]
}
```

---

## Persistent State

All state lives in `data/state/`:

```
data/state/
  state.json             — current run number and global metadata
  mind/
    {id}.md              — tool content (summary + full text)
    {id}.json            — tool metadata (score, rank, run_count, history)
  perspectives/
    {id}.md
    {id}.json
  problems/              — global problem store (shared across all sets)
    {id}.md              — problem content (summary + full text)
    {id}.json            — problem metadata (score, rank, run_count)
  problemsets/           — problem set index
    {id}.md              — set title + description
    {id}.json            — set metadata (problem_ids list, run_count)
  runs/
    NNN/
      {problem}-{tool}.md    — conjecture content
      {problem}-{tool}.json  — conjecture scores + candidate problems
      summary.md             — ranked results + promoted/demoted tools
```

`data/state/state.json`:
```json
{
  "run": 3,
  "created_at": "iso8601",
  "last_run_at": "iso8601"
}
```

The run number increments at the start of each run. `runs/NNN/` is created using the new run number before any conjectures are generated.

The seed state is checked in as `data/seed/`:

```
data/seed/
  mind/                  — starting mind tools (.md + .json each)
  perspectives/          — starting perspective tools (.md + .json each)
  problems/              — optional starting problems (empty by default)
```

On first run, seed state is copied to `data/state/`. `--fresh` resets `data/state/` from seed.

---

## The Run

A single run processes all problems against all perspective tools (or a sampled subset for large databases), generating one conjecture per (problem × perspective tool) pair.

### Phase 1 — Generate Conjectures

For each `(problem, perspective_tool)` pair:

1. Build context: mind tool summaries (as system prompt) + perspective tool summary + problem summary
2. Call LLM → conjecture text
3. Save conjecture

Runs concurrently up to `--max-concurrent`.

### Phase 2 — Evaluate Conjectures

For each conjecture:

**Pass 1 — Logical Consistency**
- System prompt: mind
- Ask: is this conjecture internally self-consistent? Score 0.0–1.0.
- If score < threshold (default 0.3), mark failed, skip Pass 2.

**Pass 2 — Hard to Vary**
- System prompt: mind
- Ask the mind to generate 10 yes/no questions that probe whether this conjecture has structure that resists arbitrary modification. Questions should be contextual — different conjectures, different problems warrant different questions.
- Score each question answer: yes = 1, no = 0. Total / 10 = hard-to-vary score.
- Combined score: `0.3 * logical_consistency + 0.7 * hard_to_vary`

**Candidate Problem Extraction**
- During Pass 2, the mind also identifies unresolved tensions or unexplored implications in the conjecture.
- Each candidate is evaluated: is this worth adding to the problem database? Score 0.0–1.0. Threshold 0.6 for admission.

### Phase 3 — Rank and Promote

**Conjecture ranking:** sort all conjectures by combined score descending.

**Perspective tool ranking:** for each perspective tool, compute mean score across all problems it was applied to this run. Update `score` as rolling average weighted by `run_count`.

**Problem ranking:** for each problem, compute mean conjecture score across all tools this run. Update `score` similarly.

**Admit candidate problems:** candidate problems extracted during Phase 2 are evaluated for admission to the current problem set (admission threshold 0.6). Admitted problems are saved to the global `problems/` store and their IDs are added to the set's `problem_ids` list.

**Problem review (one LLM call per run, scoped to the current set):**
- The mind receives summaries of all problems in the set (id + summary) and identifies any that are exact duplicates of or fully subsumed by another problem in the set. Identified problems are removed from the set's `problem_ids` (they remain in the global store — they may belong to other sets).
- Cap enforcement: if the set exceeds 10 problems after deduplication, the bottom-ranked problem (minimum `run_count` of 3) is removed from the set. This repeats until the set is at or below 10. If all remaining problems are below min_run_count (newly added, unscored), the cap overage is accepted until the next run.

**Promotion (one per run, skipped if no eligible candidates):**
- Top-ranked perspective tool (by composite of score × problem_coverage breadth, with minimum `run_count` of 3) → promoted to mind. Breadth is measured as the number of distinct problems the tool has been applied to. A tool with high score on one problem does not qualify over a tool with moderate score across many.
- Top-ranked conjecture (by score) → summarized into a new tool via `summarize_for_tool` prompt → added to perspectives

**Demotion (one per run, skipped if no eligible candidates):**
- Bottom-ranked mind tool (by score × problem_coverage breadth, minimum `run_count` of 3) → demoted to perspectives
- Bottom-ranked perspective tool (by same composite, minimum `run_count` of 3) → discarded

Promotion and demotion are skipped entirely if fewer than one tool meets the `run_count` threshold. This protects early runs where signal is thin.

### Phase 4 — Report

Write `data/state/runs/NNN/summary.md`:
- Ranked conjecture table (problem, tool, score)
- Top 5 conjectures with 20-word summaries
- Promoted tool (from perspectives → mind)
- Promoted conjecture (from conjectures → perspectives)
- Demoted tools
- Problems removed (deduplicated + bottom-ranked discard)

---

## Prompts

### conjecture_generation(mind_tools, perspective_tool, problem)
```
System: [mind tools as a structured set of principles and perspectives]

You are reasoning from a specific perspective. Your perspective is:
[perspective_tool text]

Apply this perspective to the following problem and generate a conjecture — a structured claim about what is true, what follows, or what is illuminated when this perspective meets this problem. Follow the logic of the collision. Do not invent novelty for its own sake. 500 words or fewer.

Problem: [problem text]
```

### logical_consistency_check(conjecture)
```
System: [mind tools]

Evaluate whether the following conjecture is internally self-consistent — does it contradict itself, rely on incompatible premises, or make claims that cannot simultaneously be true?

Return JSON: { "score": 0.0, "reason": "..." }

Conjecture: [conjecture text]
```

### generate_questions(conjecture, problem)
```
System: [mind tools]

Generate 10 yes/no questions that probe whether the following conjecture is "hard to vary" — meaning its parts are load-bearing and cannot be arbitrarily modified without destroying the explanation. Questions should be specific to this conjecture and this problem, not generic.

Return JSON: { "questions": ["...", ...] }

Conjecture: [conjecture text]
Problem: [problem text]
```

### answer_questions(conjecture, questions)
```
System: [mind tools]

Answer each of the following yes/no questions about this conjecture. Return JSON:
{ "answers": [{ "question": "...", "answer": true }] }

Conjecture: [conjecture text]
Questions: [questions]
```

### extract_candidate_problems(conjecture)
```
System: [mind tools]

Identify unresolved tensions, unexplored implications, or open questions raised by this conjecture that are worth exploring as new problems. For each candidate, score 0.0–1.0 whether it is worth pursuing.

Return JSON: { "candidates": [{ "text": "...", "score": 0.0 }] }

Conjecture: [conjecture text]
```

### summarize_conjecture(conjecture, score)
```
System: Summarize in 20 words or fewer. Return only the summary.

[conjecture text]
Score: [score]
```

### summarize_tool(title, full_text)
```
System: [mind tool summaries]

Summarize the following tool into 1-2 sentences suitable for use as context
in LLM prompts. The summary should capture the core lens or principle the
tool provides, concisely enough to be useful alongside other tool summaries
in a system prompt.

Return JSON: { "summary": "..." }

Title: [title]
Full text: [full_text]
```

### summarize_for_tool(conjecture, score)
```
System: You are converting a conjecture into a reusable perspective tool.
Return JSON: { "summary": "...", "full_text": "..." }
The summary must be 1-2 sentences suitable for use in LLM prompts.
The full_text must be a readable, standalone description of the perspective
this conjecture embodies — what lens it provides, what kinds of problems
it is useful for, and what it illuminates. 100-200 words.

Conjecture: [conjecture text]
Score: [score]
```

### deduplicate_problems(problems)
```
System: [mind tools]

Review the following problems and identify any that are exact duplicates of or
fully subsumed by another problem in the list. A problem is subsumed if its
core question is already captured by a broader problem in the list.
When in doubt, keep the problem. Return only the IDs to remove.

Return JSON: { "remove": ["id1", "id2"] }

Problems:
- id1: summary
- id2: summary
...
```

---

## Seed Mind

The seed mind is defined by the files in `data/seed/mind/`. Each tool has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/mind/`.

Current seed mind tools:

- **`deutschian-epistemology`** — Fallibilism and the growth of knowledge through conjecture and criticism. Abstract objects are real; progress in abstract space (mathematics, epistemology) is as genuine as progress in physics and requires no physical grounding.
- **`ontology`** — The philosophical study of what exists. Root axiom: knowledge and information are fundamental (held fallibly). Properties are informational distinctions; mechanisms are processes by which information is preserved, altered, or lost.
- **`systems-thinking`** — Donella Meadows' framework: behavior is produced by structure, not isolated causes. Stocks, flows, feedback loops, delays, and leverage points. The system's real goal is revealed by its behavior, not its stated purpose.

---

## Seed Perspectives

The seed perspectives are defined by the files in `data/seed/perspectives/`. Each tool has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/perspectives/`.

Current seed perspective tools:

- **`thought-experiments`** — Construct a hypothetical scenario, isolate the key variable, stipulate it as real, and follow the logic strictly to wherever it leads. Strangeness is information.
- **`mathematical-formalism`** — Translate the problem into mathematical structure (variables, invariants, transformations). Let the structure reveal what is preserved, what varies, and what constraints are non-negotiable.
- **`counterfactual-reasoning`** — Systematically vary assumptions: remove one, invert one, replace one with its opposite. What breaks is load-bearing. What survives is incidental.
- **`extreme-cases`** — Push to limits: infinity, zero, maximum, minimum, phase transitions. Boundary behavior reveals the structure hidden in the comfortable middle range.
- **`historical-genesis`** — Ask how the problem came to exist, what it replaced, and what it inherited without examination. Origin stories make invisible constraints visible.

---

## CLI

```sh
# Create a problem set (content drives the ID via sha256 hash)
cargo run -- create-problemset "LLMs and epistemology: exploring whether LLMs generate genuine knowledge"
cargo run -- create-problemset --file my-problemset.md
cat my-problemset.md | cargo run -- create-problemset

# Add problems to a set
cargo run -- add-problem --problemset llms-and-knowledge --text "Can LLMs create new knowledge?"

# Remove a problem from a set
cargo run -- remove-problem --problemset llms-and-knowledge --problem-id "can-llms-create-new-knowledge"

# List all problem sets
cargo run -- list-problemsets

# Run on a problem set
cargo run -- run --problemset llms-and-knowledge
cargo run -- run                              # works if only one set exists

# Run with a new problem added before running
cargo run -- run --problemset llms-and-knowledge --problem "your problem text"

# Read last run summary without running
cargo run -- read

# Reset state to seed
cargo run -- --fresh run

# Add a new tool by inline text
cargo run -- add-tool --layer mind --title "Tool Title" --text "Full text of the tool"
cargo run -- add-tool --layer perspectives --title "Tool Title" --text "Full text of the tool"

# Add a new tool from a file or stdin
cargo run -- add-tool --layer mind --title "Tool Title" --file path/to/tool.md
cat my-tool.md | cargo run -- add-tool --layer perspectives --title "Tool Title"
```

The `add-tool` command:
1. Accepts the full text of the tool via `--text`, `--file`, or stdin
2. Calls the LLM using the current mind as system prompt to generate a `summary` (1-2 sentences) from the full text
3. Derives a slug from `--title` (lowercased, spaces to hyphens)
4. Writes `{layer}/{slug}.md` and `{layer}/{slug}.json` to `data/state/`
5. Prints the generated summary for review

### Options

| Flag | Default | Description |
|---|---|---|
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--consistency-threshold` | 0.3 | Minimum logical consistency score to proceed to Pass 2 |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter the set |
| `--min-run-count` | 3 | Minimum runs before a tool or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--fresh` | false | Reset state to seed |

**`add-tool` options:**

| Flag | Required | Description |
|---|---|---|
| `--layer` | yes | `mind` or `perspectives` |
| `--title` | yes | Human-readable title, used to derive the file slug |
| `--text` | no | Inline full text of the tool |
| `--file` | no | Path to a file whose contents are the full text |
| stdin | no | Piped content used as full text if `--text` and `--file` are omitted |

---

## LLM Call Budget (per run, default settings)

| Phase | Calls per pair | Pairs | Total |
|---|---|---|---|
| Conjecture generation | 1 | problems × tools | N×M |
| Logical consistency | 1 | N×M | N×M |
| Generate questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Answer questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Candidate problem extraction | 1 | N×M | N×M |
| Problem deduplication | 1 | 1 per run | 1 |
| Summarize for tool (promotion) | 1 | 1 per run | 1 |
| Top 5 summaries | 5 | 1 | 5 |
| **Default (5 problems × 10 tools)** | | | **~257** |

Conjectures are not cached between runs — they are regenerated each run because the mind and perspective states may have changed.

---

## Module Structure

```
src/
  main.rs         — CLI parsing, dispatch
  runner.rs       — orchestrates phases 1–4
  state.rs        — load/save mind, perspectives, problems, runs
  prompts.rs      — all prompt templates
  evaluator.rs    — logical consistency + hard-to-vary scoring
  promoter.rs     — ranking, promotion, demotion logic
  types.rs        — Tool, Problem, Conjecture, Score structs
  llm/            — LlmClient (unchanged from current)
```

---

## Resolved Decisions

1. **Context efficiency:** Prompts always use summaries, never full text. Mind tools serialize as their `summary` fields into the system prompt. This keeps context lean regardless of database size.
2. **All problems × all tools per run.** No sampling. Optimize later.
3. **Conjecture → tool promotion:** Conjecture is summarized into a short, readable tool-like statement before promotion. Full conjecture text is retained in the run file and expandable by ID.
4. **Score stability:** Not addressed yet. Revisit if early tools dominate rankings after many runs.

## Open Questions

1. **Perspective tool selection:** Currently all tools apply to all problems every run. A future improvement would have the mind select which tool to apply to which problem — but that adds complexity and requires signal that doesn't exist yet.
