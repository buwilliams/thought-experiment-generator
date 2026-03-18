# Generated: does-architecture-matter-more-than-model-scale × counterfactual-reasoning

## Conjecture

**Conjecture:** The main bottleneck for AI epistemic progress is increasingly **structural rather than substrate-level**: once a system can generate and evaluate many candidate ideas, additional scale mostly amplifies throughput, while the decisive limit becomes **the architecture of conjecture, criticism, memory, and selection**. Compute is load-bearing up to a threshold; beyond that, epistemic progress depends more on the system that decides what counts as a good explanation and how errors are exposed.

Test this by varying assumptions.

**1. Remove the substrate advantage.**  
Assume model size and compute are held fixed. Can epistemic progress still improve? Yes: better decomposition of problems, stronger error-correction loops, persistent memory, adversarial testing, and improved criteria for promoting hypotheses can all raise performance without changing the substrate. This suggests structure is at least a genuine bottleneck.

**2. Remove the structural advantage.**  
Now assume massive scale but weak conjecture-testing structure: shallow objectives, next-token imitation, poor memory of failed ideas, weak criticism, no mechanism for preserving explanatory gains. What happens? The system gets more fluent, broader, and faster, but tends to recycle correlations rather than accumulate knowledge. It can search larger spaces, but not necessarily learn *what survives criticism*. This points to structure as load-bearing for epistemic progress specifically, not just capability.

**3. Invert the premise: suppose substrate is the main bottleneck.**  
Then scaling alone should reliably produce better explanations, better problem selection, and more robust error correction. But this is not generally what follows. Scale improves many competencies, yet often in ways that remain brittle, hard to direct, and weakly cumulative. The missing ingredient is not raw pattern capacity but organized processes for conjecture and refutation.

**4. Replace “promotion” with its opposite.**  
Imagine a system that forms hypotheses but promotes them by popularity, confidence, or immediate reward instead of surviving criticism. Epistemic quality degrades even if the substrate is strong. This reveals that **selection criteria** are central: bad ideas do not disappear automatically; they must be systematically exposed and filtered out.

What survives all these variations is this: epistemic progress is not mere production of candidate answers but **an evolving system for error elimination**. The substrate matters because criticism, search, and memory all cost compute. But once enough substrate exists to support rich variation and testing, returns shift from “more tokens, parameters, and FLOPs” to “better institutions of thought” inside the system.

So the illuminating distinction is not “hardware vs software” in the ordinary sense, but **capacity vs epistemology**. Compute enables; structure decides. If you want AI that accumulates knowledge rather than just outputs, the leverage lies above the substrate: in how it generates bold conjectures, preserves explanatory context, subjects ideas to genuine criticism, and promotes only what survives.

## Questions

1. If model size, training data, and inference compute are all held fixed, can the conjecture still explain a substantial gain in epistemic progress solely by adding persistent memory of failed hypotheses and explicit adversarial criticism loops? — **yes**
2. If we remove the threshold claim and say compute remains the dominant bottleneck at all scales, does the conjecture lose its explanation for why large but weakly self-critical systems become more fluent without becoming proportionally more cumulative or corrigible? — **yes**
3. If a system has massive scale but no mechanism for preserving explanatory gains across episodes, does the conjecture predict that epistemic progress will remain bottlenecked even when candidate-generation throughput is very high? — **yes**
4. If hypothesis promotion is changed from 'survives criticism' to 'maximizes confidence, popularity, or short-term reward,' does the conjecture still predict robust accumulation of knowledge, or would that destroy the claimed explanation of epistemic progress? — **no**
5. If better problem decomposition alone, under fixed substrate, could not improve performance unless compute also increased, would that directly undermine the conjecture's claim that structure is a genuine bottleneck? — **yes**
6. If scaling by itself reliably produced better error correction, better problem selection, and retention of refuted ideas without any architectural changes, would that falsify the conjecture's claim that these are primarily structural functions? — **yes**
7. If the system can generate many candidate ideas but cannot explicitly record why previous ideas failed, does the conjecture require that epistemic progress stall despite abundant search capacity? — **yes**
8. If criticism is present but memory is absent, does the conjecture treat memory as load-bearing rather than optional for cumulative epistemic progress? — **yes**
9. If we replace explicit conjecture-and-refutation with pure next-token imitation at much larger scale, does the conjecture predict that explanation quality will still plateau in the specific sense of failing to learn what survives criticism? — **yes**
10. If the substrate is only just below the claimed threshold for rich variation and testing, does the conjecture require that structural improvements have sharply limited effect until that threshold is crossed? — **no**

## Candidate Problems

- Where is the threshold at which substrate stops being the dominant bottleneck and structural epistemic architecture becomes the main limiter? The conjecture depends on a phase transition-like claim ('compute is load-bearing up to a threshold'), but it leaves unresolved how to define, detect, or measure that threshold across tasks, domains, and model classes. (score: 0.96)
- What operational metrics distinguish genuine epistemic progress from mere capability gains or fluent correlation search? The conjecture assumes that better conjecture-criticism-memory-selection structure yields knowledge accumulation, but it remains open how to measure 'surviving criticism,' explanatory improvement, cumulative error elimination, and preservation of failed ideas in a way that is empirically testable. (score: 0.98)
- Which structural components are actually causally decisive, and how do they interact as a system? The conjecture groups conjecture, criticism, memory, and selection together, but it is unresolved whether one is the main leverage point, whether they are complementary only in certain architectures, or whether some substrate scaling can substitute for missing structure under particular conditions. (score: 0.93)
