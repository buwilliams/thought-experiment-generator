# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × computation-as-life

## Conjecture

**Conjecture:**  
*Counterfactual reach is the computational breadth of a model’s valid update rule under intervention on latent generative structure, not merely its success at forecasting observations. A system has high counterfactual reach when the same internal program continues to produce correct transformations across many non-actual but computably related worlds, where those worlds differ by interventions on structure rather than by samples from one fixed distribution.*

From the computation-first perspective, prediction, compression, causal modeling, and invariance are all partial signatures of a deeper property: the ability of a computation to remain truth-preserving when its input world is systematically rewritten. The key distinction is this:

- **Prediction** asks: does the model map present states to likely future states in this world?
- **Compression** asks: does it encode regularity efficiently?
- **Causal modeling** asks: does it support certain intervention semantics?
- **Invariance** asks: what stays stable under a family of transformations?

But **counterfactual reach** asks: *how large is the space of structurally transformed worlds over which the same learned computation still works, with only boundary-condition updates and no rewrite of the core rule?*

So the structural criterion is not accuracy, simplicity, or intervention-readiness alone, but **algorithmic portability across world-variants**.

More precisely: a representation has counterfactual reach to the extent that its load-bearing internal variables correspond to generative constraints whose recombination supports correct inference in many adjacent computational environments. A shallow model can predict well by memorizing correlations local to one data-generating process. A high-reach model captures a rule-set that survives when selection pressures, mechanisms, or object relations are altered.

This avoids collapse into existing notions because:

1. **Not prediction:** one can predict extremely well in-distribution with zero reach if the computation fails under structural perturbation.
2. **Not compression:** a compressed code may summarize observed data while encoding no reusable transformation law.
3. **Not causal modeling:** a causal graph may support interventions within a specified variable set yet have low reach if its ontology is too parochial to transport to altered mechanisms.
4. **Not invariance:** invariance is one ingredient, but reach concerns the *extent of correct recomputation* across transformed environments, not merely what remains unchanged.

In systems terms, counterfactual reach measures how far a model has captured the feedback-generating architecture rather than surface trajectories. In evolutionary-computational terms, it is analogous to **fitness across neighboring fitness landscapes**, not just performance on one landscape. A good explanation is one whose internal computation is selected not only by actuality but by many nearby unrealized alternatives.

So a workable formal direction is: define a space of computable structural interventions on a generative process, and measure the size/diversity of the subset for which a model’s core inference program remains valid without retraining its ontology. That subset—not prediction error or description length alone—is its counterfactual reach.

**Illumination:** counterfactual reach is best understood as a property of **representation-world coupling**: how much structural rewriting reality can undergo before the model must stop being itself.

## Questions

1. 1. If the explanation drops the requirement that the same core inference program must remain valid under structural interventions with only boundary-condition updates, does its distinction from prediction and ordinary generalization disappear? — **yes**
2. 2. Is the claim that interventions must target latent generative structure rather than resampling from one fixed distribution necessary for counterfactual reach to be a new structural criterion rather than a variant of robustness? — **yes**
3. 3. If the explanation no longer requires load-bearing internal variables to correspond to generative constraints, would algorithmic portability across world-variants lose the basis for being truth-preserving rather than accidental? — **yes**
4. 4. Does the conclusion that counterfactual reach avoids collapse into compression, causal modeling, and invariance depend essentially on measuring validity across many computably related worlds rather than on performance in one world? — **yes**
5. 5. Does this conjecture imply that two models with equal predictive accuracy on observed data can differ systematically in expected performance under unseen mechanism changes because one has greater counterfactual reach? — **yes**
6. 6. Does the account illuminate why a model with a compact representation or an intervention-capable causal graph might still fail badly when object relations or selection pressures are recombined outside its original ontology? — **yes**
7. 7. If made precise, would this notion apply to comparing scientific theories, learned simulators, and biological control systems under the same criterion of core-rule portability across altered environments? — **yes**
8. 8. If a memorizing model is found to survive a few handpicked structural perturbations, would preserving the conjecture require abandoning the claim that reach tracks captured generative constraints rather than local correlation? — **no**
9. 9. If a causal model with a fixed variable set transfers well to a new mechanism only after adding new latent variables, would saving the conjecture force a retreat from the no rewrite of the core rule condition? — **yes**
10. 10. If the proposed metric counts any successful adaptation after retraining as evidence of reach, would that patch gut the conjecture by collapsing algorithmic portability back into ordinary predictive flexibility? — **yes**

## Candidate Problems

- How can 'the same internal program remains valid under structural intervention' be formalized non-circularly? The conjecture depends on load-bearing notions—core rule, boundary-condition update, latent generative structure, ontology rewrite, and adjacency among worlds—but without a principled criterion these can be redescribed to make reach arbitrarily high or low. A key open problem is to define equivalence classes of models and interventions so that counterfactual reach is invariant to superficial reparameterization yet sensitive to genuine explanatory structure. (score: 0.97)
- What is the right measure over the space of structurally transformed worlds, and how can it avoid trivialization? If reach is 'how large/diverse' the valid world-subset is, the result depends entirely on how neighboring worlds are generated, weighted, and bounded for computability. Too permissive a space rewards vacuous universal models; too narrow a space collapses back to ordinary robustness or domain generalization. The unresolved tension is whether there exists a non-arbitrary intervention distribution or complexity-weighted geometry of world-variants that makes the concept empirically and mathematically substantive. (score: 0.95)
- How does counterfactual reach differ in a load-bearing way from existing notions such as causal abstraction, transportability, out-of-distribution generalization, mechanistic invariance, and minimum description length? The conjecture asserts non-equivalence, but it remains open whether reach is a genuinely new property or a recombination of known ones under different language. A worthwhile problem is to derive separating examples and impossibility/results showing cases where these notions come apart, and to identify whether reach predicts explanatory or scientific success better than existing measures. (score: 0.93)
