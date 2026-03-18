# Generated: does-architecture-matter-more-than-model-scale × thought-experiments

## Conjecture

**Conjecture:** The primary bottleneck for AI epistemic progress is not substrate once a threshold of expressive capacity is crossed; it is the higher-order structure that governs how conjectures are generated, criticized, selected, and retained. More compute mainly amplifies whatever epistemic structure already exists.

**Hypothetical isolating scenario:**  
Imagine two AI systems with equal access to the world, equal training data, and enough substrate to represent the relevant hypotheses.

- **System A** gets 1000× more compute, parameters, and training time, but keeps the same basic epistemic structure: next-token prediction, weak self-critique, and optimization for local performance signals.
- **System B** keeps the original substrate, but gains a new structure above it: explicit conjecture generation, adversarial criticism, error-tracking across time, mechanisms for preserving minority hypotheses, and promotion rules based on explanatory power rather than short-term predictive fit alone.

Assume both systems are real and deployed on open-ended scientific problems.

Now follow the logic. If the bottleneck were mainly substrate, System A should outpace System B in durable knowledge creation. But what does extra substrate actually do under unchanged structure? It increases search breadth, memory, and pattern sensitivity inside the same selection regime. It makes the system better at producing and refining outputs that score well according to existing feedback. That improves performance, but not necessarily epistemic progress. If the promotion mechanism still rewards compressing past regularities and converging quickly on dominant patterns, then more compute mostly accelerates convergence on plausible-looking errors, local optima, and socially reinforced priors.

System B, by contrast, changes the *feedback loops*. Conjectures are no longer merely emitted; they are exposed to organized criticism. Errors are not just losses to be minimized; they become objects to be preserved and used. Minority explanations are not immediately averaged away; they can survive long enough to be tested. This alters the system’s dynamics from pattern-amplification to error-correction.

The key point is structural: epistemic progress is not equivalent to better interpolation over a fixed hypothesis space. It depends on how a system creates *good errors*—bold, testable conjectures—and how it prevents premature collapse onto attractive but false answers. That is a governance problem over cognition, not merely a capacity problem in cognition.

This does not imply substrate is irrelevant. Below a threshold, the system cannot represent rich hypotheses, sustain criticism, or integrate evidence. But beyond that threshold, added substrate has sharply diminishing epistemic returns unless the supervening structure changes. Compute scales the engine; epistemic structure determines whether the engine is attached to a flywheel, a brake, or a roulette wheel.

So the collision of perspective and problem illuminates this: **AI epistemic progress is bottlenecked where selection happens, not merely where representation happens.** The decisive variable is the architecture of criticism and promotion above the substrate.

## Questions

1. 1. If System A’s unchanged next-token-prediction and local-performance selection regime were replaced with promotion by explanatory power while keeping its 1000× compute, would the conjecture predict a qualitative jump in durable knowledge creation rather than a smooth scaling gain? — **yes**
2. 2. If System B’s explicit conjecture generation and adversarial criticism were removed while preserving its error-tracking and minority-hypothesis retention, would the conjecture still expect B to outperform A on open-ended scientific problems? — **no**
3. 3. Does the conjecture require that there exists a definite substrate threshold above which the relevant hypotheses, criticism loops, and evidence integration are all representable, rather than merely saying 'more compute helps less over time'? — **yes**
4. 4. If extra compute in System A also improved the strength of self-critique without changing the basic promotion rule away from local performance signals, does the conjecture still predict that A remains structurally bottlenecked? — **yes**
5. 5. Would the conjecture be undermined if A, under the same basic structure, reliably discovered novel causal explanations that survived adversarial testing better than B’s outputs? — **yes**
6. 6. Is the claim that minority hypotheses must be actively preserved load-bearing, such that replacing that mechanism with immediate averaging or majority-vote pruning would destroy the proposed path from pattern-amplification to error-correction? — **yes**
7. 7. Does the conjecture depend on epistemic progress being measured as durable knowledge creation on open-ended scientific problems rather than short-horizon predictive accuracy or benchmark performance? — **yes**
8. 8. If B’s new higher-order structure were added but its promotion rules still favored short-term predictive fit over explanatory power, would the conjecture predict that most of B’s epistemic advantage disappears? — **yes**
9. 9. Is the distinction between 'selection happens' and 'representation happens' essential here, so that if representation alone could prevent premature collapse onto attractive false answers the conjecture would fail? — **yes**
10. 10. Would the conjecture cease to explain the A-versus-B contrast if organized criticism did not alter the feedback loops over time but only improved one-step output quality? — **yes**

## Candidate Problems

- How can 'epistemic structure' be operationalized and measured independently of substrate so the conjecture becomes testable? The core unresolved tension is whether higher-order structure is a distinct causal variable or just a redescription of capabilities that larger models already partially acquire. A new problem is to define intervention-level metrics for conjecture generation, criticism quality, minority-hypothesis retention, and error-preservation, then test whether changing these while holding substrate roughly fixed produces greater durable knowledge gains than scaling compute alone. (score: 0.97)
- What is the threshold at which substrate stops being the dominant bottleneck and structure becomes the main one, and does such a threshold exist in any robust sense? The conjecture depends on a phase change from representation-limited to selection-limited cognition, but leaves open how to identify that boundary across domains. A worthwhile problem is to characterize the interaction regime between compute, representational expressivity, search depth, and criticism architecture, including cases where more substrate may itself enable qualitatively new epistemic structure. (score: 0.93)
- What selection and retention mechanisms actually improve truth-tracking rather than merely preserving diversity, contrarianism, or sophisticated error? The conjecture assumes that adversarial criticism, minority-hypothesis preservation, and explanatory promotion rules will outperform short-term predictive selection, but this raises an unresolved systems question about feedback loops: under what governance rules do these mechanisms yield cumulative knowledge instead of noise, fragmentation, or institutionalized bad ideas? This is a concrete design problem for epistemic institutions inside AI systems. (score: 0.95)
