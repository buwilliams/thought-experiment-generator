# Thought Experiment Generator
## Design Document
*Personal reference — work in progress*

---

## Abstract

A 1,000-word thought experiment drawn from a 50,000-word vocabulary has a theoretical search space of 50,000^1,000 ≈ **10^4,699** possible combinations — larger than the number of atoms in the observable universe by a factor of 10^4,619. This space is intractable by any exhaustive method. And yet Einstein, drawing on all the background knowledge accumulated throughout his life up to that point, navigated it to produce special relativity. His starting question, *what would a frozen light wave look like?*, was not a bolt from nowhere. It was an informed guess made possible by everything he already knew.

The Thought Experiment Generator is a system for compressing the intractable search space into a tractable space for discovery. The central claim is that the 10^4,699 search space can be made tractable not by exhaustive search, but by collapsing it progressively through layered filters — grammar, [coherence](#glossary), topic relevance, and [Deutsch score](#glossary) — until surviving candidates occupy a small, navigable neighborhood of genuinely novel insight.

The system works as follows. A user provides a topic they want to wonder about. The system initializes three pools of structured knowledge fragments called [quads](#glossary) — one drawn from LLM-generated corpus facts about the topic ([background pool](#glossary)), one from a domain-agnostic 50,000-term vocabulary ([universal pool](#glossary)), and one that accumulates during the run from the tree's own high-scoring discoveries ([novel pool](#glossary)). It then runs a Depth-bounded branching search over explanation space: randomly drawing quads, generating hypothetical [thought experiments](#glossary), filtering for [coherence](#glossary) and [Deutsch score](#glossary), and deepening along every [branch](#glossary) to a fixed [depth limit](#glossary) regardless of intermediate scores. Branches with complementary [unresolved tensions](#glossary) are merged via [cross-pollination](#glossary). The result is a ranked set of [trajectories](#glossary) — chains of thought experiments that progressively illuminate the topic from angles the user did not specify and the corpus did not contain.

The goal is a working system that creates new knowledge using existing concepts and an LLM judge. Not retrieval. Not summarization. Discovery.

If the system had existed in 1895 and been given the topic *"why does light always travel at the same speed regardless of the observer"*, the claim is that it would have surfaced — within hours — a thought experiment structurally equivalent to Einstein's train and lightning bolts. The test of the system is whether it can, in tractable time, find the needle that took a human genius a decade to locate.

---

## Architecture Overview

### Core Concept

The generator explores a topic by running a **Depth-bounded branching search over explanation space**. It does not search for known results — it *wonders* about a topic by randomly colliding [objects](#glossary), [relationships](#glossary), and [properties](#glossary), filtering for [coherence](#glossary) and explanatory quality, and allowing the tree to build on its own discoveries.

The topic is an anchor for filtering, not a constraint on generation. Randomness is fully preserved on the generative side. The topic only applies pressure at the filter stage.

### Search Space Collapse

The raw search space of 10^4,699 collapses at each filter layer:

With default parameters (50,000-term vocabulary, 4/3/2 atom, 10 branches, depth 10, 100 draws per depth):

```
Space per draw:
  C(50000,4) x C(50000,3) x C(50000,2) = 10^39.8 possible thought experiments

Full tree run:
  Total draws:   ~10,500  (500 root + 10 branches x 10 depth x 100 draws)
  Each draw samples one point from 10^39.8 possibilities.
  The space is effectively inexhaustible -- repeated runs
  will not revisit the same territory.
```

This means the system cannot work by coverage. 10,500 draws out of 10^39.8 possibilities is not a search strategy -- it is a rounding error. The system is tractable for one reason only: **the draws are not random**.

Every draw is an informed guess. The three-pool composition biases each draw toward regions of the space more likely to survive the filter stack before any filtering occurs:

- **Background pool** contributes quads already known to be relevant to the topic -- raising the prior probability that the draw produces something coherent
- **Universal pool** contributes cross-domain surprise -- ensuring the draw is not merely retrieving known results
- **Novel pool** contributes what the tree has already discovered -- compounding prior good guesses into better future ones

The filters then confirm or reject. But the filters are not doing the heavy lifting. The pool composition is. A purely random draw from 50,000 words would almost never produce a coherent thought experiment. A draw seeded with two background quads relevant to the topic has a vastly higher prior probability of surviving coherence and Deutsch scoring.

Tractability comes from good guessing, not coverage. The background knowledge is what makes the guessing good.

### Three-Pool [Draw](#glossary) System

Every [draw](#glossary) at every [node](#glossary) samples from three pools simultaneously:

```
BACKGROUND  — what is known       (quads from corpus facts about the topic)
UNIVERSAL   — what is possible    (quads from cross-domain vocabulary: objects, relationships, properties)
NOVEL       — what has been found (quads earned by high-scoring tree survivors)
```

**Default ratio at initialization:** 50% background / 50% universal / 0% novel

As the tree runs, the [novel pool](#glossary) grows and the ratio shifts dynamically:

```
Early tree:   Background 50% | Universal 45% | Novel  5%
Mid tree:     Background 40% | Universal 40% | Novel 20%
Late tree:    Background 30% | Universal 30% | Novel 40%
```

The [novel pool](#glossary) is what makes the tree behave like a reasoning process rather than a search. It is the memory that allows a discovery at depth 4 to become an ingredient of a discovery at depth 7 that neither the corpus nor the vocabulary could have reached directly.

### Atom Structure

Every [thought experiment](#glossary) is generated from a configurable [draw](#glossary) of objects, relationships, and properties. The default is:

```
4 objects       (2 from background/novel + 2 from universal)
3 relationships (1-2 from background/novel + 1-2 from universal)
2 properties    (1 from background/novel + 1 from universal)
```

The 4/3/2 default is derived empirically from Einstein's journal: his five core thought experiments averaged 4 [objects](#glossary), 3 [relationships](#glossary), and 2 [properties](#glossary) each. It is the minimum sufficient to create a scenario rather than a simple analogy, generate tension rather than a static observation, and identify something investigable rather than merely descriptive.

The background/novel to universal split scales proportionally when the atom size is changed — half the slots always draw from background/novel, half from universal. Complexity accumulates through content inheritance across depths, not structural change.

### Filter Stack

Each generated [thought experiment](#glossary) passes through sequential filters, ordered cheap to expensive:

```
1. Grammar filter       — is this valid language?
2. Coherence filter     — is this internally consistent
                          and non-trivially related to topic?
3. Deutsch scorer       — does this have explanatory quality?
4. Survivor threshold   — does this score above the configured threshold?
```

### Tree Structure

```
                        [TOPIC]
                           |
          ┌────────────────┼────────────────┐
          ↓                ↓                ↓
      Branch A          Branch B         Branch C
      depth 1–10        depth 1–10       depth 1–10
          |                                  |
          └──────── cross-pollination ────────┘
                           |
                      New Branch
                      depth reset to 0
                      depth limit: 10
```

**Branching rules:**
- Root branches self-select — generator runs freely until N diverse [survivors](#glossary) accumulate
- Every [branch](#glossary) runs to [depth limit](#glossary) regardless of score trajectory
- Low scores do not trigger pruning — only depth limit, circularity, or vocabulary exhaustion terminate a branch
- [Cross-pollination](#glossary) occurs when two branches carry complementary [unresolved tensions](#glossary)

**Why failures are preserved to depth 10:**
Some branches score low for several steps before a random collision discharges accumulated tension and produces a score spike. Pruning on score eliminates these late-spike paths — which are often the most valuable. The [depth limit](#glossary) is the only termination criterion that does not introduce directional bias.

### [Novel Pool](#glossary) Admission

Not every [survivor](#glossary) contributes to the [novel pool](#glossary). A higher threshold governs admission:

```
score > survivor_threshold   → node survives, branch continues
score > novel_threshold      → quads extracted, added to novel pool
                               with provenance ID
```

High-scoring [survivors](#glossary) shape future draws. Lower-scoring survivors continue the branch without influencing the pool.

---

## CLI Interaction Design

### Interaction Flow

```
> What topic would you like to explore?
  [user input]

> Generating background knowledge...
  [LLM call 1 — fact generation, nondeterministic]
  [LLM call 2 — quad extraction, deterministic]
  Background pool initialized: ~80–120 quads

> Suggested resources for deeper exploration:
  [resource list — optional, user may feed back in for enrichment]

> Building draw pools...
  Background pool:  N quads
  Universal pool:   ~50,000 terms
  Novel pool:       0 quads (empty at start)
  Ratio:            50% background / 50% universal

> Generating root branches...
  Branch 1 found after N draws    [score: 0.88]
  Branch 2 found after N draws    [score: 0.91]
  ...
  Branch 10 found after N draws   [score: 0.74]
  Root generation complete: 10 branches, N total draws

> Running tree search...
  Depth limit: 10 | Parallel branches: 10

  Depth 1  | 10 active branches | best score: 0.94
  Depth 2  |  9 active branches | best score: 0.91
  Depth 3  | 11 active branches | cross-pollination: B2 + B7
  ...
  Depth 10 |  7 active branches

> Scoring trajectories...

> Results:

  Top trajectory: B3 → B3.2 → B3.2.1 → ...
  Trajectory score: 0.97
  [thought experiment chain]

  Cross-pollination insight (depth 3): B2 + B7
  Score: 0.94
  [thought experiment]

  Late spike path: B6 depth 9
  Score: 0.89
  [thought experiment]
```

### Configurable Parameters

```
--depth            Depth limit per branch (default: 10)
--branches         Number of root branches (default: 10)
--threshold        Survivor score threshold (default: 0.6)
--novel-threshold  Novel pool admission threshold (default: 0.85)
--ratio            Background/universal ratio (default: 0.5)
--draws            Max draws per depth before moving on (default: 100)
--temperature      LLM temperature for generation (default: 1.0)
--objects          Objects per thought experiment atom (default: 4)
--relationships    Relationships per thought experiment atom (default: 3)
--properties       Properties per thought experiment atom (default: 2)
```

---

## Data Structures

### [Quad](#glossary)
```json
{
  "id": "uuid",
  "object_a": "string",
  "relationship": "string",
  "object_b": "string",
  "property": "string",
  "source": "background | universal | novel",
  "provenance": "random_id | null"
}
```

### [Node](#glossary)
```json
{
  "id": "uuid",
  "depth": "integer",
  "branch_id": "uuid",
  "thought_experiment": "string",
  "quads_used": ["quad_id"],
  "deutsch_score": "float",
  "unresolved_tension": "string",
  "score_history": ["float"],
  "accumulated_path": ["node_id"]
}
```

### [Branch](#glossary)
```json
{
  "id": "uuid",
  "parent_branch_ids": ["uuid | null"],
  "topic": "string",
  "nodes": ["node_id"],
  "current_depth": "integer",
  "depth_limit": "integer",
  "status": "active | terminated | cross-pollinated",
  "trajectory_score": "float | null"
}
```

### Draw Pool
```json
{
  "background": ["quad"],
  "universal": ["quad"],
  "novel": ["quad"],
  "ratio": {
    "background": "float",
    "universal": "float",
    "novel": "float"
  }
}
```

### Tree
```json
{
  "id": "uuid",
  "topic": "string",
  "topic_normalized": {
    "core_subject": "string",
    "core_tension": "string",
    "question_type": "causal | counterfactual | mechanistic | other"
  },
  "draw_pool": "DrawPool",
  "branches": ["branch_id"],
  "cross_pollinations": [
    {
      "branch_a": "uuid",
      "branch_b": "uuid",
      "new_branch": "uuid",
      "depth_at_merge": "integer"
    }
  ],
  "top_trajectories": ["branch_id"]
}
```

---

## LLM Prompt Templates

### Prompt 1 — Fact Generation (Background Pool Initialization)
*Temperature: high (nondeterministic)*

```
You are a knowledgeable research assistant with broad expertise.

Topic: {topic}

Generate the 50 most relevant facts, principles, known results,
and contested claims about this topic. Be specific. Include
edge cases, surprising findings, and unresolved questions.

Also list the 5 best external resources where deeper knowledge
can be found (journals, papers, books, databases).

Format your response as:
FACTS:
1. [fact]
2. [fact]
...

RESOURCES:
1. [resource]
...
```

---

### Prompt 2 — [Quad](#glossary) Extraction
*Temperature: low (deterministic)*

```
Extract all objects, relationships, and properties from the
following text.

An object is a person, place, thing, or idea.
A relationship is any interaction or structural bond between
two objects — any direction, active or static.
A property is a quality put under pressure by the relationship
between two objects.

Return JSON only. No preamble. No markdown.

Format:
[
  {
    "object_a": "string",
    "relationship": "string",
    "object_b": "string",
    "property": "string"
  }
]

Text: {facts_text}
```

---

### Prompt 3 — [Thought Experiment](#glossary) Generation
*Temperature: high (nondeterministic)*

```
You are a creative scientific thinker generating a thought experiment.

Topic: {topic}

Using the following elements, generate a single hypothetical
thought experiment. Place an observer or object in an unusual
relationship with the other elements and ask what would happen
or what would be revealed.

Be specific. Be imaginative. Do not explain known results —
generate a scenario that forces a question.

Elements ({objects} objects, {relationships} relationships, {properties} properties):
Objects: {object_list}
Relationships: {relationship_list}
Properties: {property_list}

Prior path context (if any): {accumulated_path}
Unresolved tension (if any): {unresolved_tension}

Generate the thought experiment only. No preamble.
```

---

### Prompt 4 — [Coherence](#glossary) Filter
*Temperature: low (deterministic)*

```
Evaluate the following thought experiment against two criteria.

Topic: {topic}
Thought experiment: {thought_experiment}

Answer each question with YES or NO and one sentence explanation.

1. Is this thought experiment internally consistent —
   does it make sense on its own terms?

2. Does this thought experiment illuminate anything
   non-trivial about the topic?

Return JSON only:
{
  "internally_consistent": "YES | NO",
  "consistent_reason": "string",
  "topic_relevant": "YES | NO",
  "relevant_reason": "string",
  "passes": "true | false"
}
```

---

### Prompt 5 — [Deutsch Scorer](#glossary)
*Temperature: low (deterministic)*

```
Score the following thought experiment on its explanatory quality.

Topic: {topic}
Thought experiment: {thought_experiment}
Prior path: {accumulated_path}

Evaluate on these criteria:
- Hard to vary: would changing any element break the insight?
- Reach: does it suggest something beyond its immediate inputs?
- Minimal assumptions: does it avoid unnecessary complexity?
- Tension resolution: does it address something unexplained?

Return JSON only:
{
  "hard_to_vary": float (0-1),
  "reach": float (0-1),
  "minimal_assumptions": float (0-1),
  "tension_resolution": float (0-1),
  "overall_score": float (0-1),
  "justification": "string"
}
```

---

### Prompt 6 — [Unresolved Tension](#glossary) Extraction
*Temperature: low (deterministic)*

```
Read the following thought experiment in context of the
accumulated path.

Topic: {topic}
Accumulated path: {accumulated_path}
Current thought experiment: {thought_experiment}
Current score: {score}

Identify what remains unexplained, paradoxical, or unresolved
after this thought experiment. This is the tension the next
draw should attempt to discharge.

Be specific. Name the objects and relationships involved.

Return JSON only:
{
  "tension": "string",
  "objects_involved": ["string"],
  "relationships_involved": ["string"],
  "tension_type": "paradox | gap | contradiction | open_question"
}
```

---

### Prompt 7 — [Trajectory Score](#glossary)
*Temperature: low (deterministic)*

```
Read the following sequence of thought experiments generated
while exploring a topic.

Topic: {topic}
Full branch path (root to current):
{node_sequence}

Score the cumulative explanatory reach of this path as a whole.
Consider: how much closer to understanding the topic is this
path than when it started? Has understanding deepened
progressively or stalled?

Return JSON only:
{
  "trajectory_score": float (0-1),
  "depth_of_insight": "string",
  "best_node": "node_id",
  "justification": "string"
}
```

---

### Prompt 8 — [Cross-Pollination](#glossary) Candidate Detection
*Temperature: low (deterministic)*

```
Read the unresolved tensions from two branches at the same depth.

Topic: {topic}
Branch A tension: {tension_a}
Branch B tension: {tension_b}

Are these tensions complementary — would combining them into
a single thought experiment likely produce insight that neither
branch could reach alone?

Return JSON only:
{
  "complementary": "true | false",
  "reason": "string",
  "suggested_merge_angle": "string | null"
}
```

---

## Build Order

```
1.  Universal vocabulary          three files: objects, relationships, properties
2.  Quad extractor                corpus text → quads (Prompt 2)
3.  Single TE generator           4/3/2 draw → thought experiment (Prompt 3)
4.  Coherence filter              (Prompt 4)
5.  Deutsch scorer                (Prompt 5)
6.  Tension extractor             (Prompt 6)
7.  Single branch runner          steps 3–6 in a loop to depth 10
8.  Background pool initializer   (Prompts 1 + 2)
9.  Root branch generator         runs until N diverse branches found
10. Trajectory scorer             (Prompt 7)
11. Cross-pollination detector    (Prompt 8)
12. Parallel tree runner          orchestrates branches + cross-pollination
13. CLI wrapper                   user interaction layer
```

---

## Glossary

<a name="glossary"></a>

| Term | Definition |
|---|---|
| **Object** | A person, place, thing, or idea |
| **Relationship** | Any interaction or structural bond between two objects — any direction, active or static |
| **Property** | The quality put under pressure by the relationship between two objects |
| **Quad** | The atomic unit of knowledge: (object A) + (relationship) + (object B) + (property) |
| **Thought experiment** | An LLM-generated hypothetical relying on LLM latent space |
| **Coherence** | LLM judgment that a thought experiment is internally consistent and non-trivially related to the topic |
| **Survivor** | A thought experiment scoring above the configurable threshold |
| **Deutsch score** | LLM judgment of explanatory quality — hard to vary, reaches beyond inputs, minimal assumptions |
| **Unresolved tension** | LLM judgment of what remains unexplained or paradoxical in a node |
| **Node** | The full state at a given depth: accumulated path + current survivor + unresolved tension + score history |
| **Branch** | A sequence of nodes generated during a single draw lineage |
| **Trajectory score** | LLM judgment of cumulative explanatory reach across the full branch path from root to current node |
| **Draw** | Sampling from topic + path + quads across the three pools |
| **Background pool** | Quads extracted from LLM-generated corpus facts about the topic |
| **Universal pool** | Quads built from cross-domain vocabulary (objects, relationships, properties randomly combined) |
| **Novel pool** | Quads earned by high-scoring survivors during the tree run |
| **Provenance** | A random ID assigned to each novel quad at creation time |
| **Depth limit** | Maximum number of nodes per branch (default: 10) |
| **Cross-pollination** | Merging two branches with complementary unresolved tensions into a new branch with a reset depth counter |

---

*Last updated: work in progress*
