# Problems to Fix

Status: [ ] unresolved  [x] resolved  [!] blocked

1. [x] [Evaluation frame] - The evaluation prompts were confirmatory rather than falsificationist. Fixed: all four passes now seek to falsify. The evaluator treats genericness as a failure mode — a conjecture too vague to commit to specific claims scores low, not because it is wrong but because it cannot be wrong. The LLM evaluating its own output is not inherently circular; a thinker can criticize their own conjectures. The problem was the frame, not the identity of the evaluator.

2. [!] [Prompt sharpness in practice] - Whether the new evaluation frame is sharp enough can only be determined by running the system and reading outputs critically. The substitution tests in generate_questions are more mechanical than the old framing, which helps. But this remains an open conjecture about the prompts, not a settled matter. After the next full run: read several generated outputs alongside their question-answer pairs — are the "no" answers identifying real weaknesses, or do the questions still feel like softballs? If scores cluster high again, the frame isn't sharp enough.

3. [ ] [Promotion generalization] - A generated output is a 500-word response to one specific problem × lens collision. When promoted to a candidate lens, the LLM extracts a generalizable perspective from it. The extraction step may introduce fabricated breadth — the LLM is good at writing things that sound like general principles. The promoted conjecture may not carry what made the output score well.

4. [ ] [Problem self-generation quality] - The system admits up to 3 candidate problems per run from what the LLM notices in outputs. Problems admitted through high-scoring outputs may reflect fluent generation rather than genuinely important questions. Cap enforcement removes low-scoring problems, which could be the harder, more valuable ones.

5. [!] [LLM call budget] - ~357 calls per run at default settings (5 problems × ~10 lenses × 7 calls per pair). If the evaluation passes are not all adding signal, some calls are wasted. Blocked on item 2 — worth revisiting once the new prompts have been tested and we know which passes are pulling weight.
