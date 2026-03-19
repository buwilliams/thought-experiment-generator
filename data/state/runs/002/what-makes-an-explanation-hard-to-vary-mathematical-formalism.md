# Generated: what-makes-an-explanation-hard-to-vary × mathematical-formalism

## Conjecture

**Conjecture:**  
An explanation whose parts are all load-bearing is one that occupies a **small equivalence class** under admissible transformations: if you formalize the explanation as a relational structure \(E = (O,R,C)\) of objects, relations, and constraints, then a good explanation is one for which most perturbations of \(O\), \(R\), or \(C\) destroy the target phenomenon. A bad explanation lies in a large equivalence class: many substitutions preserve surface plausibility because the account is underconstrained.

More concretely, let an explanation be a map from a structured scenario to a set of consequences:
\[
E : (O,R,C) \to P
\]
where \(P\) is the phenomenon to be explained. Now define a family of allowed variations \(T\): renamings, substitutions of entities, altered properties, reordered events, changed background assumptions, etc. The key structural quantity is the **stability profile** of \(E\) under \(T\).

- If many \(t \in T\) leave \(P\) intact, then the explanation is **freely variable**. Its details are ornamental; they do not constrain the outcome.
- If only a narrow set of \(t\) preserve \(P\), then the explanation is **load-bearing**. Its details are doing real work.

This gives a mathematical restatement of “hard to vary”: not a psychological impression, but a property of the explanation’s **constraint geometry**. Good explanations have high dependency density: each major element participates in multiple inferential paths, so changing one part breaks consistency elsewhere. Bad explanations have low dependency density: elements are loosely coupled, so replacements do not propagate failure.

Applied to your LLM/search-space framing, the relevant distinction is not whether the model can sample from a huge combinatorial space, but whether it can discover structures with **low variation volume** relative to the phenomenon. Search becomes tractable when guided toward regions of explanation-space where:
1. the explanatory graph is tightly constrained,
2. consequences are invariant under irrelevant transformations,
3. and failure occurs under most relevant perturbations.

Einstein’s train-and-lightning thought experiment is powerful because its components are not arbitrary tokens. “Train,” “platform,” “lightning,” and “observer motion” instantiate a relational structure whose consequences depend on simultaneity, signal propagation, and frame dependence. Swap those relations carelessly and the explanatory force collapses. By contrast, a merely narrative retelling may preserve wording while losing the constraint structure.

So the structural property that distinguishes load-bearing explanations is:

**They maximize consequence under minimal admissible variation.**

Or in another form:

**A genuine explanation is one with a high ratio of derived necessity to replaceable detail.**

This suggests an operational test for LLM-generated knowledge: not novelty of wording, but whether the generated explanation defines a tightly constrained relational structure whose essential parts resist substitution without loss of explanatory reach.

## Questions


## Candidate Problems

- Define the admissible transformation family T for a given domain so that 'hard to vary' distinguishes relevant from irrelevant perturbations rather than merely punishing any change. (score: 0.92)
- Quantify 'dependency density' in explanatory graphs and test whether higher density predicts better out-of-sample explanatory performance or merely brittleness. (score: 0.86)
- Evaluate whether small equivalence class size under T correlates with truth, or whether some false but tightly constrained explanations also score as hard to vary. (score: 0.90)
- Operationalize the ratio of derived necessity to replaceable detail and compare it across scientific, historical, and narrative explanations. (score: 0.81)
