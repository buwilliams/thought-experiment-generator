# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × mathematical-formalism

## Conjecture

**Conjecture:** *Counterfactual reach is best formalized as the size and structure of the set of admissible world-model transformations under which an agent’s internal representation continues to support correct downstream reasoning about non-actual possibilities.* It is not prediction, compression, causal modeling, or invariance alone, but a higher-order structural property of a representation relative to a family of counterfactual maps.

More concretely, let:

- \(W\) be a space of possible worlds or generative structures,
- \(R\) be an agent’s representation,
- \(T\) a class of transformations on \(W\) corresponding to counterfactual alterations,
- \(Q\) a family of queries about consequences in transformed worlds.

Then define counterfactual reach not by how well \(R\) predicts the actual world, nor by its code length, nor merely by whether it encodes causal arrows, but by the extent to which there exists a **transport map**
\[
\phi_T : R(W) \to R(T(W))
\]
such that answers to \(Q\) are preserved or computably recoverable after transformation:
\[
Q(T(W)) \approx \Psi(Q, \phi_T(R(W))).
\]

The key structural criterion is this: a representation has high counterfactual reach when it supports a **large, compositional semigroup** of such transformations with low repair cost. “Low repair cost” means that after modifying the world by \(T\), one does not need to rebuild the representation from raw data; one can *push forward* the existing representation through structured adjustment.

This distinguishes counterfactual reach from nearby notions:

- **Prediction** concerns accuracy on actual or sampled trajectories; reach concerns competence on systematically transformed ones.
- **Compression** minimizes description length; reach requires factorization into parts that remain manipulable under intervention.
- **Causal modeling** supplies intervention semantics, but reach measures how *far* and *richly* those interventions can propagate through a representation while preserving answerability.
- **Invariance** asks what stays fixed across transformations; reach asks whether the representation tracks the *lawful variation* induced by transformations.

So the relevant invariant is not object-level stability, but **equivariance of inferential structure**. A representation with reach is one where changing assumptions induces correspondingly structured changes in conclusions.

This suggests a quantitative measure:
\[
\mathrm{CR}(R) = \mu\big(\{T \in \mathcal{T} : R \text{ supports faithful transport under } T\}\big),
\]
possibly weighted by transformation complexity and query depth. But the more load-bearing feature is algebraic: whether the admissible transformations compose, localize, and preserve inferential scaffolding.

The illuminating consequence is that counterfactual reach is fundamentally a property of **representation geometry**. It measures not how much a model knows, but how well its knowledge is organized for systematic re-use under unrealized alternatives. A system has reach when its internal structure mirrors the transformation structure of the world closely enough that “what if?” can be answered by *mapping*, not by retraining or reobserving.

## Questions

1. 1. Necessity: If the explanation drops the requirement that the same internal representation be transported to the transformed world rather than rebuilt from raw data, does its account of counterfactual reach collapse back into ordinary predictive competence? — **yes**
2. 2. Necessity: Is the claim that admissible transformations form a large compositional semigroup required for the conjecture to distinguish reach from a mere list of unrelated counterfactual tricks? — **yes**
3. 3. Necessity: If faithful transport only preserved final answers to queries but not the inferential structure relating assumptions to conclusions, would the conjecture lose its stated distinction from invariance and causal modeling? — **yes**
4. 4. Necessity: Does the conclusion that counterfactual reach is a higher-order structural property depend essentially on low repair cost, rather than on transformation coverage alone? — **yes**
5. 5. Reach: Does the conjecture imply that two models with equal predictive accuracy on actual data can differ systematically in counterfactual reach because one supports transport across more structured transformations? — **yes**
6. 6. Reach: Does the representation geometry view illuminate why a compressed representation can fail at what if reasoning even when it retains the same information needed for prediction on observed trajectories? — **yes**
7. 7. Reach: Does the semigroup and localization requirement extend the explanation to multi-step or modular counterfactual edits that the original problem statement did not explicitly mention? — **yes**
8. 8. Resistance to patching: If a black-box predictor answers many counterfactual queries correctly only after separate fine-tuning for each intervention, would saving the conjecture require abandoning the low repair cost condition rather than adding a minor exception? — **yes**
9. 9. Resistance to patching: If a causal model supports intervention semantics but lacks any reusable transport map on its internal representation, would accommodating it force the conjecture to give up its core claim that reach concerns representation-level equivariance? — **yes**
10. 10. Resistance to patching: If an invariant representation performs well only for transformations that leave most latent factors fixed, would treating that as high counterfactual reach require replacing the large structured family of admissible transformations with a much weaker notion? — **yes**

## Candidate Problems

- What exactly makes a transformation class T 'admissible' and non-circularly tied to the representation R? The conjecture defines counterfactual reach via transformations under which R can be transported, but this risks trivialization unless there are independent constraints on which world-transformations count, how they are specified, and what counts as 'faithful transport' versus hidden recomputation. A central open problem is to give a representation-independent criterion for admissibility, repair cost, and faithfulness that does not collapse CR into a restatement of whatever transformations a chosen model already handles. (score: 0.97)
- How can counterfactual reach be distinguished in a load-bearing way from existing notions such as causal abstraction, equivariant representation learning, sufficient statistics, and model-based planning? The conjecture claims reach is not prediction, compression, causal modeling, or invariance alone, but it remains unresolved whether CR is genuinely a new property or a recombination of these under different language. A worthwhile problem is to derive separating examples, impossibility results, or equivalence theorems showing when high CR diverges from high predictive accuracy, compactness, causal identifiability, or symmetry-equivariance. (score: 0.95)
- What is the right quantitative and algebraic formalization of 'large, compositional semigroup with low repair cost,' especially when queries Q have varying depth and transformations only locally compose? The current measure mu({T : ...}) is suggestive but underspecified: different choices of measure, weighting, query family, and computational budget could radically change rankings of representations. An important open question is whether there exists a robust metric or family of metrics for CR that is stable under reparameterization and supports meaningful comparison across agents, tasks, and world-model classes. (score: 0.93)
