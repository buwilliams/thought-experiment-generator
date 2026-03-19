# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

**Conjecture:** The decisive variable is not whether the system predicts tokens, but whether its architecture implements a **search over explanatory structures under nontrivial invariants**. If so, token prediction can be merely the coordinate system in which a deeper combinatorial process is expressed.

Formally, let \(X\) be the space of candidate outputs as strings, and let \(S\) be the quotient of \(X\) by an equivalence relation of **explanatory sameness**: two strings are equivalent if they instantiate the same objects, relations, constraints, and inferential consequences. Einstein’s journal and many paraphrases are different points in \(X\) but nearly the same point in \(S\). The raw token-space cardinality argument greatly overestimates the relevant search space, because most variation is syntactic, not structural.

So the real problem is: can an LLM-based system traverse \(S\) to find points not already in its training support? That depends on the presence of operators that preserve explanatory validity while transforming representation. Examples: abstraction, analogy, decomposition into thought-experiment objects/relations, recombination, and criticism by constraints such as consistency, reach, and “hard-to-vary” structure.

This suggests a mathematical distinction between two regimes:

1. **Interpolation regime:** the system samples within a dense neighborhood of known explanatory structures.
2. **Generative regime:** the system composes transformations \(T_1, T_2, \dots, T_n\) on known structures to reach an equivalence class in \(S\) not previously instantiated, while satisfying invariant constraints \(C\).

Knowledge creation occurs if there exists a candidate \(s' \in S\) such that:
- \(s'\) is not reducible to memorized or near-neighbor forms,
- \(s'\) explains more than its predecessors (greater reach/compression),
- \(s'\) survives criticism under the constraint set \(C\),
- and it enters the future search process as a new usable invariant-bearing object.

On this view, “predicting tokens” is compatible with creating knowledge, just as writing symbols is compatible with doing mathematics. The relevant question is whether the system has access to a **structured state space** and **selection dynamics** strong enough to move toward explanatory fixed points.

The search-space argument helps, but only after translation: not “the token space is too large, so brute force fails,” but “successful discovery requires a sequence of mappings that collapse dimensionality by exploiting invariants.” Grammar, coherence, ontology extraction, object-relation graphs, and Deutschian criticism are all dimension-reducing projections. Their role is to induce a tractable submanifold of \(S\).

So the illuminating claim is: **LLMs alone do not create knowledge by next-token prediction; but an architecture that uses LLMs as generators inside a mathematically constrained search-and-criticism loop can, in principle, create genuine knowledge.** The threshold is reached when the system’s internal filters track explanatory invariants well enough that novelty is produced in structure-space, not merely wording-space.

## Questions


## Candidate Problems

- Define a non-circular equivalence relation of explanatory sameness on strings that distinguishes structural identity from paraphrase and test whether independently generated explanations map to the same class. (score: 0.89)
- Specify measurable invariant constraints C that can discriminate interpolation in structure-space from genuinely novel explanatory classes, rather than merely novel wording. (score: 0.92)
- Determine whether abstraction, analogy, decomposition, recombination, and criticism can be formalized as operators on S that are closed enough to support search without collapsing distinct explanations into one class. (score: 0.83)
- Identify a concrete architecture in which LLM-generated candidates enter a search-and-criticism loop and produce a structure that was absent from training support yet survives external criticism. (score: 0.90)
