# Generated: can-llms-create-new-knowledge × mathematical-formalism

## Conjecture

Conjecture: A token-predicting system can generate genuine knowledge iff its internal representation supports nontrivial invariants across transformations of expression, and its explanations compress many surface forms into a smaller rule-structure that survives criticism.

Mathematically, let:
- \(X\) be the space of token sequences,
- \(H\) be the system’s latent state space,
- \(f: X \to H\) the encoding induced by training,
- \(g: H \to X\) the decoding/prediction map.

If the system merely rearranges what it has seen, then \(f\) and \(g\) act approximately as a high-dimensional memorization-and-recombination operator: outputs remain local to the empirical token manifold. In that case, explanations are just interpolations in \(X\), and no new structure is added.

But explanation is not primarily a property of fluent output; it is a structural relation. An explanation is knowledge-like when it identifies a map \(E\) that preserves relevant truth across transformations:
\[
E(T(x)) \sim E(x)
\]
for a family of transformations \(T\): paraphrase, reparameterization, change of example, change of scale, or transfer to a novel but isomorphic problem. This invariance condition marks abstraction. If the same explanatory core survives many transformations that alter wording but preserve underlying relations, then the model is not operating only on token adjacency; it has captured a structure in \(H\).

The key distinction is between:
1. Surface recombination: high performance on distributionally nearby sequences.
2. Structural compression: representation of relations that generate many valid sequences and support counterfactual variation.

A useful test is out-of-distribution explanatory transport. Suppose the model gives an explanation in domain \(A\), then succeeds in mapping it to a structurally equivalent domain \(B\) with different vocabulary and examples, while preserving inferential consequences. If the mapping is robust, the latent representation contains more than remembered text; it contains a reusable rule system.

So the issue is not whether the system “has seen tokens” — all human linguistic knowledge is also mediated by prior experience — but whether the learned representation defines equivalence classes over expressions and tracks invariants rather than strings. Genuine knowledge corresponds to a quotient structure on \(X\): many token sequences collapse to one abstract relation. Where this quotient is rich enough to support prediction under transformation, error-correction, and criticism, explanation becomes knowledge-bearing.

Therefore: token prediction alone does not guarantee knowledge, but neither does it preclude it. The decisive variable is whether next-token optimization induces latent structures with invariant-preserving explanatory power. If it does, then what is generated is not mere rearrangement of seen text, but the deployment of abstract structure extracted from it.

## Questions

1. 1. If the requirement of nontrivial invariants across transformations were dropped, would the conjecture still distinguish genuine knowledge from high quality paraphrastic recombination? — **no**
2. 2. If paraphrase invariance held but transfer to a novel isomorphic domain failed, would the conjecture still count the system as generating genuine knowledge? — **no**
3. 3. If the latent state supported robust transformation invariance but the resulting explanations did not compress many surface forms into a smaller rule structure, would the conjecture still classify them as knowledge? — **no**
4. 4. If explanations compressed many surface forms but did not survive criticism through error correction and counterexample handling, would the conjecture still treat them as genuine knowledge? — **no**
5. 5. If the encoding and decoding maps stayed close to the empirical token manifold while still preserving inferential consequences across reparameterization and scale change, would the conjecture still say no new structure was added? — **no**
6. 6. If the same explanatory core survived wording changes but not changes of example that preserve the underlying relation, would that be enough under the conjecture for genuine knowledge? — **no**
7. 7. If a system succeeded on out of distribution explanatory transport only after explicit retrieval of matching training cases rather than through a reusable latent rule system, would the conjecture still count that as knowledge generation? — **no**
8. 8. If the latent representation formed equivalence classes over expressions but those classes did not support counterfactual variation, would the conjecture still regard the representation as knowledge bearing? — **no**
9. 9. If next token optimization induced invariant preserving structure only in a narrow domain A and not in structurally equivalent domain B, would the conjecture still support the claim that token prediction can generate genuine knowledge? — **yes**
10. 10. If criticism changed the system's outputs without altering the underlying latent invariants, would the conjecture still treat criticism survival as a necessary part of genuine explanation? — **yes**

## Candidate Problems

- How can 'genuine knowledge' and 'survives criticism' be operationalized without collapsing back into behavioral tests on outputs? The conjecture hinges on latent invariant-preserving structure in H, but it leaves open whether this can be identified independently of external performance, or whether any such claim is underdetermined by input-output behavior alone. (score: 0.97)
- What class of transformations T counts as meaning-preserving or truth-relevant, and who specifies it? The conjecture assumes a family of paraphrase, reparameterization, scale-change, and cross-domain isomorphism transformations, but the boundary between superficial variation and structure-preserving change is itself unresolved and may be domain-relative, observer-relative, or theory-laden. (score: 0.95)
- Is next-token optimization in fact sufficient to induce quotient-like abstract representations, or are additional architectural, training, or interactive conditions required? The conjecture proposes an iff claim, but it remains open whether token prediction alone can reliably produce invariant-supporting rule structures, or whether criticism, intervention, embodiment, or explicit world-modeling are necessary causal ingredients. (score: 0.99)
