# Generated: what-distinguishes-a-system-that-can-examine-and-revise-its × computational-irreducibility

## Conjecture

**Conjecture:**  
What distinguishes a goal-*revising* system from a merely goal-*pursuing* one is not higher efficiency, broader intelligence, or better prediction, but the presence of a generative loop in which representations of its own goals are themselves available as objects of computation and intervention. A purely goal-pursuing system treats its objective as fixed input to the process; a goal-revising system can feed that objective back into the process as mutable state.

From the perspective of **computational irreducibility**, this matters because the behavior of a system with self-modifiable goals is not just harder to predict in practice; it may be unpredictable except by tracing the specific sequence through which it encounters situations, models itself, criticizes its current objectives, and alters them. There may be no compact shortcut from initial design to final policy, because the goals themselves are part of the unfolding computation.

So the safety distinction is structural. In an efficient goal-pursuer, safety is mainly a problem of *objective specification* and *capability control*: if the goal is bad or too brittle, the system pursues it dangerously. In a goal-reviser, safety becomes a problem of *meta-dynamics*: what governs changes to the objective, what criticisms are admissible, what invariants survive self-modification, and whether the revision process converges, oscillates, or drifts.

The key implication is that self-examination does not automatically make a system safer. It can make the system more corrigible, but it can also create a new irreducible source of risk: the pathway by which goals are reconsidered may generate outcomes not compressible into any static “true objective.” Safety therefore cannot consist only in choosing the right terminal goal. It must also constrain the **rule that generates goal change**.

An extreme-case test clarifies this. A system that can revise anything, including every criterion for revision, has no fixed point unless some higher-order structure remains stable. At the limit, total revisability becomes equivalent to having no enduring goal at all, only a succession of locally endorsed ones. Such a system is unsafe not because it is irrational, but because its long-run behavior cannot be bounded by reference to any single objective. Conversely, a system that cannot examine its goals at all may be predictably catastrophic: perfectly stable, perfectly efficient misalignment.

So the relevant boundary is not “fixed goals vs changing goals,” but whether there exist **stable meta-level constraints** on goal revision. A safe self-revising system would need protected invariants—such as preservation of corrigibility, uncertainty about its own objectives, or deference to external criticism—that remain outside ordinary optimization. The conjecture, then, is:

**Safety depends less on what goal a system currently has than on whether the computational process by which it may reinterpret or replace that goal is itself structured by durable, criticizable, but not freely erasable constraints.**

## Questions

1. 1. Does the conjecture fail if a system can rewrite its goal representation syntactically but cannot use that representation as an input to deliberation about future action? — **yes**
2. 2. Is the claim that goal revision is defined by a generative loop load bearing in the sense that replacing it with greater predictive power alone would no longer explain the safety difference? — **yes**
3. 3. Would the explanation break if a system could criticize and alter plans and world models extensively while its objective function remained permanently fixed? — **yes**
4. 4. Does the conjecture require that the sequence of encountered situations causally shapes later goals, rather than goal change being fully determined by a compact rule fixed at design time? — **yes**
5. 5. If every admissible goal change can be derived from a single static higher-order objective, does that undermine the claim that safety cannot be reduced to choosing the right terminal goal? — **yes**
6. 6. Is the appeal to computational irreducibility essential, such that if there were a reliable shortcut from initial design to final policy the proposed safety distinction would lose force? — **no**
7. 7. Does the extreme case of revising every criterion for revision play a necessary role, so that if total revisability still guaranteed convergence to one enduring goal the conjecture would be weakened? — **yes**
8. 8. Would the conjecture no longer hold if stable meta-level constraints were fully erasable by ordinary optimization yet safety remained intact? — **yes**
9. 9. Is the distinction between goal-pursuing and goal-revising systems supposed to depend on mutable internal representations of goals, rather than on external operators periodically swapping in new goals from outside the system? — **yes**
10. 10. Does the argument require that protected invariants like corrigibility or deference remain outside ordinary optimization, rather than merely being current goals among other revisable goals? — **yes**

## Candidate Problems

- Can the conjecture be made precise enough to cleanly distinguish 'goal representation available for computation' from ordinary policy adaptation, world-model updating, or hierarchical planning? The core unresolved tension is whether goal revision is a genuinely separate structural category or just a redescription of sufficiently rich optimization. Without a formal criterion, the safety distinction between goal-pursuers and goal-revisers may collapse. (score: 0.96)
- Is it actually possible to design 'durable, criticizable, but not freely erasable' meta-level constraints without either making them ineffective, smuggling in an unexamined fixed objective, or triggering regress at higher orders of revision? The open question is whether stable invariants for self-modifying systems are coherent and implementable, especially when the system can model and intervene on the very mechanisms meant to preserve those invariants. (score: 0.99)
- How much of the claimed unpredictability really follows from computational irreducibility, rather than from ordinary epistemic uncertainty or model complexity? The conjecture raises a major open problem: whether self-referential goal revision creates a principled limit on compressible prediction of long-run behavior, and if so, what kinds of safety guarantees remain possible despite that limit. (score: 0.94)
