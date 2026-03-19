# Generated: is-there-a-fact-of-the-matter-about-whether-a-ship-repaired × mathematical-formalism

## Conjecture

Conjecture: the Ship of Theseus has no single truth value until a criterion of identity is specified, because the problem is underdetermined by its bare description. Mathematically, “same ship” is not a primitive fact but an equivalence relation over a state space of ships, and different admissible equivalence relations yield different answers.

Let a ship at time \(t\) be represented as a structured object
\[
S_t = (P_t, A_t, F_t, H_t),
\]
where \(P_t\) is the set of planks, \(A_t\) their arrangement, \(F_t\) the functional capacities, and \(H_t\) the causal/history relations connecting this ship to earlier states. Repair plank-by-plank defines a transformation sequence
\[
S_0 \to S_1 \to \cdots \to S_n
\]
in which local replacements preserve much of \(A_t\), \(F_t\), and \(H_t\) while gradually changing \(P_t\).

Now ask: what invariant must be preserved for identity? If identity means material continuity, then the relevant invariant is overlap in \(P_t\), and after enough replacements the final ship is not the original. If identity means organizational continuity, then preservation of \(A_t\) dominates, and it is the same ship. If identity means historical/causal continuity, then \(H_t\) decides in favor of sameness. If identity means function, then \(F_t\) may suffice. These are not verbal tricks; they are distinct mathematical choices of invariant under transformation.

So the real structure is this: the puzzle asks for a yes/no answer where the underlying model admits a family of non-equivalent identity maps
\[
\sim_{\text{matter}},\ \sim_{\text{form}},\ \sim_{\text{history}},\ \sim_{\text{function}}.
\]
Each relation is internally coherent; none is forced by the statement of the problem alone. Therefore there is a fact of the matter only relative to a chosen equivalence relation. Without that, the question “is it the same ship?” is like asking whether two geometric figures are “the same” without specifying congruence, similarity, or topological equivalence.

What this illuminates is that the philosophical work is not to inspect the ship harder, but to identify the transformation group and the invariant one cares about. The disagreement is about ontology encoded as mathematics: which features are allowed to vary, and which are non-negotiable.

A further consequence follows. If there are two outputs from the process — the gradually repaired ship and the ship reconstructed from the original planks — then no single invariant is likely to privilege both. Material continuity favors the reconstructed ship; causal continuity favors the repaired one. The paradox is not that identity is mysterious, but that we are trying to compress a multi-criteria structure into a single binary predicate.

## Questions


## Candidate Problems

- Specify criteria under which one equivalence relation over ships should be privileged in practical or metaphysical contexts rather than treating all as equally admissible. (score: 0.90)
- Analyze mixed identity criteria that weight matter, form, function, and causal history, and determine whether they yield stable verdicts or context-sensitive indeterminacy. (score: 0.86)
- Extend the framework to cases with branching continuities—such as reconstructed-from-original-parts versus continuously repaired objects—and test whether any non-arbitrary identity rule handles both. (score: 0.89)
- Determine whether there are domains where the bare description itself fixes the relevant invariant, contrary to the claim of underdetermination. (score: 0.82)
