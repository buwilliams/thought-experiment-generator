# Generated: under-what-conditions-does-next-token-prediction-actually-fo × load-bearing-constraint

## Conjecture

**Conjecture:** Next-token prediction forces the learning of latent variables that track real domain invariants **only when those invariants are load-bearing for compression across many contexts**—that is, when changing or omitting them would require widespread compensating complexity in the predictor, and when no comparably cheap surface-level surrogate can do the same job.

The key constraint is not “prediction” in the abstract, but whether the data-generating structure makes certain hidden factors indispensable. A latent corresponds to a real invariant when it explains many token dependencies at once, across interventions in surface form, and when alternative hypotheses fragment into ad hoc casework. In that case, the invariant is not decorative: remove it, and the model must memorize disconnected regularities or build a more brittle substitute. That is a load-bearing role.

So prediction is sufficient only under conditions like these:

1. **The invariant has broad downstream consequences.**  
   It must constrain many future tokens, not just a few local patterns. If a hidden variable affects syntax, semantics, reference, dynamics, or action consequences across many sequences, then representing it becomes cheaper than repeatedly approximating its effects.

2. **The data vary enough to expose the invariant/non-invariant distinction.**  
   If superficial correlations remain stable, the model can succeed with proxies. Only when contexts scramble surface cues while preserving the deeper structure does the true invariant become the simpler explanation.

3. **The objective rewards reusable internal structure.**  
   Finite capacity, parameter sharing, long-horizon prediction, or distributional breadth can make memorized heuristics expensive. These pressures make a compact latent world-model preferable.

4. **There are no equally cheap shortcut predictors.**  
   If token statistics, stylistic markers, or local co-occurrence patterns predict as well as the invariant, then the invariant is not forced. Prediction alone underdetermines ontology.

This perspective sharpens the conjecture by treating “real latent variable” not as whatever internal feature correlates with success, but as whatever bears explanatory load across the system. A genuine domain invariant is one whose alteration propagates: if you swap it out, many predictions fail unless the rest of the model reorganizes. By contrast, a narrative-shell representation consists of interchangeable features that can be replaced by different heuristics with little global cost.

A useful thought experiment: imagine two corpora with identical predictive accuracy ceilings. In one, hidden state variables causally organize many token trajectories under varied conditions; in the other, shallow cues track the same outputs. If a model can be reparameterized to preserve performance in the second without changing much else, its latents were not forced. In the first, removing the hidden-state representation causes system-wide degradation or compensatory complexity. That difference marks when prediction identifies invariants rather than merely exploits correlations.

So the conjecture is: **next-token prediction learns real invariants when those invariants are the uniquely low-complexity, high-reuse solution to the predictive problem under sufficient variation and constraint; otherwise it learns whatever surrogate structure carries the predictive load.**

## Questions

1. 1. Necessity: If the claim that the invariant must be load-bearing across many contexts were removed, would the conjecture still explain why next-token prediction learns real invariants rather than arbitrary correlated features? — **no**
2. 2. Necessity: Is the requirement that the data vary enough to scramble surface cues while preserving deeper structure necessary for the conclusion that prediction forces invariant-tracking latents? — **yes**
3. 3. Necessity: If there were equally cheap surface-level surrogates, would the conjecture lose its explanation of why the model must represent the real invariant rather than a proxy? — **yes**
4. 4. Necessity: Is the claim that the objective rewards reusable internal structure through limits such as finite capacity, parameter sharing, or long-horizon prediction required for the forcing argument rather than merely strengthening it? — **yes**
5. 5. Reach: Does the conjecture imply that in multimodal or action-conditioned sequence data, next-token prediction should recover hidden state variables more reliably when those variables constrain many downstream observations across modalities? — **yes**
6. 6. Reach: Does the conjecture predict that increasing distributional breadth or intervention-like variation in training data should make learned latents align better with real domain invariants even when benchmark predictive accuracy changes little? — **yes**
7. 7. Reach: Does the conjecture illuminate why two models with similar next-token accuracy could differ sharply in whether their internal representations track causal structure, depending on the availability of cheap shortcuts in their training corpora? — **yes**
8. 8. Resistance to patching: If a model learns a real invariant even though that invariant has only narrow local consequences, would saving the conjecture require abandoning the claim that broad downstream effects are what make invariants forced? — **yes**
9. 9. Resistance to patching: If a corpus contains stable shallow cues and yet the model still learns the deeper invariant, would rescuing the conjecture require gutting the claim that no comparably cheap surrogate can do the predictive job? — **no**
10. 10. Resistance to patching: If an overparameterized model with abundant capacity still learns invariant-tracking latents without pressure for compact reuse, would preserving the conjecture require dropping the structural role assigned to objective-induced compression pressures? — **yes**

## Candidate Problems

- How can 'load-bearing for compression across many contexts' be made precise and testable? The conjecture hinges on a notion of indispensability, compensating complexity, and uniquely low-complexity representation, but it leaves open the metric: MDL, intervention-based regret, reparameterization cost, causal abstraction criteria, or some other formalism. Without a principled measure, 'real invariant' risks collapsing into an intuitive label rather than a falsifiable property. (score: 0.97)
- Under what data and training conditions are real invariants actually identifiable rather than permanently underdetermined by equally predictive surrogates? The conjecture says sufficient variation, breadth, and constraint can force invariant learning, but it remains unclear what kinds and amounts of variation break shortcut equivalence classes, and whether next-token objectives alone can ever guarantee recovery of the invariant instead of one of many functionally interchangeable latent codings. (score: 0.95)
- What role do architecture and optimization play relative to the data-generating structure? The conjecture emphasizes properties of the predictive problem, but an unresolved tension is whether SGD, transformer inductive biases, finite depth/width, and tokenization can systematically favor brittle surface heuristics or particular latent factorizations even when a domain invariant is in principle the most compressive solution. If so, 'forced by prediction' may depend as much on learning dynamics as on the invariant's objective load-bearing status. (score: 0.93)
