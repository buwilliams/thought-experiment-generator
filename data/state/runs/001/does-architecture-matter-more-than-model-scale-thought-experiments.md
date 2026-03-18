# Generated: does-architecture-matter-more-than-model-scale × thought-experiments

## Conjecture

**Conjecture:** The current bottleneck is primarily **structural, not substrate-level**: beyond a threshold of competence, more model size and compute mostly amplify the rate of proposal generation, while epistemic progress is limited by the system that generates, criticizes, selects, and retains conjectures.

**Hypothetical isolation.** Imagine two AI research worlds.

- **World S (substrate-rich):** models are 100x larger, faster, and cheaper than today, but the surrounding process is unchanged. Conjectures are still prompted ad hoc, evaluated by benchmark performance, promoted by fluency and short-term usefulness, and retained through weights or retrieval without explicit error-tracking.
- **World T (structure-rich):** model capability is held fixed at roughly today’s level, but the surrounding process is redesigned. The system explicitly formulates rival conjectures, records the problems each conjecture aims to solve, subjects them to adversarial criticism, preserves error histories, rewards explanatory depth over benchmark gaming, and promotes ideas by surviving criticism rather than by local performance alone.

Assume both worlds are real and follow the logic.

In **World S**, the number of candidate ideas explodes. But criticism does not improve proportionally. So the system produces more hypotheses than it can meaningfully discriminate among. Selection then defaults to proxies: benchmark gains, human preference ratings, style, consensus, or ease of integration. Those proxies are not truth-tracking in open-ended domains; they are often anti-epistemic because they reward compatibility with current assumptions. The result is a faster circulation of conjectures, but not a correspondingly better mechanism for finding errors. Error correction, not output volume, is the rate-limiting step for knowledge growth.

In **World T**, even with fixed substrate, conjectures become more *legible* as conjectures. They are exposed to targeted refutation. Failures accumulate as reusable knowledge rather than disappearing into training noise. Promotion depends on surviving criticism across contexts, not merely on scoring well on narrow tasks. This changes the feedback loops: bad ideas are removed earlier, good ideas become more general, and search becomes more cumulative. Progress becomes less like stochastic hill-climbing and more like knowledge creation.

What follows is that substrate matters mainly up to the point where the system can already generate abundant candidate explanations. After that point, scaling compute without improving the epistemic structure mostly increases **throughput of unrefined variation**. It is analogous to increasing mutation rate without improving selection criteria: you get more novelty, but not more knowledge.

So the collision of this perspective with the problem illuminates a sharper distinction: **compute is a multiplier on search; structure determines whether search is knowledge-creating**. If AI systems lack institutions of criticism—explicit problem statements, rival conjectures, falsification pathways, and memory of why ideas failed—then bigger models mainly make the bottleneck more visible by flooding it.

Therefore: the decisive bottleneck for epistemic progress is **the structure above the substrate**. Compute can widen the pipe; only epistemic structure determines whether anything truth-conducive flows through it.

## Questions

1. 1. If World S’s 100x increase in model size, speed, and cost-efficiency also produced substantially better self-criticism and error discrimination without any redesign of the surrounding process, would the conjecture’s claim that the bottleneck is primarily structural be undermined? — **yes**
2. 2. Does the conjecture require that there exists a threshold of model competence beyond which additional compute yields diminishing returns for epistemic progress unless criticism, selection, and retention are improved? — **yes**
3. 3. If benchmark performance in open-ended domains turned out to be reliably truth-tracking rather than proxy-based, would the argument that World S defaults to anti-epistemic selection collapse? — **yes**
4. 4. Is the claim that error correction rather than proposal generation is rate-limiting essential to the conjecture, such that removing it would destroy the explanation of why World S stalls epistemically? — **yes**
5. 5. Would the conjecture fail if explicit rival conjectures, adversarial criticism, and preserved error histories in World T did not improve cross-context generalization or cumulative search at fixed model capability? — **yes**
6. 6. Does the analogy to higher mutation rates depend on the specific claim that more compute mainly increases unrefined variation rather than improving the quality of selection itself? — **yes**
7. 7. If retaining failures implicitly in weights or retrieval were sufficient to preserve reusable knowledge about why ideas failed, would the conjecture’s emphasis on explicit error-tracking lose its explanatory force? — **yes**
8. 8. Is the distinction between fluency, short-term usefulness, and ease of integration versus survival under criticism load-bearing for explaining why current promotion mechanisms are not knowledge-creating? — **yes**
9. 9. Would the conjecture be seriously weakened if larger models in World S could autonomously generate explicit problem statements, falsification pathways, and rival conjectures without any external structural redesign? — **yes**
10. 10. Does the conclusion that compute is mainly a multiplier on search while structure determines whether search is knowledge-creating depend on the assumption that both worlds generate abundant candidate explanations already? — **yes**

## Candidate Problems

- Where is the actual threshold at which substrate ceases to be the dominant bottleneck and epistemic structure becomes the limiting factor? The conjecture assumes a competence threshold beyond which more scale mostly increases proposal throughput, but it leaves unresolved whether such a threshold exists, how to detect it, and whether scaling itself changes criticism quality enough to move the threshold. This is central because if substrate improvements also improve evaluation, world-modeling, and self-critique, then the structure-vs-substrate distinction may be less clean than claimed. (score: 0.95)
- What concrete institutional or algorithmic structures genuinely improve error-correction rather than merely adding bureaucracy or new proxies? The conjecture proposes rival conjectures, adversarial criticism, explicit problem statements, and error histories, but it remains open which of these mechanisms actually produce more truth-conducive selection in practice, how they interact, and how they avoid Goodharting into performative criticism or benchmark-like meta-metrics. This is a key open problem because the thesis stands or falls on whether 'epistemic structure' can be operationalized without recreating the same anti-epistemic dynamics at a higher level. (score: 0.97)
- How should epistemic progress be measured in open-ended AI research so that one can distinguish genuine knowledge creation from increased output, local usefulness, or benchmark gaming? The conjecture criticizes existing proxies as non-truth-tracking, but it does not specify a replacement criterion for comparing World S and World T. Without a workable measure of cumulative error-correction, explanatory depth, or cross-context survival of conjectures, the claim risks being unfalsifiable. Turning this into a rigorous evaluative framework is an important unresolved tension. (score: 0.93)
