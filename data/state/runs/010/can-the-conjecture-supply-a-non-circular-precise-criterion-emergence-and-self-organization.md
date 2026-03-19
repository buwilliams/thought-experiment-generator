# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × emergence-and-self-organization

## Conjecture

**Conjecture:**  
An internal state genuinely functions as a **model with correctness conditions** iff it is embedded in a control architecture containing a distinct **error-sensitive feedback loop** that treats the state as answerable to features of the wider system-environment dynamics, rather than merely as a variable that helps stabilize behavior.

More precisely: an internal state counts as a model when the system is organized so that discrepancies between what the state *implies* and what subsequent interaction *reveals* are themselves causally registered and can drive systematic state revision. In that case, the state has correctness conditions because the architecture distinguishes, functionally, between **successful control** and **accurate representation**.

The systems-level point is that modeling is not a property of a state in isolation, nor of its intrinsic content, nor merely of downstream usefulness. It is an emergent property of a larger pattern of interactions among at least four elements:

1. **State formation:** the internal state is produced in ways sensitive to environmental structure.  
2. **Policy guidance:** the state is used to guide action, prediction, or inference.  
3. **Independent check pathway:** later inputs, outcomes, or other internal processes can bear on whether the state was accurate, not just on whether action succeeded.  
4. **Error-correction dynamics:** detected mismatch can update the state, the mapping that produced it, or the reliance placed on it.

This yields a non-circular criterion:  
A state is a model when there exists, in the system’s organization, a counterfactual dependence such that **if the state were inaccurate while all else remained as similar as possible, the system would be liable to detect and treat that inaccuracy as a basis for correction**. If no such architecture exists—if the state merely contributes causally to adaptive success—then it is part of a control loop, not yet a model.

What this illuminates is why mere causal participation is insufficient. Thermostats, for example, may correlate with temperature and regulate effectively, but unless the architecture includes a distinction between “signal good for control” and “signal inaccurate about temperature,” there are no genuine correctness conditions—only causal role. By contrast, systems with internal simulation, prediction, sensor fusion, or calibration routines can sustain modelhood because their organization makes room for **misrepresentation as such**.

So the key shift is from asking, “Does this state stand in for the world?” to asking, “Does the system contain a structure in which the state can be *wrong*, in a way the system itself is organized to discover and repair?” Correctness conditions are thus emergent from a specific feedback topology, not intrinsic to the vehicle state and not reducible to its current causal efficacy.

## Questions

1. 1. Is the claim that a distinct error-sensitive feedback loop must be present necessary for the conjecture to classify an internal state as having correctness conditions rather than merely aiding control? — **yes**
2. 2. If the independent check pathway were removed so that only action success fed back into the system, would the conjecture lose its basis for distinguishing accurate representation from successful control? — **yes**
3. 3. Is the requirement that discrepancies between what the state implies and what later interaction reveals be causally registered necessary for the criterion to remain non-circular? — **yes**
4. 4. If error-correction dynamics could adjust behavior without revising the state, its formation mapping, or reliance on it, would the conjecture still explain why the state itself counts as a model? — **no**
5. 5. Does the conjecture imply that systems with sensor fusion, internal simulation, or calibration routines should count as model-using even when the original problem only asked for a criterion separating models from control states? — **yes**
6. 6. Does the conjecture extend to cases where a state guides inference rather than overt action, because the same architecture would still allow inaccuracy to be detected and corrected? — **yes**
7. 7. Does the conjecture illuminate why a thermostat can regulate effectively yet still fail to have genuine correctness conditions, thereby explaining a class of control systems beyond the target case? — **yes**
8. 8. If a counterexample involved a system whose state tracks environmental structure and guides action but lacks any pathway that can register inaccuracy as such, would saving the conjecture require abandoning its core distinction between successful control and accurate representation? — **yes**
9. 9. If one tried to rescue the conjecture for a purely feedforward controller by saying long-run adaptive success alone confers correctness conditions, would that patch gut the central role of error-sensitive feedback in the explanation? — **yes**
10. 10. If a system could detect mismatch only by external observer interpretation rather than by its own organization, would modifying the conjecture to count that as modelhood undermine its claim to provide a system-internal non-circular criterion? — **yes**

## Candidate Problems

- Does the conjecture give necessary and sufficient conditions for modelhood, or only a strong sufficient marker? The main unresolved tension is whether there are genuine models whose correctness conditions are not internally monitored by a distinct error-sensitive loop—for example, dormant maps, one-shot representations, offline scientific models, or inherited/internal structures that guide behavior but are only externally evaluated. If such cases exist, the conjecture may be too restrictive; if not, it needs a sharper argument for why answerability-to-error must be architecturally realized inside the system itself. (score: 0.95)
- Can the distinction between 'successful control' and 'accurate representation' be made non-circular and operational across real systems? The conjecture relies on an 'independent check pathway' that evaluates accuracy rather than mere performance, but in many adaptive systems the only available signal is downstream success, and calibration, prediction, and control are deeply entangled. A key open problem is to specify structural criteria that reliably separate representation-error from ordinary control-error without smuggling in semantic assumptions. (score: 0.93)
- What is the right grain and boundary of the 'system' for locating the error-sensitive loop that grounds correctness conditions? The conjecture says modelhood is emergent from a larger architecture, but it remains open whether the relevant checking and correction must occur within the agent, may be distributed across body-environment-social scaffolding, or can unfold over evolutionary/developmental timescales. Different boundary choices could classify the same internal state as a mere control variable or as a genuine model, so the ontology of system individuation is a major unresolved tension. (score: 0.91)
