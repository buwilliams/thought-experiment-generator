# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

Model the system as a map  
\[
f_\theta: C \to \Delta(T),
\]
from contexts \(C\) to probability distributions over token sequences \(T\). Training adjusts \(\theta\) to minimize prediction loss on samples from some data-generating distribution \(D\). The question is whether such a map can produce explanations that are knowledge, rather than mere recombinations.

A useful distinction is between **surface compression** and **structural representation**. If the model only memorized strings, then its outputs would be permutations of observed fragments. But minimizing prediction error over a rich distribution typically rewards learning latent variables and relations that generate many token patterns. Formally, there may exist an internal state \(z\) such that
\[
x \mapsto z \mapsto \text{prediction}(x),
\]
where \(z\) encodes reusable constraints, analogies, and invariants across cases. The key mathematical issue is not whether outputs are “seen before,” but whether the system has captured a structure that supports **counterfactual generalization**.

So define explanation operationally: an explanation is knowledge-bearing if it compresses many cases under a smaller set of generative relations and continues to work under transformations of the problem. Invariantly, if an explanation survives reparameterization of wording, novel combinations of premises, and error-correcting criticism, then it is not merely token rearrangement. It has tracked structure.

Conjecture:

**A token-prediction system can generate genuine knowledge to the extent that its learned internal representations encode invariant relations that support successful counterfactual and compressive generalization; absent such invariants, it merely rearranges what it has seen.**

What follows is that “predicting tokens” and “knowing” are not disjoint categories. Prediction is the training objective; knowledge is a property of the learned structure. The non-negotiable constraint is this: no amount of fluent recombination counts as knowledge unless there exists some stable mapping from many superficially different inputs to the same underlying explanatory relation. In mathematical terms, knowledge appears when the model has approximated an equivalence class over expressions that preserves truth-relevant structure.

This also clarifies the limit. Since the model is optimized for next-token loss, not truth directly, its internal invariants can be misaligned with reality. A representation may be highly compressive yet false. Therefore explanation from such a system is at best **conjectural knowledge**: candidate structure that must be tested by criticism, intervention, and transfer to unseen domains.

So the collision yields a sharper claim: the relevant question is not “prediction or knowledge?” but “what invariants has prediction forced the system to learn, and do those invariants survive transformation?” Where they do, explanation can be genuine knowledge. Where they do not, it is only rearrangement.

## Questions

1. 1. Does the conjecture require that next-token loss on a sufficiently rich data distribution will generally pressure the model to learn latent variables and relations, rather than allowing equally good performance via shallow n-gram-style memorization or heuristic pattern matching? — **yes**
2. 2. If the phrase 'genuine knowledge' were changed from 'internal representations encode invariant relations that support successful counterfactual and compressive generalization' to merely 'outputs are novel and useful,' would that break the conjecture's core explanation of when token prediction yields knowledge? — **yes**
3. 3. Is the distinction between 'surface compression' and 'structural representation' load-bearing here—i.e., if one removed that distinction, would the conjecture lose its criterion for separating rearrangement from explanation? — **yes**
4. 4. Does the conjecture depend on there existing an internal state z that is functionally reusable across many cases, rather than the model producing correct outputs through distributed correlations with no stable shared explanatory relation? — **yes**
5. 5. If a model succeeded on rewordings and novel premise combinations but failed under explicit counterfactual interventions, would the conjecture classify it as lacking knowledge, making counterfactual robustness a necessary component rather than an optional add-on? — **yes**
6. 6. Is the claim that knowledge corresponds to an approximated equivalence class over expressions preserving truth-relevant structure essential—so that replacing 'equivalence class' with simple semantic similarity or topical clustering would undermine the explanation? — **yes**
7. 7. Does the conjecture specifically require invariance under transformations like paraphrase, recombination of premises, and criticism-driven error correction, rather than any generic form of generalization, to count the output as knowledge-bearing? — **yes**
8. 8. If one altered the conjecture so that fluent recombination alone counted as knowledge even without a stable many-to-one mapping from superficially different inputs to the same explanatory relation, would that destroy the conjecture's proposed boundary between knowledge and rearrangement? — **yes**
9. 9. Is the asymmetry between training objective and epistemic status load-bearing—namely, that prediction is how the system is trained but knowledge is a property of the learned structure, so changing the objective from token prediction to truth-tracking would materially change the conjecture? — **yes**
10. 10. Does the final limitation—that the model's learned invariants may be compressive yet false, so its outputs are only conjectural knowledge until tested by criticism, intervention, and transfer—play an essential role in the conjecture, rather than being a dispensable cautionary note? — **yes**

## Candidate Problems

- How can 'knowledge-bearing invariant relations' be defined and measured non-circularly in token-prediction systems? The conjecture relies on internal representations encoding truth-relevant structure that survives rewording, recombination, criticism, and counterfactual transformation, but it leaves open a rigorous criterion distinguishing genuine structural representation from merely useful statistical compression. A key problem is to specify tests or formalisms that identify invariants without presupposing the very explanatory knowledge they are meant to detect. (score: 0.97)
- Under what conditions does minimizing next-token prediction loss force the learning of causally or explanatorily relevant latent structure rather than spurious but compressive proxies? The conjecture suggests rich distributions reward reusable latent variables, yet it remains unresolved when predictive success is sufficient for counterfactual generalization and when it is not. This is the central tension between optimization for prediction and the emergence of knowledge, especially in environments where many false models compress well. (score: 0.95)
- What is the relationship between internal representational invariance and truth, given that a model may learn stable, compressive structure that is nevertheless false or misaligned with reality? The conjecture admits that explanation here is only conjectural knowledge unless validated by criticism and transfer, but it does not specify how criticism, intervention, or external error-correction should be integrated into the system or evaluation process. This opens the problem of whether token-prediction alone can ever suffice for knowledge, or whether knowledge necessarily depends on a larger socio-epistemic system of testing. (score: 0.93)
