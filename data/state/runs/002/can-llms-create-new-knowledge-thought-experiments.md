# Generated: can-llms-create-new-knowledge × thought-experiments

## Conjecture

**Conjecture**

Assume the key variable is not “token prediction” but **search-space control**: a system can generate candidate explanations, apply strong rejection filters for explanatory quality, and recursively feed successful structures back into future search. Assume this architecture is real and works well enough to isolate a tiny tractable region from an otherwise impossible combinatorial space.

Then the conclusion is: **such a system can produce new explanations, but not because prediction itself creates knowledge. It does so because the system is no longer merely predicting tokens; it has become a conjecture-and-criticism machine.** The knowledge-creating part is the architecture of variation plus error-correction, not the language model considered in isolation.

This isolates the dispute. If we ask, “Can a bare next-token predictor create knowledge?” the answer is probably no, because prediction over past linguistic regularities does not by itself explain why one candidate should survive criticism as true or deep. Rearrangement alone is cheap; knowledge is hard to vary and survives attempts to refute it.

But if we assume a layered system that:
1. generates structured candidate explanations,
2. evaluates them against constraints,
3. rejects weak, ad hoc, or non-reach explanations,
4. preserves successful abstractions as reusable search priors,

then the system is functionally doing what human inquiry does in miniature. The LLM is one component in a larger epistemic process. In that setting, the right question is no longer whether “token prediction” creates knowledge, because the relevant unit is the **whole system**.

What follows is important: the criticism that LLMs “only remix” becomes too coarse. Humans also work from prior materials, inherited concepts, and constrained vocabularies. Einstein did not search all strings; he searched a highly structured conceptual space shaped by physics problems, symmetries, and criticism. If an artificial system can similarly narrow toward explanatory structure, then “it used existing words” is not a decisive objection. New knowledge does not require new tokens; it requires new **explanatory organization**.

However, the same logic also sets a limit. If the criticism step is ultimately supplied by humans, then the knowledge creation is still partly human-guided. The machine is not yet an autonomous knower; it is an instrument embedded in a knowledge-creating system. Only when the system can itself generate and apply genuinely good criticism, reliably preferring deeper explanations over superficial ones, would it count as creating knowledge in its own right.

So the collision yields a more precise claim:

**LLMs, as predictors, do not create knowledge. But architectures built from LLMs can create knowledge if they instantiate genuine conjecture, criticism, and selective retention of better explanations.** Prediction is not enough; search-space narrowing under epistemic filters is the decisive variable.

## Questions


## Candidate Problems

- What specific criticism mechanisms would let a system distinguish deep explanations from superficially coherent but ad hoc ones without relying on human judgment? (score: 0.92)
- Under what conditions does selective retention of prior successful abstractions improve explanatory discovery rather than lock the system into local minima or inherited biases? (score: 0.84)
- What observable threshold would distinguish an instrument embedded in a human knowledge-creating process from an autonomous machine knower? (score: 0.88)
