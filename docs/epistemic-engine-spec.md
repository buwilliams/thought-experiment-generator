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
  mind/
    {id}.md              — tool content (summary + full text)
    {id}.json            — tool metadata (score, rank, run_count, history)
  perspectives/
    {id}.md
    {id}.json
  problems/
    {id}.md              — problem content (summary + full text)
    {id}.json            — problem metadata (score, rank, run_count)
  runs/
    NNN/
      {problem}-{tool}.md    — conjecture content
      {problem}-{tool}.json  — conjecture scores + candidate problems
      summary.md             — ranked results + promoted/demoted tools
```

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

1. Build context: full mind (as system prompt) + perspective tool text + problem text
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

**Promotion (one per run):**
- Top-ranked perspective tool (by score, with minimum `run_count` of 3) → promoted to mind
- Top-ranked conjecture (by score) → promoted to perspectives as a new tool

**Demotion (one per run):**
- Bottom-ranked mind tool (by score across recent runs) → demoted to perspectives
- Bottom-ranked perspective tool (by score, with minimum `run_count` of 3) → discarded

Promotion and demotion do not apply to tools with `run_count < 3` — new entries need time to accumulate signal.

### Phase 4 — Report

Write `data/state/runs/NNN/summary.md`:
- Ranked conjecture table (problem, tool, score)
- Top 5 conjectures with 20-word summaries
- Promoted tool (from perspectives → mind)
- Promoted conjecture (from conjectures → perspectives)
- Demoted tools
- New problems added

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

---

## Seed Mind

The seed mind encodes Deutschian epistemology as the starting ideology. Expressed as a set of tools:

1. Knowledge grows through conjecture and criticism, not justification or accumulation.
2. A good explanation is hard to vary — its parts are load-bearing and cannot be arbitrarily modified without destroying the explanation.
3. Reach matters: a good explanation applies beyond the domain it was constructed for.
4. Logical consistency is necessary but not sufficient — an explanation can be consistent and shallow.
5. Progress in abstract domains (mathematics, philosophy, epistemology) is real and does not require grounding in physical observation.
6. The goal of criticism is not to destroy but to identify what is load-bearing and what is arbitrary.
7. The mind itself is a fallible conjecture — its tools and criteria are revisable.

---

## Seed Perspectives

A curated starting set of perspective tools — concepts that have proven generative across many domains:

1. Universality: what would it mean for this to apply to all cases, not just some?
2. Emergence: what properties appear at the level of the whole that are not present in the parts?
3. Compression: what is the shortest explanation that preserves the essential structure?
4. Inversion: what does the opposite of this claim imply? Does the inversion illuminate anything?
5. Abstraction ladder: can this be stated at a higher level of abstraction without losing content?
6. Analogical transfer: what domain uses a structurally identical mechanism? What does the analogy reveal?
7. Tension identification: what two claims in this space are in unresolved tension?
8. Boundary conditions: where does this explanation break down? What does the boundary reveal?
9. Minimal assumption: what is the weakest set of assumptions that generates this conclusion?
10. Second-order: what does this claim imply about the process that generated it?

---

## CLI

```sh
# Run one full cycle across all problems
cargo run -- run

# Run with a new user-supplied problem
cargo run -- run --problem "your problem text"

# Read last run summary without running
cargo run -- read

# Reset state to seed
cargo run -- --fresh run

# Control concurrency and sampling
cargo run -- run --max-concurrent 5 --problems-per-run 10 --tools-per-problem 5
```

### Options

| Flag | Default | Description |
|---|---|---|
| `--max-concurrent` | 5 | Max concurrent LLM calls |
| `--problems-per-run` | all | Sample N problems per run (for large databases) |
| `--tools-per-run` | all | Sample N perspective tools per run |
| `--consistency-threshold` | 0.3 | Minimum logical consistency score to proceed to Pass 2 |
| `--problem-admission-threshold` | 0.6 | Minimum score for candidate problems to enter database |
| `--min-run-count` | 3 | Minimum runs before a tool is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--fresh` | false | Reset state to seed |

---

## LLM Call Budget (per run, default settings)

| Phase | Calls per pair | Pairs | Total |
|---|---|---|---|
| Conjecture generation | 1 | problems × tools | N×M |
| Logical consistency | 1 | N×M | N×M |
| Generate questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Answer questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Candidate problem extraction | 1 | N×M | N×M |
| Top 5 summaries | 5 | 1 | 5 |
| **Default (5 problems × 10 tools)** | | | **~255** |

Pools are not cached between runs — conjectures are regenerated each run because the mind and perspective states may have changed.

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
