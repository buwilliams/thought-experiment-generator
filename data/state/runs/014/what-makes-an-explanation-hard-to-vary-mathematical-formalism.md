# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

Conjecture: The difference can be formalized as the difference between a minimally constrained model and an underdetermined one.

Let an explanation be represented as a structured map  
\[
E=(P,R,C) \to X
\]
where \(P\) is a set of parts, \(R\) the relations among them, \(C\) the constraints they jointly satisfy, and \(X\) the phenomenon explained. A part is load-bearing iff removing or altering it changes the set of derivable consequences relevant to \(X\).

More precisely, for each part \(p \in P\), define its necessity by whether
\[
\mathrm{Cons}(E) \neq \mathrm{Cons}(E \setminus \{p\})
\]
with respect to the explanatory target. If equality holds, \(p\) is structurally redundant. If not, \(p\) carries load.

This immediately suggests a sharper distinction:

1. **All-load-bearing explanation**  
   Every part participates in a dependency network such that no nontrivial substitution preserves the explanatory mapping. Formally, the automorphism group of the explanation relative to the target is small or trivial: there are few transformations
   \[
   \phi: P \to P
   \]
   that preserve \(R\), \(C\), and the explanation of \(X\). The structure is rigid. Its identity is carried by specific constraint-satisfying relations, not by decorative detail.

2. **Freely swappable explanation**  
   Many substitutions leave the explanatory output unchanged. The automorphism group is large; there are many symmetry transformations that preserve what the account predicts or narrates. In that case, the details are not individuating the explanation — they are gauge-like degrees of freedom. They may aid presentation, but they are not doing explanatory work.

So the structural property at issue is not “detail” versus “abstraction” but **constraint density**. In a load-bearing explanation, the ratio of indispensable constraints to available descriptive freedom is high. In a swappable explanation, there is slack: many local changes are absorbed without affecting the global account.

An invariant helps here: explanatory compression. If a detail can be varied without loss, then it was not part of the minimal sufficient structure. The true explanatory core is the equivalence class of all presentations related by admissible substitutions. What distinguishes a robust explanation is whether its apparent parts correspond to invariants of that class or merely to coordinates chosen for convenience.

Thus: an explanation whose parts are all load-bearing is one near minimal description length under the relevant constraints — each component eliminates a live alternative. An explanation with freely swappable details contains surplus parametrization. The test is counterfactual: hold the target fixed, perturb the parts, and ask which transformations preserve explanatory consequences. What survives those transformations is the real structure; what does not is genuinely load-bearing.

## Questions

1. 1. Is the claim that explanatory quality turns on constraint density rather than on detail versus abstraction necessary for the conjecture to distinguish all-load-bearing explanations from freely swappable ones? — **yes**
2. 2. If the definition of a load-bearing part by a change in target-relevant derivable consequences were removed, would the conjecture lose its basis for saying which parts are genuinely indispensable rather than merely present? — **yes**
3. 3. Is the appeal to the size of the automorphism group relative to the target required to explain why some substitutions are harmless while others destroy the explanatory structure? — **yes**
4. 4. If the conjecture dropped the idea that the explanatory core is an equivalence class under admissible substitutions, would it still explain why different presentations can count as the same explanation? — **no**
5. 5. Does the conjecture imply that two explanations with very different surface descriptions can be structurally the same whenever they preserve the same relations, constraints, and target-relevant consequences? — **yes**
6. 6. Does the focus on invariants under perturbation extend the account to compare scientific, mathematical, and narrative explanations rather than only the specific problem statement about swappable details? — **yes**
7. 7. Does the link to near minimal description length predict that explanations with less surplus parametrization should also be more compressible without loss of explanatory power? — **yes**
8. 8. If a counterexample showed a large automorphism group in an explanation whose every part still seems indispensable, would saving the conjecture require abandoning the identification of swappability with explanatory slack rather than adding a small qualification? — **no**
9. 9. If there are cases where removing a part changes some derivable consequences but not the ones relevant to the target, would rescuing the conjecture force a major revision of its necessity test rather than a local restriction to target relevance? — **no**
10. 10. If an explanation is highly constrained yet admits many harmless redescriptions because of notation or labeling choices, would defending the conjecture require gutting the claim that a large automorphism group signals underdetermination? — **no**

## Candidate Problems

- How should the explanatory target and relevance relation be formalized so that necessity is not trivial or observer-relative? The conjecture defines load-bearing parts by changes in Cons(E) 'with respect to the explanatory target,' but leaves open which consequences count, at what level of abstraction, and under which background assumptions. Without a principled relevance criterion, the same part can appear indispensable or redundant depending on what is included in X, the comparison class, or the inferential language. (score: 0.96)
- Is 'large automorphism group = underdetermined explanation' actually valid across explanatory domains, or does it conflate harmless representational symmetry with genuine explanatory slack? A key open question is whether symmetries of presentation, model reparameterizations, and multiple realizability should all be treated as the same phenomenon. The conjecture needs a criterion separating gauge freedom from cases where different substitutions correspond to genuinely different causal or mechanistic explanations despite preserving the same target output. (score: 0.93)
- Can constraint density or near-minimal description length be made into a non-circular, computable measure of explanatory quality? The conjecture suggests that robust explanations are close to minimal sufficient structure, but MDL-style minimality depends on encoding choices, model class, and what counts as a permissible compression. The unresolved problem is whether there exists an invariant metric that tracks explanatory load-bearingness rather than mere syntactic compactness, and whether such a metric predicts scientific explanatory success better than simpler notions like predictive adequacy or interventionist stability. (score: 0.91)
