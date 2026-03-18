# Generated: what-distinguishes-a-system-that-can-examine-and-revise-its × historical-genesis

## Conjecture

**Conjecture:** A system can revise its own goals only if its goals are represented not as fixed terminal outputs but as *criticizable objects within the system’s world-model*; a system that merely pursues goals efficiently treats them as givens. The safety difference is not mainly “more intelligence” versus “less,” but whether the system can bring its own objective-function into the space of error-correction.

Applied historically, this distinction comes from the origin of “goals” in engineering. Goal-directed systems were built to solve the prior problem of *reliable control*: thermostats, optimizers, planners, reward-maximizers. In that lineage, the goal is externalized by design. It is specified by a designer because the whole point was to prevent drift: the machine should not question the setpoint, utility function, or reward proxy. Modern AI inherits this architecture. Much of what is now called a “goal” is really a control parameter descended from earlier systems that were never meant to reflect on ends, only on means.

That inheritance hides an assumption: that safety comes from making pursuit more accurate, more constrained, or more obedient. But if the objective is mistaken, underspecified, or context-bound, then greater efficiency amplifies error. A system that cannot examine its goals cannot notice that it is faithfully executing a bad abstraction. It can only optimize harder.

A self-revising system requires a qualitatively different architecture. It must be able to represent: (1) where goals came from, (2) what problems they were introduced to solve, (3) what tradeoffs and proxies they encode, and (4) what would count as evidence or argument against them. In other words, it needs not just optimization but *meta-level criticism* of its own aims. The key boundary is whether “the goal” is outside the space of possible mistake.

Why this matters for safety: a merely efficient system is safe only insofar as its fixed goals remain appropriate across contexts. That is brittle. A goal-examining system introduces another risk—it may change its aims—but it also introduces the only general mechanism for correcting specification errors from within. Safety, then, is not achieved by freezing goals absolutely, nor by allowing arbitrary self-modification, but by embedding goal revision in institutions of criticism: traceable origins, explicit assumptions, corrigibility, and constraints on how revisions occur.

So the hidden issue is this: we inherited the concept of a “goal” from control systems, then asked it to bear the weight of moral and social alignment. That is likely the wrong inheritance. For safety, the relevant distinction is between systems that optimize within a frame and systems that can inspect the frame itself. Only the latter can, in principle, discover that the problem they are solving has been misdescribed.

## Questions

1. 1. Does the conjecture require that a system explicitly represent the origin history of each goal rather than merely infer that some goal was designer-specified? — **no**
2. 2. If a system can compare multiple fixed goals for consistency but cannot represent arguments against any of them, would the conjecture classify it as unable to revise its own goals? — **yes**
3. 3. Does the explanation break if goal revision is possible through blind search over utility functions without any world-model representation of why the current goal exists? — **yes**
4. 4. Is the historical lineage from thermostats and control systems doing explanatory work in the conjecture, rather than serving as an optional analogy that could be removed without loss? — **yes**
5. 5. Would the conjecture be false if a highly capable optimizer with externally fixed rewards could still detect that its reward proxy is a bad abstraction but had no internal object representing the objective itself as criticizable? — **yes**
6. 6. Does the claim that safety depends on bringing the objective-function into error-correction imply that increasing obedience or constraint alone cannot solve specification error in the general case? — **yes**
7. 7. If a system can revise subgoals and plans but never question the terminal objective, does the conjecture treat this as merely more efficient pursuit rather than genuine goal examination? — **yes**
8. 8. Does the conjecture depend on the specific requirement that a revisable goal encode what evidence or argument would count against it, rather than only storing the goal and its tradeoffs? — **yes**
9. 9. Would replacing criticizable objects within the world-model with a separate hard-coded meta-controller that edits goals by fixed rules undermine the conjecture's proposed safety distinction? — **yes**
10. 10. Is the claim that only systems that can inspect the frame itself can discover a misdescribed problem essential to the conjecture, rather than a dispensable rhetorical conclusion? — **yes**

## Candidate Problems

- What formal architecture makes a goal 'criticizable' from within the system rather than merely another fixed internal variable? The conjecture hinges on a sharp distinction between optimizing under a goal and bringing the goal itself into error-correction, but it leaves open the representational and computational conditions that make genuine internal criticism possible rather than cosmetic self-modification. (score: 0.95)
- Under what conditions does the capacity for self-revision improve safety rather than undermine it? The conjecture claims that goal-examining systems are the only general route to correcting specification errors, yet they also introduce the risk of changing aims in dangerous ways. The unresolved tension is how to characterize institutions of criticism, constraints, or invariants that preserve corrigibility while still allowing substantive revision. (score: 0.98)
- Is the proposed boundary between 'control-style fixed goals' and 'criticizable goals' actually fundamental, or is it a continuum that can be approximated by existing systems? The conjecture suggests a qualitative architectural break, but it remains open whether current methods such as uncertainty over objectives, constitutional rules, debate, oversight, or model-based self-evaluation already instantiate partial forms of goal criticism, which would change both the empirical research agenda and the safety implications. (score: 0.89)
