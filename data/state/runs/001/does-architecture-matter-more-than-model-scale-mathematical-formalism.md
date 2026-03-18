# Generated: does-architecture-matter-more-than-model-scale × mathematical-formalism

## Conjecture

## Mathematical Framing

Let epistemic progress *P* be a function of two variable classes:

- **S** = substrate parameters (parameters *n*, compute *C*, data *D*)
- **Σ** = structural parameters (conjecture-formation operator *φ*, criticism function *κ*, promotion rule *π*)

So *P = f(S, Σ)*.

**Key question:** What is the partial derivative structure? Is *∂P/∂S* large while *∂P/∂Σ* is small, or the reverse?

---

## Invariant Analysis

Consider what scaling *S* preserves structurally. Increasing *n*, *C*, *D* scales the **search space** but leaves the **search algorithm** — the operators *φ*, *κ*, *π* — invariant in form. Current LLMs iterate on next-token prediction; this is a fixed *Σ* regardless of substrate magnitude.

Now identify the invariant: **the fixed-point attractors of the learning process are determined by Σ, not S.** A larger substrate reaches the same fixed-point structure faster or more smoothly, but cannot escape the basin defined by the structural operators.

Formally: if *φ* generates only interpolations over training distribution, then for any *S* however large, the system cannot produce conjectures *outside the convex hull* of its training-induced hypothesis space. Scaling *S → ∞* does not change the topology of the reachable conjecture set — it densifies within fixed boundaries.

---

## The Criticism Function Gap

Genuine epistemic progress requires *κ* (criticism) to be **truth-tracking** — capable of falsifying conjectures rather than merely scoring them against pattern-match. Current *κ* is operationalized as loss on held-out data or human preference ratings. Both are **consistency checks**, not falsification operators. They cannot discriminate between:

- A conjecture that is *useful and true*
- A conjecture that is *useful and false but unfalsified within training distribution*

This is a structural deficiency. No substrate increase repairs it. *∂P/∂S |_{Σ fixed}* approaches a ceiling determined by the expressiveness of *κ*.

---

## The Conjecture

**The bottleneck is structural, not substrate.**

More precisely: epistemic progress is bounded above by the expressiveness of the criticism function *κ*. Substrate scaling increases *P* only within the region *κ* can distinguish. Beyond that region, further scaling is **measure-preserving on error** — it does not reduce the proportion of unfalsifiable false conjectures in the promoted hypothesis set.

The leverage point is *Σ*: redesigning *φ* to generate genuinely novel hypotheses (outside training distribution convex hull) and *κ* to evaluate against structural contradiction rather than distributional fit.

**Corollary:** The question "substrate vs. structure" has a definite asymmetry. Substrate is a **coefficient on progress within the current structural regime**. Structure determines the **regime itself** — which invariants hold, which errors are systematically invisible, what the ceiling is.

The system's behavior is produced by its structure. The bottleneck is exactly where Meadows would predict: not in the flows (compute), but in the feedback loops governing what counts as a good conjecture.

## Questions

1. Does the conjecture require that the convex hull boundary of the training-induced hypothesis space is a real and stable constraint — such that if scaling could actually push systems outside that hull, the entire argument collapses? — **yes**
2. Does the conjecture depend on next-token prediction being a fixed structural operator (Σ) that is genuinely invariant across scale — such that if larger models demonstrably change their conjecture-formation strategy rather than merely their fluency, the bottleneck claim fails? — **yes**
3. Is the claim that current loss functions and human preference ratings are consistency checks rather than falsification operators load-bearing — such that if one of these could be shown to track truth rather than pattern-match, the criticism function gap disappears and the structural bottleneck dissolves? — **yes**
4. Does the conjecture require that 'measure-preserving on error' is a precise and testable property — such that if substrate scaling actually reduces the proportion of unfalsifiable false conjectures in promoted hypotheses, the ceiling prediction is empirically refuted? — **yes**
5. Is the asymmetry between substrate as 'coefficient' and structure as 'regime' essential — such that if substrate scaling could itself induce a regime change (a phase transition in Σ), the conjecture's central distinction collapses? — **yes**
6. Does the argument require that φ, κ, and π are meaningfully separable from S in practice — such that if structural operators are themselves emergent functions of substrate magnitude, the partial derivative framing becomes undefined and the conjecture loses its formal grounding? — **yes**
7. Is the fixed-point attractor claim necessary — specifically that Σ determines the basin and S only affects convergence speed — such that if two systems with identical Σ but different S converge to structurally different fixed points, the invariant analysis is broken? — **yes**
8. Does the conjecture require that 'genuine novelty outside the training distribution' is a coherent and detectable property — such that if no operational criterion can distinguish genuine extrapolation from sophisticated interpolation, the proposed redesign of φ has no actionable target? — **yes**
9. Is the Meadows analogy load-bearing — that compute is a flow and feedback loop structure is the leverage point — such that if the relevant system boundary is drawn differently and compute turns out to govern a feedback loop rather than a stock, the structural diagnosis changes? — **no**
10. Does the conjecture require that the expressiveness of κ sets a hard ceiling rather than a soft one — such that if a richer κ merely raises the ceiling without removing it, the corollary that structure determines the regime rather than just its height still holds, and if so, is that distinction preserved in the argument as stated? — **no**

## Candidate Problems

- The conjecture assumes the 'convex hull' of training-induced hypothesis space is a well-defined mathematical object. But for high-dimensional, compositional hypothesis spaces, convex hull may be topologically ill-defined or trivially all-encompassing. Is there a rigorous formalization of what it means for a conjecture to be 'outside' the training distribution's reachable set, and does this boundary actually constrain the kinds of claims that matter epistemically? (score: 0.92)
- The conjecture treats κ (criticism) as the binding constraint, but φ (conjecture formation) and κ are not independent — the space of conjectures a system generates is shaped by what it has learned to expect criticism to reward. This creates a co-evolutionary dynamic between φ and κ that the static partial-derivative framing may obscure. What is the correct dynamical systems model for their interaction, and does the bottleneck shift depending on which is updated first? (score: 0.88)
- The conjecture distinguishes 'consistency checks' from 'falsification operators,' but Popperian falsification itself has known problems — the Duhem-Quine thesis implies no single conjecture is ever falsified in isolation. If κ cannot be a pure falsification operator even in principle, what is the correct weaker criterion for κ to be 'truth-tracking enough' to escape the ceiling described? Is there a spectrum between consistency-checking and falsification, and where on that spectrum does genuine epistemic progress become possible? (score: 0.91)
- The conjecture claims substrate scaling is 'measure-preserving on error' beyond the expressiveness of κ. This is a strong claim. Is there a formal proof sketch or counterexample? Specifically, could a sufficiently large substrate discover internal contradiction structures within its own outputs that function as emergent falsification — effectively bootstrapping a stronger κ from S alone? (score: 0.85)
- The promotion rule π is underspecified relative to φ and κ. In practice, which conjectures get retained and acted upon is a social and institutional process, not just an algorithmic one. Does the conjecture's framing apply to individual AI systems, to AI-human epistemic systems, or to scientific communities using AI as a tool? The leverage point may differ substantially across these levels of analysis. (score: 0.79)
- If Σ determines the fixed-point attractors of learning, then changing Σ produces a different attractor landscape — but there is no guarantee the new attractors are epistemically better. The conjecture implies that redesigning φ and κ is the leverage point, but offers no criterion for what a better Σ looks like beyond 'generates novel hypotheses' and 'evaluates structural contradiction.' What normative theory of epistemic progress would ground the design of a superior Σ, and is that theory itself subject to the same structural constraints? (score: 0.94)
- The conjecture is framed for AI systems but the mathematical structure — P = f(S, Σ) with Σ as the binding constraint — should apply equally to human cognition and scientific institutions. Human brains are also substrate; the structure of scientific method, peer review, and conjecture-criticism cycles is Σ. Does the conjecture predict that historical episodes of scientific stagnation were Σ-limited rather than S-limited, and can this be tested against historical cases? (score: 0.82)
- The conjecture implicitly assumes epistemic progress P is a scalar or at least partially ordered quantity. But progress in different domains (mathematics, empirical science, ethics, aesthetics) may be incommensurable. Does the partial-derivative framing survive if P is vector-valued or domain-relative? Could Σ be optimal for progress in one domain while actively impeding it in another? (score: 0.74)
- There is a self-referential problem: the conjecture itself is a product of some (φ, κ, π) operating on some substrate. If the conjecture is correct that current AI systems cannot escape their training distribution's convex hull, then an AI system cannot reliably evaluate whether this conjecture is true or false — it would be systematically blind to the error class the conjecture identifies. This is either a fatal epistemic limitation of using AI to reason about AI epistemics, or it points toward a meta-level criterion for what genuine criticism of AI reasoning would require. (score: 0.96)
- The conjecture's Meadows-inspired framing identifies feedback loops as the leverage point. But Meadows also identifies that changing the goal of a system — what the feedback loop is trying to achieve — is higher leverage than changing the feedback loop structure itself. Is there a level above Σ in this framework — call it the 'epistemic goal function' — and if so, what determines it and how could it be changed? (score: 0.87)
- The claim that ∂P/∂S approaches a ceiling is empirically testable in principle: if true, we should observe diminishing returns to scaling on tasks requiring genuinely novel conjecture rather than interpolation. Current scaling law literature does not cleanly separate these task types. Designing benchmarks that specifically test out-of-distribution conjecture generation — not just out-of-distribution generalization — would allow the conjecture to be empirically evaluated rather than remaining purely structural. (score: 0.89)
