# Problem to Fix

## The Core Issue

The evaluation function cannot distinguish insight from fluency.

The same LLM that generates an output also generates the 10 yes/no questions that probe it, then answers those questions. It will reliably produce questions it knows how to answer "yes" to, about text it just wrote. The self-assessment already caught this: outputs score high by being fluent restatements. The four-pass evaluation adds sophistication, but all four passes share the same flaw — the model is grading its own homework. Adding more passes does not fix the circularity; it amplifies it. The hard-to-vary score, explanatory reach, and resistance to refutation may all be measuring how confidently the LLM writes rather than genuine explanatory quality.

Everything downstream depends on this. If the evaluation function tracks fluency rather than insight, then the promotion mechanism, the score trajectories, and the mind composition are all artifacts of which outputs the LLM writes most confidently — not which ones are most insightful.

## Secondary Problems

**Promoting generated outputs to candidate lenses may not work as intended.**
A generated output is a 500-word response to one specific problem × lens collision. When promoted, the LLM extracts a generalizable lens from it. But the generalization step can introduce fabricated breadth — the LLM is good at writing things that sound like general principles. The promoted conjecture may not carry the insight that made the output score well; it may carry fluent-sounding abstraction instead.

**The problem self-generation may be bloating the problem set.**
The system admits up to 3 candidate problems per run from what the LLM notices in outputs. But the problems admitted are scored by the same flawed evaluation — meaning they appear in high-scoring (fluent) outputs, not necessarily because they are the most important questions. Cap enforcement removes low-scoring problems, which might be the genuinely hard ones.

**The LLM call budget is high relative to signal quality.**
~357 calls per run (5 problems × ~10 lenses × 7 calls per pair). If passes 2–4 are circular, then each is burning calls without improving the ranking signal. The simpler consistency gate (pass 1) might be doing most of the useful filtering on its own.

## What Is Not the Problem

The core loop structure is sound: `problem × lens → generate → evaluate → promote`. The two-layer architecture with the resistance model is right. The filesystem state is right. The ask command's multi-lens consolidation is right. The code is clean and not over-engineered. The design has the right shape — the problem is a load-bearing assumption within it.

## The Question to Answer Before Proceeding

Can the evaluation function be made to distinguish insight from fluency?

One direction: use a separate, adversarial evaluator model with a different system prompt — one explicitly instructed to find where the output is fluent but empty, where it restates the question rather than illuminating it, and to score harshly on those grounds. The evaluator should not have seen the generation prompt.

Another direction: reduce the evaluation to the one pass least susceptible to circularity (logical consistency is hardest to fake; resistance to refutation is most susceptible) and accept that ranking is coarse but honest.

The worst outcome is a sophisticated evaluation function that produces confident but meaningless scores.
