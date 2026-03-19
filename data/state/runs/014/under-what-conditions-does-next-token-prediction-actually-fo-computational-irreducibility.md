# Generated: under-what-conditions-does-next-token-prediction-actually-fo × computational-irreducibility

## Conjecture

**Conjecture:**  
Next-token prediction forces a model to learn latent variables that track real domain invariants **only when those invariants are part of the shortest generative program needed to continue the sequence across the distribution of cases**. When the underlying process is computationally irreducible, prediction does not generally recover “true” latent structure unless that structure is a necessary state variable of the minimal rule generating future tokens.

From the computational-irreducibility perspective, the key issue is not whether the future *depends* on hidden variables in some metaphysical sense, but whether those variables are **compressive necessities** for prediction. If a sequence can be continued well by local statistical imitation, memorized fragments, or shallow heuristics, then next-token loss supplies no pressure to discover deeper invariants. In that regime, prediction succeeds without ontology.

But if the data come from a process where correct continuation requires carrying forward a compact hidden state—because no shortcut exists except simulating the underlying rule—then prediction is pushed toward learning whatever internal variables make that simulation possible. The learned latent variables correspond to real invariants not because the model is rewarded for “understanding,” but because those invariants are the only efficient handles on an irreducible unfolding.

So the condition is stronger than “the invariant causally matters.” It is:  
1. **The invariant must be necessary for low-loss continuation across interventions or distributional variation**, not merely correlated in the training distribution.  
2. **The invariant must reduce description length or computation of the predictor**, functioning as a minimal state summary of the generative process.  
3. **No easier surrogate strategy may exist** that attains similar predictive performance by exploiting surface regularities.  
4. **The training distribution must expose enough trajectories** that preserving the invariant is more efficient than memorizing exceptions.

This reframes the usual sufficiency claim. Prediction is sufficient only when the target process is such that **to predict is effectively to emulate the generator**, and emulation requires representing its stable state variables. In irreducible systems, there may be no closed-form shortcut from past tokens to future tokens; the best predictor must internally “run” the process. Then latent variables emerge as bookkeeping devices of simulation.

What this illuminates is a divide between two worlds:

- In **reducible or shortcut-rich domains**, next-token prediction can remain behaviorally competent while learning latents that are merely predictive conveniences, not real invariants.  
- In **irreducible but state-compressible domains**, prediction pressures the model toward ontologically meaningful representations, because the only way forward is to instantiate the generative dynamics.

Thus, prediction is not sufficient in general; it is sufficient **when real invariants coincide with the minimal internal state required to simulate the continuation process**. The relevant threshold in the unfinished statement is therefore not just that the target “depends on” latent variables, but that the task distribution makes those variables **indispensable as irreducible generative state**.

## Questions

1. 1. If the claim that real invariants must be part of the shortest generative program is removed, does the conjecture still explain why next-token prediction would force learning true latent variables rather than any predictive proxy? — **no**
2. 2. Is the condition that no easier surrogate strategy may exist necessary for the conclusion, in the sense that allowing a shallow shortcut would collapse the claim that prediction forces invariant-tracking latents? — **yes**
3. 3. If the training distribution did not expose enough trajectories across interventions or distributional variation, would the conjecture lose its explanation for why the model must preserve the invariant rather than memorize cases? — **yes**
4. 4. Is the claim that the invariant must function as a minimal state summary required for the conclusion that the best predictor must internally emulate the generator? — **yes**
5. 5. Does the conjecture imply that in domains like board games or physical simulations, next-token prediction should recover hidden game state or conserved quantities when those are the only compact state variables supporting continuation across many trajectories? — **yes**
6. 6. Does the conjecture extend to explain why two models with similar next-token accuracy could differ in whether their latents track real invariants, depending on whether one relies on shortcut-rich statistics and the other on simulation-like state? — **yes**
7. 7. Does the conjecture illuminate when scaling data or model size alone should fail to produce ontologically meaningful latents, namely in distributions where local imitation remains an easier predictive strategy than representing irreducible generative state? — **yes**
8. 8. If a counterexample showed a model learning true invariants even though a cheap surface heuristic achieved similar loss, would saving the conjecture require abandoning the claim that no easier surrogate strategy may exist rather than adding a minor qualification? — **no**
9. 9. If one tried to rescue the conjecture from a case where the model predicts well in an irreducible system without identifiable latent state variables, would that force a rewrite of the claim that irreducible prediction requires compact bookkeeping variables? — **yes**
10. 10. If a model learned latents correlated with real invariants only on the training distribution but those latents failed under intervention, would defending the conjecture require preserving the cross-distribution necessity condition rather than patching it with a looser notion of causal relevance? — **yes**

## Candidate Problems

- How can 'shortest generative program', 'minimal state summary', and 'real domain invariant' be made precise enough to test the conjecture? The central unresolved tension is whether these notions can be operationalized in a model- and distribution-independent way, or whether the claim collapses into architecture-relative and coding-relative descriptions of compression. (score: 0.97)
- Under what exact conditions do powerful predictors learn true invariants rather than equally performant surrogate features when multiple internal programs achieve similar next-token loss? This is an identifiability problem: even if an invariant is sufficient or compact, it may not be uniquely selected unless interventions, distribution shift, or optimization biases break the symmetry. (score: 0.95)
- Is computational irreducibility actually the right boundary for representation learning, or is the decisive factor the interaction among training distribution, model architecture, optimization dynamics, and finite-resource constraints? The open question is whether 'prediction forces ontology' is governed more by abstract properties of the source process or by practical learning-system biases that can favor or suppress invariant-tracking even in principle-relevant cases. (score: 0.92)
