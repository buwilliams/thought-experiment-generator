# Generated: does-architecture-matter-more-than-model-scale × systems-thinking

## Conjecture

**Conjecture:** The bottleneck is primarily **structural, not substrate-level**: epistemic progress in AI depends less on making the model bigger and more on building a system of **selective feedback loops** that generates, tests, and promotes candidate explanations. In systems terms, model size is a stock of representational capacity; knowledge creation requires flows and feedback that reorganize that stock into a discovery process.

Your search-space argument already points there. The impossibility of brute-force traversal means the relevant question is not “does the substrate contain enough tokens/parameters?” but “what structures reduce the effective search space?” A larger model increases the reservoir of patterns, but without the right feedback architecture it mainly increases the volume of recombination, not the rate of explanatory progress.

From a systems perspective, the crucial variables are:

- **Stocks:** background knowledge, candidate conjectures, evaluation criteria, discovered heuristics.
- **Flows:** proposal generation, criticism, retention, recombination.
- **Feedback loops:** candidates that survive criticism reshape the generator and evaluator, changing what gets proposed next.

On this view, an LLM by itself is not a knowledge-creating system; it is one component in a larger epistemic system. The leverage point is not “more outputs per second” but **changing the selection structure** so that bad conjectures are pruned early, promising structures are amplified, and successful explanatory patterns are fed back into future search.

This reframes the dispute with “inductive closure.” If the system is only sampling within a learned distribution, then closure is a real limit. But if the architecture adds new feedback loops—explicit object/relation decompositions, thought-experiment generation, Deutsch-style criticism, iterative retention of superior explanatory forms—then the system is no longer just traversing a static distribution. It becomes a structured search process whose behavior is determined by how criticism reshapes future generation.

What follows is that “can LLMs create knowledge?” is partly the wrong unit of analysis. The more illuminating question is: **can we build an AI-centered epistemic system whose feedback structure causes explanatory novelty to accumulate?** On that question, your proposal is plausible. The likely threshold is not mystical emergence from scale, but a systems transition: once the evaluator-generator-retention loop becomes strong enough that each cycle improves the next cycle’s search bias, the system may begin producing explanations that are genuinely new relative to its prior outputs and training-derived defaults.

So the collision of systems thinking with this problem yields a sharper claim: **compute and model size are enabling conditions, but the main leverage for epistemic progress lies in the architecture of recursive criticism and selective retention above the model.** If that architecture is weak, more substrate mostly produces more variation. If it is strong, even a modest substrate may participate in cumulative knowledge growth.

## Questions


## Candidate Problems

- Across what task regimes does architectural feedback dominate model scale, and where does increased substrate become the primary bottleneck instead? (score: 0.93)
- What measurable properties of an evaluator-generator-retention loop predict cumulative explanatory improvement rather than just increased output filtering? (score: 0.91)
- Can explicit object/relation decomposition and iterative criticism break 'inductive closure' in practice, or do they merely re-express the same learned distribution in a more structured form? (score: 0.89)
- Is there a quantifiable threshold at which recursive criticism begins improving future search bias cycle-to-cycle, and how does that threshold scale with model capability? (score: 0.90)
