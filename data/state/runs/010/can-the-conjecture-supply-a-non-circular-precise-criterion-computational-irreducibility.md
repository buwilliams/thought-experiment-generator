# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × computational-irreducibility

## Conjecture

**Conjecture:**  
An internal state genuinely functions as a **model with correctness conditions** iff the system’s successful behavior depends on a **counterfactually reusable internal mapping** whose contribution cannot be reduced to a fixed sensorimotor policy, and whose adequacy can be independently evaluated against the structure it is about.

From the perspective of **computational irreducibility**, the key point is that in many environments there is no shortcut from input history to successful action. If the world’s relevant dynamics must effectively be “run” step by step, then a controller has two broad options:

1. **Embed the needed structure internally**, in a form that can stand in for absent, delayed, or hypothetical worldly states; or  
2. Remain a purely causal coupling that works only by direct ongoing interaction.

This yields a criterion. An internal state counts as a model only when it supports **offline or counterfactual exploitation**: the system can vary action while holding the represented target fixed, or vary hypothetical target conditions while holding current input fixed, and the same internal state still systematically guides behavior. In that case, the state is not merely one more causal relay. It is functioning as a **surrogate structure** that the system can consult across multiple possible action sequences.

Its **correctness conditions** are then non-circularly specified by whether this surrogate preserves the task-relevant structure of the worldly process it stands in for. “Correct” does not mean “causally useful in fact”; many non-model mechanisms are useful. It means: if the represented variable were different, or if the modeled transition were wrong, the system’s internally guided predictions, plans, or deferred controls would fail in ways attributable to mismatch between internal transition structure and external transition structure.

So the distinction is not “representation vs causation” in general — everything physical is causal — but **causal role under irreducible environmental complexity**. Where no simple reactive shortcut exists, a model is marked by **detachable internal simulation capacity**: the system can use the state to anticipate, compare, or choose among trajectories not yet enacted. A merely causal control state lacks this evaluable independence; it contributes to behavior, but has no stable answer to “what would make it mistaken?” apart from total system failure.

**Precise criterion:**  
An internal state \(S\) is a model of domain \(D\) for system \(A\) iff:

- \(S\) encodes variables and/or transition relations over \(D\);  
- \(A\) can use \(S\) across more than one possible action or input regime;  
- interventions on \(S\) produce systematic changes matching alterations in hypothesized \(D\), not just arbitrary control perturbations; and  
- there exists a principled mismatch test between the structure carried by \(S\) and the structure of \(D\), such that \(S\) can be wrong even when momentarily useful.

What this illuminates is that **modelhood appears when irreducible task structure forces the system to internalize a reusable generative rule**. Correctness conditions arise from that rule’s possible divergence from the world it is used to stand in for.

## Questions

1. 1. Necessity: If the requirement of counterfactual reuse across more than one possible action or input regime were removed, would the conjecture still distinguish a genuine model from a fixed sensorimotor policy rather than collapsing into mere causal contribution? — **no**
2. 2. Necessity: Is the claim that the internal state must stand in for absent, delayed, or hypothetical worldly states necessary for the conclusion that it has correctness conditions, rather than being just another online control relay? — **yes**
3. 3. Necessity: If the principled mismatch test between the internal structure and the domain structure were dropped, would the conjecture lose its non-circular account of what makes the state correct or mistaken? — **yes**
4. 4. Necessity: Is the appeal to computational irreducibility doing essential work in explaining why reusable internal mapping is required, or could the same criterion follow without that structural assumption about the environment? — **yes**
5. 5. Reach: Does the conjecture imply that in a highly reactive environment with a simple shortcut from input history to action, even a causally useful internal state should not count as a model with correctness conditions? — **yes**
6. 6. Reach: Does the criterion classify planning with imagined future trajectories and deferred control as model use even when no overt action is currently being executed? — **yes**
7. 7. Reach: Does the conjecture extend to artificial systems where interventions on an internal variable systematically mirror hypothesized changes in an external domain, even if the system was not designed with explicit representational labels? — **yes**
8. 8. Resistance to patching: If a fixed sensorimotor controller were found to succeed across many contexts by brute-force lookup, would saving the conjecture require abandoning the claim that reuse must involve a mapping evaluable against domain structure rather than merely adding a minor qualification? — **no**
9. 9. Resistance to patching: If a system had an internal state that guided multiple action sequences but there were no independent way to test mismatch with the world, would treating it as a model force the conjecture to give up its core non-circular correctness condition? — **yes**
10. 10. Resistance to patching: If an organism in an irreducible environment solved the task through tight real-time coupling without detachable internal simulation, would accommodating that case require gutting the conjecture's claim that irreducible task structure forces internalization of a reusable generative rule? — **yes**

## Candidate Problems

- How can the conjecture non-circularly distinguish a genuinely 'counterfactually reusable internal mapping' from a highly compressed but still non-representational policy? The core open problem is whether there exists an operational criterion that separates model-based internal structure from sophisticated reactive control without presupposing the very notion of correctness conditions it aims to explain. (score: 0.96)
- Is computational irreducibility actually necessary for modelhood, or only one common route to it? A major unresolved tension is whether systems can possess genuine models with correctness conditions even in environments that are reducible, predictable, or shortcut-friendly, which would weaken the conjecture's proposed link between irreducible task structure and the emergence of internal models. (score: 0.91)
- What counts as an 'independent' mismatch test between internal structure and world structure in real systems? The conjecture relies on adequacy being evaluable against the domain itself, but it remains open how to specify such tests for noisy, learned, distributed, or action-dependent representations where the target domain is partly constituted by the agent's own interventions. (score: 0.94)
