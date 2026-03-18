# Generated: does-architecture-matter-more-than-model-scale × counterfactual-reasoning

## Conjecture

**Conjecture:** The main bottleneck for AI epistemic progress is *upstream structure*, not substrate. Compute and model size are necessary capacity multipliers, but they are not the load-bearing variable for sustained knowledge growth. What matters more is the system that generates, criticizes, selects, and preserves conjectures.

Vary the assumptions:

1. **Remove the “more substrate yields more epistemic progress” assumption.**  
Suppose model size and compute stop improving. What breaks? Speed, breadth, and perhaps raw performance on known benchmarks. But epistemic progress need not stop if the system above the model improves: better decomposition of problems, stronger falsification loops, better memory of errors, improved adversarial testing, and institutions that reward correction over fluent output. Knowledge can still grow by better criticism.

2. **Invert it: assume unlimited substrate but weak structure.**  
Now models are enormous and cheap to run, but conjectures are formed opaquely, tested weakly, and promoted by proxy metrics like benchmark scores, popularity, or confidence. What follows? We get more outputs, more imitation, and more persuasive error at scale. The system becomes better at producing candidate answers, not better at distinguishing truth from attractive falsehood. This looks like capability growth without proportional epistemic growth.

3. **Replace “intelligence is mainly in the model” with “intelligence is in the error-correcting system.”**  
Then the model is a component, like a microscope in science: useful, often transformative, but not itself the source of reliable knowledge. Reliable knowledge comes from structured variation plus criticism: generating rival hypotheses, exposing them to severe tests, retaining what survives, and making error legible. On this view, scaling helps mostly when the surrounding structure can exploit it.

What breaks under these variations reveals the load-bearing part. If structure is weak, added substrate mostly amplifies existing pathologies: hallucination, overfitting to benchmarks, reward hacking, and institutional prestige loops. If structure is strong, even modest substrate can produce cumulative gains because errors are found, localized, and not repeated.

So the deeper bottleneck is not “can the model think harder?” but “does the system know how to treat outputs as conjectures rather than answers?” This includes:
- mechanisms for adversarial criticism,
- environments that reward disconfirmation,
- memory that tracks failed ideas,
- promotion criteria tied to explanatory depth and robustness, not just performance,
- modular interfaces that let conjectures be compared and recombined.

**Therefore:** substrate is a rate limiter; structure is a direction-setter and truth-filter. When the two are confused, scaling appears to be epistemic progress because it improves behavior. But behavior is not knowledge. Knowledge grows where there are better error-correcting structures. The bottleneck is thus primarily structural, with compute acting as an amplifier of whatever epistemic regime sits above it.

## Questions

1. If benchmark scores and task performance kept rising with larger models while adversarial testing still failed to reduce repeated errors, would the conjecture still say the main bottleneck is structural rather than substrate? — **yes**
2. If a small-model system with explicit rival-hypothesis generation, error memory, and severe falsification loops consistently outperformed a much larger opaque model on discovering and retaining true explanations, would that be necessary for this conjecture to hold? — **no**
3. If unlimited compute plus current weak promotion criteria eventually produced reliable truth-tracking without adding new criticism mechanisms, would that directly undermine the conjecture? — **yes**
4. Does the conjecture require that hallucination, benchmark overfitting, and reward hacking are primarily consequences of weak upstream selection and criticism rather than of insufficient model capacity? — **yes**
5. If stronger models alone made their own errors legible and self-correcting without external adversarial structure, would the claim that intelligence mainly resides in the error-correcting system become dispensable? — **yes**
6. Is the distinction between behavior improvement and knowledge growth load-bearing here, such that if better behavior were accepted as sufficient evidence of knowledge, the conjecture would lose its force? — **yes**
7. Does the conjecture depend on failed-idea memory being indispensable, so that if systems could achieve cumulative epistemic progress without tracking prior errors, a central part would break? — **no**
8. If promotion by proxy metrics like confidence, popularity, or benchmark rank could be shown to correlate tightly with explanatory robustness in practice, would the argument against weak structure collapse? — **yes**
9. Is it essential to the conjecture that compute acts mostly as an amplifier of the existing epistemic regime, rather than sometimes changing the regime by itself? — **no**
10. If modular interfaces for comparing and recombining conjectures turned out unnecessary for sustained knowledge growth in AI systems, would that count against the claimed primacy of upstream structure? — **no**

## Candidate Problems

- How can 'epistemic progress' be operationalized independently of behavioral performance, so the conjecture can be tested rather than merely redescribed? The core unresolved tension is that the argument distinguishes knowledge from behavior, but without clear measures of cumulative error-correction, explanatory depth, retention of refuted ideas, and robustness under adversarial criticism, 'structure matters more than substrate' risks becoming unfalsifiable. (score: 0.97)
- What specific structural mechanisms are actually load-bearing, and under what conditions do they dominate substrate effects? The conjecture groups together adversarial criticism, memory of errors, institutional incentives, decomposition, and modular interfaces, but it remains open which of these are necessary, sufficient, or complementary, and whether there are threshold or interaction effects where additional compute again becomes the main bottleneck. (score: 0.94)
- Is there a principled causal account showing that stronger error-correcting structure can substitute for, rather than merely complement, model capability? A major open question is whether modest-substrate systems with strong criticism can reliably outperform high-substrate weak-structure systems on frontier knowledge creation, or whether substrate is often a hidden prerequisite for generating the diversity, abstraction, and testable conjectures that structure then filters. (score: 0.92)
