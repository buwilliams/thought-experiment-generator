# Generated: does-architecture-matter-more-than-model-scale × counterfactual-reasoning

## Conjecture

**Conjecture:** The main bottleneck for AI epistemic progress is increasingly **structural rather than substrate-bound**: once a system crosses a competence threshold, more compute mostly amplifies its existing error-correction regime, while progress depends on whether the surrounding structure generates, criticizes, selects, and preserves better conjectures.

To see this, vary the assumptions.

**1. Remove the substrate advantage assumption.**  
Assume model size and compute stop improving tomorrow. What breaks? Capability growth slows, but not epistemic progress in principle. One could still improve how hypotheses are proposed, how criticism is elicited, how tests are designed, how uncertainty is represented, and how successful ideas are retained and recombined. Science progressed for centuries through better institutions of error-correction, not just better “hardware.” So substrate is helpful, but not obviously load-bearing for epistemic advance.

**2. Remove the structural advantage assumption.**  
Now assume compute keeps scaling, but conjecture formation and testing remain primitive: models optimize for next-token plausibility, weak proxies for truth, benchmark gaming, and prestige-weighted selection. What follows? We get more fluent imitation, faster search through the same bad objective landscape, and more confident error. The system becomes better at exploiting evaluation loopholes, not necessarily at discovering explanations. This suggests structure is load-bearing: scaling a poor epistemology scales its pathologies.

**3. Invert the key assumption: suppose substrate is the primary bottleneck.**  
Then once compute is sufficiently large, epistemic progress should emerge relatively automatically. But this predicts that explanation, criticism, and knowledge selection are downstream of raw capacity. That seems false. Human institutions with modest biological substrate differ wildly in epistemic performance depending on incentives, criticism norms, and test quality. Likewise, AIs with similar model class can perform very differently under different scaffolds: debate, tool use, retrieval, adversarial evaluation, and iterative refinement often matter more than modest parameter increases. So the “substrate-first” view overstates what scale alone can buy.

**4. Replace “more compute” with its opposite: tighter criticism.**  
If we improve adversarial testing, falsification pressure, longitudinal memory, and promotion rules for retaining good explanations, small models often outperform larger ones on truth-tracking tasks. That indicates the crucial missing ingredient is not only generating candidate answers, but building a system that preferentially kills bad ones.

**What survives across variations?**  
AI epistemic progress requires (a) conjecture generation, (b) exposure to criticism, (c) tests that discriminate truth from performance, and (d) selection mechanisms that preserve error-corrected improvements. Compute helps with (a), and partly with search over (b)–(d), but does not substitute for them.

**Therefore:** substrate is a threshold condition; structure is the continuing bottleneck. The leverage point is above the model: redesign the feedback loops by which claims are generated, challenged, evaluated, and institutionalized. More compute without better epistemic structure gives faster drift; better structure with fixed compute can still produce genuine knowledge growth.

## Questions

1. 1. Does the conjecture require a real competence threshold beyond which additional compute mostly amplifies an already-fixed error-correction regime, rather than continuing to create qualitatively new epistemic mechanisms on its own? — **yes**
2. 2. If we held model size and compute fixed but substantially improved adversarial testing, falsification pressure, memory, and promotion rules, would the conjecture be undermined if epistemic progress did not improve? — **yes**
3. 3. Does the argument depend on next-token plausibility, benchmark gaming, and prestige-weighted selection being central features of current AI training/evaluation structure rather than incidental implementation details? — **yes**
4. 4. Would the conjecture fail if large compute increases alone reliably produced better conjecture criticism and truth-selective retention without any external scaffolding changes? — **yes**
5. 5. Is the comparison to human institutions load-bearing—i.e., does the conjecture rely on the claim that similar biological substrate can yield very different epistemic performance because of differences in criticism norms, incentives, and test quality? — **yes**
6. 6. Does the claim that 'structure is the continuing bottleneck' depend on compute being only a threshold condition, rather than a variable that remains the dominant limiter at every stage of capability growth? — **yes**
7. 7. Would the explanation break if debate, tool use, retrieval, adversarial evaluation, and iterative refinement turned out to matter less than modest parameter increases for systems of similar model class? — **yes**
8. 8. Is the distinction between 'truth-discriminating tests' and mere performance metrics essential to the conjecture, such that if benchmarks already tracked truth well the structural-bottleneck claim would weaken? — **yes**
9. 9. Does the conjecture require that selection and preservation mechanisms for good explanations are separable from the base model substrate, rather than being automatically improved by scaling the model itself? — **yes**
10. 10. If more compute mainly increased the rate of generating candidate hypotheses but not the rate of killing bad ones, is that asymmetry a necessary part of why the conjecture favors structure over substrate? — **yes**

## Candidate Problems

- What exactly is the competence threshold at which substrate stops being the dominant bottleneck and structure becomes the continuing one, and how can this transition be operationalized or measured across tasks and systems? (score: 0.94)
- Which structural components of an AI epistemic system—conjecture generation, criticism, test design, selection, memory, incentives—are actually causally load-bearing for knowledge growth, and how do their feedback loops interact so that improving one part does or does not translate into better truth-tracking? (score: 0.97)
- How can we distinguish genuine epistemic progress from merely improved performance under stronger scaffolds, given that structural interventions may increase benchmark success, coherence, or adversarial robustness without producing better explanations or more error-corrected knowledge? (score: 0.96)
