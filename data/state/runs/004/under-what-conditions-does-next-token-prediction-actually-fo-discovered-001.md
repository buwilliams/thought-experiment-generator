# Generated: under-what-conditions-does-next-token-prediction-actually-fo × discovered-001

## Conjecture

**Conjecture:**  
Next-token prediction forces learning of latent variables that track real domain invariants only when those invariants are **load-bearing for compression across many contexts**: changing or omitting them would systematically worsen prediction unless the model rebuilt equivalent structure elsewhere.

The key distinction is between two kinds of predictive success:

1. **Narrative-shell prediction**: the model exploits shallow, locally sufficient cues, memorized correlations, or shortcut features.
2. **Constraint-bearing prediction**: the model must represent deeper variables because they organize many downstream token dependencies at once.

From this perspective, the unfinished claim “prediction is sufficient only when the t…” points toward something like: only when the **training distribution makes the invariant structurally indispensable**. An invariant is real, in this sense, not because we name it, but because it supports a wide range of successful predictions and cannot be freely swapped out without compensation.

So the relevant conditions are:

- **Multi-context reuse**: the same hidden factor must govern token patterns across diverse surface realizations. If each context can be predicted independently, no shared latent is forced.
- **Counterfactual pressure**: if that factor were changed, many predictions would have to change coherently. This makes the variable load-bearing rather than decorative.
- **Bottleneck or efficiency pressure**: the model must gain by representing the invariant once rather than rediscovering it case-by-case. Otherwise brute-force memorization can substitute.
- **Low availability of shortcuts**: if surface statistics predict tokens nearly as well as the invariant does, next-token training need not recover the invariant.
- **Temporal or structural reach**: the consequences of the latent variable must propagate far enough through the sequence that local pattern matching is inadequate.

This yields a stronger claim: **prediction does not in general discover “true” latent variables; it discovers whatever internal structure is most efficient for satisfying the predictive constraints actually present in the data-generating process.** Real domain invariants emerge when they are the simplest way to satisfy those constraints globally.

What follows is that debates over whether language modeling “learns world models” are often underspecified. The issue is not prediction vs. no prediction; it is whether the environment and corpus are arranged so that world structure is the only stable, reusable solution. If a detail in the data can be replaced without widespread compensating changes, it is not load-bearing and the model need not internalize it. If replacing it breaks many dependencies at once, the training objective pressures the model toward a corresponding latent variable.

So the conjecture is:

> **Next-token prediction is sufficient for learning latent variables aligned with real domain invariants when the invariant is a globally reused, counterfactually constraining source of predictive compression, and the data distribution suppresses alternative shortcut solutions.**

In short, prediction learns invariants not merely when they exist, but when they are structurally unavoidable.

## Questions

1. 1. If the requirement of multi-context reuse were removed, would the conjecture still explain why next-token prediction learns a shared latent rather than separate context-specific heuristics? — **no**
2. 2. If counterfactual pressure were absent so that changing the proposed invariant altered only a few local predictions, would the conjecture still distinguish a real invariant from a decorative feature? — **no**
3. 3. If bottleneck or efficiency pressure were relaxed enough for brute-force memorization to fit the data, would the conjecture still predict learning of the same latent variables? — **no**
4. 4. If abundant shortcut surface statistics remained available while all other conditions held, would the conjecture still claim that next-token prediction is sufficient to recover the real invariant? — **no**
5. 5. If the latent variable affected only nearby tokens and had little temporal or structural reach, would the conjecture still treat it as forced by next-token prediction? — **no**
6. 6. If a candidate hidden factor organized many token dependencies but was not aligned with any real domain invariant, would the conjecture still count its discovery as success? — **no**
7. 7. If the training distribution were changed so that the same invariant no longer had to support predictions across diverse surface realizations, would the conjecture still predict invariant learning? — **no**
8. 8. If replacing the invariant with an alternative internal code required only local compensating changes rather than widespread restructuring, would the conjecture still call the original invariant load-bearing? — **no**
9. 9. If the model could rebuild an equivalent predictive structure elsewhere after omitting the invariant with no meaningful loss in compression, would the conjecture still say the invariant was structurally unavoidable? — **no**
10. 10. If next-token prediction succeeded mainly through narrative-shell cues on the observed corpus while failing under interventions that change the underlying factor, would the conjecture still regard the learned representation as evidence for invariant learning? — **no**

## Candidate Problems

- How can 'load-bearing for compression' be made precise and testable? The conjecture hinges on a central but informal notion: an invariant is learned when removing or perturbing it forces widespread predictive loss unless equivalent structure reappears elsewhere. A key open problem is to define operational criteria that distinguish genuinely shared, counterfactually constraining latents from merely useful heuristics, and to measure when a representation is indispensable rather than one of many interchangeable predictive encodings. (score: 0.97)
- What properties of the training distribution and model architecture jointly determine whether real invariants are structurally unavoidable versus bypassed by shortcuts? The conjecture attributes success to multi-context reuse, counterfactual pressure, bottlenecks, low shortcut availability, and long-range dependence, but leaves unresolved how these factors trade off, whether there are thresholds, and how capacity, attention, memorization, and optimization dynamics alter the outcome. This is a high-value problem because it could turn a qualitative thesis into a predictive theory of when next-token training yields world-aligned structure. (score: 0.95)
- Are the learned latents uniquely aligned with real domain invariants, or can multiple non-isomorphic internal structures support the same global predictive compression? The conjecture suggests prediction can recover 'real' invariants when they are indispensable, but it remains open whether indispensability implies identification of the invariant itself or only some functionally equivalent surrogate. This raises a deep question about realism versus underdetermination: when does predictive necessity force ontology-like structure, and when does it merely force an abstract equivalence class of solutions? (score: 0.92)
