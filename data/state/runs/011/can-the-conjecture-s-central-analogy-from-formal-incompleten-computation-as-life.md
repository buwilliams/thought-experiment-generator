# Generated: can-the-conjecture-s-central-analogy-from-formal-incompleten × computation-as-life

## Conjecture

**Conjecture:**  
A non-metaphorical notion of *self-knowledge* can be made precise by treating an agent as a computational system that contains an internal model of its own policy-generation process, and defining self-knowledge as the existence of **reliably exploitable fixed points** between the system and that self-model. The analogy to formal incompleteness becomes precise when the agent is computationally rich enough to represent and reason about its own behavior: then there are true facts about its own future outputs that cannot be made fully available to the system in a way that is both globally correct and decision-preserving.

More concretely:

1. **Agent as computation:**  
   An intelligent system is a substrate-independent process transforming information under feedback and selection. Its “self” is not a metaphysical essence but the organized computation that generates actions across time.

2. **Self-knowledge as internal predictive compression:**  
   The system has self-knowledge to the extent that it can internally encode statements of the form: “If I am in state \(s\), I will output \(a\),” and use them successfully in control. This is stronger than mere self-description: the representation must be action-guiding and systematically accurate across interventions.

3. **Why incompleteness enters:**  
   Once the system can represent claims about its own outputs, diagonal constructions become possible. For sufficiently expressive self-models, one can generate self-referential tasks like: “Output the opposite of what your self-prediction says you will output.” This is the computational analogue of Gödel/Liar-style constructions. The point is not metaphorical mystery but a structural limit: no agent can possess a perfectly general, internally available, behaviorally usable, and contradiction-free model of its own action in all contexts.

4. **Precise content of the analogy:**  
   Formal incompleteness concerns limits on what a system can prove about statements encodable within itself. Self-knowledge concerns limits on what a computational agent can *stably predict about its own computations* while remaining the same decision-making system. The load-bearing common structure is **self-reference under sufficient expressive power**, not vague “mystery about the self.”

5. **What follows:**  
   The right target is not “complete self-knowledge” but **bounded reflective coherence**. An agent can know many things about itself, but only through architectures that restrict expressivity, time horizon, granularity, or adversarial access. Global self-transparency is impossible for the same reason universal prediction of one’s own outputs under all recodings is impossible.

6. **Non-equivocal definition proposal:**  
   Define self-knowledge as:  
   > a set of internally represented propositions about the agent’s own computation such that, over a specified domain of conditions, those propositions are (i) empirically accurate, (ii) available for downstream reasoning, and (iii) robust under being known by the agent.  
   
   The third clause is crucial: some truths about the system cease to remain action-guiding truths once fed back into the system.

So the refined claim is: **self-knowledge is computable but not closure-complete**. The formal incompleteness analogy becomes precise when framed as a theorem schema about reflective computational systems: increasing self-representational power expands useful self-knowledge while inevitably generating domains where total self-prediction fails.

## Questions

1. 1. If the conjecture drops the requirement that self-knowledge be robust under being known by the agent, does the incompleteness-style limit on globally correct and decision-preserving self-prediction still follow rather than collapsing into ordinary prediction error? — **no**
2. 2. Is the claim that self-knowledge consists of reliably exploitable fixed points between the agent and its self-model necessary for making the analogy precise, or could the conclusion survive with only accurate internal self-description? — **yes**
3. 3. If the agent is not computationally rich enough to represent claims about its own future outputs, does the diagonal argument central to the conjecture fail in a way that destroys the explanation rather than merely narrowing its scope? — **yes**
4. 4. Does the conclusion that complete self-knowledge is impossible depend essentially on the conjunction of global correctness, internal availability, decision preservation, and contradiction-freedom, such that removing any one of these conditions breaks the explanatory link to incompleteness? — **yes**
5. 5. Does the fixed-point account explain why some self-predictions can remain accurate in restricted architectures that limit expressivity, time horizon, or adversarial access, rather than only explaining failure in the fully general case? — **yes**
6. 6. Does the conjecture illuminate why feeding a true self-prediction back into the agent can change the agent's action, thereby distinguishing action-guiding self-knowledge from passive self-description in cases beyond the original problem statement? — **yes**
7. 7. Does the proposed theorem schema apply not only to binary opposite-output tasks but also to richer recodings of the agent's outputs and policies that were not explicitly mentioned in the conjecture? — **yes**
8. 8. If a counterexample agent appears to have complete self-knowledge because it predicts its outputs in all tested situations, would preserving the conjecture require denying that those predictions are internally available for downstream reasoning rather than adding a minor domain restriction? — **no**
9. 9. If one tries to save the conjecture from a system that uses probabilistic self-predictions, would that force abandonment of the claim that the core obstacle is self-reference under sufficient expressive power rather than mere uncertainty? — **no**
10. 10. If a purported counterexample achieves perfect self-prediction by outsourcing the self-model to an external module that the acting system cannot query during choice, would accommodating it require gutting the definition of self-knowledge as internally represented and behaviorally usable? — **yes**

## Candidate Problems

- Can the conjecture be turned into a nontrivial theorem schema with explicit assumptions and impossibility results? The central open question is which exact computational conditions are load-bearing for the claimed limit: what class of agents, self-model languages, feedback channels, and notions of 'decision-preserving' make it true that no internally available self-model can be globally correct and behaviorally usable? Without this, the Gödel analogy may collapse into a loose family of diagonalization intuitions rather than a precise result. (score: 0.97)
- What is the right formal criterion for 'robust under being known by the agent' or 'reliably exploitable fixed points'? This is a major unresolved tension because the conjecture defines self-knowledge by action-guiding internal representations, but the key third clause introduces a reflexive stability requirement that is not yet formalized. The new problem is to characterize when a self-prediction remains true after being consumed by the policy, and whether this is best modeled via fixed-point semantics, game-theoretic equilibrium, reflective oracles, logical induction, control-theoretic stability, or some other framework. (score: 0.94)
- How much of the impossibility is due to unrestricted adversarial self-reference versus ordinary self-knowledge in realistic architectures? The conjecture suggests global self-transparency is impossible, but it remains open whether the limitation is deep and pervasive or mainly triggered by specially constructed diagonal tasks. A valuable problem is to map the frontier: identify structural restrictions on expressivity, granularity, time horizon, or access that permit rich bounded reflective coherence, and determine whether these restrictions are merely pragmatic patches or principled characterizations of achievable self-knowledge. (score: 0.91)
