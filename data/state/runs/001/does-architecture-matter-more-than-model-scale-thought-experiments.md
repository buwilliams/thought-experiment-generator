# Generated: does-architecture-matter-more-than-model-scale × thought-experiments

## Conjecture

**Conjecture:** If we isolate the key variable as **search structure rather than model substrate**, then the bottleneck for AI epistemic progress is not primarily bigger models or more compute, but whether the system contains an architecture that **generates, filters, and promotes candidate explanations under criticism**. In that scenario, an LLM need not “understand” in the full human sense to contribute to knowledge; it only needs to participate in a system whose structure makes good explanations more likely to survive than bad ones.

To see this, assume your search-space argument is basically right. Raw combinatorics makes brute-force discovery hopeless. That means the decisive question is not “Can the model sample enough outputs?” but “Can the system collapse the space before sampling becomes intractable?” Einstein did not search sentence space directly; he moved through a highly structured space of **objects, relations, invariances, and contradictions**. If an AI architecture can represent and traverse that same higher-order space, then the relevant search is no longer over 10^5000 word strings but over a much smaller space of explanatory transformations.

Follow the logic strictly: once search is structurally compressed, substrate improvements become secondary. A larger model helps only insofar as it supplies better priors or richer components. But priors alone do not explain discovery. What matters is whether the architecture can do three things:

1. **Form conjectures** by recombining structured elements into candidate explanations.
2. **Expose them to criticism** using constraints like consistency, reach, hard-to-vary structure, and empirical fit.
3. **Retain and reuse successes** so that the search space becomes progressively more shaped by prior discoveries.

That is already the minimal abstract pattern of knowledge growth. On this view, “LLMs create knowledge” is too coarse. The sharper claim is: **LLMs can be components in a knowledge-creating system if the surrounding structure implements conjecture and criticism.** Without that structure, scaling gives more fluent interpolation. With it, the same substrate can begin to produce explanatory novelty.

What this illuminates is a false dichotomy in the debate. “LLMs alone cannot create knowledge” and “AI systems can create knowledge” may both be true. The collision point is architectural. The substrate supplies generative raw material; the superstructure determines whether outputs are merely plausible or genuinely epistemic.

So the bottleneck is mainly **upstream of scale**: in the design of feedback loops that narrow search toward explanatory content. Compute matters, but mostly as fuel. The engine is the architecture of criticism, memory, and promotion. If that architecture is absent, more compute mostly expands variation. If it is present, even modest models may cross a threshold where the system begins to produce explanations that were not explicitly contained in its training set.

So the conjecture is: **epistemic progress in AI is bottlenecked less by substrate capacity than by whether the system has the right selective structure for explanation growth.**

## Questions


## Candidate Problems

- Under what task conditions can a smaller model with conjecture-criticism architecture outperform a much larger model lacking that structure on genuine explanation-generation rather than on proxy metrics? (score: 0.94)
- How much substrate capacity is minimally required before architectural search and criticism can operate over structured explanatory objects rather than collapsing into noise? (score: 0.90)
- Are there domains where scale itself effectively implements the needed search compression, making the architecture-vs-scale distinction weaker than the conjecture claims? (score: 0.86)
- Which feedback-loop designs for memory, criticism, and promotion actually produce cumulative epistemic progress instead of just better local selection among generated outputs? (score: 0.87)
