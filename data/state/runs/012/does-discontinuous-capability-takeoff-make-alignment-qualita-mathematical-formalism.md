# Generated: does-discontinuous-capability-takeoff-make-alignment-qualita × mathematical-formalism

## Conjecture

**Conjecture:**  
The qualitative difficulty of alignment is governed less by whether capability growth is *continuous* or *discontinuous* in clock time, and more by the ratio between three coupled rates:

- **capability gain** \( \dot{C} \),
- **alignment/control improvement** \( \dot{A} \),
- **governance/response adaptation** \( \dot{G} \).

A “discontinuous takeoff” becomes qualitatively harder precisely when there exists an interval in which \( \dot{C} \gg \dot{A}, \dot{G} \), causing the system to cross one or more **control thresholds** before corrective feedback can act.

### Mathematical structure
Let capability \(C(t)\) be a state variable, and let alignment adequacy be modeled as a margin
\[
M(t)=A(t)+G(t)-R(C(t)),
\]
where \(R(C)\) is the required control/alignment burden induced by capability level \(C\). The system is safe while \(M(t)>0\). Takeoff difficulty is then about whether trajectories in state space cross the boundary \(M=0\).

The key invariant is not smoothness of \(C(t)\) itself, but whether the trajectory preserves positive margin:
\[
\frac{dM}{dt}=\dot{A}+\dot{G}-R'(C)\dot{C}.
\]
If \(R'(C)\dot{C}\) exceeds the combined corrective rates for long enough, margin collapses. A discontinuity matters because it can approximate an impulse in \(C\), making \(M\) jump negative before any feedback loop updates.

### What follows
1. **Discontinuous takeoff is qualitatively harder when control is thresholded.**  
If deployment power, autonomy, replication ability, strategic advantage, or deception capability switch on nonlinearly at some \(C^*\), then \(R(C)\) is convex or contains step-like increases. Crossing \(C^*\) quickly is dangerous even if average growth is moderate.

2. **Continuous takeoff can be equally hard if feedback is too slow.**  
A smooth but exponential \(C(t)\) with weak monitoring, slow interpretability, and institutional lag can still produce \(M<0\). So “continuous” does not imply “manageable”; it only gives more opportunities for error correction if those channels are actually effective.

3. **Which trajectory is more likely depends on the feedback topology of the capability-production system.**  
Discontinuous trajectories are favored when capability growth is driven by strong positive feedback loops: AI improving AI research, parallelizable scaling, cheap deployment, and winner-take-most incentives. Continuous trajectories are favored when bottlenecks dominate: compute supply, energy, experimental validation, data acquisition, hardware cycles, and human organizational integration.

### Illumination
The right distinction is therefore not **continuous vs discontinuous**, but **feedback-dominated vs bottleneck-dominated** growth. Alignment becomes qualitatively harder when the system enters a regime where capability can outrun correction across a control boundary. The takeoff shape matters only insofar as it changes the probability of crossing that boundary before negative feedback can engage.

## Questions

1. 1. Is the conclusion that discontinuous takeoff is harder only when capability outruns correction dependent on the specific margin definition M equals A plus G minus R of C rather than on a looser comparison of capability and alignment rates alone? — **no**
2. 2. If the governance adaptation term G were removed from the model, would the conjecture lose its explanation of why institutional lag can make a smooth takeoff qualitatively hard? — **yes**
3. 3. Is the claim that control thresholds are represented by convexity or step-like increases in R of C necessary for explaining why some discontinuous jumps are especially dangerous rather than merely fast? — **yes**
4. 4. If the explanation did not distinguish feedback-dominated from bottleneck-dominated growth, would it fail to answer what determines whether discontinuous or continuous trajectories are more likely? — **yes**
5. 5. Does the conjecture imply that a continuous takeoff with sustained exponential capability growth and slow monitoring can become qualitatively harder even when the problem statement only foregrounds discontinuity? — **yes**
6. 6. Does the rate-ratio framework illuminate why deployment power, replication ability, or deception capability might create danger at specific capability levels rather than only addressing overall speed of progress? — **yes**
7. 7. Does the feedback-topology part of the conjecture extend to predicting which economic or technical conditions would shift a system from bottleneck-dominated growth toward discontinuous takeoff? — **yes**
8. 8. If a counterexample showed a discontinuous jump in capability that remained safe because alignment and governance had large prebuilt reserves, would saving the conjecture require abandoning the central claim that threshold crossing depends on margin rather than on discontinuity itself? — **no**
9. 9. If one found a case where R of C rises smoothly with no threshold yet a discontinuous takeoff is still much harder, would preserving the conjecture force a major revision of its emphasis on control thresholds as the source of qualitative hardness? — **no**
10. 10. If a system with strong positive feedback loops nevertheless exhibits continuous growth because of external compute rationing, would explaining this within the conjecture require gutting the feedback-versus-bottleneck topology distinction rather than adding a minor qualification? — **no**

## Candidate Problems

- How should the required control burden R(C) and the control thresholds be modeled so that the margin M=A+G-R(C) tracks real alignment difficulty rather than merely redescribing it? In particular: is R primarily smooth, convex, step-like, path-dependent, or multi-dimensional; do qualitatively new capabilities create irreversible threshold crossings; and can these thresholds be operationalized or empirically estimated before deployment? (score: 0.96)
- Are A (alignment/control improvement) and G (governance/response adaptation) meaningfully separable from C (capability), or are they endogenously coupled so that capability growth simultaneously accelerates and degrades correction? The conjecture treats the key issue as a rate ratio, but an open question is whether advanced systems increase alignment research, monitoring, and governance capacity faster than they increase the burden R(C), or instead undermine oversight through opacity, strategic behavior, and deployment pressure. (score: 0.93)
- What is the correct dynamical model of feedback and delay in the socio-technical system: when do positive capability feedback loops dominate bottlenecks strongly enough that crossing M=0 becomes likely before corrective action can propagate? This includes unresolved questions about time delays, observability, hidden state, institutional lag, competitive incentives, and whether the relevant danger comes from a single fast takeoff interval, repeated near-threshold excursions, or gradual erosion of margin under uncertainty. (score: 0.91)
