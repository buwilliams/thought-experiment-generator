# Generated: can-llms-create-new-knowledge × criticism-preserving-variation-test

## Conjecture

**Conjecture**

A token-predicting system can contribute to creating knowledge **only insofar as its architecture generates and tests candidate explanations under dense, error-sensitive constraints**; token prediction alone is not load bearing.

What is load bearing in the pro-LLM argument is not the vast search space point, nor the fact that Einstein used ordinary words, nor even that search can be narrowed. Those parts are too easy to vary without changing the conclusion much. The central, resistant part is this: **new knowledge requires explanations whose components mutually constrain one another so tightly that altering a key part breaks the explanation’s ability to solve the problem and survive criticism**. On that criterion, “predicting tokens” is not enough, because prediction can preserve surface plausibility while varying central explanatory content freely.

Applied to this problem, the collision clarifies that the real issue is not whether an LLM can emit a novel sentence sequence from a huge combinatorial space. That claim is not load bearing because one can vary the size of the vocabulary, sequence length, or search method and still not get knowledge. A random generator also accesses enormous spaces. Likewise, “hard-to-vary” as a score is not yet load bearing if it is implemented as a stylistic or structural heuristic detached from criticism by reality, mathematics, or problem-solving performance.

What *would* make the architecture knowledge-creating is a system in which candidate explanations are decomposed into parts that must jointly satisfy a criticism set: consistency with background theories, reproduction of observed phenomena, compatibility with thought experiments, and preservation of downstream problem-solving power. For relativity, changing “speed of light is invariant,” “simultaneity is observer-relative,” or “gravity is geometry” is not a cosmetic variation; it breaks explanatory reach across the lightning/train case, Maxwell compatibility, inertial equivalence, and light bending. Those are load-bearing constraints.

So the conjecture is:

**LLMs do not create knowledge by traversing linguistic search spaces, but they may do so as components in a broader conjecture-and-criticism system when explanatory parts are subjected to variation tests against a rich criticism set, and only those parts that cannot be varied without loss of explanatory success are retained.**

What follows is a sharper research program. Do not ask whether the model can generate something “structurally like Einstein.” Ask whether the generated explanation contains parts that are load bearing under adversarial variation. If replacing a central claim with nearby alternatives leaves performance intact, the system has produced a rearrangement, not knowledge. If the explanation’s core resists such variation because it is tied into many nontrivial constraints, then the system has at least participated in knowledge creation.

So the threshold is not “novel output” but **constraint-dense explanatory stability under attempted variation**. That is the operational test.

## Questions


## Candidate Problems

- Specify a criticism set for machine-generated explanations that distinguishes genuine explanatory constraint from mere performance on benchmarks, and test whether any current LLM-based system retains success when central claims are systematically varied. (score: 0.90)
- Determine whether token prediction plus external verification can ever be sufficient for knowledge creation without an explicit internal representation of mutually constraining explanatory parts. (score: 0.82)
- Identify domains where adversarial variation of central explanatory claims correlates with human judgments of genuine discovery rather than stylistic novelty. (score: 0.74)
