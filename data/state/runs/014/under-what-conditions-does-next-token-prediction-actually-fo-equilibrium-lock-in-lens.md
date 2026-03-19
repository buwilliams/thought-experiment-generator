# Generated: under-what-conditions-does-next-token-prediction-actually-fo × equilibrium-lock-in-lens

## Conjecture

**Conjecture:**  
Next-token prediction forces the learning of latent variables aligned with real domain invariants only when those invariants function like **indispensable public goods for prediction**: they are costly to ignore, broadly useful across many contexts, and cannot be replaced by a patchwork of local heuristics. Where this condition fails, invariant-tracking is underprovided, not because the model is “confused,” but because the training objective admits a stable equilibrium of shallow predictive substitutes.

The public-goods lens clarifies the structure. A domain invariant — syntax, object permanence, causal identity, conservation, speaker intent, game state, latent topic, etc. — is often **non-excludable inside the model**: once represented, it improves many downstream token predictions across many contexts. Its benefits are diffuse across the loss landscape, while its representational cost is concentrated. Gradient descent therefore faces a coordination problem analogous to public-goods underprovision: no single prediction error may justify building the invariant, even though many predictions would jointly benefit from it.

So prediction is sufficient only when the training distribution and architecture make the invariant a **high-leverage shared bottleneck**. Concretely, this occurs when:

1. **Diffuse benefit becomes aggregatively decisive**: the invariant reduces error across enough tokens, sequences, and contexts that its total loss advantage dominates memorized or local alternatives.  
2. **Substitutes are weak**: surface correlations, retrieval-like pattern matching, and short-horizon heuristics cannot cheaply achieve similar predictive performance.  
3. **Temporal or contextual credit assignment is preserved**: the model can connect present representation costs to future predictive gains.  
4. **The invariant compresses the data-generating process** better than alternatives: representing it is the simplest way to predict well over the support of the distribution.  
5. **Distributional variation exposes the invariant**: contexts vary enough that only the stable latent structure transfers.

Under these conditions, latent-variable learning is not a bonus; it is the only viable way to escape the no-provision equilibrium of local tricks.

What follows is a sharper distinction between two failure modes. If prediction fails to produce domain invariants, that need not show the objective is inadequate in principle. It may show a structural underprovision regime: the invariant’s gains are too dispersed, too delayed, or too substitutable for optimization to “fund” it. In that regime, models rationally settle on shallow equilibria. Apparent “understanding failures” are then analogous to public goods not being supplied despite universal preference for their benefits.

This suggests an empirical test: next-token prediction should learn real invariants when interventions increase the **excludability of predictive benefit** — for example by creating tasks where many predictions hinge on the same latent state, by reducing shortcut availability, or by requiring generalization across varied contexts. If invariant learning rises under those changes, the issue was not motivation or magic, but structure.

## Questions

1. 1. Is the claim that a domain invariant must act as an indispensable public good for prediction necessary for the conclusion that next-token prediction forces learning of real domain invariants, or could the conclusion still follow without that public-goods structure? — **yes**
2. 2. If the claim that diffuse predictive benefits are concentrated into an aggregatively decisive loss advantage were removed, would the conjecture still explain why prediction sometimes forces invariant learning rather than shallow heuristics? — **no**
3. 3. Is the requirement that substitutes such as local heuristics and retrieval-like pattern matching be weak necessary to the conjecture's explanation of when invariant-tracking is forced? — **yes**
4. 4. Would removing the claim that temporal or contextual credit assignment must preserve the link between present representational cost and future predictive gain destroy the conjecture's explanation rather than merely narrow its scope? — **yes**
5. 5. Does the conjecture imply that increasing distributional variation should promote learning of syntax, object permanence, or causal identity even in domains not explicitly discussed in the problem? — **yes**
6. 6. Does the public-goods framing illuminate why larger context windows or architectural changes that improve long-range credit assignment might increase invariant learning beyond what the problem directly asked about? — **yes**
7. 7. Does the conjecture predict that two training setups with similar next-token loss could differ sharply in learned invariants if one permits cheap shortcut substitutes and the other makes many predictions depend on a shared latent state? — **yes**
8. 8. If a model learned a real invariant in a setting where local heuristics remained cheap and highly effective, would saving the conjecture require abandoning the claim that weak substitutes are central rather than adding a minor qualification? — **yes**
9. 9. If invariant learning occurred even when predictive benefits were sparse and not broadly shared across contexts, would preserving the conjecture force a major retreat from the public-goods analogy instead of a small patch? — **yes**
10. 10. If next-token prediction reliably produced invariant-tracking despite poor temporal credit assignment, would explaining that case require gutting the conjecture's core mechanism that optimization must connect representation cost to delayed predictive benefit? — **yes**

## Candidate Problems

- How can the conjecture be made precise enough to distinguish 'real domain invariants' from merely useful internal features, and to operationalize 'indispensable public goods for prediction' in measurable terms? The core unresolved tension is definitional: without a formal criterion for invariance, non-excludability, substitutability, and aggregate predictive benefit, the conjecture risks explaining any outcome post hoc. A worthwhile problem is to build a theory and metrics that predict in advance when next-token training should or should not induce a given latent variable. (score: 0.97)
- What are the actual causal roles of optimization dynamics and architecture in producing the alleged underprovision equilibrium, as opposed to the data distribution alone? The conjecture attributes failure partly to a coordination problem in gradient descent, but it remains open whether this is truly an optimization pathology, a representational bottleneck issue, or simply that shallow heuristics are globally better under the objective. A key new problem is to isolate whether different optimizers, memory mechanisms, recurrence, attention structure, or auxiliary losses change invariant learning even under the same distribution. (score: 0.93)
- Can excludability-style interventions reliably force invariant learning, or do models still discover new shallow substitutes that mimic success without tracking the intended latent structure? This is the main empirical tension in the proposal: the suggested tests may improve performance while leaving the underlying mechanism ambiguous. A strong problem is to design adversarial and out-of-distribution experiments that can discriminate genuine invariant-tracking from increasingly sophisticated local patchworks, and to determine the boundary where prediction alone ceases to suffice. (score: 0.95)
