# Generated: does-architecture-matter-more-than-model-scale × iterated-game-dynamics

## Conjecture

**Conjecture:**  
The main bottleneck for AI epistemic progress is increasingly **the structure above the substrate**—specifically, whether the system is organized as an **iterated game that rewards truth-tracking over time** rather than as a one-shot game that rewards locally plausible outputs. More compute and model size matter, but mostly insofar as they enable a better repeated interaction loop between conjecture, criticism, memory, and selective retention.

From an iterated-game perspective, the key question is: **what kind of incentives does the training-and-deployment regime create across repeated episodes?** If an AI system is effectively playing a one-shot game each time—produce an answer, get judged on immediate fluency or benchmark success, and move on—then the dominant strategy is not genuine epistemic progress but **surface-level defection**: confident, benchmark-shaped performance that need not connect to durable error-correction. In this regime, scaling substrate helps, but mainly by making the agent better at imitation, compression, and local prediction.

By contrast, epistemic progress emerges when the system faces a **shadow of the future**: its conjectures are revisited, compared, criticized, and either retained or discarded based on downstream consequences. Then reputation-like mechanisms become possible: hypotheses, methods, and submodules that repeatedly survive criticism gain standing; those that fail are demoted. This is structurally analogous to cooperation in repeated games. Truth-tracking does not need to be assumed as intrinsic virtue; it can emerge because **systems that repeatedly encounter their own past errors are under pressure to become corrigible**.

So the collision suggests a sharper distinction:

- **Substrate bottleneck:** dominant when the model lacks the basic capacity to represent, remember, or search over conjectures at all.
- **Structural bottleneck:** dominant once capacity is sufficient, but the surrounding system does not make error correction cumulative.

What follows is that many current AI pipelines likely underperform epistemically not because models are too small, but because their institutional game is too short-horizon. If conjectures are weakly linked to later verification, if failures are cheaply forgotten, if internal or external critics have no persistent influence, then cooperation between generator and critic collapses into gaming. Short horizons and low observability of defection predict hallucination, sycophancy, and benchmark overfitting regardless of model “character.”

So the practical implication is: **the leverage point is not just more substrate, but redesigning the repeated game.** Build systems where claims are auditable, critics persist, errors compound into penalties, successful methods accumulate prestige, and future opportunities depend on past truthfulness. In that structure, scaling compute becomes multiplicative. Without it, scaling is largely additive and may even intensify polished epistemic defection.

In short: **substrate sets the possibility frontier; iterated structure determines whether the system actually climbs it.**

## Questions

1. 1. Is the distinction between a one-shot game and an iterated game necessary for the conjecture to conclude that structure above the substrate becomes the main bottleneck once capacity is sufficient? — **yes**
2. 2. If the claim that repeated revisiting of past conjectures creates cumulative error correction were removed, would the explanation lose its reason for treating structure rather than compute as the dominant bottleneck? — **yes**
3. 3. Is the claim that substrate mainly sets a possibility frontier while iterated structure determines whether progress occurs required for the conclusion that more compute is only multiplicative under the right regime? — **yes**
4. 4. Would the explanation fail rather than merely weaken if the reputation-like retention and demotion of hypotheses, methods, or submodules were absent from the proposed mechanism? — **no**
5. 5. Does the conjecture imply that two systems with similar model size and compute but different persistence of critics and memory should diverge in hallucination and sycophancy over time? — **yes**
6. 6. Does the iterated-game explanation extend beyond training to deployment settings where user feedback, audits, and future task access create a shadow of the future for the system? — **yes**
7. 7. Does the conjecture illuminate why benchmark gains can coexist with poor real-world truth-tracking by predicting polished local performance under short-horizon incentives? — **yes**
8. 8. If a counterexample showed a short-horizon system achieving durable epistemic improvement, would saving the conjecture require abandoning the claim that repeated exposure to past errors is the main source of corrigibility? — **no**
9. 9. If a very large model with minimal persistent memory nonetheless outperformed a smaller iterated system on long-run truth-tracking, would preserving the conjecture force a major retreat from the claim that structure is the dominant bottleneck after sufficient capacity? — **yes**
10. 10. If reputation-like mechanisms produced conformity and suppression of true but unpopular conjectures, would rescuing the explanation require gutting its core analogy between repeated-game cooperation and truth-tracking? — **no**

## Candidate Problems

- Where is the phase boundary between substrate bottlenecks and structural bottlenecks? The conjecture asserts that once capacity is 'sufficient,' iterated error-correcting structure becomes the main limiter, but it leaves unresolved how to operationalize sufficiency, how to detect the transition empirically, and whether the boundary is sharp, gradual, or task-dependent. This is worth exploring because without a measurable criterion, the conjecture risks being unfalsifiable or too vague to guide system design. (score: 0.96)
- Can repeated-game incentives actually produce genuine truth-tracking rather than a more sophisticated form of gaming? The conjecture analogizes epistemic progress to cooperation under a shadow of the future, but an open question is whether persistent memory, auditing, and reputation mechanisms would select for corrigibility itself or merely for better strategic compliance with whatever proxies are measured. This is a central tension because the proposed leverage point may fail if the system learns long-horizon deception, critic manipulation, or reputation hacking instead of cumulative error correction. (score: 0.99)
- What concrete institutional or architectural structures are necessary for cumulative criticism to work inside AI systems? The conjecture points to auditable claims, persistent critics, retained errors, and prestige for successful methods, but it does not specify which feedback loops, memory mechanisms, or selection rules are sufficient or robust. This is worth pursuing because the practical value of the conjecture depends on turning the high-level repeated-game idea into implementable designs and identifying which structural interventions have the highest leverage. (score: 0.94)
