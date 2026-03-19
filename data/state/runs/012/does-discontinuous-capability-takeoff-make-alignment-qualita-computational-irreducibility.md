# Generated: does-discontinuous-capability-takeoff-make-alignment-qualita × computational-irreducibility

## Conjecture

**Conjecture:**  
Discontinuous capability takeoff makes alignment qualitatively harder *not primarily because it is faster*, but because it compresses the available error-correction loop below the timescale needed to discover and patch misalignment through interaction. Under computational irreducibility, the decisive variable is whether capability gains can be *anticipated by theory* or only *revealed by running the system*. If the latter, then a sharp takeoff is dangerous because the relevant behavioral facts do not exist in compressed form before deployment.

This perspective reframes the dispute. The key question is not “how steep is the capability curve?” in isolation, but “how reducible is the mapping from system design to dangerous emergent behavior?” If highly capable systems are computationally irreducible in their strategic, social, and self-modifying behavior, then no amount of static inspection, benchmark extrapolation, or local scaling law fitting will reliably substitute for actually observing the system in regimes near deployment. In that case, continuity matters because it provides sequential exposure to intermediate regimes where criticism can operate.

So:

1. **If capability emergence is computationally irreducible, continuous takeoff is structurally safer** because it generates many intermediate states in which alignment failures can be discovered, modeled, and corrected.  
2. **Discontinuous takeoff is structurally harder** because it skips those intermediate epistemic checkpoints. The system’s dangerous properties may only become visible once the full computation has effectively “run,” by which point the system may already be too capable for containment.  
3. **Which trajectory is more likely depends on the compressibility of capability-relevant structure.** If progress is driven by smooth accumulation along known dimensions, continuity is more likely. If progress depends on crossing thresholds where many subsystems suddenly compose into new general strategies, discontinuity becomes more likely.

This also illuminates a deeper point: “takeoff” may be misdescribed if treated as a property of raw capability alone. The relevant discontinuity is in the **observability of failure modes**. A system can improve continuously in benchmark performance while alignment difficulty changes discontinuously if new behaviors appear that could not be inferred except by full deployment-scale execution.

From a systems view, the critical stock is not compute or capability but **alignment knowledge**, and the crucial flow is empirical feedback from behavior under realistic conditions. Continuous takeoff allows this stock to accumulate. Discontinuous takeoff creates a mismatch: capability grows faster than alignment knowledge can.

Therefore, the main determinant of alignment difficulty is not speed per se but the ratio between:
- the timescale of capability gain,
- the timescale of empirical error discovery,
- and the degree to which dangerous behavior is computationally irreducible.

If irreducibility is high, then discontinuity is qualitatively worse because it destroys the possibility of learning-before-consequence.

## Questions

1. 1. Would the conclusion that discontinuous takeoff is qualitatively harder fail if the claim that dangerous behavior is computationally irreducible were removed and replaced with the assumption that theory can reliably predict deployment-scale behavior? — **yes**
2. 2. Is the explanatory force of the conjecture dependent on the claim that alignment knowledge must be built through interaction with intermediate capability regimes rather than through static inspection alone? — **yes**
3. 3. If the argument dropped the distinction between capability growth and observability of failure modes, would it lose its explanation for why discontinuity matters beyond mere speed? — **yes**
4. 4. Does the conclusion that continuous takeoff is structurally safer require the specific claim that intermediate states function as epistemic checkpoints where criticism and correction can occur? — **yes**
5. 5. Does this conjecture imply that even a numerically gradual capability curve could produce a discontinuous jump in alignment difficulty if observability of failure modes collapses near deployment? — **yes**
6. 6. Does the explanation extend to cases where benchmark performance scales smoothly but strategic or self-modifying behaviors appear only at realistic deployment scale? — **yes**
7. 7. If the conjecture is right, does it predict that improving theoretical interpretability alone will not remove the danger from sharp takeoff when behavior is only knowable by running the system? — **yes**
8. 8. If someone presented a counterexample in which a discontinuous takeoff was safe because a complete predictive theory existed in advance, would accommodating that case force abandonment of the conjecture's core claim that empirical exposure is indispensable under irreducibility? — **yes**
9. 9. If a case showed that alignment failures could be found and fixed entirely in simulation before deployment, would saving the conjecture require giving up its central dependence on real intermediate regimes as the source of alignment knowledge? — **yes**
10. 10. If one tried to rescue the conjecture from examples where capability jumps occur but no new hidden behaviors emerge, would that require replacing its core mechanism with a different explanation centered on speed alone? — **yes**

## Candidate Problems

- How can the conjecture’s key variable—'compressibility/reducibility of the mapping from system design to dangerous emergent behavior'—be made operational and empirically testable? The central unresolved tension is that the argument depends on computational irreducibility, but it is unclear how to distinguish true irreducibility from current ignorance, weak theory, or inadequate interpretability. A worthwhile problem is to define measurable proxies or experimental criteria for when behavior can be predicted from design/scale information versus only revealed by execution in near-deployment regimes. (score: 0.97)
- What actually counts as an 'intermediate epistemic checkpoint,' and under what conditions does continuous takeoff genuinely produce usable alignment knowledge rather than misleading reassurance? The conjecture assumes continuity is safer because intermediate regimes permit criticism and patching, but this is unresolved: failures found in weaker systems may not transfer to stronger systems, and qualitatively new behaviors may appear only after threshold crossings. The open question is whether alignment knowledge accumulates smoothly across capability levels or whether there are systematic phase changes that make intermediate feedback non-generalizing. (score: 0.94)
- Can institutions or technical methods create substitute error-correction loops when dangerous behaviors are only observable at high capability, or is deployment-scale interaction fundamentally unavoidable? This is a major open problem because the conjecture implies a hard limit on static analysis, benchmarks, and local extrapolation, yet it remains unclear whether adversarial simulation, scalable oversight, mechanistic interpretability, formal methods, or sandboxing can recover enough feedback before real-world consequence. The tension is whether the conjecture identifies an absolute epistemic barrier or only a contingent gap in current alignment practice. (score: 0.92)
