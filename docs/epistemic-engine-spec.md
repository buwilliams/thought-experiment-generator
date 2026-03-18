# Epistemic Engine — Design Spec

## What This Is

The core loop is:

```
conjecture + problem → candidate
candidate + criticism → conjecture (if it survives)
```

This loop is the mechanism for narrowing the search space. Novel theories exist in a vast possibility space — the challenge is not coverage but steering. Random search has no memory; each draw is as blind as the last. Here, only candidates that survive criticism become conjectures, and only conjectures that generate high-scoring candidates survive long enough to be promoted. The conjecture pool progressively biases toward whatever is generative. The search space is not narrowed by exclusion — it is steered by accumulated success.

The loop is self-referential: the same conjecture pool that generates candidates also criticizes them. The mind is conjectures. Perspectives are conjectures. The distinction between layers is stability and trust, not ontology.

This is a thinking engine, not a search engine. The goal is not to find a pre-existing answer but to generate structured candidates that illuminate a problem in ways neither the conjectures nor the problem alone could produce.

---

## Conceptual Hierarchy

All layers contain the same atom: a **Conjecture** — a unit of explanatory knowledge or perspective, held fallibly. The layers differ only in trust and stability, not in kind. This follows directly from Deutschian fallibilism: there is no bedrock of certain knowledge, only conjectures held with varying degrees of confidence.

```
Mind          — most trusted conjectures, slowest to change
  ↑ promote / ↓ demote
Perspectives  — active conjectures under test, medium stability
  ↑ promote / ↓ demote
Candidates    — new conjectures generated each run, scored and ephemeral
```

Conjectures (mind and perspectives) are shared across all problem sets.

**Problem Sets** are scoped collections of problems (cap: 10). Problems only exist within sets — there is no global problem pool. Problem sets are not in the promotion hierarchy but are scored and pruned by the system each run.

---

## Data Structures

The "database" is the filesystem. All state is stored as markdown files in subdirectories. No JSON databases. This keeps state human-readable, diffable, and inspectable without tooling.

Each entity has two files: a `.md` for human-readable content and a `.json` sidecar for machine-readable metadata. Prompts read from JSON (summaries, scores). Humans read `.md`.

**Conjectures (mind and perspectives) are shared across all problem sets.** Problem sets are scoped collections — problems only exist within sets.

### Conjecture content — `{layer}/{id}.md`

```markdown
# {title}

## Summary

One or two sentences. This is what goes into prompts.

## Full Text

Full text of the conjecture — as long as needed.
```

### Conjecture metadata — `{layer}/{id}.json`

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

The ID is the first 8 characters of `sha256(content)`. A problem set is capped at **10 problems**. Problems are stored in `problems/` and referenced by ID in the set. A problem can belong to multiple sets. Conjectures (mind/perspectives) are shared across all sets.

### Candidate content — `runs/NNN/{problem_id}-{conjecture_id}.md`

```markdown
# Candidate: {problem_id} × {conjecture_id}

## Conjecture

Full candidate text.

## Questions

1. Question text — **yes**
2. Question text — **no**

## Candidate Problems

- Candidate problem text
```

### Candidate metadata — `runs/NNN/{problem_id}-{conjecture_id}.json`

```json
{
  "problem_id": "slug",
  "conjecture_id": "slug",
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
    {id}.md              — conjecture content (summary + full text)
    {id}.json            — conjecture metadata (score, rank, run_count, history)
  perspectives/
    {id}.md
    {id}.json
  problems/
    {id}.md              — problem content (summary + full text)
    {id}.json            — problem metadata (score, rank, run_count)
  problemsets/           — problem set index
    {id}.md              — set content (raw text)
    {id}.json            — set metadata (problem_ids list, run_count)
  runs/
    NNN/
      {problem}-{conjecture}.md    — candidate content
      {problem}-{conjecture}.json  — candidate scores + candidate problems
      summary.md                   — ranked results + promoted/demoted conjectures
```

`data/state/state.json`:
```json
{
  "run": 3,
  "created_at": "iso8601",
  "last_run_at": "iso8601"
}
```

The run number increments at the start of each run. `runs/NNN/` is created using the new run number before any candidates are generated.

The seed state is checked in as `data/seed/`:

```
data/seed/
  mind/                  — starting mind conjectures (.md + .json each)
  perspectives/          — starting perspective conjectures (.md + .json each)
  problems/              — optional starting problems (empty by default)
```

On first run, seed state is copied to `data/state/`. `--fresh` resets `data/state/` from seed.

---

## The Run

A single run processes all problems against all perspective conjectures, generating one candidate per (problem × perspective conjecture) pair.

### Phase 1 — Generate Candidates

For each `(problem, perspective_conjecture)` pair:

1. Build context: mind conjecture summaries (as system prompt) + perspective conjecture summary + problem summary
2. Call LLM → candidate text
3. Save candidate

Runs concurrently up to `--max-concurrent`.

### Phase 2 — Evaluate Candidates

For each candidate:

**Pass 1 — Logical Consistency**
- System prompt: mind
- Ask: is this candidate internally self-consistent? Score 0.0–1.0.
- If score < threshold (default 0.3), mark failed, skip Pass 2.

**Pass 2 — Hard to Vary**
- System prompt: mind
- Ask the mind to generate 10 yes/no questions that probe whether this candidate has structure that resists arbitrary modification. Questions should be contextual — different candidates, different problems warrant different questions.
- Score each question answer: yes = 1, no = 0. Total / 10 = hard-to-vary score.
- Combined score: `0.3 * logical_consistency + 0.7 * hard_to_vary`

**Candidate Problem Extraction**
- During Pass 2, the mind also identifies unresolved tensions or unexplored implications in the candidate.
- Each candidate problem is evaluated: is this worth adding to the problem set? Score 0.0–1.0. Threshold 0.6 for admission.

### Phase 3 — Rank and Promote

**Candidate ranking:** sort all candidates by combined score descending.

**Perspective conjecture ranking:** for each perspective conjecture, compute mean score across all problems it was applied to this run. Update `score` as rolling average weighted by `run_count`.

**Problem ranking:** for each problem, compute mean candidate score across all conjectures this run. Update `score` similarly.

**Admit candidate problems:** candidate problems extracted during Phase 2 are evaluated for admission to the current problem set (admission threshold 0.6). Admitted problems are saved to `problems/` and their IDs are added to the set's `problem_ids` list.

**Problem review (one LLM call per run, scoped to the current set):**
- The mind receives summaries of all problems in the set (id + summary) and identifies any that are exact duplicates of or fully subsumed by another problem in the set. Identified problems are removed from the set's `problem_ids` (they remain in `problems/` — they may belong to other sets).
- Cap enforcement: if the set exceeds 10 problems after deduplication, the bottom-ranked problem (minimum `run_count` of 3) is removed from the set. This repeats until the set is at or below 10. If all remaining problems are below min_run_count (newly added, unscored), the cap overage is accepted until the next run.

**Promotion (one per run, skipped if no eligible candidates):**
- Top-ranked perspective conjecture (by composite of score × problem_coverage breadth, with minimum `run_count` of 3) → promoted to mind. Breadth is measured as the number of distinct problems the conjecture has been applied to.
- Top-ranked candidate (by score) → summarized into a new conjecture via `promote_candidate` prompt → added to perspectives

**Demotion (one per run, skipped if no eligible candidates):**
- Bottom-ranked mind conjecture (by score × problem_coverage breadth, minimum `run_count` of 3) → demoted to perspectives
- Bottom-ranked perspective conjecture (by same composite, minimum `run_count` of 3) → discarded

Promotion and demotion are skipped entirely if fewer than one conjecture meets the `run_count` threshold. This protects early runs where signal is thin.

### Phase 4 — Report

Write `data/state/runs/NNN/summary.md`:
- Ranked candidate table (problem, conjecture, score)
- Top 5 candidates with 20-word summaries
- Promoted perspective conjecture (→ mind)
- Promoted candidate (→ perspectives)
- Demoted conjectures
- Problems removed (deduplicated + bottom-ranked discard)

---

## Prompts

### generate_candidate(mind, perspective_conjecture, problem)
```
System: [mind conjectures as a structured set of principles and perspectives]

You are reasoning from a specific perspective. Your perspective is:
[perspective_conjecture summary]

Apply this perspective to the following problem and generate a conjecture — a structured claim about what is true, what follows, or what is illuminated when this perspective meets this problem. Follow the logic of the collision. Do not invent novelty for its own sake. 500 words or fewer.

Problem: [problem summary]
```

### logical_consistency_check(candidate)
```
System: [mind conjectures]

Evaluate whether the following conjecture is internally self-consistent — does it contradict itself, rely on incompatible premises, or make claims that cannot simultaneously be true?

Return JSON: { "score": 0.0, "reason": "..." }

Conjecture: [candidate text]
```

### generate_questions(candidate, problem)
```
System: [mind conjectures]

Generate 10 yes/no questions that probe whether the following conjecture is "hard to vary" — meaning its parts are load-bearing and cannot be arbitrarily modified without destroying the explanation. Questions should be specific to this conjecture and this problem, not generic.

Return JSON: { "questions": ["...", ...] }

Conjecture: [candidate text]
Problem: [problem summary]
```

### answer_questions(candidate, questions)
```
System: [mind conjectures]

Answer each of the following yes/no questions about this conjecture. Return JSON:
{ "answers": [{ "question": "...", "answer": true }] }

Conjecture: [candidate text]
Questions: [questions]
```

### extract_candidate_problems(candidate)
```
System: [mind conjectures]

Identify unresolved tensions, unexplored implications, or open questions raised by this conjecture that are worth exploring as new problems. For each candidate, score 0.0–1.0 whether it is worth pursuing.

Return JSON: { "candidates": [{ "text": "...", "score": 0.0 }] }

Conjecture: [candidate text]
```

### summarize_candidate(candidate, score)
```
System: You are summarizing a thought experiment. Return only a 20-word summary of what the thought experiment claims or illuminates. No preamble, no meta-commentary.

Thought experiment (quality score [score]/1.0):

[candidate text]
```

### conjecture_summary(title, full_text)
```
System: [mind conjecture summaries]

Summarize the following conjecture into 1-2 sentences suitable for use as context in LLM prompts. The summary should capture the core lens or principle the conjecture provides.

Return JSON: { "summary": "..." }

Title: [title]
Full text: [full_text]
```

### promote_candidate(candidate, score)
```
System: [mind conjectures]

Convert the following conjecture into a reusable perspective tool.

Return JSON: { "summary": "...", "full_text": "..." }

The summary must be 1-2 sentences suitable for use in LLM prompts.
The full_text must be a readable, standalone description of the perspective this conjecture embodies — what lens it provides, what kinds of problems it is useful for, and what it illuminates. 100-200 words.

Conjecture: [candidate text]
Score: [score]
```

### deduplicate_problems(problems)
```
System: [mind conjectures]

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

The seed mind is defined by the files in `data/seed/mind/`. Each conjecture has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/mind/`.

Current seed mind conjectures:

- **`deutschian-epistemology`** — Fallibilism and the growth of knowledge through conjecture and criticism. Abstract objects are real; progress in abstract space (mathematics, epistemology) is as genuine as progress in physics and requires no physical grounding.
- **`ontology`** — The philosophical study of what exists. Root axiom: knowledge and information are fundamental (held fallibly). Properties are informational distinctions; mechanisms are processes by which information is preserved, altered, or lost.
- **`systems-thinking`** — Donella Meadows' framework: behavior is produced by structure, not isolated causes. Stocks, flows, feedback loops, delays, and leverage points. The system's real goal is revealed by its behavior, not its stated purpose.

---

## Seed Perspectives

The seed perspectives are defined by the files in `data/seed/perspectives/`. Each conjecture has a `.md` (content) and `.json` (metadata). On first run, these are copied to `data/state/perspectives/`.

Current seed perspective conjectures:

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

# Add a new conjecture by inline text
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --text "Full text of the conjecture"
cargo run -- add-conjecture --layer perspectives --title "Conjecture Title" --text "Full text of the conjecture"

# Add a new conjecture from a file or stdin
cargo run -- add-conjecture --layer mind --title "Conjecture Title" --file path/to/conjecture.md
cat my-conjecture.md | cargo run -- add-conjecture --layer perspectives --title "Conjecture Title"
```

The `add-conjecture` command:
1. Accepts the full text of the conjecture via `--text`, `--file`, or stdin
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
| `--min-run-count` | 3 | Minimum runs before a conjecture or problem is eligible for promotion/demotion |
| `--provider` | anthropic | `anthropic`, `anthropic-token`, or `openai` |
| `--model` | claude-sonnet-4-6 | Model name |
| `--fresh` | false | Reset state to seed |

**`add-conjecture` options:**

| Flag | Required | Description |
|---|---|---|
| `--layer` | yes | `mind` or `perspectives` |
| `--title` | yes | Human-readable title, used to derive the file slug |
| `--text` | no | Inline full text of the conjecture |
| `--file` | no | Path to a file whose contents are the full text |
| stdin | no | Piped content used as full text if `--text` and `--file` are omitted |

---

## LLM Call Budget (per run, default settings)

| Phase | Calls per pair | Pairs | Total |
|---|---|---|---|
| Candidate generation | 1 | problems × conjectures | N×M |
| Logical consistency | 1 | N×M | N×M |
| Generate questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Answer questions | 1 | N×M (pass 1 survivors) | ~N×M |
| Candidate problem extraction | 1 | N×M | N×M |
| Problem deduplication | 1 | 1 per run | 1 |
| Promote candidate (promotion) | 1 | 1 per run | 1 |
| Top 5 summaries | 5 | 1 | 5 |
| **Default (5 problems × 10 conjectures)** | | | **~257** |

Candidates are not cached between runs — they are regenerated each run because the mind and perspective states may have changed.

---

## Module Structure

```
src/
  main.rs         — CLI parsing, dispatch
  runner.rs       — orchestrates phases 1–4
  state.rs        — load/save mind, perspectives, problems, candidates, runs
  prompts.rs      — all prompt templates
  evaluator.rs    — logical consistency + hard-to-vary scoring
  promoter.rs     — ranking, promotion, demotion logic
  types.rs        — Conjecture, Candidate, Problem, ProblemSet structs
  llm/            — LlmClient (Anthropic + OpenAI)
```

---

## Resolved Decisions

1. **Context efficiency:** Prompts always use summaries, never full text. Mind conjectures serialize as their `summary` fields into the system prompt. This keeps context lean regardless of database size.
2. **All problems × all conjectures per run.** No sampling. Optimize later.
3. **Candidate → conjecture promotion:** Candidate is summarized into a short, readable conjecture-like statement before promotion. Full candidate text is retained in the run file and expandable by ID.
4. **Score stability:** Not addressed yet. Revisit if early conjectures dominate rankings after many runs.

## Open Questions

1. **Perspective conjecture selection:** Currently all conjectures apply to all problems every run. A future improvement would have the mind select which conjecture to apply to which problem — but that adds complexity and requires signal that doesn't exist yet.
