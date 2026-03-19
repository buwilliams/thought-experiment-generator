## System

{{mind_system}}

## User

Generate 10 yes/no questions designed to test whether the following conjecture is genuinely specific — meaning it commits to claims that could fail. The goal is to find where the conjecture is too generic to be falsifiable: where its components could be swapped for alternatives without changing the conclusion, or where its claims are so broad they would survive any specific case.

Distribute questions across three categories:

- 4 questions about load-bearing specificity: Take a specific claim or structural element. If you replaced it with its negation or a generic alternative, would the conclusion change? A component that survives substitution is decorative, not load-bearing. Frame each question as a substitution test on a named element of this conjecture.
- 3 questions about falsifiability of reach: If this conjecture applies beyond the immediate problem, does it commit to something specific in those adjacent cases — something that could be false there? Generic assertions of broad applicability ("this applies to all systems") do not count. Ask whether a named adjacent case would produce a specific prediction that could fail.
- 3 questions about resistance to rescue: If a specific objection were raised against a named claim, would deflecting it require abandoning that claim, or just appending a qualification? A conjecture that can always be rescued by adding "except when..." is easy to vary. Ask whether a named rescue move would gut a specific part of the structure.

Questions must be specific to this conjecture and this problem — name the actual claims, elements, and cases involved. Do not generate generic structural questions that could apply to any conjecture.

A "yes" answer means the conjecture passes that test. A "no" answer means it fails — the named element is decorative, the reach claim is vague, or the rescue is too easy.

Rules for question text:
- Do not use quotation marks within a question. Rephrase using 'the claim that' or similar constructions instead.
- Do not use backslashes, LaTeX notation, or special characters.
- Do not nest JSON-unsafe characters inside question strings.

Return JSON only. Do not use markdown code fences. Schema: {"questions": ["...", ...]}

Conjecture: {{generated}}

Problem: {{problem_summary}}
