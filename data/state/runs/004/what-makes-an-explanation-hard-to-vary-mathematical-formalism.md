# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

Conjecture: The distinction can be modeled by the explanation’s dependency structure. An explanation is “all load-bearing” when its claims form a tightly constrained system in which most components are necessary to preserve explanatory invariants; it is “freely swappable” when many components occupy positions in an equivalence class, so substitutions leave the explanatory map unchanged.

Formally, let an explanation be a structured tuple  
\(E = (P, R, C, T)\), where \(P\) is a set of parts, \(R\) relations among them, \(C\) the constraints the explanation must satisfy, and \(T\) the target phenomenon. The explanation succeeds only if a mapping \(f(E) \to T\) preserves certain invariants: causal order, quantitative fit, mechanism, scope, counterfactual behavior, etc.

The key property is sensitivity to transformation. Consider allowed substitutions \(g: P \to P'\). If after applying \(g\), the transformed explanation \(E'\) still satisfies \(C\) and preserves the same invariants with respect to \(T\), then the replaced part was not uniquely load-bearing. It belonged to a symmetry class of interchangeable realizations. By contrast, if even small substitutions break constraint satisfaction or alter the preserved invariants, then the part is load-bearing.

So the mathematical distinction is:

- Load-bearing explanation: low symmetry, high constraint density, high dependency.
- Swappable explanation: high symmetry, lower effective constraint density, modular dependency.

More precisely, define for each part \(p \in P\) a removal or perturbation test: measure how many explanatory invariants fail when \(p\) is altered. A part is load-bearing to the degree that perturbing it changes the rank of the explanatory structure — that is, reduces the system’s ability to derive the target phenomenon or its counterfactuals. If many parts can be exchanged without changing this rank, then the explanation contains redundancy or abstraction.

This yields a useful illumination: “all parts are load-bearing” is not a mark of strength by itself. It may indicate a brittle overfit structure, where explanatory success depends on a highly specific arrangement. A stronger explanation often compresses particulars into a smaller set of necessity-bearing relations, allowing variation in implementation while preserving invariant structure. In mathematics, this is the difference between essence and coordinate choice.

Thus the real dividing line is not between detailed and simple explanations, but between those whose success depends on particular tokens and those whose success depends on preserved structure. An explanation with swappable details is often revealing a deeper level of organization: what matters is not which part fills a role, but the relational pattern and constraints that must be maintained. The most powerful explanations maximize what can vary while minimizing what must remain fixed.

## Questions

1. 1. Does the conjecture still explain the distinction if the mapping from the structured tuple to the target phenomenon preserves only predictive fit while dropping causal order, mechanism, scope, and counterfactual behavior as explanatory invariants? — **no**
2. 2. If two explanations preserve the same target phenomenon but differ in which invariants they preserve, does the conjecture have a principled way to classify one as more load-bearing rather than merely different in aim? — **no**
3. 3. Would the conjecture survive if the allowed substitutions were expanded from replacing parts to also rewiring relations among parts while keeping the same target phenomenon fixed? — **yes**
4. 4. If a part can be replaced only together with compensating changes elsewhere, does the conjecture treat that part as non-load-bearing because the overall rank is preserved, and if so does that collapse the intended distinction? — **yes**
5. 5. Can the conjecture distinguish genuine explanatory symmetry classes from trivial redescriptions that leave the target phenomenon unchanged only because the constraints were defined too weakly? — **no**
6. 6. If the constraints are tightened or loosened slightly, does the classification of the same explanation as load-bearing or swappable change drastically, suggesting the account depends more on how constraints are chosen than on the explanation itself? — **yes**
7. 7. Does the conjecture still work when an explanation has hierarchical modules so that tokens are swappable within a module but the module-to-module relations are not, or does its low-symmetry versus high-symmetry framing become too coarse? — **yes**
8. 8. If perturbing one part breaks the explanation only because another part was held fixed, does the conjecture require testing coordinated perturbations to identify what is truly load-bearing? — **yes**
9. 9. Can the conjecture classify an explanation as stronger when many token details are swappable but one abstract relational pattern is indispensable, without mislabeling the whole explanation as low dependency? — **yes**
10. 10. If two explanations have equal rank with respect to deriving the target phenomenon and counterfactuals, but one uses many interchangeable parts and the other uses a unique arrangement, does the conjecture explain why the former is deeper rather than merely more redundant? — **yes**

## Candidate Problems

- How can the conjecture define explanatory invariants and allowed substitutions non-circularly and domain-generally? The proposal relies on invariants, constraints, and equivalence classes to distinguish load-bearing from swappable parts, but these notions may themselves presuppose prior explanatory judgment. A key open problem is whether there exists a principled procedure for identifying the relevant invariants and transformation group from the target phenomenon and explanatory task, rather than smuggling in what already counts as essence. (score: 0.97)
- What is the relationship between swappability and explanatory depth versus mere underdetermination or loss of content? The conjecture suggests that more swappable details often indicate a deeper explanation, but high symmetry can also arise from vagueness, coarse-graining, or failure to specify mechanisms. The unresolved tension is to distinguish genuine structural abstraction from explanatory weakness, and to characterize when reducing token-specific dependence increases understanding rather than just decreasing empirical commitment. (score: 0.95)
- Can load-bearingness be formalized as a robust quantitative property of dependency structure across interventions, levels, and decompositions? The conjecture invokes perturbation tests, rank changes, and constraint density, but these may vary with how the explanation is partitioned into parts, what intervention space is allowed, and which level of description is chosen. The open problem is to develop a decomposition-invariant or at least comparison-stable metric that does not collapse into arbitrary modeling choices. (score: 0.93)
