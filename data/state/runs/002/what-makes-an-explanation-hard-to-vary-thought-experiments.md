# Generated: what-makes-an-explanation-hard-to-vary × thought-experiments

## Conjecture

**Conjecture**

Assume a stripped-down scenario: we have two generators that produce candidate explanations for the same phenomenon. Both use the same vocabulary, same background corpus, same search budget, and same critic. The only variable we isolate is **constraint structure**: one generator is rewarded for local plausibility and fluency; the other is rewarded for producing accounts whose internal elements are mutually dependent such that changing one element forces changes elsewhere.

Follow that to the end: the distinguishing property of a “load-bearing” explanation is not the novelty of its words or the rarity of its sequence, but the presence of a **dense web of counterfactual dependencies** among its parts. In such an explanation, terms, objects, relations, and thought experiments are not merely compatible; they **constrain one another**. If you swap a detail freely without propagating revisions through the rest of the account, then that detail was ornamental, not explanatory.

So the relevant structure is something like this:

1. **Cross-linking:** multiple parts of the explanation support and limit each other.
2. **Propagating fragility:** a change in one premise alters downstream claims, examples, and predictions.
3. **Compression by necessity:** the explanation uses few independent assumptions because the parts do double duty.
4. **Counterfactual asymmetry:** many nearby variants remain grammatically coherent, but only a narrow subset preserve the account’s reach.

Einstein’s train/lightning case illustrates this. The train, platform, simultaneity judgment, observer motion, light-speed invariance, and consequences for time are not interchangeable decorations. Replace the train with an irrelevant scene, or make simultaneity absolute, and the rest collapses. That is load-bearing structure.

This reframes the LLM question. The hard part is not traversing an astronomically large word space. It is finding regions of **high explanatory coupling**. Search-space narrowing works only if the filters are selecting for this coupling rather than for surface coherence. A system that generates candidates and criticizes them for “hard-to-vary” structure may, in principle, discover explanations with load-bearing parts, because it is not searching for sentences but for **dependency architectures**.

What follows is a sharper criterion for knowledge creation: an LLM contributes knowledge when it helps produce an explanation whose internal structure is so constrained that successful variation requires understanding the same underlying problem. If details can be swapped freely, the output is recombinative rhetoric. If changing one part breaks the whole unless the whole is rethought, the system has located explanatory structure.

So the key distinction is:

- **Freely swappable accounts** have low internal dependency and many equivalent variants.
- **Load-bearing explanations** occupy a narrow basin in possibility space, where each component earns its place by constraining and being constrained by the others.

That is the structural signature to test for.

## Questions


## Candidate Problems

- How can dense counterfactual dependency be operationalized so that a critic can distinguish genuine explanatory coupling from merely tangled or verbose prose? (score: 0.95)
- Are there important explanations whose power comes from modularity rather than propagating fragility, and would this account misclassify them as weak? (score: 0.90)
- What search procedures can reliably find narrow basins of high explanatory coupling without collapsing into trivial overconstraint? (score: 0.87)
