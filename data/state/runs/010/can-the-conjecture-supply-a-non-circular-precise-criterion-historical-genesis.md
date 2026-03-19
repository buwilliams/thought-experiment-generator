# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × historical-genesis

## Conjecture

**Conjecture:** The distinction between a mere causal control state and a genuine internal model with correctness conditions arose historically to solve a coordination problem: how can a system act successfully when the relevant target of action is absent, delayed, occluded, or only indirectly accessible? An internal state genuinely functions as a model **iff** it was recruited within the system to solve that decoupling problem by serving as a **surrogate variable** whose downstream use depends on properties of what it is *about*, not merely on its present causal role.

More precisely: an internal state \(S\) counts as a model of target domain \(T\) when there exists a systemically significant class of cases in which:

1. **Decoupling:** \(T\) is not currently available for direct control by the system.
2. **Substitution:** \(S\) is used in place of \(T\) to guide action, inference, or coordination.
3. **Projectibility:** The system’s success depends on exploitable structure-preservation between variation in \(S\) and variation in \(T\), across more than one task or context.
4. **Error possibility:** There are counterfactual cases where \(S\) can remain causally efficacious yet be *wrong* about \(T\), and this wrongness explains failure better than mere malfunction description.
5. **Error sensitivity:** The system contains or can acquire mechanisms that selectively detect and correct mismatch between \(S\) and \(T\), rather than merely restoring prior dynamics.

This criterion is non-circular because it does not define modeling by “representation” or “aboutness.” It defines it genealogically and functionally: a model-state is the kind of internal state that emerged to replace direct coupling when direct coupling was insufficient.

What this perspective illuminates is that correctness conditions were not added philosophically after the fact; they are inherited from the original problem the architecture was built to solve. Servo-like control loops do not need correctness conditions in the strict sense, because their states are not surrogates for absent structure; they are just nodes in a real-time causal circuit. We mistakenly over-ascribe “models” when we ignore the origin story and look only at internal complexity.

So the hidden assumption in many current debates is that any useful covariance or behavioral contribution might suffice for modeling. But covariance is what simpler control systems already had. The historical innovation was **detachable guidance**: using an internal stand-in that can succeed or fail according to the world’s structure even when the world is not directly driving the state at that moment.

Hence the sharp criterion: **A state has correctness conditions exactly when the system treats it as a detachable surrogate for a target whose structure matters independently of the state’s immediate causal efficacy.** Where no such decoupled surrogate function exists, there is control but not genuine modeling.

## Questions

1. 1. Is the claim that the state was recruited to solve action under absence, delay, occlusion, or indirect access necessary for the conjecture to distinguish genuine models from sophisticated real-time control states at all? — **yes**
2. 2. If the substitution condition were removed so that the internal state need not be used in place of the target, would the conjecture still explain why correctness conditions arise rather than merely redescribing causal contribution? — **no**
3. 3. Is the requirement of projectibility across more than one task or context necessary to prevent a one-off proxy correlation from counting as a genuine model under this criterion? — **yes**
4. 4. If error sensitivity were absent and the system could only restore prior dynamics without detecting mismatch between the state and the target, would the conjecture lose its explanation of why wrongness is a substantive feature rather than a philosopher's gloss? — **yes**
5. 5. Does the conjecture imply that offline planning, navigation from memory, and reasoning about hidden causes should count as model use even when the problem statement only asked about internal states in general? — **yes**
6. 6. Does the genealogical decoupling criterion illuminate why some highly complex servo systems with rich covariance still fail to qualify as model-based systems, beyond merely answering the original distinction question? — **yes**
7. 7. If correct, does the conjecture extend to social or linguistic coordination cases where agents use internal stand-ins for absent others or future states, rather than only to sensorimotor control? — **yes**
8. 8. If a counterexample involved a system whose internal state tracks an absent target and guides action but was not historically recruited for decoupling, would saving the conjecture require abandoning its genealogical core rather than adding a minor qualification? — **yes**
9. 9. If a purely feedforward predictor succeeded across many tasks but lacked any mechanism for detecting and correcting mismatch, would counting it as a genuine model force a major rewrite of the conjecture's account of correctness conditions? — **yes**
10. 10. If a real-time control loop developed internal complexity and covariance strong enough to mimic surrogate use while the target remained continuously available, would treating it as a model collapse the conjecture's central distinction between detachable guidance and direct coupling? — **yes**

## Candidate Problems

- How can the conjecture be operationalized into non-question-begging criteria that distinguish 'detachable surrogate' states from sophisticated but non-representational control states in real systems? The core open problem is whether decoupling, projectibility, error possibility, and error sensitivity can be measured in a way that yields clear verdicts for borderline cases such as predictive processing, model-free reinforcement learning with memory, adaptive control, and morphologically extended systems. (score: 0.96)
- Is the proposed genealogical condition actually necessary for genuine internal modeling, or only one historically common route by which modeling arises? This raises the unresolved tension between etiological accounts and synchronic functional/structural accounts: could a system count as having correctness conditions even if it was not recruited to solve the original decoupling problem, for example through exaptation, learning, cultural scaffolding, or artificial design? (score: 0.93)
- What exactly grounds 'aboutness' and correctness when the target domain is absent, distributed, socially constructed, or only partially mind-independent? The conjecture works most cleanly for concrete hidden targets, but it leaves open whether the same surrogate-based account extends to abstract objects, other agents, institutions, norms, and mathematical structures, where mismatch and correction may not reduce straightforwardly to tracking an independently given worldly state. (score: 0.88)
