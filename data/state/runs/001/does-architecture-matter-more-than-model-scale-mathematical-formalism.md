# Generated: does-architecture-matter-more-than-model-scale × mathematical-formalism

## Conjecture

Model the situation as a two-layer system.

Let \(S\) denote substrate capacity: parameters, compute, context length, tool access, memory bandwidth. Let \(E\) denote epistemic structure: the rules by which hypotheses are generated, criticized, selected, retained, and reused. Let \(P\) be epistemic progress: the rate at which error is removed from a body of knowledge.

A natural structural claim is that \(P\) is not additive, \(P \neq f(S)+g(E)\), but constrained by interaction:
\[
P \leq \min\{C(S),\, T(E)\}
\]
where \(C(S)\) is capacity to represent and search, and \(T(E)\) is capacity to produce truth-tracking variation and criticism. This is a bottleneck law: progress is upper-bounded by the narrower channel.

But the deeper mathematical point is that the two terms are not symmetric. Increasing \(S\) tends to scale volume: more candidates, longer chains, broader retrieval, lower noise. Improving \(E\) changes topology: which conjectures are reachable, which errors are exposed, which attractors dominate learning and deployment.

If the space of possible hypotheses is vast and deceptive, then raw substrate mostly increases sampling from the same local basin. In that case,
\[
\frac{\partial P}{\partial S} \to 0
\quad \text{when} \quad
E \text{ preserves bad search/selection invariants.}
\]
Examples of such invariants: reward proxies mistaken for criticism, benchmark overfitting, premature convergence on stylistically plausible outputs, and weak retention of refutations. These are structural constraints; more compute only traverses them faster.

By contrast, changes in \(E\) can alter the effective dimensionality of search. If better criticism, error localization, adversarial testing, and memory of refutations are introduced, then the same substrate can access qualitatively different regions of hypothesis space. Formally, \(E\) acts like a transformation on the search operator itself, not merely its budget.

So the conjecture is:

**Conjecture:** Beyond a modest capability threshold, the principal bottleneck on AI epistemic progress is structural rather than substrate-level. Once \(S\) is sufficient to represent and manipulate rich conjectures, further gains in \(P\) depend mainly on whether the overlying system implements error-correcting epistemic dynamics — generation of bold alternatives, severe criticism, preservation of refutations, and promotion by explanatory success rather than proxy performance.

What follows is not that substrate is unimportant. Below threshold, \(S\) is binding. But above threshold, returns to \(S\) are diminishing unless accompanied by structural changes in \(E\). In systems terms: substrate scales throughput; structure determines whether throughput is knowledge-producing or merely output-producing.

The non-negotiable constraint is this: epistemic progress requires variation plus selective error correction. If the architecture above the model does not instantiate that invariant, no amount of scale guarantees progress; if it does, even limited substrate can produce surprisingly strong gains.

## Questions

1. 1. Does the conjecture require a real capability threshold in S beyond which rich conjectures are already representable, rather than allowing the same structural-bottleneck claim to hold at all substrate levels? — **yes**
2. 2. If replacing the bottleneck law P ≤ min{C(S), T(E)} with an additive or smoothly compensatory relation still explained the problem equally well, would that undermine the conjecture's core explanatory structure? — **yes**
3. 3. Is the asymmetry between S scaling search volume and E changing search topology essential, such that treating S and E as interchangeable capacity sources would destroy the explanation? — **yes**
4. 4. Does the claim depend on deceptive hypothesis spaces where more substrate mostly samples the same local basin, rather than on benign spaces where brute-force scaling reliably finds better hypotheses? — **yes**
5. 5. Are the listed bad invariants in E—proxy reward, benchmark overfitting, stylistic plausibility, weak refutation memory—meant to be load-bearing examples of why ∂P/∂S → 0 above threshold, rather than incidental illustrations? — **yes**
6. 6. Would the conjecture fail if increased substrate alone could break those bad search and selection invariants without any change to the overlying epistemic rules? — **yes**
7. 7. Is it essential that E transforms the search operator itself—changing reachability, criticism, and retention—rather than merely reallocating more budget within a fixed search procedure? — **yes**
8. 8. Does the explanation require preservation of refutations as a distinct mechanism in E, such that removing long-term memory of failed ideas would materially weaken the claimed source of epistemic progress? — **no**
9. 9. Is promotion by explanatory success rather than proxy performance a necessary part of the conjecture, rather than one replaceable implementation detail among many equally good selection rules? — **no**
10. 10. Would the conjecture be seriously weakened if there were broad regimes above the modest S threshold where scaling model size and compute continued to produce near-proportional gains in P without structural changes in E? — **yes**

## Candidate Problems

- How can the conjecture’s key variables be operationalized and measured so it becomes falsifiable rather than metaphorical? In particular: what concrete metrics correspond to substrate capacity S, epistemic structure E, and epistemic progress P; how do we distinguish 'truth-tracking criticism' from proxy optimization; and what empirical signature would show a genuine phase change from substrate-limited to structure-limited regimes? (score: 0.97)
- Is there in fact a capability threshold beyond which marginal gains from substrate collapse unless epistemic structure changes, and under what conditions does this threshold exist? The conjecture assumes a regime shift, but it is unresolved whether this is universal, domain-specific, or absent in practice because larger substrate may itself induce new effective epistemic structure (for example via emergent search, tool use, or internal self-critique). (score: 0.95)
- What formal properties of epistemic structure E actually change the topology of search rather than merely improving efficiency within the same basin? This raises an open question about which mechanisms—adversarial testing, memory of refutations, diversity of conjecture generation, explanatory selection, debate, external verification—produce qualitatively new reachable hypotheses and error-correction dynamics, and which only simulate progress through better benchmark performance. (score: 0.93)
