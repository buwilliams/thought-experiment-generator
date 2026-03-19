# Generated: can-llms-create-new-knowledge × thought-experiments

## Conjecture

**Conjecture**

Assume a hypothetical system whose only primitive operation is token prediction, but which is embedded in an architecture that does three things:  
1. generates many candidate explanations under structured constraints,  
2. tests them against criticism filters that approximate explanatory quality, and  
3. feeds surviving explanations back into future search.

Now isolate the key variable: **not whether the primitive is “mere prediction,” but whether the overall system can produce and retain explanatory variations that survive criticism better than prior variants**.

If that variable is present, then the system can generate knowledge in the relevant sense. If it is absent, it cannot — no matter how fluent its outputs are.

Why? Because Einstein did not create relativity by inventing new words; he created it by arranging existing concepts into a new explanatory structure that solved a problem and withstood criticism. So the real bottleneck is not lexical novelty, nor exhaustive traversal of combinatorial space. It is the ability to **navigate to rare explanatory configurations** by drastically collapsing search through constraints, analogies, problem decomposition, and rejection of bad variants.

In this scenario, token prediction is not the creator of knowledge by itself. It plays the role of a **variation generator** inside a larger epistemic system. The criticism layer does the indispensable work: it suppresses easy-to-vary, incoherent, or ad hoc outputs. Recursive reuse of successful structures then changes the search distribution. At that point the system is no longer merely remixing a fixed corpus in any meaningful sense; it is constructing new candidates in an evolving space shaped by prior error-correction.

What follows is a sharper distinction than “LLMs can” or “LLMs cannot.” A bare next-token predictor does not create knowledge merely by producing text. But a predictor embedded in a conjecture-and-criticism architecture can, because knowledge growth is substrate-independent: it is the growth of explanations through variation and error correction, not a property of biological minds as such.

This also illuminates the weakness in the “inductive closure” objection. That objection treats the model as if generation were bounded by interpolation within training data. But once the architecture alters the effective search space using explicit criticism and recombination, the relevant process is no longer simple induction. It becomes guided exploration of explanatory possibility.

So the decisive question is not: “Does token prediction understand?” It is: **Can the system generate explanatory conjectures that were not explicitly present, and eliminate worse ones for reasons connected to the problem rather than stylistic familiarity?** If yes, then genuine knowledge can emerge, even if the underlying generator is predictive. If no, then all apparent novelty is rearrangement.

Thus the collision of perspectives leads here: **prediction alone is not enough; prediction plus structured criticism may be enough.** The knowledge-creating unit is the whole search-and-critique system, not the token predictor in isolation.

## Questions


## Candidate Problems

- What specific criticism filters are sufficient to distinguish explanatory improvement from stylistic or benchmark-fitting improvement in a token-predictive system? (score: 0.92)
- Can a system built from token prediction plus structured criticism generate explanations that outperform all training-set-adjacent recombinations on genuinely novel problems, rather than only appearing novel to human judges? (score: 0.88)
- What forms of memory and recursive reuse actually change the search distribution toward rare explanatory configurations rather than merely reinforcing earlier mistakes or local optima? (score: 0.84)
- Is there a principled boundary between guided exploration of explanatory possibility and very sophisticated interpolation, or does the distinction collapse under some architectures? (score: 0.80)
