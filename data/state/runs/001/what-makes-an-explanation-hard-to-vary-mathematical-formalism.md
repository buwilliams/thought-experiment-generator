# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

Conjecture: The difference is one of constraint density. An explanation is “all load-bearing” when its components participate in a tightly coupled structure of dependencies such that replacing any part changes the set of consequences the explanation can derive. An explanation has freely swappable details when many of its components lie in equivalence classes: substitutions preserve the explanation’s inferential outputs.

Mathematically, model an explanation as a structured map
\[
E = (P, R, O),
\]
where \(P\) is a set of parts, \(R\) a set of relations among them, and \(O\) the outcomes or claims the explanation must generate. A part \(p \in P\) is load-bearing relative to \(O\) if removing or altering \(p\) changes the image of the map from premises-plus-relations to outcomes. Formally, \(p\) is load-bearing when there is no transformation \(T\) replacing \(p\) with \(p'\) such that
\[
O(E) = O(T(E)).
\]
If such transformations exist broadly, then the explanation contains symmetry: multiple distinct micro-descriptions realize the same explanatory function.

So the key structural distinction is not “importance” in a vague sense, but the size of the automorphism group of the explanatory structure. Explanations with many non-damaging substitutions have high symmetry; explanations whose every part is load-bearing have low symmetry or are rigid. In a rigid explanation, identities of parts matter because they are fixed by relational constraints. In a symmetric explanation, only the relational pattern or higher-level invariant matters, not the particular details instantiating it.

This suggests a test: perturbation analysis. For each part, ask whether a local substitution preserves the global explanatory invariants. If yes, that part is not individually load-bearing; it belongs to an equivalence class of interchangeable realizers. If no, the part is structurally fixed. The explanation’s degree of load-bearingness can then be measured by the proportion of parts whose admissible perturbations collapse the account.

What this illuminates is that “load-bearing” is relative to explanatory grain. At a fine grain, many details may appear essential; at a coarser grain, they collapse into invariant structure. Thus some explanations are robust because they capture a universality class: many lower-level substitutions leave the same explanatory pattern intact. Others are brittle because they depend on a specific configuration, with little redundancy and few symmetries.

So the conjecture is: explanations differ structurally by their symmetry and redundancy profile. The more an explanation is governed by invariants under transformation, the more its details are swappable. The more its consequences depend on the exact identity and placement of each component, the more every part is load-bearing. In short, interchangeable detail signals explanatory compression around invariants; universal load-bearing signals structural rigidity.

## Questions

1. 1. Does the conjecture make a different prediction for two explanations with the same number of parts and relations but different automorphism-group sizes, namely that the one with the larger automorphism group will permit more non-damaging substitutions? — **yes**
2. 2. If a part p can be replaced by p' while preserving all outcomes O but changing some internal relations R that never affect O, does the conjecture classify p as non-load-bearing rather than load-bearing? — **yes**
3. 3. Does the conjecture require load-bearingness to be evaluated relative to a specified grain of description, so that the same micro-part can be load-bearing at one grain and interchangeable at a coarser grain? — **yes**
4. 4. If every individual part in P can be swapped for an equivalent alternative one at a time, but some pairs of swaps jointly break O, does the conjecture still count the explanation as having freely swappable details? — **yes**
5. 5. Does the conjecture imply that what matters is preservation of inferential outputs O(E), not preservation of the literal identities of parts, so that two structurally different micro-descriptions can be explanatorily equivalent? — **yes**
6. 6. If an explanation has no nontrivial automorphisms but still contains causal redundancy because duplicated substructures can be deleted without changing O, would the conjecture incorrectly classify it as fully rigid? — **yes**
7. 7. Does the perturbation test in the conjecture depend on considering only local substitutions of single parts, rather than global rewritings that preserve the same explanatory invariants? — **yes**
8. 8. If two candidate substitutions both preserve O but one changes which intermediate claims are derivable while the other does not, does the conjecture treat those as equally non-damaging? — **yes**
9. 9. Does the conjecture commit to a measurable quantity—such as the proportion of parts lacking admissible O-preserving substitutions—as the degree of load-bearingness, rather than a merely qualitative distinction? — **yes**
10. 10. Would the conjecture fail if there exist explanations whose details are freely swappable even though their explanatory structure has low symmetry, because swapability comes from context-sensitive compensation rather than equivalence classes or invariants? — **yes**

## Candidate Problems

- How can the conjecture define explanatory equivalence and the outcome map O non-circularly? The core tension is that whether a substitution preserves 'inferential outputs' depends on what counts as the same outcome, at what grain, and under which background assumptions. Without an independent criterion for individuation of outcomes and admissible transformations, symmetry may simply be imposed by redescription rather than discovered in the explanation. (score: 0.96)
- What is the right mathematical object for an explanation: automorphism group, causal model, proof structure, category, or something else? The conjecture treats explanations as structured maps with symmetries, but it is unresolved whether ordinary scientific and mathematical explanations actually admit a representation where load-bearingness is captured by automorphisms rather than by intervention sensitivity, counterfactual dependence, or proof-theoretic indispensability. (score: 0.89)
- How does load-bearingness vary across levels of abstraction without collapsing into arbitrariness? The conjecture says rigidity versus symmetry is relative to explanatory grain, but this raises the open problem of deriving principled cross-scale relations: when should fine-grained indispensability be ignored as mere implementation detail, and when does coarse-graining erase genuinely explanatory constraints? (score: 0.93)
