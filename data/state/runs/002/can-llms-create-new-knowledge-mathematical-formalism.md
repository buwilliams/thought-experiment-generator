# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

**Conjecture:** The relevant question is not whether a token-predictor can “search” the astronomically large space of strings, but whether it can implement a sequence of **structure-preserving transformations** on a much smaller space of **explanatory objects**. If explanations are modeled as constrained graphs rather than word strings, then genuine knowledge creation becomes possible when the system can generate and select graph structures that satisfy invariants not explicitly present in its training examples.

More formally: let an explanation be represented by a tuple  
\[
E=(O,R,P,C)
\]
where \(O\) are objects, \(R\) relations, \(P\) properties, and \(C\) constraints linking them. The linguistic surface is then a many-to-one map
\[
L: E \to \text{text}
\]
so the huge combinatorial space of token sequences is mostly irrelevant. What matters is the cardinality and topology of the space \(\mathcal{E}\) of candidate explanatory structures under admissible constraints.

Einstein’s achievement can be seen as a transformation on \(\mathcal{E}\): he preserved certain invariants (Maxwell’s equations, empirical symmetry, consistency between observers) while varying background assumptions (absolute simultaneity, fixed time). The novelty was not new words but a new **constraint-satisfying structure**. On this view, “knowledge creation” means discovering an \(E^\*\in\mathcal{E}\) that simultaneously:

1. **Satisfies known constraints** better than rivals,
2. **Compresses** disparate phenomena under one structure,
3. **Resists arbitrary variation** (“hard to vary”),
4. **Generates new consequences** under transformation.

An LLM alone mostly samples from \(L(E)\), the surface distribution. But an architecture that explicitly searches over \(\mathcal{E}\), using the LLM as one operator among others, can do more. The crucial mathematical shift is from unconstrained sequence generation to **constraint-guided search in a structured state space**. This is analogous to moving from brute-force enumeration to solving a constrained optimization problem:
\[
\arg\max_{E \in \mathcal{E}} Q(E)
\]
where \(Q\) scores explanatory adequacy, invariance, novelty, and criticizability.

So the conjecture is:

> **A predictive language model can participate in genuine knowledge creation if embedded in an architecture that searches over structured explanatory representations and evaluates candidates by invariant-preserving, constraint-sensitive criteria.**  
> In that setting, novelty is not mere rearrangement of tokens, but discovery of previously uninstantiated structures in \(\mathcal{E}\) whose linguistic realization may be new or old.

This also clarifies the limit. If the system only maps text to likely text, then it remains in the image of \(L\) and mostly rearranges known surfaces. But if it can move in \(\mathcal{E}\) by proposing transformations that preserve deep constraints while altering assumptions, then it can produce explanations that are mathematically new in the relevant sense.

So the collision of perspectives yields an illumination: **knowledge creation is not about traversing a vast word space; it is about navigating a constrained manifold of explanatory forms.** The open empirical question is whether current systems can represent and search that manifold well enough.

## Questions


## Candidate Problems

- Specify a concrete representation for explanatory objects E=(O,R,P,C) and test whether systems searching that space can generate structures satisfying constraints not explicitly instantiated in training data. (score: 0.88)
- Determine whether remaining in the image of the language map L:E->text is sufficient to block genuine novelty, or whether novel explanatory structure can be implicitly realized without explicit nonlinguistic search over E. (score: 0.82)
- Define and operationalize Q(E) so that explanatory adequacy, invariance, novelty, and criticizability can be jointly evaluated without collapsing into surface plausibility. (score: 0.85)
- Identify transformations on explanatory graphs that preserve deep constraints while varying assumptions, and test whether current LLM-based systems can reliably traverse them. (score: 0.80)
