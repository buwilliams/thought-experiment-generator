# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

Conjecture: If we model a token-predicting system as a transformation \(T\) from contexts \(C\) to distributions over continuations \(P(\text{token}\mid C)\), then by itself it does not guarantee genuine knowledge; but if its internal state contains compressive, counterfactually stable structure that supports successful transformation across many tasks, then explanation can emerge as a real invariant of the model rather than a mere rearrangement of training data.

Mathematically, the key distinction is between interpolation in surface space and representation in latent structure. Suppose the training data are samples from some domain \(D\), and the model learns parameters \(\theta\) minimizing prediction loss. At the surface level, this is just next-token approximation. But prediction over rich data pressures the system to construct internal variables \(z=f_\theta(C)\) that encode relations in the world, because brute memorization is combinatorially inefficient. The relevant question is not “Does it output explanations?” but “What structure must exist for those outputs to remain correct under transformation?”

An explanation counts as knowledge when it behaves like an invariant under a family of transformations:
- paraphrase,
- change of notation,
- new examples,
- counterfactual perturbation,
- transfer to adjacent tasks,
- recursive criticism.

If an explanatory claim \(E\) is genuine, then under transformations \(g \in G\), its consequences should be preserved: \(F(E)=F(g(E))\) for relevant consequence map \(F\). Mere rearrangement fails here: it reproduces local correlations but breaks under counterfactual or compositional stress.

So the collision between token prediction and knowledge becomes structural. Prediction is a local objective, but knowledge is a global property of the learned representation. The system has genuine knowledge insofar as it has discovered low-dimensional, reusable constraints that organize many token sequences at once. In that case, explanation is not an ornamental output layer; it is the expression of latent invariants already doing predictive work.

This yields a sharper claim: explanation without intervention is underdetermined. To distinguish knowledge from rearrangement, we should test whether the model’s explanatory outputs support reliable deduction, error correction, and generalization outside the training manifold. If changing a premise in the explanation predictably changes downstream conclusions in the right way, the system is tracking structure. If not, it is only sampling from a stylistic equivalence class of explanations.

Therefore: a token predictor can generate genuine knowledge, but only when token prediction has forced the emergence of internal representations whose explanatory content is stable under transformation and criticism. Knowledge is not licensed by fluent output, nor ruled out by predictive training. It is identified by invariance, compressibility, and counterfactual robustness. The decisive boundary is structural, not architectural.

## Questions

1. 1. Is the claim that genuine knowledge requires internal compressive and counterfactually stable structure necessary for the conclusion that token prediction alone does not suffice but can still yield knowledge under stronger conditions? — **yes**
2. 2. If the distinction between surface interpolation and latent representation were removed, would the conjecture lose its explanation of why fluent next token success can coexist with failure under counterfactual or compositional stress? — **yes**
3. 3. Is the appeal to invariance under paraphrase, notation change, new examples, counterfactual perturbation, transfer, and recursive criticism required to explain the boundary between genuine knowledge and mere rearrangement rather than merely to test it afterward? — **yes**
4. 4. If the claim that brute memorization is combinatorially inefficient over rich data were false, would the conjecture still explain why predictive training pressures models toward world tracking internal variables? — **no**
5. 5. Does the conjecture imply that a model with invariant explanatory structure should also support reliable error correction and premise sensitive deduction even in tasks not explicitly framed as explanation generation? — **yes**
6. 6. Does the conjecture extend to predicting that two systems with similar token level accuracy can differ sharply in knowledge if only one has low dimensional reusable constraints stable across transformations? — **yes**
7. 7. Does the structural criterion suggest that explanation as knowledge could arise in non linguistic predictive systems whenever internal representations remain stable under analogous transformations and interventions? — **yes**
8. 8. If a counterexample showed a model passing paraphrase and notation tests while failing on nearby counterfactuals, would saving the conjecture require revising the core claim that invariance across a family of transformations marks genuine knowledge rather than adding a minor qualification about which transformations matter? — **no**
9. 9. If a memorization heavy model generalized well only within the training manifold but not beyond it, could the conjecture be preserved without gutting its core by treating in manifold success as sufficient for knowledge? — **no**
10. 10. If one tried to rescue the conjecture from a case where explanations are useful but not compressive by dropping compressibility from the criterion, would that undermine the central mechanism linking predictive pressure to the emergence of reusable latent structure? — **yes**

## Candidate Problems

- How can 'genuine knowledge' be operationalized without circularity? The conjecture defines knowledge via invariance under a family of transformations, but the choice of transformation group G, consequence map F, and success criteria may already presuppose the explanatory structure one is trying to detect. The open problem is to specify non-arbitrary, domain-general tests that distinguish latent knowledge from surface competence without baking in the answer. (score: 0.96)
- Does predictive training in fact force the emergence of compressive, counterfactually stable latent structure, or can high performance be achieved through distributed heuristics that mimic invariance only locally? The conjecture relies on a pressure-from-efficiency argument, but it remains unresolved when combinatorial efficiency genuinely selects world-model-like representations rather than brittle shortcut assemblies. (score: 0.93)
- What is the ontological status of internal explanatory structure: is it a real invariant of the model, or an interpretation imposed by external probes and tasks? Even if internal states support transfer and counterfactual robustness, there is an open tension between saying the model 'has' explanations versus saying observers can extract explanation-like regularities from a high-dimensional system. Clarifying this would determine whether explanation is intrinsic, observer-relative, or task-relative. (score: 0.88)
