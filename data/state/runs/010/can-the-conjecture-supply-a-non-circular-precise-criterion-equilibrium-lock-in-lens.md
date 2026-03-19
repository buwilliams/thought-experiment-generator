# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × equilibrium-lock-in-lens

## Conjecture

**Conjecture:**  
A non-circular criterion for when an internal state genuinely functions as a **model with correctness conditions** is this:

> **An internal state is a model iff the system’s architecture makes the state a reusable, decouplable source of guidance whose contribution to success depends on a structurally stable mapping between the state’s content-sensitive variations and independently varying features of a target domain, such that the state can be wrong in ways the system can in principle detect and correct.**

What this perspective illuminates is the difference between **mere causal contribution** and **representational underprovision**. In a pure control loop, internal states may help produce adaptive behavior, but there is no distinct good of *accuracy* being provisioned. Their role is exhausted by immediate causal efficacy. By contrast, a model supplies a kind of internal public good to the system: a shared, reusable informational resource whose benefits are diffuse across many downstream tasks. Because its usefulness is spread across contexts, its value is not reducible to any one output episode.

This yields a criterion with four precise structural marks:

1. **Decouplability:** the state can be tokened, maintained, or manipulated outside the immediate stimulus-response loop.  
2. **Systematic target dependence:** changes in the state covary with changes in a target domain in a way that supports multiple inferences or action-guiding uses, not just one fixed response.  
3. **Possibility of misrepresentation:** there are specifiable conditions under which the state would guide the system differently from how the target domain actually is.  
4. **Error-sensitive uptake:** the architecture contains feedback processes that can, at least in principle, detect and adjust such mismatches.

The non-circularity comes from locating representation not in “aboutness” posited from the start, but in **organizational role**. Correctness conditions are not assigned because the state is useful; they arise because the system is built so that success depends on preserving a mapping that can fail independently of immediate control success. A thermostat’s bimetal strip usually does not qualify: its state participates in regulation, but not as a decoupled, reusable structure with content-sensitive error correction about temperature as a domain. A cognitive map often does qualify: it can be consulted offline, recombined, used for novel routes, and evaluated as inaccurate.

The public-goods lens sharpens the point: where informational structure is non-excludable across tasks, selection or learning may build internal states whose benefits are diffuse and whose maintenance is costly. Such architecture is worth constructing only when the same internally stored structure can serve many contexts. That is precisely when correctness conditions become real rather than observer-imposed. So the boundary is not “inner state versus outer behavior,” but **whether the system has solved the internal coordination problem of creating an accuracy-sensitive informational resource**.

## Questions

1. 1. Is the claim that the state must be decouplable from the immediate stimulus response loop necessary for the conjecture to distinguish a cognitive map from a thermostat style control state rather than merely redescribing both as useful internal causes? — **yes**
2. 2. Is the claim that success depends on a structurally stable mapping to independently varying features of a target domain necessary for the conjecture to avoid circularly inferring representation from whatever state happens to aid behavior? — **yes**
3. 3. Is the requirement that the state can be wrong in ways the system can in principle detect and correct necessary for the conjecture to yield correctness conditions rather than only a notion of multiuse causal efficacy? — **yes**
4. 4. If the public goods idea were removed while keeping decouplability, target dependence, and error sensitive uptake, would the conjecture lose an essential part of its explanation of why correctness conditions are real rather than observer imposed? — **yes**
5. 5. Does the conjecture imply that a learned world model used offline for planning in a robot would count as representational even when it is not currently driving behavior, thereby reaching beyond the original control loop contrast? — **yes**
6. 6. Does the criterion illuminate why a cached latent state reused across perception, navigation, and prediction tasks would have stronger claim to correctness conditions than a task specific hidden state tuned only for one reflexive response? — **yes**
7. 7. Does the conjecture extend to biological cases such as hippocampal place coding by predicting that offline replay and route recombination are evidence for genuine modeling rather than mere control participation? — **yes**
8. 8. If a thermostat were augmented with a memory buffer and a calibration routine that occasionally corrects drift, would treating it as a genuine model require abandoning the conjecture's core distinction between reusable target tracking and simple regulation? — **no**
9. 9. If there were a system whose internal state is highly decouplable and reused across many tasks but whose success does not depend on any stable mapping to an independently varying domain, would saving the conjecture from counting it as a model require more than a minor qualification? — **no**
10. 10. If an agent reliably succeeds because its internal state tracks a target domain but the architecture lacks any mechanism that could in principle detect and correct mismatch, would preserving the conjecture force a major revision of the claim that correctness conditions require error sensitive uptake? — **yes**

## Candidate Problems

- How can the conjecture distinguish genuine models from sophisticated control states in borderline architectures where decoupling, reuse, and error-correction come in degrees rather than kinds? The main unresolved tension is whether the proposed marks yield a principled threshold or only a graded similarity space, especially for predictive controllers, adaptive regulators, and learned latent states that support limited offline use. (score: 0.95)
- Can 'structurally stable mapping' and 'content-sensitive variation' be specified non-circularly and operationally enough to avoid smuggling in representation through the back door? The open problem is to formulate architecture-level tests that identify target-domain dependence without presupposing aboutness, semantic interpretation, or observer-selected success criteria. (score: 0.98)
- Is error-sensitive uptake really necessary for correctness conditions, or only for a system's capacity to improve them? This raises a deep challenge case: systems whose internal states appear reusable and target-guided but lack endogenous detection or correction of misrepresentation, such as frozen models, one-shot evolved mechanisms, or externally interpreted scientific instruments. (score: 0.91)
