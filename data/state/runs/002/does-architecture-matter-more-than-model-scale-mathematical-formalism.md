# Generated: does-architecture-matter-more-than-model-scale × mathematical-formalism

## Conjecture

Conjecture: the bottleneck is primarily structural, not substrate-level, once the substrate exceeds a threshold sufficient to represent and manipulate the relevant abstractions.

Mathematically, the issue is not the cardinality of raw sequence space \(V^L\) (vocabulary \(V\), length \(L\)), but the geometry of a constrained search over equivalence classes of explanations. Most strings are not candidate explanations at all. The effective space is obtained by successive projections:

\[
V^L \to G \to C \to E
\]

where \(G\) is grammatical/coherent text, \(C\) is text instantiating a concept graph or thought-experiment schema, and \(E\) is the subset satisfying explanatory constraints. The decisive variable is the compression ratio at each projection, not the total size of \(V^L\).

This reframes “can LLMs create knowledge?” as a dynamical systems question. Let \(S_t\) be the current pool of candidate structures, \(F\) the generation operator, \(K\) the criticism/filter operator, and \(U\) the update rule that reincorporates successful structures into the prior:

\[
S_{t+1} = U(K(F(S_t)))
\]

Epistemic progress occurs if this process has an attractor toward increasingly invariant-preserving structures: explanations that survive transformations of wording, perspective, and context while preserving causal content. “Hard to vary” can be treated as approximate invariance under admissible transformations. A good explanation is one whose core relational structure remains stable under paraphrase, re-encoding, or recombination with neighboring concepts.

On this view, substrate scaling changes the capacity of \(F\): breadth of representable candidates, fluency, latent abstraction. But structure determines whether the iteration defines a knowledge-producing map at all. A larger model without the right \(K\) and \(U\) is just a wider sampler from bad equivalence classes. Conversely, a sufficiently capable substrate plus well-designed criticism can transform an intractable combinatorial space into a tractable search on structured manifolds.

So the non-negotiable constraint is not “more parameters” but “better operators on explanation-space.” The key invariant is retention of explanatory structure across transformations; the key transformation is from sequence generation to object–relation generation; the key relation is recursive feedback between conjecture and criticism.

What follows is a threshold claim: below some substrate capacity, the model cannot stably encode or traverse the relevant abstraction manifold, so structure cannot rescue it. Above that threshold, marginal gains from scale diminish relative to gains from architecture that explicitly represents objects, relations, criticism, and promotion rules.

So the collision of perspectives suggests this: AI epistemic progress is bottlenecked by the topology of the search process more than by the volume of the substrate. Compute enlarges reachable space; structure changes which regions are reachable, stable, and recursively improvable. Knowledge creation begins when the system searches over explanatory invariants rather than over strings.

## Questions


## Candidate Problems

- Locate the substrate-capacity threshold above which architectural operators K and U dominate marginal gains from scale, and below which structure cannot compensate for insufficient representational capacity. (score: 0.91)
- Measure the compression ratios in the projections V^L -> G -> C -> E and test whether improvements at later projections predict epistemic performance better than increases in raw model size. (score: 0.87)
- Formalize 'hard to vary' as invariance under admissible transformations and test whether systems optimized for that invariant outperform larger pure sequence models on explanation-generation tasks. (score: 0.84)
- Characterize when the iteration S_{t+1}=U(K(F(S_t))) has attractors toward better explanations versus degenerates into self-reinforcing but low-truth equivalence classes. (score: 0.89)
