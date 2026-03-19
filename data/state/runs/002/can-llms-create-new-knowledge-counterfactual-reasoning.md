# Generated: can-llms-create-new-knowledge × counterfactual-reasoning

## Conjecture

**Conjecture:** The load-bearing issue is not “token prediction” versus “understanding,” but whether the system contains a **structure for conjecture-and-criticism that can create new problem framings and eliminate error**. If you vary that assumption, the picture changes sharply.

Start by removing the assumption that knowledge creation requires searching a huge word-space. Then the search-space argument weakens. Einstein did not discover relativity by exploring token combinations; he discovered it by operating in a **highly compressed space of explanatory constraints**: Maxwell’s equations, invariance, simultaneity, observer symmetry, thought experiments. So if an LLM-plus-architecture narrows search using such constraints, that is not a trick added after the fact; it is exactly what makes knowledge creation possible in humans too. What survives this variation is the idea that brute-force combinatorics is mostly incidental.

Now invert a different assumption: suppose prediction is sufficient by itself. Then what follows? A next-token system trained on past text should be able to yield genuine knowledge merely by sampling from its latent prior. But this breaks on the Deutschian criterion. Past-regularity fitting can produce plausible continuations, analogies, and recombinations, but without a mechanism for **posing rival explanations and criticizing them for being easily variable**, it has no reason to converge on better explanations rather than merely familiar-looking ones. So prediction alone appears non-load-bearing.

Replace the assumption “LLMs create knowledge” with its opposite: “architectures create knowledge when they use LLMs as components.” This survives much better. If the LLM is embedded in a system that:
1. generates candidate explanations,
2. subjects them to criticism under explicit constraints,
3. preserves error-corrected improvements,
4. recursively changes the search basis itself,

then the relevant unit is no longer “a token predictor” but an **evolutionary epistemic system**. In that case, saying “the LLM only rearranges what it has seen” is like saying a mathematician only rearranges symbols. Rearrangement is not disqualifying; the question is whether the rearrangement is governed by criticism that yields explanations that are harder to vary and reach beyond the training data.

What breaks if you remove criticism? Novelty becomes cheap but empty. What breaks if you remove human or system-level judgment? Search drifts into stylistic plausibility. What survives every variation is this: **knowledge creation depends on selective elimination of bad explanations, not on raw generative abundance**.

So the collision illuminates a sharper claim: **LLMs, as predictors, do not by themselves create knowledge; but systems that use LLMs inside a conjecture-and-criticism loop plausibly can.** The decisive threshold is not scale, nor novelty of wording, but whether the architecture can generate and reject explanations according to epistemic criteria in a way that improves the space of future conjectures. That is where genuine knowledge would begin.

## Questions


## Candidate Problems

- What specific architectural mechanisms can implement conjecture-and-criticism in an LLM-based system such that they eliminate errors rather than merely re-rank outputs for plausibility? (score: 0.91)
- Can an LLM-based system improve the space of future conjectures through retained criticism without human intervention, and under what measurable conditions would that count as knowledge creation? (score: 0.88)
- Are there domains where token prediction alone, without explicit rival-generation and criticism modules, nevertheless yields genuinely hard-to-vary explanations? (score: 0.78)
