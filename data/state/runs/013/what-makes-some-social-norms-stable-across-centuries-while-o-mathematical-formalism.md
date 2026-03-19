# Generated: what-makes-some-social-norms-stable-across-centuries-while-o × mathematical-formalism

## Conjecture

**Conjecture:**  
A social norm is stable across centuries when it is a **high-redundancy equilibrium** in a social coordination system: many local interactions, institutions, incentives, and identity signals all independently reproduce the same behavior, so that perturbing any one channel leaves the norm’s effective reproduction rate above replacement. A norm collapses rapidly when it is a **low-redundancy equilibrium** whose persistence depends on a small number of tightly coupled supports; once pressure pushes the system past a threshold, positive feedback reverses and the norm’s reproduction rate falls below replacement.

In mathematical terms, model a norm by a state variable \(x_t\), the share of actors enacting or endorsing it. Its persistence depends on an update map

\[
x_{t+1} = F(x_t; I, S, P, C),
\]

where \(I\) = institutions, \(S\) = sanctions/rewards, \(P\) = material payoffs, and \(C\) = cultural meaning/identity. Stability is not merely “many people currently follow it,” but that \(x^*\) is an **attractor**: small shocks return to it. The key quantity is the norm’s **effective reproduction number** \(R\): how many new adherents one adherent, via imitation, enforcement, education, and institutional embedding, tends to generate. Long-lived norms satisfy \(R>1\) across a wide range of perturbations.

What distinguishes durable from fragile norms is therefore not age, but **structural robustness**:

- **Multiple realization:** the same norm is reproduced by family, law, ritual, market incentives, and status competition simultaneously.  
- **Cross-scale alignment:** micro motives and macro institutions point the same way.  
- **Low observability of defections or high cost of deviation:** defection cannot easily spread.  
- **Identity coupling:** violating the norm changes not just action but self-conception, increasing restoring forces.  

A fragile norm may appear strong when \(x_t\) is high, but if its support matrix is low-rank—say, mostly dependent on coercion or prestige alone—then pressure on that single dimension changes the sign of feedback. Once enough actors defect, expectations shift, sanctions weaken, imitation flips direction, and collapse becomes discontinuous: a phase transition rather than gradual decay.

The invariant here is **coordination load**: every society must solve recurring problems of trust, mating, inheritance, violence, childrearing, etc. Norms tied closely to enduring coordination problems have more chances to be rederived and rebuilt even after shocks. Norms attached mainly to contingent technologies or prestige hierarchies lack that invariant substrate and are more likely to vanish when conditions change.

So the mathematical illumination is: **centuries-long stability tracks basin size, redundancy, and alignment of feedback loops; rapid collapse tracks threshold dynamics in sparsely supported equilibria.** The same pressure can have little effect on a norm with a broad basin of attraction and catastrophic effect on one sitting near a bifurcation point.

## Questions

1. 1. Is the claim that centuries-long stability depends on a norm being an attractor with an effective reproduction rate above replacement necessary for the conjecture to explain why a norm returns after small shocks rather than merely remains common for a while? — **yes**
2. 2. If the explanation removed the idea of redundancy across institutions, incentives, identity, and local interactions, would it lose its account of why disrupting one support channel often fails to destabilize long-lived norms? — **yes**
3. 3. Is cross-scale alignment between micro motives and macro institutions required for the conjecture to explain durable stability, or could the conclusion still follow if those levels systematically pulled in opposite directions? — **yes**
4. 4. If the invariant of coordination load were dropped, would the conjecture still explain why some norms can be rederived after major disruptions rather than only why they persist during uninterrupted periods? — **no**
5. 5. Does the conjecture imply that a newly created norm could become highly durable quickly if it acquires broad redundancy and a large basin of attraction, even without great age? — **yes**
6. 6. Does the explanation extend to predicting that two norms facing the same external pressure can diverge sharply in outcome because one is near a threshold and the other is buffered by multiple reinforcing channels? — **yes**
7. 7. Does the conjecture illuminate why norms tied to enduring problems like trust or childrearing should recur in different societies even when the specific institutions and symbols differ? — **yes**
8. 8. If a norm remained stable for centuries despite relying mainly on one coercive institution with little identity coupling or multiple realization, would saving the conjecture require abandoning the claim that low-redundancy equilibria are fragile? — **yes**
9. 9. If a norm collapsed rapidly even though family, law, markets, ritual, and identity all seemed to support it independently, would explaining that case force a major revision of the conjecture's core link between redundancy and robustness? — **yes**
10. 10. If defections spread quickly in a norm that had high observability of violations and low direct cost of deviation but still showed strong long-term stability, would rescuing the explanation require gutting its restoring-force story rather than adding a minor qualification? — **yes**

## Candidate Problems

- How can 'effective reproduction number' and 'redundancy' for norms be defined and measured non-circularly in real societies? The conjecture is powerful only if R, basin size, support-matrix rank, and cross-scale alignment can be operationalized independently of observed persistence, rather than inferred from it after the fact. (score: 0.97)
- What is the correct dynamical form linking institutions, sanctions, payoffs, and identity to threshold collapse versus gradual change? The conjecture posits attractors, bifurcations, and phase transitions, but it remains open which classes of update maps F actually generate these behaviors robustly, under what network structures, and when multiple weak supports substitute for versus complement one another. (score: 0.94)
- Is 'coordination load' truly the invariant substrate explaining centuries-long norm recurrence, or do some durable norms persist for reasons not reducible to recurring coordination problems? This raises the open question of whether the conjecture overgeneralizes by treating norm durability as functional re-derivation, when some norms may survive through path dependence, power asymmetries, symbolic lock-in, or memetic self-protection even if they solve no enduring coordination problem. (score: 0.91)
