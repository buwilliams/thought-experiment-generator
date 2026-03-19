# Generated: what-makes-an-explanation-hard-to-vary × extreme-cases

## Conjecture

**Conjecture:** At the limit, a “load-bearing” explanation is one whose valid variants collapse toward **one narrow equivalence class** under criticism, while a freely swappable explanation expands into **many interchangeable variants** without loss. The structural difference is not mainly in vocabulary or length, but in the explanation’s **error-surface**: for a good explanation, small changes break key consequences; for a bad one, many changes preserve acceptability.

Push the problem to extremes.

If every part were load-bearing, then replacing almost any object, relation, or property would either produce contradiction, lose explanatory reach, or stop deriving the target phenomenon. At the boundary, the explanation becomes **brittle in the productive sense**: not fragile because it is weak, but because its parts are mutually constrained by the problem. This is Deutsch’s “hard to vary” made structural. The account survives criticism not because it tolerates many rewrites, but because the surviving rewrites are all effectively the same explanation in disguise.

Now take the opposite limit: an explanation with maximally swappable parts. If you can exchange train for boat, lightning for bells, simultaneity for “timing weirdness,” geometry for “some hidden effect,” and still retain plausibility, then the explanation is not doing much work. Its details are decorative. It occupies a large region of search space because the constraints are weak. Such an account may sound coherent, but structurally it has low content: it rules out little.

So the hidden structure in the middle is this: **load-bearingness is measurable by counterfactual sensitivity under constrained variation**. The more an explanation tightly couples its components so that changing one forces changes in the others, the more those parts are genuinely doing explanatory work. In systems terms, the explanation has dense internal feedback: assumptions, objects, relations, and derived consequences constrain one another. In a loose explanation, these links are sparse; parts can be edited locally without global collapse.

What follows for your LLM architecture is important. Search-space narrowing should not merely optimize for coherence or elegance; it should optimize for **low-variance survival under adversarial perturbation**. Generate variants by swapping entities, changing relations, weakening assumptions, or altering thought-experiment structure. Then test whether the explanation still yields the same downstream consequences. If many variants survive, the account is probably shallow. If almost all fail except a tightly clustered family, it is likely load-bearing.

So the conjecture is:

> **An explanation’s parts are load-bearing exactly to the extent that the explanation occupies a small, criticism-stable region of possibility space: most nearby variations destroy its explanatory power, and the surviving variations are structurally equivalent rather than genuinely different.**

This gives a practical criterion. Not “is it persuasive?” but “how rapidly does it die when varied?” At the boundary between infinite swappability and zero swappability, we see the structure: explanatory depth is compression by necessity.

## Questions


## Candidate Problems

- How can 'small criticism-stable region of possibility space' be operationalized so that structurally equivalent variants are distinguished from genuinely different explanations? (score: 0.95)
- Are there good explanations in probabilistic or highly redundant domains where many nearby variations survive, making low swappability an unreliable marker of explanatory depth? (score: 0.93)
- What perturbation set is appropriate for testing load-bearingness without making the result arbitrary to the chosen variation operators? (score: 0.92)
- Can an explanation appear hard to vary only because it is underspecified at the right abstraction level, thereby passing perturbation tests without genuinely constraining the phenomenon? (score: 0.89)
