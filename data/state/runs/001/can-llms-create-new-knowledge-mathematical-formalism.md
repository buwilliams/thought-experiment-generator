# Conjecture: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

## Conjecture: LLMs as Structure-Preserving Maps Cannot Generate Novel Fixed Points

**Formalization**

Let K be the space of knowledge states. Define an LLM as a function:

**f: K^n → K**

mapping n knowledge states (training corpus) to outputs in K. The function f is learned — it approximates a compression of patterns across K^n into a parametric map.

**The Invariant**

f is, by construction, a *measure-preserving* transformation on the probability distribution over K. It generates outputs by interpolating within, or extrapolating near, the convex hull of its training distribution. Call this hull H(K^n).

The critical invariant: **f cannot place probability mass outside H(K^n) by design**. Its outputs are constrained to the closure of the span of its training structure.

**What This Means for Knowledge Creation**

New knowledge, in the Deutschian sense, requires *conjecture that is not entailed by prior conjectures* — a genuinely new explanatory structure, a fixed point in K that lies outside the reachable set of existing transformations. Call such a point k* where k* ∉ H(K^n).

**Conjecture:** f cannot generate k* without an external perturbation — a genuinely novel problem, constraint, or criticism injected from outside the system.

But here is the structural tension: *criticism and problems are also elements of K*. A sufficiently strange input — a new experimental result, a paradox, an anomalous constraint — is itself a new vector in K-space that *shifts H(K^n)*. 

**The Resolution**

This reframes the question. The right question is not "does f produce k*?" but rather: **does the system (human + f) traverse K differently than human alone?**

Structurally: let the human operate as a transformation h: K → K. The composite system is h ∘ f ∘ h ∘ f ∘ ... — an iterated map. 

Fixed points of this iterated map need not be fixed points of h or f alone. Novel knowledge can emerge as **attractors of the coupled dynamical system** that neither component could reach independently.

**What Is Preserved vs. What Varies**

- *Preserved*: f alone cannot originate explanatory novelty. It has no error-correction process, no conjecture-refutation loop, no stake in truth.
- *Varies*: The reachable set of the coupled human-LLM system expands because f dramatically accelerates traversal of H(K^n), surfacing non-obvious connections, compressing search time over known structure.
- *Non-negotiable constraint*: Genuine new knowledge requires a critic — someone who can recognize when an output fails against reality, not just against training distribution.

**Conjecture (stated cleanly):**

*LLMs do not create new knowledge; they densify the traversal of existing knowledge-space. New knowledge arises only when the human-LLM iterated system encounters a fixed point unreachable by either alone — and this requires the human to supply genuine criticism from outside the system's current hull.*

The leverage point is not the LLM's generative power. It is the **quality of the external perturbation** the human introduces.

## Questions

1. If f is truly measure-preserving on H(K^n), does it follow that no interpolation or extrapolation within H(K^n) could constitute new knowledge — or could a previously unreached interior point of H(K^n) itself qualify as novel knowledge under the Deutschian definition? — **no**
2. Is the claim that 'criticism and problems are also elements of K' load-bearing — and if so, does it undermine the conjecture by implying that the human's external perturbation is itself already inside K, making it unclear what 'outside the hull' actually means? — **yes**
3. Does the conjecture require that the iterated map h ∘ f ∘ h ∘ f ∘ ... provably reaches fixed points unreachable by h alone, or is this asserted without a structural argument for why the composition expands the reachable set rather than merely accelerating traversal within it? — **yes**
4. Is the distinction between 'densifying traversal of existing knowledge-space' and 'generating new knowledge' stable — specifically, could a sufficiently dense traversal that surfaces a non-obvious connection between two distant points in H(K^n) itself constitute a new explanatory structure in the Deutschian sense? — **no**
5. Does the conjecture depend on the human's error-correction process being categorically different in kind from any process f could implement, or only different in degree — and if only in degree, does the hard boundary between f and h collapse? — **yes**
6. If the 'quality of the external perturbation' is the true leverage point, does the conjecture predict that a low-quality human critic paired with a powerful LLM produces less new knowledge than a high-quality human critic paired with no LLM — and is this prediction falsifiable? — **yes**
7. Is the claim that f 'has no stake in truth' doing explanatory work in the conjecture, or is it ornamental — specifically, would the conjecture's conclusions change if f were modified to include an internal consistency-checking loop? — **no**
8. Does the conjecture commit to a specific topology or metric on K-space that makes 'convex hull' and 'outside the hull' well-defined, or does the geometric language function as metaphor — and if metaphor, does the formal argument still hold without it? — **no**
9. The conjecture states that novel knowledge requires 'conjecture not entailed by prior conjectures' — does this definition exclude the possibility that f, by combining training elements in a statistically improbable configuration, produces an output that is functionally non-entailed even if it lies within H(K^n)? — **yes**
10. Does the resolution section's claim that 'fixed points of the iterated map need not be fixed points of h or f alone' require that h and f be non-commuting transformations on K — and if they commute, does the emergence of novel attractors from their composition fail to follow? — **yes**

## Candidate Problems

- The conjecture assumes H(K^n) is well-defined and static, but training distributions are not convex hulls in any tractable geometric sense. What is the actual topology of LLM output space, and does the convex hull metaphor survive contact with high-dimensional, non-Euclidean embedding geometry? If the metaphor breaks, does the conjecture's core claim survive in a more rigorous form? (score: 0.85)
- The conjecture treats 'genuine criticism from outside the system' as the leverage point, but criticism is itself an element of K. This creates a regress: what makes a criticism genuinely external rather than a recombination of existing critical moves? Is there a principled distinction between a novel perturbation and a novel-seeming interpolation, and can it be operationalized? (score: 0.92)
- The iterated map h ∘ f ∘ h ∘ f ∘ ... is claimed to reach fixed points unreachable by either component alone. This is a strong dynamical claim. Under what conditions do iterated maps of this form actually produce novel attractors versus converging to attractors already reachable by h alone? The conjecture asserts this without a proof sketch or even a worked analogy. (score: 0.88)
- The conjecture implicitly assumes human cognition (h) is not subject to the same hull-constraint criticism. But human knowledge is also trained on prior knowledge states — the brain is itself a pattern-compressing system. What principled distinction licenses treating h as capable of genuine novelty while f is not? If none exists, the conjecture may prove too much or too little. (score: 0.91)
- The claim that f 'densifies traversal' of existing knowledge-space is doing significant work but is left informal. Densification could mean faster search, better interpolation, or surfacing latent structure. These have different implications for whether the human-LLM system is epistemically superior to human-alone. What would it mean to measure densification, and does it matter whether the density increase is uniform or concentrated in particular regions of K? (score: 0.76)
- The conjecture is framed in Deutschian terms — new knowledge requires conjecture not entailed by prior conjectures. But Deutsch's epistemology is about explanatory reach, not geometric position in a probability space. Is the mapping from 'outside H(K^n)' to 'not entailed by prior conjectures' valid? A point inside the convex hull could still constitute a genuinely new explanation if explanation is not reducible to interpolation. (score: 0.89)
- If the leverage point is the quality of external perturbation the human introduces, this implies a theory of what makes a perturbation high-quality. Is this reducible to existing accounts of good problem-posing, or does the human-LLM coupling create new criteria for perturbation quality that don't exist in human-alone inquiry? (score: 0.74)
- The conjecture says f has no error-correction process and no stake in truth. But RLHF and related training methods introduce a weak form of feedback. Does this constitute a degenerate error-correction loop, and if so, does it shift the boundary of what f can reach — or does it merely reshape H(K^n) without expanding it in the relevant sense? (score: 0.70)
- The systems-thinking framing suggests leverage lies in structural points, not in pushing harder on existing forces. But the conjecture identifies the leverage point (quality of human perturbation) without analyzing the feedback structure that governs how perturbation quality itself changes over time in a human-LLM system. Does repeated use of LLMs degrade the human's capacity to generate genuine external perturbations — a stock-depletion dynamic — and if so, what are the long-run attractors of the coupled system? (score: 0.93)
- The conjecture is silent on multi-agent LLM systems where outputs of one instance perturb another. Does the hull H(K^n) expand when multiple LLM instances interact without human mediation, or does the absence of a genuine external critic mean the system merely explores the interior of the same hull more thoroughly? This tests whether the human is structurally necessary or merely contingently the current source of perturbation. (score: 0.82)
- There is an implicit claim that novel fixed points of the iterated system are identifiable as novel — that the human can recognize when k* ∉ H(K^n). But if the human's recognition capacity is itself bounded by their own knowledge hull, genuine novelty may be unrecognizable at the moment of generation. This creates an epistemic access problem: the conjecture may be unfalsifiable in practice even if true in principle. (score: 0.86)
