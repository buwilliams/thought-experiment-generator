# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

Conjecture: The difference can be modeled as the difference between a rigid structure and one with many symmetries.

Let an explanation be represented as a structured object \(E = (P, R, C)\), where \(P\) is a set of parts, \(R\) the relations among them, and \(C\) the explanatory constraints: what must be derived, predicted, or made intelligible. A part is load-bearing when removing or altering it changes whether \(C\) can still be satisfied. Formally, part \(p \in P\) is load-bearing if the transformed explanation \(E \setminus p\), or \(E\) with \(p\) replaced by a non-equivalent part, fails to preserve the relevant explanatory map to \(C\).

This suggests a structural criterion: explanations with all parts load-bearing are minimally sufficient and low in automorphism. They admit few nontrivial transformations that preserve explanatory content. Their components are tightly constrained by the role they play in the derivation. In graph-theoretic language, they are closer to rigid graphs: perturbations destroy key paths or dependencies. In algebraic language, they have a small symmetry group.

By contrast, explanations whose details can be swapped freely have high redundancy or high symmetry. Distinct surface parts instantiate the same abstract role. If a permutation or substitution of components leaves the explanatory function invariant, then those details are not individually essential; they belong to an equivalence class. What matters is the invariant pattern, not the specific token. Such explanations are structurally modular: many local changes preserve global performance.

So the key distinction is not “specificity” versus “generality,” but constraint density. A fully load-bearing explanation is one in which the explanatory task determines its components up to near-uniqueness. A freely swappable explanation is one in which many realizations satisfy the same higher-level relations. The former has little slack; the latter has degrees of freedom.

This also yields a test. For any candidate explanation, consider its space of admissible transformations \(T\): substitutions, reorderings, paraphrases, parameter changes. Ask which transformations preserve the explanatory success conditions. If only trivial transformations do, the explanation is structurally rigid and its parts are load-bearing. If many nontrivial transformations preserve success, then much of the explanation consists of replaceable implementation detail.

What this illuminates is that “load-bearing” is not a rhetorical property but a counterfactual one. A detail is essential exactly when variation along that dimension is not allowed by the structure. Thus the border between essence and ornament is mathematically the border between invariants and symmetries: essence lies in what cannot vary without collapse; ornament lies in what can vary while preserving the explanatory map.

## Questions

1. 1. Does the conjecture require that explanations with all parts load-bearing have a small automorphism group, rather than merely a small number of interchangeable surface descriptions? — **yes**
2. 2. If two explanations satisfy the same constraints C but differ only by a relabeling of parts in P that preserves relations R, does the conjecture classify them as non-rigid in exactly the same way as a genuine component substitution? — **no**
3. 3. Would the conjecture fail if there exist explanations whose parts are all load-bearing even though the underlying graph has many symmetries unrelated to explanatory roles? — **yes**
4. 4. Does the proposed test over admissible transformations T depend essentially on including substitutions, reorderings, paraphrases, and parameter changes together, rather than some narrower class of transformations? — **no**
5. 5. If a part can be replaced only by a token from a very small equivalence class while preserving C, does the conjecture count that as evidence against full load-bearingness? — **yes**
6. 6. Is the identification of 'essence' with invariants committed to the claim that every non-load-bearing detail corresponds to a nontrivial symmetry of E=(P,R,C)? — **yes**
7. 7. Would the rigid-versus-symmetric distinction break down for explanations that are minimally sufficient but modular, where each module is load-bearing internally yet swappable at a higher level? — **no**
8. 8. Does the conjecture rely on near-uniqueness of components being determined by the explanatory task C, rather than by external conventions of representation or notation? — **yes**
9. 9. If altering a part changes only the derivation path but not what is derived, predicted, or made intelligible in C, does the conjecture treat that part as non-load-bearing? — **yes**
10. 10. Is the graph/algebra analogy load-bearing in the conjecture, such that removing the link to rigidity and symmetry would leave no substantive structural criterion for distinguishing the two kinds of explanation? — **yes**

## Candidate Problems

- How should explanatory equivalence and the symmetry group be defined non-circularly? The conjecture says a transformation is symmetry-like when it preserves explanatory success relative to constraints C, but what counts as 'the same explanation,' 'non-equivalent part,' or preservation of the explanatory map is itself unclear and theory-laden. A new problem is to construct a precise criterion of explanatory equivalence that does not merely restate the original notion of load-bearingness. (score: 0.97)
- Does low automorphism actually track explanatory depth or essence across levels of abstraction, or does it confuse representation-rigidity with world-rigidity? The same explanation may look highly symmetric in one encoding and rigid in another, and higher-level explanations often intentionally ignore micro-differences while remaining powerful. A key open question is which symmetries are objective features of the explanatory structure versus artifacts of description, granularity, or modeling choice. (score: 0.94)
- What is the relation between minimal sufficiency, robustness, and truth-conduciveness? The conjecture links 'all parts load-bearing' with minimal sufficiency and structural rigidity, but many good explanations are robust, redundant, and multiply realizable; redundancy can increase error-correction and generality rather than indicate ornament. A worthwhile problem is to determine when redundancy is explanatory slack versus when it is itself part of the explanatory virtue. (score: 0.92)
