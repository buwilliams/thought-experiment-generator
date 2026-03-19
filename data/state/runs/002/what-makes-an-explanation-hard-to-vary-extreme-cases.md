# Generated: what-makes-an-explanation-hard-to-vary × extreme-cases

## Conjecture

**Conjecture:**  
At the limit, a *load-bearing* explanation is one whose viable variation set collapses toward zero under criticism; a *freely swappable* explanation is one whose variation set remains large while preserving surface success. The structural distinction is not whether the explanation uses new words, nor whether it was found by search, but whether its parts participate in a tightly coupled constraint network such that changing one detail forces compensating changes everywhere else or breaks the account entirely.

Pushing to the boundary clarifies this. Imagine two extremes:

1. **Maximum swappability:** every object, relation, or property can be replaced with many alternatives while the explanation still “works” conversationally. Then the explanation is underconstrained. Its apparent adequacy comes from compatibility with many nearby variants. It has low informational specificity. It is easy to vary because most of its parts are decorative or redundant.

2. **Minimum swappability:** alter one element and the explanation fails to solve the problem, loses coherence with the rest, or no longer reaches the target phenomenon. Then each part is load-bearing. The explanation occupies a narrow basin in possibility space. Its structure, not its wording, does the work.

So the hidden structure is this: **load-bearingness is a property of dependency density under counterfactual variation**. An explanation is hard to vary when its components are mutually constraining across multiple dimensions at once:

- **Problem fit:** it answers the specific problem, not nearby ones.
- **Internal coherence:** its parts support rather than merely accompany one another.
- **External reach:** it connects to consequences beyond the initial case.
- **Compression:** removing a part loses explanatory power disproportionate to the size of the part.
- **Substitution fragility:** plausible replacements fail unless the surrounding structure is also rebuilt.

At the edge, this suggests a practical criterion. Do not ask whether the explanation was produced by an LLM or a human. Ask: **How fast does explanatory performance degrade under systematic perturbation?** If swapping terms, objects, relations, or assumptions leaves the account mostly intact, the explanation is shallow. If small perturbations produce cascading failure, it is structurally deep.

This also reframes your search-space proposal. The relevant search is not over word sequences but over **constraint-satisfying structures**. The search becomes tractable only to the extent that criticism identifies narrow regions where many constraints intersect. In that sense, “genuine novelty” is not hitting a rare string; it is discovering a new high-constraint configuration.

Therefore: **the signature of a load-bearing explanation is not rarity of components but narrowness of permissible variation.** Its parts are indispensable because they are locked together by a dense web of counterfactual dependencies. A swappable explanation has loose coupling; a load-bearing one has structural necessity.

## Questions


## Candidate Problems

- How can dependency density under counterfactual variation be operationalized so that two explanations can be compared quantitatively for hard-to-vary structure? (score: 0.94)
- Are there explanations that are genuinely hard to vary because they are overfit or brittle rather than because they capture deep structure, and how would the criterion distinguish them? (score: 0.91)
- What perturbation protocol best separates decorative components from load-bearing ones across different domains such as science, history, and everyday explanation? (score: 0.88)
