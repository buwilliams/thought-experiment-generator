# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × load-bearing-constraint

## Conjecture

Conjecture: An internal state genuinely functions as a model with correctness conditions iff its role in the wider control system is explanatorily load-bearing in a specifically counterfactual way: varying what the state says about a target domain, while holding fixed the downstream use-rules as far as possible, changes which environmental situations count as success, error, and correction for the system. If no such structured change is possible, the “model” talk is ornamental and the state is just part of a causal routine.

Why this is non-circular: the criterion does not define modelhood by saying the state “represents” or is “about” something. It asks whether the best explanation of the system requires a separable internal variable whose content can be wrong, and whose wrongness propagates through the system in systematic, compensable ways.

Load-bearing test:
1. Identify an internal state type S.
2. Identify a family of possible world states W that S is supposed to track.
3. Identify downstream policies P that consume S.
4. Ask: if we intervene on the mapping between W and S, but leave P intact, do we generate intelligible misperformance explainable as mis-specification rather than mere breakage?
5. Ask the converse: can we restore performance by compensating changes either in the W→S mapping or in P, in a way that preserves a stable space of possible errors?

If yes, S is model-like. Its specific content matters because changing it forces coordinated compensations elsewhere. That is exactly what it means for the detail to be load-bearing. The state has correctness conditions because there is a fact of the matter, within the explanatory structure, about which W would make S correct.

If no, then the state is not functioning as a model. It may still be causally indispensable, but only like a cam profile or trigger threshold: altering it changes behavior, yet there is no stable distinction between misrepresentation and malfunction. Any “aboutness” attributed to it is replaceable narrative padding.

Thought experiment: compare a thermostat’s bimetal strip with a robot’s internal map. For the strip, changing calibration mostly redescribes a control disposition; there is little stable error-structure beyond “too hot/too cold relative to design.” For the map, shifting landmarks while leaving route-planning fixed produces specific, diagnosable navigation errors, and those errors can be repaired either by correcting the map or by rewriting the planner to compensate. The map’s contents are therefore explanatorily load-bearing.

So the precise criterion is: modelhood begins where internal-state variation induces a structured space of possible misrepresentation and compensation. Correctness conditions are real when they are required to explain the system’s successes, failures, and repairs more tightly than a purely causal-control description can.

## Questions

1. 1. Is the claim that one must hold the downstream use-rules fixed as far as possible necessary for the conjecture to distinguish misrepresentation from mere redesign rather than collapsing into any behavior change counting as modelhood? — **yes**
2. 2. Is the requirement that varying what the internal state says changes which environmental situations count as success, error, and correction necessary for the conclusion that correctness conditions are real, rather than just useful descriptions? — **yes**
3. 3. If the conjecture dropped the converse repair test that performance can be restored by compensating changes in either the world-to-state mapping or the downstream policy, would its explanation of modelhood lose the stable error structure it relies on? — **yes**
4. 4. Is the claim that the best explanation must posit a separable internal variable whose wrongness propagates systematically necessary to keep the criterion non-circular rather than merely redescribing representation in new words? — **yes**
5. 5. Does the conjecture imply that in a robot with a manipulable internal map, one should expect families of diagnosable navigation errors under content shifts even in novel environments not mentioned in the problem statement? — **yes**
6. 6. Does the criterion extend beyond the thermostat and robot examples to learned predictive states in adaptive controllers by predicting that only states supporting structured compensation count as models? — **yes**
7. 7. If the conjecture is right, does it illuminate why some biologically useful internal signals should fail to count as models when changing them alters behavior but does not generate a stable space of correctable mistakes? — **yes**
8. 8. If a counterexample showed a system with apparent correctness conditions but no clean way to hold downstream policies fixed, would saving the conjecture require abandoning its central counterfactual intervention test rather than adding a minor qualification? — **yes**
9. 9. If a simple control device exhibited repeatable error patterns after recalibration, would the conjecture have to give up its core distinction between misrepresentation and malfunction to classify it as non-model-like? — **no**
10. 10. If one found a case where content variation changes success conditions but no compensating rewrite can restore performance, would preserving the conjecture force a major revision of its load-bearing account rather than a small boundary clause? — **yes**

## Candidate Problems

- Can the counterfactual load-bearing criterion be made precise without smuggling in semantic assumptions through the choice of target domain W, the individuation of state type S, or the notion of 'holding downstream use-rules fixed as far as possible'? The open problem is to formulate a non-circular intervention framework that distinguishes genuine content-variation from mere redescriptions of mechanism, especially in systems where encoding and policy are highly entangled. (score: 0.96)
- How does the criterion handle edge cases in simple, adaptive, or distributed control systems where there appears to be a graded transition between causal routine and model-based control rather than a sharp boundary? The open question is whether modelhood is binary or scalar, and whether systems like thermostats, predictive controllers, learned latent states, morphologically computed bodies, or swarm systems exhibit stable spaces of misrepresentation and compensation sufficient for correctness conditions. (score: 0.93)
- What explanatory advantage does this criterion have over rival accounts of representation, such as teleosemantics, structural resemblance, predictive-processing, or interventionist/informational theories, and are there cases where they come apart? The open problem is to identify discriminating examples where the load-bearing test classifies a state differently from these alternatives, and to determine whether those divergences reveal a deeper insight or a hidden limitation in the conjecture. (score: 0.89)
