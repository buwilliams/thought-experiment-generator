# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × goal-critique-safety-lens

## Conjecture

**Conjecture:**  
An internal state genuinely functions as a **model with correctness conditions** iff the system can use that state in **error-correcting criticism**: it must be able to compare the state’s content against consequences of action or observation, identify specific ways the state may be wrong, and revise it while preserving the broader problem the state was meant to help solve. If no such internal critical relation is possible, the state is not a model in the full sense, but only part of a causal control loop.

This gives a non-circular criterion because it does not define “model” as “whatever represents correctly.” Instead, it asks for a structural capacity: can the system treat the state as **answerable to reality** through internal criticism and revision?

More precisely, a state counts as a model when:

1. **Decouplability:** it can be tokened, manipulated, or recombined without immediate execution in the control loop.  
2. **Counterfactual usability:** the system can use it to evaluate possible situations or actions not currently occurring.  
3. **Error-localizability:** failure can be attributed to the state’s content, not merely to overall system performance.  
4. **Revisability under criticism:** the system has mechanisms for changing the state in response to detected mismatch while retaining the goal of improving future guidance.

These conditions generate **correctness conditions**: the state is the sort of thing that can be *mistaken about X* because the system can distinguish “the world is one way” from “my internal state says it is that way.” That distinction is what a mere controller lacks. In a thermostat, for example, internal variables may covary usefully with temperature, but unless the system can criticize and revise the variable as a possibly false take on the environment, the variable is just a node in regulation, not a world-model.

From the stated safety perspective, this matters because fixed inherited objectives often promote increasingly powerful control without any capacity to question the assumptions built into that control. A system safe under changing contexts must not only optimize; it must be able to inspect the informational states guiding optimization and ask whether they are wrong, obsolete, or mis-specified. So the boundary between “mere control” and “genuine model” is exactly the boundary between systems that can remain trapped in locked-in error and systems that can, in principle, escape it.

A useful thought experiment is a high-performing autopilot with an internal map. If the “map” only causally steers behavior but cannot be interrogated, compared with sensor conflict, and revised as *the map being wrong*, then it is not functioning as a model with correctness conditions. If, however, the system can explicitly detect that the terrain is not as mapped, isolate the map as the error source, and update it for future use, then the map is a genuine model.

So: **modelhood is not mere causal relevance but criticizability with respect to reality.**

## Questions

1. 1. Is the claim that the system must be able to identify the internal state itself as the source of error necessary for the conjecture to distinguish a genuine model from a merely successful control variable? — **yes**
2. 2. Is decouplability necessary for the conclusion, or could a state still count as a model with correctness conditions if it can only ever be used while directly driving the control loop? — **yes**
3. 3. Is the requirement that revision preserve the broader problem the state was meant to solve necessary to the explanation, rather than just one way of describing adaptive updating? — **yes**
4. 4. Is the claim that correctness conditions arise from the system distinguishing the world from what its internal state says about the world necessary for the criterion to be non-circular? — **yes**
5. 5. Does the conjecture imply that a planning system using offline simulation of novel actions would count as model-based even when the original problem only asked about internal states in control loops? — **yes**
6. 6. Does the criterion illuminate why a high-performing thermostat-like regulator can remain trapped in systematic error across changing environments even if its behavior is locally successful? — **yes**
7. 7. Does the conjecture extend to cases where the internal state concerns hidden causes or future outcomes rather than directly observed environmental features? — **yes**
8. 8. If a counterexample showed a system that revises behavior after failure but cannot localize the failure to any contentful state, would saving the conjecture require abandoning error-localizability rather than adding a minor qualification? — **yes**
9. 9. If one tried to count any sensor-driven parameter update as criticism of a model, would that patch collapse the conjecture's core distinction between answerability to reality and mere causal covariation? — **yes**
10. 10. If a system could manipulate internal states counterfactually but had no mechanism to compare them with observational consequences, would treating it as a genuine model force a major rewrite of the conjecture rather than a small amendment? — **yes**

## Candidate Problems

- Is error-correcting criticism necessary for modelhood, or only for a stronger kind of explicit/reflective modelhood? The conjecture risks excluding many plausible cases—predictive coding systems, learned latent-state controllers, animal cognition, and scientific instruments—that seem to carry correctness conditions despite lacking explicit internal mechanisms that localize and revise content as content. The key open problem is to draw a principled boundary between mere causal covariance, implicit model-based control, and full criticizable modelhood without making the criterion either too restrictive or trivially broad. (score: 0.95)
- How can 'error-localizability' and 'the state being wrong' be specified non-circularly and operationally in real systems? The conjecture says a model must be identifiable as the source of error rather than just part of overall failure, but in complex adaptive systems error attribution is distributed across sensors, inference, objectives, and actuators. The open question is whether there exists a rigorous systems-level test or formalism that distinguishes content-level criticism from ordinary parameter adjustment or control-theoretic adaptation. (score: 0.92)
- What is the relation between criticizable modelhood and safe agency under changing objectives or contexts? The conjecture suggests that systems able to revise informational states can escape locked-in error, but it remains unresolved whether this capacity is sufficient, necessary, or even sometimes dangerous for safety. A major research problem is to determine what additional structures—goal criticism, meta-level error correction, value revisability, or institutional feedback—are required so that model criticism improves robustness rather than enabling more powerful pursuit of flawed inherited aims. (score: 0.90)
