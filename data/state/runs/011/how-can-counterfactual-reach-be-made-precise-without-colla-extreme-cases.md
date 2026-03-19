# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × extreme-cases

## Conjecture

**Conjecture:**  
*Counterfactual reach is best made precise as the boundary-case capacity of a representation to remain action-guiding across an expanding space of interventions, where the expansion is not merely variation in observed data but variation in the generative structure that produces the data. Its distinctive structural criterion is not predictive accuracy, compressibility, causal identifiability, or invariance alone, but the extent to which a model’s internal constraints continue to generate correct answers when pushed toward interventions it was not optimized on, without requiring ad hoc reparameterization.*

Push the problem to the edge:

- If “reach” goes to **zero**, a system only answers within the exact distribution it was trained on. Then it is just prediction or compression over actuals.
- If “reach” goes to **infinity**, the representation would answer under arbitrary interventions across arbitrarily altered circumstances. That would amount to universal explanatory knowledge, not just a fitted model.

So the middle is illuminated by the limit: counterfactual reach is about **how far from actuality a representation can be pushed before its competence fails**.

This separates it from neighboring notions:

1. **Not prediction:** prediction can succeed at the actual trajectory while failing immediately under intervention. At the zero-limit, prediction and reach coincide; beyond that, they diverge.
2. **Not compression:** a compressed representation may summarize regularities of observed data yet carry no guidance for altered regimes. Compression captures economy over actuals; reach measures portability beyond them.
3. **Not causal modeling:** a causal graph can encode intervention semantics, but counterfactual reach concerns the *radius of valid redeployment* of a representation under structural perturbation. Two models with the same causal formalism may differ in reach if one depends on brittle, local parameterizations.
4. **Not invariance:** invariance marks what stays fixed across some transformations; reach measures how far one can vary the system while preserving enough structure for the representation to still work. Invariance is one ingredient, not the whole criterion.

The load-bearing detail is this:  
A representation has greater counterfactual reach **iff** its success under intervention is explained by structural constraints that would force widespread failure were they false. In other words, high-reach representations are those whose internal organization is tightly coupled to the real modal structure of the system, so that they survive boundary-case perturbations without patching.

This suggests a precision criterion:

> **Counterfactual reach = the maximal class or distance of interventions over which a representation remains reliably truth-tracking and policy-guiding while preserving its core explanatory structure.**

“Preserving core explanatory structure” is essential; otherwise one can smuggle in endless local fixes and call that reach. The boundary test is whether the same underlying explanation continues to bear the load as interventions become more alien.

So the conjecture is that counterfactual reach should be formalized not as another static property of models, but as a **robustness-to-structural-intervention profile under explanation-preserving reuse**. What it illuminates is modal depth: the difference between a model that fits reality and one that has latched onto why reality would continue to make sense when bent.

## Questions

1. 1. If the explanation drops the boundary-case limit argument using zero reach and infinite reach, does it lose its basis for defining the middle notion of counterfactual reach rather than merely becoming less vivid? — **yes**
2. 2. Is the claim that interventions must vary the generative structure of the data, not just the observed distribution, necessary to keep counterfactual reach from collapsing into prediction or compression? — **yes**
3. 3. If preserving core explanatory structure under reuse is removed, would the proposed precision criterion fail to distinguish genuine reach from a collection of locally retuned models? — **yes**
4. 4. Is the iff claim that high reach requires internal constraints whose falsity would force widespread failure essential to the explanation rather than an optional strengthening? — **yes**
5. 5. Does the conjecture imply that two models with equal predictive accuracy and the same causal graph can still differ in counterfactual reach because one relies on brittle local parameterizations? — **yes**
6. 6. Does the proposed notion illuminate why a highly compressed representation of observed regularities may still be useless for policy guidance in altered regimes not present in the training data? — **yes**
7. 7. Does defining reach as a robustness-to-structural-intervention profile allow the explanation to rank models by modal depth across increasingly alien interventions rather than only classify them as causal or noncausal? — **yes**
8. 8. If a counterexample is a model that succeeds under many interventions only after retuning parameters for each new regime, would saving the conjecture require abandoning the claim that the same core explanatory structure must bear the load? — **yes**
9. 9. If a simple invariant feature remains fixed across transformations yet stops guiding action once the mechanism is structurally perturbed, would rescuing the explanation force it to treat invariance alone as sufficient and thus gut its central distinction? — **yes**
10. 10. If a predictor trained on one distribution generalizes to nearby interventions by chance correlations rather than structural coupling, would defending the conjecture require replacing its load-bearing criterion with a looser success-only standard? — **yes**

## Candidate Problems

- How can 'preserving core explanatory structure' be made non-circular and operational? The conjecture relies on excluding ad hoc reparameterization, but without a principled criterion for what counts as the same explanation versus a patched variant, counterfactual reach risks collapsing into an intuitive label rather than a measurable property. A key open problem is to define identity conditions for explanations/models under intervention that are load-bearing enough to distinguish genuine structural reuse from disguised local repair. (score: 0.97)
- What intervention space and distance notion should define the 'radius of valid redeployment' when the conjecture explicitly allows changes in generative structure, not just data distribution? The central unresolved tension is that reach depends on how interventions are typed, bounded, and metrized; different choices could radically change which models count as high-reach. A worthwhile problem is to construct a principled intervention geometry that captures structural perturbations without making the notion either trivial (too narrow) or equivalent to universal knowledge (too broad). (score: 0.95)
- How can counterfactual reach be empirically separated from neighboring properties like causal correctness, invariance, OOD robustness, and mechanistic faithfulness? The conjecture claims reach is distinct, but it remains open whether this is a genuinely new property or a composite of existing ones under another description. The problem is to produce discriminating examples, formal separations, or impossibility results showing when two models match on prediction/causal formalism/invariance yet differ in reach for principled reasons. (score: 0.92)
