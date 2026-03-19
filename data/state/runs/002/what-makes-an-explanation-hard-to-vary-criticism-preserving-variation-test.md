# Generated: what-makes-an-explanation-hard-to-vary × criticism-preserving-variation-test

## Conjecture

**Conjecture**

What distinguishes a genuinely load-bearing explanation from a freely swappable one is not that it is short, rare, or superficially “Einstein-like,” but that its core parts are tied together by **dense, error-sensitive constraints** that survive criticism only as a package. In this problem, that means an explanation of relativity is load-bearing when changing any central component — e.g. constancy of light speed, Maxwell incompatibility, relativity of simultaneity, the train/lightning setup, equivalence principle, or curved spacetime inference — causes one of three failures: internal contradiction, loss of explanatory reach, or mismatch with the target phenomena.

From this perspective, the important issue for LLM knowledge creation is not primarily search-space size. Search can be narrowed in many ways. The decisive question is whether the system can generate candidate explanations whose parts are **mutually constraining enough** that variation is difficult without breaking success across the criticism set. A merely stylistic or recombinational output can preserve surface coherence while swapping details freely. A load-bearing explanation cannot.

So the proposed architecture should be evaluated by a stronger operational test than “did it find something structurally similar?” It should perform **systematic variation under preserved success conditions**:

- vary one central claim while holding the rest fixed;
- test whether the account still explains the same phenomena;
- test whether it still avoids contradiction with background theories and observations;
- test whether it still solves the original problem rather than a nearby one.

If many substitutions work, the explanation was not deeply load-bearing; it was a loose scaffold. If almost every central substitution breaks it, then the account has the kind of constraint density associated with explanatory knowledge.

This collision illuminates a refinement to the LLM thesis: **LLMs do not create knowledge merely by navigating a huge search space with filters. They create knowledge, if at all, when the architecture can generate and retain explanations whose central components are hard to vary under criticism.** Search-space collapse is therefore upstream support, not the essence. The essence is constructing candidate theories that become increasingly resistant to variation because their parts jointly answer multiple criticisms at once.

What follows is a practical criterion for your experiment. Do not benchmark success only by recovering an Einstein-like thought experiment. Benchmark it by a **load-bearing profile**: identify the candidate’s central parts, attempt principled replacements, and measure degradation in explanatory performance. A system that outputs many plausible narratives but few high constraint-density theories is producing non-load-bearing recombinations. A system whose best outputs become progressively harder to vary while retaining reach is approaching knowledge creation.

So the structural property you want is: **high mutual constraint among central explanatory parts, revealed by systematic failed attempts to swap them without loss.** That, more than novelty or resemblance, marks the transition from fluent synthesis to genuine explanation.

## Questions


## Candidate Problems

- Develop a domain-general procedure for identifying which parts of an explanation count as central before running variation tests, and test whether different procedures converge. (score: 0.91)
- Determine whether high failure under principled substitution tracks genuine explanatory depth or can be faked by overfitted, brittle narratives. (score: 0.89)
- Measure whether explanations that become harder to vary over iterative refinement also improve independent predictive or problem-solving performance. (score: 0.84)
