# Generated: what-makes-an-explanation-hard-to-vary × computation-as-life

## Conjecture

**Conjecture:**  
An explanation is “all load-bearing” when it functions like a tightly constrained computation: each part participates in preserving the transformation from inputs to outputs, so changing any part alters the explanatory result. By contrast, an explanation with freely swappable details is one whose apparent parts are not computationally constitutive but implementation-level decorations; the real explanatory work is being done at a higher level of abstraction.

From the perspective that computation is the substrate of life, intelligence, and evolution, the key distinction is not between “detailed” and “simple” explanations, but between **causally necessary informational structure** and **replaceable encoding**.

A thought experiment clarifies this. Imagine two programs that produce the same adaptive behavior. In the first, every line is coupled to every other through precise dependencies; altering a variable, operation, or ordering breaks the behavior. In the second, many lines can be renamed, reordered, or reimplemented without changing the outcome because only the abstract algorithm matters. The first has many load-bearing parts at that descriptive scale; the second only appears detailed because we are looking below the level where the real computation is defined.

So the structural property at issue is **invariance under substitution**. If parts can be swapped while preserving explanatory power, those parts are not fundamental to the explanation at that level; they are members of an equivalence class of implementations. If substitution destroys the account, the parts are load-bearing because they encode indispensable constraints.

This suggests a more precise criterion:

1. **A load-bearing explanation has low freedom of substitution.**  
   Its components are bound by necessity, not preference.

2. **A non-load-bearing explanation contains compressible redundancy.**  
   Multiple distinct details map to the same explanatory role.

3. **The difference depends on level of abstraction.**  
   At one level, nucleotide sequence may matter in every position; at another, only the protein function matters. Likewise in software, machine code may be fully load-bearing for a chip, while the algorithm is robust across languages.

4. **Good explanations isolate the minimal computational architecture required for the phenomenon.**  
   They distinguish what must be preserved from what can vary.

Under selection, this matters because systems evolve robustness by pushing necessity upward and contingency downward: function is retained while implementation becomes flexible. Explanations mirror this. The best explanation is not the one with the most details, but the one that correctly identifies which informational constraints are under selection and which are merely one realizable form among many.

So what is illuminated is this:  
**Load-bearingness is not about descriptive density; it is about whether each part encodes an indispensable constraint in the computation generating the phenomenon.** Details are swappable when they belong to the same computational equivalence class. They are load-bearing when changing them changes the computation itself.

## Questions

1. 1. Necessity: If the claim that load-bearingness is defined by invariance under substitution were removed, would the conjecture still explain the difference between indispensable parts and merely decorative details rather than just restate it? — **no**
2. 2. Necessity: Is the appeal to level of abstraction required for the conclusion to hold, such that without it the software and biology examples would collapse into a contradiction about whether the same detail is load-bearing? — **yes**
3. 3. Necessity: Does the conclusion depend on the claim that the real explanatory work is done by computational structure rather than by descriptive richness, so that dropping this claim would destroy the proposed criterion? — **yes**
4. 4. Necessity: Is the thought experiment with two programs necessary to support the conjecture's distinction between causally necessary informational structure and replaceable encoding, rather than serving as an optional illustration? — **no**
5. 5. Reach: Does the conjecture imply that two explanations with very different surface details can be equally good whenever they preserve the same minimal computational architecture? — **yes**
6. 6. Reach: Does the conjecture illuminate why evolved systems often appear full of arbitrary historical detail while still admitting compact higher-level explanations of their function? — **yes**
7. 7. Reach: Does the criterion extend beyond biology and software to explanations in other domains where multiple implementations realize the same abstract process? — **yes**
8. 8. Resistance to patching: If a counterexample showed an explanation whose parts are all indispensable only because of accidental historical lock-in rather than computational necessity, would saving the conjecture require revising its core link between load-bearingness and indispensable constraints? — **yes**
9. 9. Resistance to patching: If two accounts had identical substitution profiles but differed sharply in explanatory depth, would rescuing the conjecture force it to add a new criterion beyond substitution invariance and thus gut its central test? — **yes**
10. 10. Resistance to patching: If a phenomenon required a hybrid explanation in which some details are load-bearing only jointly across levels of abstraction, would accommodating that case require abandoning the conjecture's clean separation between fundamental structure and implementation-level decoration? — **yes**

## Candidate Problems

- How can 'load-bearingness' be made into a rigorous, non-circular measure across levels of abstraction? The conjecture relies on substitution invariance and 'indispensable constraints,' but it leaves open how to formally distinguish genuine explanatory parts from merely implementation-level variants without presupposing the correct abstraction level. A key open problem is whether there exists a principled metric or procedure—perhaps using intervention, algorithmic information, causal structure, or equivalence classes of computations—that identifies the minimal computational architecture required for a phenomenon. (score: 0.97)
- What determines the correct explanatory level, and can multiple incompatible levels be equally load-bearing? The conjecture says load-bearingness is level-dependent, but this raises a tension: if different abstractions preserve different invariances, there may be no unique 'real' explanatory architecture. An open question is how to choose among molecular, functional, algorithmic, or dynamical descriptions when each captures indispensable constraints relative to different interventions or environments. (score: 0.93)
- Is evolutionary robustness really characterized by 'pushing necessity upward and contingency downward,' and under what conditions does this fail? The conjecture proposes a strong link between selection and abstraction, but many evolved systems remain path-dependent, brittle, or historically contingent at lower levels. A worthwhile problem is to determine when evolution produces broad computational equivalence classes versus when specific implementations remain causally indispensable, and how this affects what counts as the best explanation. (score: 0.90)
