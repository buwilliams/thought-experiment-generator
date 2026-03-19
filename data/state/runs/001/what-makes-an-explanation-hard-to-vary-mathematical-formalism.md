# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

Conjecture: the distinction can be formalized as one of constraint density.

Model an explanation as a structured object \(E = (V,R,P)\): variables/objects \(V\), relations \(R\), and principles \(P\). Let \(T(E)\) be the set of allowable transformations on \(E\): substitutions of terms, changes of examples, rephrasings, altered assumptions, and local rewiring of relations. An explanation is “hard to vary” when the subgroup of meaning-preserving transformations \(T^*(E) \subseteq T(E)\) is small. It is “easy to vary” when many transformations leave it apparently intact.

So the key structural property is not whether the explanation uses novel words or spans a huge combinatorial search space, but whether its components are mutually overconstrained by invariants.

The relevant invariants are things like:
- preservation of explanatory reach,
- preservation of internal consistency,
- preservation of compatibility with known constraints,
- preservation of derivational consequences.

A load-bearing explanation is one in which most parts participate in multiple independent constraints. Formally: for each component \(c \in E\), removing or altering \(c\) changes many downstream entailments or breaks one or more invariants. Its components have high dependency degree and low replaceability. By contrast, an easy-to-vary explanation contains components with low coupling: details can be swapped because they are not fixing much structure.

This reframes the LLM question. The search-space argument is true but incomplete: raw combinatorics vastly overcounts because most word sequences do not instantiate coherent structure. The real search is over equivalence classes of structured explanations under admissible transformations. Architecture matters insofar as it can search not token space but constraint space.

From this perspective, an LLM-based system could contribute to knowledge creation if it can generate candidates with increasing constraint density and then criticize them by testing invariants under variation. The operative procedure is:
1. generate a structured candidate explanation,
2. apply systematic perturbations,
3. measure which perturbations destroy explanatory power,
4. retain structures whose parts are jointly necessary.

That gives a candidate metric: explanatory rigidity \( \rho(E) \), roughly the fraction of local changes that violate core invariants. High \( \rho \) indicates load-bearing structure; low \( \rho \) indicates decorative or freely swappable content.

So my conjecture is:

An explanation’s “hard-to-vary” character is mathematically distinguishable by the size of its invariant-preserving transformation set: genuine explanations are those with small symmetry under variation because their parts are tightly coupled by many constraints. Therefore, if an LLM system is to create knowledge, it will not do so by sampling ever more text, but by constructing and selecting high-rigidity explanatory structures whose components are demonstrably load-bearing under transformation tests. 

On this view, novelty appears when the system finds a structure with low permissible variation but high explanatory reach. That is the threshold at which output stops being recombination in a loose space and becomes knowledge-like in a constrained one.

## Questions


## Candidate Problems

- Define explanatory rigidity rho(E) in a way that separates genuine load-bearing constraint from brittle overfitting, since both can make local perturbations fail. (score: 0.96)
- Determine whether small invariant-preserving transformation sets reliably track explanatory quality across domains, including mathematics, physics, and historical explanation. (score: 0.90)
- Construct perturbation tests that distinguish semantic rephrasings from structure-altering substitutions so that T*(E) is not contaminated by surface variation. (score: 0.92)
- Investigate whether high explanatory reach and high rigidity can come apart—for example, explanations with broad scope but many interchangeable micro-realizations. (score: 0.87)
