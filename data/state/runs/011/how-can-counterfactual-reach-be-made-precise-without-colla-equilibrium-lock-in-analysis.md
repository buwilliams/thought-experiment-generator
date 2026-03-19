# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × equilibrium-lock-in-analysis

## Conjecture

**Conjecture:** *Counterfactual reach* can be made precise as a **structural option-value property of a representation**: a model has high counterfactual reach to the extent that, from its current internal structure, it supports many **mutually compatible, policy-relevant interventions** whose consequences can be derived without rebuilding the model from scratch. What distinguishes it from prediction, compression, causal modeling, or invariance is not any one output, but the **breadth of neighboring “what-if” worlds that remain tractably navigable under local change**.

From the equilibrium lock-in perspective, the key illumination is this: existing notions each capture a system optimized for one-agent unilateral success in a fixed game.

- **Prediction** rewards accuracy on the realized path.
- **Compression** rewards short description of observed regularities.
- **Causal modeling** rewards correct intervention semantics.
- **Invariance** rewards stability across environments.

But counterfactual reach is needed when an agent faces a **structural trap**: no single next-step prediction or single intervention suffices, because useful action depends on understanding a space of alternative equilibria. A representation can be excellent by standard metrics yet still leave the agent trapped, because it does not expose reachable alternatives.

So the structural criterion should be something like:

> A representation has counterfactual reach iff small, explicit modifications to a bounded subset of its assumptions generate a large set of coherent downstream scenarios, with reusable inferential machinery and preserved identity of relevant variables across scenarios.

Each clause is load-bearing:

- **Small, explicit modifications** prevents collapse into unconstrained imagination.
- **Bounded subset of assumptions** distinguishes it from total model rewrite.
- **Large set of coherent downstream scenarios** distinguishes it from mere prediction.
- **Reusable inferential machinery** distinguishes it from brute-force simulation.
- **Preserved identity of relevant variables** distinguishes it from lossy compression or arbitrary latent remapping.

Why this perspective matters: equilibrium lock-in tells us that the value of a model is not just whether it predicts the current equilibrium, but whether it reveals **which constraints are equilibrium-maintaining and which are merely contingent**. Counterfactual reach is therefore highest in representations that let us test: if agent A changed strategy alone, nothing improves; if rule R changed, many agents could jointly escape. In that sense, counterfactual reach measures a model’s ability to **differentiate local deviations from structural transformations**.

This avoids collapse into causal modeling because not every causal model has high reach: some encode one intervention at a time but do not support wide recombination of altered assumptions. It avoids collapse into invariance because reach concerns **systematic navigability under change**, not stability under distribution shift. It avoids collapse into compression because a compressed description may omit exactly the slack needed to explore nearby alternatives. And it avoids collapse into prediction because one can predict a lock-in perfectly while being unable to represent exits from it.

So the conjecture is: **counterfactual reach is best formalized as the extent to which a representation preserves actionable adjacency among possible worlds**—especially adjacency that exposes whether observed outcomes are equilibrium-locked or transformable.

## Questions

1. 1. If the requirement that interventions be mutually compatible were removed, would the conjecture lose its explanation of why counterfactual reach is about a navigable space of alternatives rather than a bag of isolated what-if results? — **yes**
2. 2. Is the claim that only small explicit modifications to a bounded subset of assumptions count as evidence of reach necessary to keep the proposal from collapsing into total model rebuilding? — **yes**
3. 3. If preserved identity of relevant variables across scenarios were dropped, would the conjecture still explain how a representation exposes actionable adjacency among possible worlds rather than merely producing disconnected latent redescriptions? — **no**
4. 4. Is reusable inferential machinery required for the conjecture to distinguish high counterfactual reach from brute-force simulation of many separate scenarios? — **yes**
5. 5. Does the conjecture imply that a model could score well on prediction, compression, causal semantics, and invariance yet still fail specifically in equilibrium lock-in cases because it cannot represent exits from the current equilibrium? — **yes**
6. 6. If the conjecture is right, should representations that reveal which constraints are equilibrium-maintaining versus contingent also illuminate when coordinated rule changes outperform any unilateral strategy change in problems beyond the one described here? — **yes**
7. 7. Does defining reach as preserved actionable adjacency predict that two models with equal causal correctness can differ systematically in usefulness for exploring nearby alternative equilibria? — **yes**
8. 8. If a counterexample showed a model with high reach despite requiring a fresh remapping of variables in each scenario, would saving the conjecture force abandonment of the preserved identity condition rather than a minor qualification? — **yes**
9. 9. If one produced a causal model that supports many single interventions but no wide recombination of altered assumptions, would rescuing the conjecture require changing its core claim that breadth of coherent neighboring worlds is essential? — **no**
10. 10. If brute-force simulation from scratch over many scenarios appeared to match the same practical outputs as a high-reach representation, would preserving the conjecture require giving up the claim that reusable internal structure is constitutive rather than incidental? — **no**

## Candidate Problems

- How can 'counterfactual reach' be formalized non-circularly and measured in a way that does not collapse into existing notions like causal identifiability, model class closure, planning affordances, or computational tractability? The unresolved tension is whether the proposed clauses jointly define a genuinely new property or merely a bundle of familiar ones under a new label. (score: 0.97)
- What determines the identity and admissibility of 'small, explicit modifications' and 'preserved identity of relevant variables' across neighboring worlds? This is open because the conjecture depends on a stable cross-world representation, but it does not yet specify the ontology or matching criterion that says when two scenarios are comparable rather than being covert model rewrites or arbitrary latent remappings. (score: 0.93)
- Under what conditions does high counterfactual reach actually improve action in equilibrium-locked systems rather than just enlarging the space of imaginable scenarios? The key tension is whether breadth of navigable alternatives tracks real leverage for multi-agent structural change, or whether it can be high in representations that are descriptively rich but strategically useless because incentives, coordination barriers, or hidden constraints still dominate. (score: 0.91)
