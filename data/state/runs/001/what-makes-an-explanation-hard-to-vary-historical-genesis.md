# Generated: what-makes-an-explanation-hard-to-vary × historical-genesis

## Conjecture

**Conjecture:** The distinction between a load-bearing explanation and a freely swappable one is historical before it is formal: load-bearing explanations are descendants of a problem-situation that forced each part into place, while non-load-bearing accounts are typically descendants of representation systems optimized for recombination, plausibility, or compression rather than for solving the originating problem.

Applied here, the key hidden assumption is that explanation can be treated primarily as a search problem over combinatorial possibilities. That framing inherits from earlier problems LLMs were built to solve: next-token prediction, broad linguistic imitation, and smooth interpolation across human-produced text. Those systems replaced symbolic hand-coding and brittle expert systems. In doing so, they inherited an unexamined bias: if a text can be made coherent and criteria can be layered on top, then explanatory structure can be recovered by filtering outputs.

But the origin story of explanatory knowledge is different. Einstein’s train thought experiment did not arise because a huge verbal space was narrowed effectively. It arose because there was already a live contradiction: Maxwell’s equations, the principle of relativity, and the impossibility of a frozen light wave could not all be retained under the old conception of simultaneity. In that setting, the parts of the explanation became load-bearing because each answered to a specific constraint imposed by the problem. Change “simultaneity is relative,” and the account no longer resolves the conflict. Change the role of light signals, and the argument no longer connects observation to frame dependence. The explanation is hard to vary because it is tightly coupled to the problem-history that selected it.

So the structural property you are looking for is not merely internal elegance or minimality, but **counterfactual necessity relative to the originating contradiction**. In a load-bearing explanation, each major component is there because removing or altering it reopens the problem the explanation was created to solve. In a swappable account, details are underdetermined by any such problem-history; they survive because they preserve style, local coherence, or fit to priors.

What follows is a refinement of your architecture claim: narrowing search may generate knowledge only if the search space is organized around **problem lineages and constraints**, not merely around candidate texts plus criticism. The system must encode what the explanation is for, what earlier ideas it replaces, and what recalcitrant conflict each component resolves. Without that, “Deutsch score” risks being applied to polished redescriptions of existing conceptual inheritances.

So: an explanation’s parts are load-bearing when they are jointly selected by a specific inherited problem-situation such that varying them revives the original contradiction. The deepest leverage for LLM knowledge creation is therefore not better sampling over words, but reconstructing and navigating the genealogy of problems that made certain ideas necessary in the first place.

## Questions


## Candidate Problems

- How can a system represent the originating contradiction of an explanation well enough to test whether varying a component reopens the original problem? (score: 0.94)
- Are there explanations that are hard to vary for reasons other than genealogy from a specific problem-history, and if so how would this account distinguish them? (score: 0.90)
- What formal or computational proxy could capture 'counterfactual necessity relative to the originating contradiction' for use in evaluating machine-generated explanations? (score: 0.92)
- Can criticism over candidate texts recover load-bearing explanatory structure without explicit encoding of what prior ideas the explanation replaces? (score: 0.87)
- When multiple historical contradictions converge on the same explanation, how should load-bearingness be attributed to individual components? (score: 0.83)
