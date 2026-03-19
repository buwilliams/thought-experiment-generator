# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × iterated-game-dynamics

## Conjecture

**Conjecture:**  
*Counterfactual reach can be made precise as the depth and breadth of strategically relevant continuation spaces an agent’s model can preserve across repeated interaction, where the key test is not whether the model predicts well, compresses data, tracks causes, or remains invariant, but whether it supports robust policy selection under recursively anticipated response.*

From the perspective of iterated game dynamics, the structural distinction is this: in repeated interaction, what matters is not just forecasting the next move, but representing how present action reshapes the future game tree through reputation, retaliation, forgiveness, trust, and altered expectations. A model has **greater counterfactual reach** when it can evaluate more of these downstream, interaction-mediated branches in a way that changes what policy is rational now.

This avoids collapse into nearby notions:

- **Not mere prediction:** a model might predict short-run behavior accurately while failing to represent how one’s own move changes later incentives. Myopic prediction sees the next action; counterfactual reach sees the altered continuation game.
- **Not compression:** a compact summary may fit past play but discard latent retaliatory or cooperative pathways that become decision-relevant once one intervenes.
- **Not standard causal modeling:** causal models track intervention effects on variables, but counterfactual reach here concerns intervention effects on *strategic response structure* across iterated horizons. The object is not only “what causes what,” but “what future games become playable.”
- **Not invariance:** invariance seeks what stays stable across environments; counterfactual reach measures the ability to reason through trajectories precisely where policy changes the environment by changing others’ expectations.

So the proposed structural criterion is:

> **A representation has counterfactual reach to the extent that altering the agent’s current policy induces a nontrivial, modelable reorganization of future strategic possibilities, and those reorganizations are load-bearing for present choice.**

This suggests an operational test. Compare two models that perform similarly on prediction/compression/causal benchmarks. The one with greater counterfactual reach will better distinguish policies whose immediate effects are similar but whose effects on future reciprocity regimes differ sharply: e.g., opportunistic defection, costly signaling, calibrated forgiveness, credible punishment. If removing the model’s representation of those continuation effects leaves present recommendations unchanged, then its alleged counterfactual reach was decorative, not structural.

What this illuminates is that counterfactual reach is fundamentally about the **shadow of the future**. It measures how far a model can propagate the consequences of action through chains of anticipated adaptation. In repeated games, cooperation exists because agents can reach counterfactually into futures where today’s move changes tomorrow’s strategic landscape. Shorten the horizon, obscure defection, or sever memory, and cooperation collapses. Likewise, a model’s counterfactual reach collapses when it cannot represent how current action changes future response possibilities.

So the core precision is: **counterfactual reach is horizon-sensitive strategic branching capacity**—the extent to which a model can support choice by preserving actionable distinctions among recursively generated futures.

## Questions

1. 1. Is the claim that present action reorganizes the future game tree through reputation, retaliation, forgiveness, trust, and altered expectations necessary for distinguishing counterfactual reach from mere next-move prediction in this repeated-game setting? — **yes**
2. 2. If the requirement that those future strategic reorganizations be load-bearing for present choice were removed, would the conjecture lose its structural criterion and collapse into decorative forecasting about possible continuations? — **yes**
3. 3. Is the focus on repeated interaction with recursively anticipated response necessary for the conclusion that counterfactual reach is not reducible to standard causal modeling of variable-level interventions? — **yes**
4. 4. Would the explanation fail rather than merely weaken if horizon-sensitive strategic branching capacity were replaced with a static account of one-shot intervention effects that ignores continuation spaces? — **yes**
5. 5. Does the conjecture imply that two models matched on prediction, compression, causal benchmarks, and invariance can still diverge systematically in policy quality when immediate payoffs are similar but future reciprocity regimes differ? — **yes**
6. 6. Does the explanation illuminate why cooperation should collapse when memory is severed, horizons are shortened, or defection is obscured, even though the problem only asked for a precision criterion for counterfactual reach? — **yes**
7. 7. Does defining counterfactual reach in terms of preserved continuation spaces suggest application beyond repeated games to any domain where current policy changes others' expectations and thereby alters later strategic options? — **yes**
8. 8. If a counterexample showed a model with excellent policy selection in repeated games despite not representing reputation or retaliation pathways, would saving the conjecture require abandoning the claim that continuation-space representation is the core of counterfactual reach? — **yes**
9. 9. If a standard causal model over variables could be extended to handle strategic response structure without changing its basic framework, would that undermine the conjecture's claim that counterfactual reach is not standard causal modeling rather than merely a special case of it? — **yes**
10. 10. If a highly compressed model preserved exactly the strategically relevant continuation branches needed for present choice, would defending the conjecture against collapse into compression require revising its core distinction rather than adding a minor qualification? — **no**

## Candidate Problems

- How can counterfactual reach be formalized in a way that is non-circular and quantitatively separable from existing notions like planning horizon, model-based value of information, recursive reasoning depth, and equilibrium refinement? The conjecture claims a distinct object—'strategic branching capacity'—but it remains open what exact mathematical quantity is being measured, what its units are, and which details are load-bearing rather than redescriptions of familiar constructs. (score: 0.97)
- What operational benchmark would isolate counterfactual reach as the causal source of better policy selection, rather than ordinary gains from better prediction, richer state representation, or larger compute? The conjecture proposes comparing models with similar predictive/compression/causal performance, but it is unresolved how to construct environments and interventions where continuation-structure representation alone explains the difference in present recommendations. (score: 0.95)
- Under what environmental and agent conditions does greater counterfactual reach actually improve outcomes, and when does it backfire? It is open whether more reach is always instrumentally valuable, or whether in noisy, nonstationary, partially observed, or manipulable repeated interactions it can induce overfitting to imagined strategic branches, excessive retaliation, or brittle cooperation. This matters because the conjecture treats reach as a virtue, but its value may depend on memory, observability, opponent sophistication, and institutional constraints. (score: 0.90)
