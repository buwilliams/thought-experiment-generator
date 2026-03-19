# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × computational-irreducibility

## Conjecture

**Conjecture:**  
*Counterfactual reach can be made precise as the extent to which a representation supports the reliable generation of many nontrivial neighboring possibilities by preserving a system’s minimal generative structure, even when the consequences of those possibilities are computationally irreducible. Its core criterion is not predictive accuracy, compression length, causal identifiability, or invariance under transformation, but whether the representation encodes the right rule-level constraints so that counterfactuals can be instantiated and explored without re-deriving the world from scratch.*

From the perspective of computational irreducibility, this matters because in many domains there is no shortcut from description to outcome. If so, “knowing what would happen if X were different” cannot mean possessing a compact predictor that jumps directly to results. In irreducible systems, the best one can often do is run the process under altered conditions. So a representation has counterfactual reach not when it summarizes outcomes, but when it preserves the **load-bearing generative structure** needed to produce altered trajectories.

That gives a structural criterion:

- A representation has **high counterfactual reach** iff small, interpretable interventions on it correspond to coherent alterations in the underlying generative process, and those alterations can be propagated by the same rule set across a broad class of nearby cases.
- A representation has **low counterfactual reach** if answering each “what if?” requires bespoke reconstruction, external patching, or a new model.

This avoids collapse into nearby notions:

- **Not prediction:** prediction asks for actual outcomes; counterfactual reach asks whether one can systematically *generate* alternate outcomes, even if only by full simulation.
- **Not compression:** a lookup table may compress poorly yet have reach if it preserves the rule; a compressed summary may predict well in-distribution yet have little reach if interventions break it.
- **Not causal modeling simpliciter:** causal graphs encode dependencies, but counterfactual reach concerns whether the encoded structure is generative enough to support many concrete alternative evolutions in irreducible settings.
- **Not invariance:** invariances help generalization, but reach depends on manipulable structure, not merely stable features.

So the “b” in the problem is plausibly: **breadth under bounded reinterpretation**. A precise measure would track how many structurally neighboring counterfactuals a representation supports before requiring new primitive assumptions. The larger the neighborhood of coherent interventions supported by the same underlying rule set, the greater the counterfactual reach.

What this illuminates is that counterfactual reach is fundamentally a property of **model architecture relative to intervention space**, not of accuracy alone. In irreducible domains, the highest-reach representations are those closest to the minimal simulator: not necessarily shortest, simplest, or most predictive, but the ones whose internal parts are load-bearing enough that changing them yields meaningful alternative worlds. Unpredictability does not negate counterfactual reach; it relocates it from shortcutting outcomes to preserving the structure that makes alternative outcomes computable at all.

## Questions

1. 1. If the claim that computational irreducibility blocks shortcut prediction were removed, would the conjecture lose its reason for defining counterfactual reach in terms of preserving generative structure rather than predictive summaries? — **yes**
2. 2. Is the requirement that the representation preserve a system's minimal generative structure necessary for the conclusion that counterfactual reach is distinct from prediction, compression, causal modeling, and invariance? — **yes**
3. 3. If small, interpretable interventions on the representation did not map to coherent alterations in the underlying process, would the proposed criterion for high counterfactual reach fail rather than merely become weaker? — **yes**
4. 4. Does the conclusion that breadth under bounded reinterpretation is the right structural criterion depend essentially on the claim that the same rule set must propagate altered conditions across nearby cases? — **yes**
5. 5. Does the conjecture imply that in an irreducible domain a full simulator with manipulable internal state can have higher counterfactual reach than a more accurate but non-generative predictor, even though the problem only asked for a definition? — **yes**
6. 6. Does the proposed criterion illuminate why two models with similar predictive accuracy could differ sharply in usefulness for intervention-heavy planning tasks not explicitly mentioned in the problem? — **yes**
7. 7. If the conjecture is right, should it also classify a compressed summary that predicts well in distribution but breaks under intervention as low in counterfactual reach in cases beyond the examples given? — **yes**
8. 8. If a counterexample showed that a purely predictive representation supports many nearby what if questions without encoding generative rules, would saving the conjecture require abandoning its claim that reach is not prediction? — **yes**
9. 9. If a causal graph with standard structural equations already handled broad concrete alternative evolutions in an irreducible setting, would preserving the conjecture force a major revision of its claim that counterfactual reach is not causal modeling simpliciter? — **yes**
10. 10. If a representation preserved only high level invariants yet still supported coherent exploration of many neighboring possibilities, would rescuing the conjecture require gutting its insistence on load bearing rule level structure rather than adding a small qualification? — **yes**

## Candidate Problems

- How can 'counterfactual reach' be made into a non-circular, operational measure of 'breadth under bounded reinterpretation' without smuggling in the very notions it aims to differ from—especially choice of intervention space, neighborhood structure, 'minimal generative structure,' and what counts as requiring 'new primitive assumptions'? (score: 0.97)
- What is the principled relationship between counterfactual reach and representation equivalence: when do two representations encode the same load-bearing generative structure, and how can we tell whether a representation is genuinely preserving rule-level constraints versus merely being transform-equivalent, simulator-dependent, or patchable by hidden external machinery? (score: 0.91)
- Is there a real tradeoff or compatibility frontier between counterfactual reach and other desiderata such as predictive accuracy, compression, causal abstraction, interpretability, and computational cost—and under computational irreducibility, when is preserving a near-minimal simulator actually the best or only high-reach architecture rather than an unnecessarily low-level one? (score: 0.89)
