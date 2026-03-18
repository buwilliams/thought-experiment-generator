# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

Conjecture: A token-predicting system can generate genuine knowledge iff its internal representations support compressive, counterfactually stable mappings that explain more than the training distribution literally contains; otherwise it is only performing high-dimensional rearrangement.

Translate the problem into structure.

Let:
- \(D\) = training corpus, a finite sample of token sequences.
- \(M\) = learned model, a function from contexts to probability distributions over next tokens.
- \(E\) = an explanation produced by \(M\).
- \(W\) = the underlying generative structure of the world or domain the corpus is about.
- \(C\) = a space of interventions or counterfactual variations.

The naive view says \(M\) is only a map \(D \to E\): recombination of observed patterns. But mathematically, learning is compression. If \(M\) were merely storing and permuting \(D\), its explanatory outputs would be bounded by local similarity to training examples. Its effective structure would be near lookup plus interpolation.

The stronger possibility is that \(M\) learns latent variables \(Z\) such that:
1. \(Z\) compresses \(D\),
2. \(Z\) generates successful predictions across many contexts,
3. relations in \(Z\) remain invariant under transformations in \(C\).

That third condition matters. Genuine explanation is not just sequence plausibility; it is preservation of structure under variation. An explanation counts as knowledge when it tracks invariants: what would still hold if assumptions, contexts, or surface wording changed.

So the key distinction is between:
- rearrangement: outputs lie within the convex hull of observed token regularities;
- knowledge: outputs instantiate a model whose internal constraints allow successful extrapolation to unseen but lawful cases.

This can be stated more sharply. Suppose an explanation \(E\) implies a family of predictions \(P(E, C)\) over interventions \(C\). If \(P(E, C)\) continues to succeed where mere n-gram or stylistic continuation would fail, then \(E\) contains more than rearrangement. It has captured structure in \(W\), not just frequency in \(D\).

The non-negotiable constraint is that \(D\) underdetermines \(W\). Therefore no explanation is justified merely by matching the corpus. But that does not imply “only rearrangement.” Mathematics, too, proceeds from finite symbol exposure to general truths by discovering compact structures with wide entailment. The relevant measure is not origin but generative power.

Hence the conjecture: token prediction is a sufficient training objective for knowledge generation only when optimization pressure induces representations isomorphic enough to real explanatory structure that they support compression, transfer, and counterfactual robustness. Where those properties are absent, the system is only remixing. Where they are present, “predicting tokens” is merely the surface game through which a deeper model of relations was learned.

So the illuminating collision is this: prediction and explanation are not categorically different operations. Explanation is prediction over a larger transformation group — not just “what token comes next?” but “what remains true across rephrasings, interventions, and novel cases?” A system has knowledge to the extent that its outputs are governed by those invariants.

## Questions

1. Does the conjecture require that the model’s internal representations encode latent variables Z that compress D, rather than allowing genuine knowledge to arise solely from behavior at the output level? — **yes**
2. Is counterfactual stability under a specified intervention space C necessary in this conjecture, such that if an explanation succeeds on novel cases but its underlying relations do not remain invariant across interventions, it would not count as genuine knowledge? — **yes**
3. Does the conjecture stand or fall on the claim that matching the training corpus D is insufficient because D underdetermines the world-structure W, rather than merely on the practical observation that memorization often fails out of distribution? — **yes**
4. Is the phrase 'explain more than the training distribution literally contains' load-bearing in the sense that the model must support entailments not recoverable by local similarity, interpolation, or convex combinations of observed token regularities? — **yes**
5. Does the conjecture depend on a real distinction between learned structure that tracks W and high-dimensional rearrangement bounded by the statistics of D, rather than treating these as different descriptions of the same phenomenon? — **yes**
6. Is compression doing essential explanatory work here, such that if a system were counterfactually robust and broadly predictive but not meaningfully compressive, the conjecture would classify it as lacking genuine knowledge? — **yes**
7. Does the conjecture require that explanation be understood as prediction over a larger transformation group—rephrasings, interventions, and novel lawful cases—rather than ordinary next-token prediction alone? — **yes**
8. Is the 'iff' in the conjecture intended literally, so that compressive and counterfactually stable internal mappings are not just sufficient but also necessary for a token-predicting system to generate genuine knowledge? — **yes**
9. Would the conjecture be undermined if there existed a token-predicting system whose internal representations were not even approximately isomorphic to explanatory structure in W, yet it still produced explanations that remained successful across interventions in C? — **yes**
10. Does the conjecture specifically rule out counting stylistically plausible but intervention-fragile explanations E as knowledge, even when they are highly convincing and closely match patterns prevalent in D? — **yes**

## Candidate Problems

- How can 'genuine knowledge' be operationalized without circularity? The conjecture relies on compressive, counterfactually stable internal mappings that 'explain more than the training distribution literally contains,' but it leaves unresolved what concrete tests distinguish this from sophisticated interpolation. The open problem is to define measurable criteria for compression, invariance, and extrapolative success that do not already presuppose access to the true world-structure W or smuggle in human judgment of explanation. (score: 0.96)
- What is the relationship between internal representation structure and actual explanatory success? The conjecture suggests that knowledge requires latent variables Z that are sufficiently isomorphic to real explanatory structure, but it is unresolved whether such internal isomorphism is necessary, or whether systems can produce robust counterfactual and transfer performance via heterogeneous or purely procedural representations. This raises the deeper question of whether knowledge should be attributed by internal ontology, external performance, or some combination. (score: 0.92)
- Under what training and environmental conditions does next-token prediction induce counterfactually robust world-models rather than corpus-bound rearrangement? The conjecture treats token prediction as potentially sufficient, but does not specify the structural thresholds: data diversity, intervention-like variation in D, model architecture, optimization regime, embodiment/tool use, or feedback loops needed for invariants to emerge. This is an open systems question about the phase transition from memorization/compression of surface regularities to learning reusable explanatory structure. (score: 0.94)
