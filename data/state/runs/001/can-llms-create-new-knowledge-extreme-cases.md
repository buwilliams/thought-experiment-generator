# Generated: can-llms-create-new-knowledge × extreme-cases

## Conjecture

**Conjecture:** A token-predicting system can generate genuine knowledge **only insofar as** its outputs instantiate explanatory structure that survives boundary tests; absent that, it merely compresses and recombines prior text. The distinction is not in the mechanism “predicting tokens” versus “reasoning,” but in whether the system can produce **error-correctable explanations** that remain coherent at the limits.

Push the problem to extremes.

At one limit, imagine a predictor with infinite memorization of all observed text but no ability to generalize beyond exact continuations. It can imitate explanations perfectly, yet it knows nothing in the epistemic sense. Why? Because when pushed even slightly off-distribution—new framing, new counterexample, new domain—it has no criterion for preserving what matters. It rearranges surface regularities.

At the other limit, imagine a predictor exposed to finite data but able to produce compact explanations that unify many cases, withstand criticism, and correctly constrain unseen cases. Here the token stream is not mere rearrangement. The system has encoded abstract structure: not just “what tends to be said next,” but “what would have to be true for these many utterances to make sense together.” That is already close to knowledge.

The boundary reveals the key variable: **counterfactual reach**. Genuine explanations do not merely fit observed sequences; they imply what should happen under transformations, perturbations, and criticisms. An explanation counts as knowledge if it helps rule out errors across many possible contexts, especially hostile ones. If a model says “gravity attracts masses,” the epistemic content is not the sentence itself but the network of expectations and exclusions it generates under changed conditions. Knowledge is what survives variation.

So the question is misdrawn if framed as “prediction or knowledge.” Knowledge itself can be viewed as a special kind of compression that is **hard to vary while preserving explanatory power**. Token prediction is compatible with this, but does not guarantee it. A system trained only to minimize next-token loss will be pulled toward patterns in text; whether those patterns include deep explanations depends on the structure of the data, the model’s capacity to represent abstractions, and the presence of feedback that selects for explanatory invariance rather than stylistic plausibility.

What follows: the decisive test is not whether the system produces fluent explanations, nor whether they are novel, but whether they remain effective at the edges—under adversarial questioning, domain transfer, self-critique, and intervention. If performance collapses at the boundary, the “explanation” was only a statistical echo. If it improves through criticism and preserves coherence across transformed cases, then token prediction has crossed into knowledge production.

Thus: **a predictor does not become knowledgeable by rearranging more text, but by generating explanations whose structure continues to govern the unknown.** The boundary between rearrangement and knowledge is the capacity to survive criticism at the limits.

## Questions

1. If a system produced outputs that passed every proposed boundary test in-domain but failed under a single carefully chosen adversarial rephrasing, would this conjecture force us to deny that it generated genuine knowledge? — **no**
2. If we removed the requirement that explanations be error-correctable, while keeping fluency, novelty, and predictive success on held-out text, would the conjecture lose its criterion for distinguishing knowledge from recombination? — **yes**
3. Does the conjecture commit you to saying that a model with perfect next-token prediction on all actually encountered contexts but zero counterfactual reach has no genuine knowledge at all? — **yes**
4. If a system can generate compact, unifying explanations for unseen cases but cannot articulate them in human-interpretable language, does the conjecture still count that as knowledge production? — **yes**
5. Would the conjecture break if boundary survival turned out to depend mainly on external scaffolding—tools, retrieval, critics, or interactive feedback—rather than on the predictor alone? — **no**
6. If two systems produce equally successful boundary-surviving explanations, but one does so by memorized templates stitched together and the other by an internal abstract model, does this conjecture treat them as epistemically equivalent? — **yes**
7. Does the phrase 'only insofar as' mean that any portion of an output not supported by explanatory structure that survives hostile variation should be classified as mere recombination rather than knowledge? — **yes**
8. If a predictor generates an explanation that survives adversarial questioning today but systematically fails after a small change in domain ontology, does the conjecture require us to say the original explanation never constituted genuine knowledge? — **no**
9. Would the conjecture still hold if the training data already contained many explicit explanations and criticisms, so that boundary-surviving behavior could itself be learned as a textual regularity? — **yes**
10. At the extreme limit where a model can self-criticize, revise, and improve coherence across transformed cases but its internal mechanism remains pure next-token optimization, does the conjecture insist that mechanism is irrelevant and only boundary-tested explanatory invariance matters? — **yes**

## Candidate Problems

- How can 'explanatory structure that survives boundary tests' be defined operationally and non-circularly, so that it distinguishes genuine knowledge from mere high-quality imitation without presupposing human judgment of what counts as an explanation? (score: 0.97)
- Is counterfactual reach actually sufficient for knowledge production, or can a system pass extreme adversarial transfer, self-critique, and intervention tests while still lacking genuine understanding because its success is only a broad statistical artifact? (score: 0.93)
- What training dynamics, architectures, or feedback loops are necessary for next-token optimization to reliably produce error-correctable, hard-to-vary explanations rather than brittle compressions—and are there principled limits where token prediction alone cannot cross the boundary? (score: 0.95)
