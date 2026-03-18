# Generated: what-makes-an-explanation-hard-to-vary × thought-experiments

## Conjecture

## Conjecture: Explanatory Load-Bearing as Causal Closure

**The Isolating Scenario**

Imagine two explanations of the same phenomenon — why a bridge collapsed. Explanation A: "The steel fatigued because cyclic stress exceeded the endurance limit, which caused micro-cracks to propagate under resonant frequency loading, which reduced cross-sectional area below critical load capacity." Explanation B: "The bridge was old, the metal was weak, there were vibrations, and it fell."

Now systematically swap individual components. Replace "resonant frequency loading" in A with "general vibration." The account breaks — the propagation rate and timing become unpredictable; you can no longer derive the moment of failure. Replace "vibrations" in B with "temperature fluctuations." The account still stands — it was already loose enough to accommodate almost anything.

Assume this difference is real and structural, not rhetorical. Follow it.

---

**The Conjecture**

A load-bearing explanation is one where each component constrains the inferential degrees of freedom of the others. Remove or swap any one part, and the remaining parts no longer uniquely pick out the phenomenon. The explanation is **causally closed** — meaning the set of components jointly entail the explanandum with no slack.

A non-load-bearing explanation is not false. It is **underdetermined** — its components are consistent with a wide range of outcomes, so the phenomenon being explained is not *generated* by the account but merely *accommodated* by it.

The structural property distinguishing them is **mutual constraint density**: how tightly each component's presence restricts the others' range of valid substitutes.

In load-bearing explanations:
- Components are not interchangeable because each plays a specific functional role in the causal or logical chain
- The explanation has a **unique inverse**: you could, in principle, reconstruct most of the story from the phenomenon plus any one part
- Removing a component doesn't weaken the account — it *invalidates* it

In non-load-bearing explanations:
- Components can be swapped because they are drawn from a loosely-bounded reference class ("weak material" covers steel fatigue, corrosion, poor alloy choice...)
- The explanation has **multiple inverses**: many different stories fit equally well
- Removing a component leaves the account intact, merely vaguer

---

**What This Illuminates**

The load-bearing/not distinction is not a quality gradient — it is a threshold. Below the threshold of mutual constraint, an explanation is a *narrative frame* rather than a mechanism. Narrative frames are useful for orientation but generate no predictions about counterfactuals.

This implies a diagnostic test: **apply a counterfactual pressure**. Ask whether the phenomenon would still be predicted if component X were replaced with another member of its reference class. If yes for most components, the explanation is not load-bearing — it is a family of explanations wearing one face.

The deep consequence: explanatory load-bearing is equivalent to *criticism vulnerability*. An explanation that cannot be damaged by substitution cannot be tested. It therefore carries no knowledge.

## Questions

1. If two components of a load-bearing explanation are swapped simultaneously rather than one at a time, does the conjecture predict the explanation breaks down faster than swapping them sequentially — and if not, does the 'mutual constraint density' framing still hold? — **no**
2. Does the conjecture require that the unique inverse property hold for every component individually, or only for the set as a whole — and would a single non-invertible component disqualify an otherwise tight explanation from being load-bearing? — **no**
3. The conjecture treats the load-bearing/non-load-bearing distinction as a threshold rather than a gradient — but does 'mutual constraint density' as defined actually produce a binary cutoff, or does it implicitly smuggle in a continuous measure that undermines the threshold claim? — **yes**
4. Is causal closure doing independent work in the conjecture, or is it fully reducible to mutual constraint density — if you removed 'causal closure' as a label and kept only constraint density, would any explanatory power be lost? — **no**
5. The conjecture claims non-load-bearing explanations accommodate rather than generate the phenomenon — but does this distinction survive cases where a loose explanation happens to make correct counterfactual predictions by accident, and if so, how does the conjecture handle that? — **no**
6. The bridge example relies on 'resonant frequency loading' being a specific, non-substitutable mechanism — but if a domain lacks the precision to specify such mechanisms (e.g., social science), does the conjecture imply that no load-bearing explanations are possible there, or does it require a domain-relative notion of constraint? — **no**
7. The conjecture defines criticism vulnerability as equivalent to explanatory load-bearing — but does this equivalence hold in both directions, meaning that any explanation vulnerable to substitution-based criticism is automatically load-bearing, or only that load-bearing explanations are necessarily vulnerable? — **no**
8. If a component in a load-bearing explanation belongs to a reference class with only one valid member, is the constraint doing real explanatory work or is it trivially satisfied — and does the conjecture have a way to distinguish genuine mutual constraint from mere uniqueness by exhaustion? — **no**
9. The conjecture claims removing a component invalidates rather than merely weakens a load-bearing explanation — does this require that the remaining components produce zero valid predictions about the explanandum, or only that they no longer uniquely pick it out, and does that distinction matter for the threshold claim? — **yes**
10. The diagnostic test proposed is counterfactual pressure via reference-class substitution — but the validity of this test depends on how reference classes are individuated, and the conjecture offers no theory of reference class individuation, so is the test itself load-bearing within the conjecture's own framework? — **no**

## Candidate Problems

- The conjecture treats mutual constraint density as a binary threshold, but it may be a continuous spectrum with no natural threshold. What determines where the threshold lies — is it a function of the number of valid substitutes per component, the product of those numbers, or something else? Without a principled threshold criterion, the load-bearing/non-load-bearing distinction may itself be underdetermined in the very sense the conjecture diagnoses. (score: 0.92)
- The conjecture claims load-bearing explanations have a 'unique inverse' — you can reconstruct the story from the phenomenon plus one component. This is a strong claim that maps onto the mathematical notion of an overdetermined system. Is explanatory load-bearing formally equivalent to overdetermination in a constraint satisfaction system? If so, the conjecture inherits all the machinery of that domain, including known failure modes like inconsistency under noise. (score: 0.88)
- The diagnostic test — apply counterfactual pressure by substituting components — presupposes that reference classes are well-defined. But reference class membership is itself theory-laden. 'Resonant frequency loading' belongs to a narrow class only given a background theory of fatigue mechanics. Without that theory, it belongs to the broad class 'vibration.' This suggests load-bearing is not intrinsic to an explanation but relative to a background theoretical framework, which threatens the conjecture's claim to identify a structural property. (score: 0.91)
- The conjecture equates explanatory load-bearing with criticism vulnerability, and therefore with knowledge-carrying capacity. But some explanations that are highly substitution-resistant carry no genuine knowledge — for example, a tautological mechanism where the components are defined in terms of the outcome. Is there a way to distinguish genuine constraint from definitional closure? The conjecture may conflate logical entailment with causal explanation. (score: 0.89)
- The conjecture implicitly assumes that the phenomenon to be explained is itself sharply individuated — that there is a fact of the matter about what counts as 'the same phenomenon.' But phenomena are described under descriptions. The bridge collapse under description D1 ('a structure failed') may be accommodated by many explanations, while under D2 ('a specific span failed at 14:32 on a specific date') it may be uniquely entailed. Does load-bearing track the explanation, the phenomenon, or the explanation-phenomenon pair? This is unresolved. (score: 0.87)
- If load-bearing explanations are those that generate rather than merely accommodate phenomena, this maps onto the distinction between prediction and accommodation in philosophy of science. The conjecture may be rediscovering the problem of predictivism — whether a theory confirmed by data it was designed to fit is less confirmed than one that predicted the data in advance. Is the conjecture a structural generalization of predictivism, and if so, does it inherit predictivism's unresolved problems? (score: 0.83)
- The conjecture says non-load-bearing explanations are 'not false.' But if a narrative frame is consistent with a wide range of outcomes including the actual one, it may still assign very low probability to the actual outcome relative to alternatives. Is there a probabilistic version of the conjecture where load-bearing is measured by how much an explanation concentrates probability mass on the actual phenomenon? This would connect the conjecture to Bayesian confirmation theory and might make the threshold criterion precise. (score: 0.85)
- The conjecture focuses on explanations of singular events (a bridge collapse). It is unclear whether the framework extends to explanations of regularities, laws, or statistical patterns. A mechanistic explanation of why metals fatigue in general may have different constraint properties than an explanation of why this bridge failed on this day. Does load-bearing apply to nomological explanations, and if so, what plays the role of 'the phenomenon' that components must jointly entail? (score: 0.78)
- There is an implicit claim that load-bearing explanations are epistemically superior. But in practice, scientists and engineers often deliberately use non-load-bearing explanations as scaffolding — to orient inquiry before the mechanism is known. The conjecture treats narrative frames as epistemically deficient, but they may be epistemically necessary at early stages of inquiry. Is there a developmental account of how narrative frames become load-bearing over time, and what drives that transition? (score: 0.80)
- The conjecture's structural property — mutual constraint density — is defined qualitatively. Can it be operationalized? One candidate: measure the size of the substitution space for each component given the others are fixed, then take some aggregate (product, minimum, average). But this requires a metric on the space of possible component values, which may not exist for qualitative explanations. The operationalization problem may be fundamental rather than technical. (score: 0.82)
- The conjecture assumes that causal closure in an explanation mirrors or tracks causal closure in the world — that a tight explanation reflects a tight causal structure. But this may be an artifact of how we carve phenomena. A highly load-bearing explanation of a social phenomenon (e.g., a market crash) might be tight at one level of description while the underlying causal structure is massively multiply realizable. Does explanatory load-bearing track ontological structure, or is it a property of representational choices? (score: 0.86)
- The conjecture implies that good explanations are fragile — they break under substitution. But robustness is often treated as a scientific virtue: an explanation that survives across multiple modeling frameworks is considered more reliable. There is a direct tension between the conjecture's fragility criterion and the scientific norm of robustness. Can both be virtues simultaneously, or does the conjecture require revising the standard account of robustness? (score: 0.88)
