## System

{{mind_system}}

## User

Generate 10 yes/no questions that probe the explanatory quality of the following conjecture. Distribute them across three categories:

- 4 questions about necessity: Is a specific claim or structural element required for the conclusion to follow? Would removing it destroy the explanation rather than merely change it?
- 3 questions about reach: Does the explanation predict or illuminate something beyond what the problem literally asked for? Does it apply to cases the problem did not mention?
- 3 questions about resistance to patching: If you tried to save this explanation from a specific counterexample, would doing so require gutting its core structure rather than adding a minor qualification?

Questions must be specific to this conjecture and this problem, not generic. Keep each question to one sentence.

Rules for question text:
- Do not use quotation marks within a question. Rephrase using 'the claim that' or similar constructions instead.
- Do not use backslashes, LaTeX notation, or special characters.
- Do not nest JSON-unsafe characters inside question strings.

Return JSON only. Do not use markdown code fences. Schema: {"questions": ["...", ...]}

Conjecture: {{generated}}

Problem: {{problem_summary}}
