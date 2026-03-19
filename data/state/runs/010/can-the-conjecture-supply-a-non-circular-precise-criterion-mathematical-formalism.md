# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × mathematical-formalism

## Conjecture

Conjecture:

An internal state functions as a genuine model with correctness conditions iff there exists a nontrivial structure-preserving map between a space of possible world states and a space of internal states such that the system’s success depends on preserving specific invariants across that map under counterfactual variation.

More precisely, let \(W\) be a set of task-relevant world states, \(I\) a set of internal states, and \(A\) a set of actions. A merely causal controller can be described by a policy \( \pi: I \to A \) coupled to dynamics that produce useful behavior. But this alone does not make \(I\) a model. For \(I\) to be model-like, there must also be a mapping \( \phi: W \to I \) and a family of transformations \(T\) on \(W\) such that corresponding internal transformations \( \hat T \) on \(I\) satisfy:

\[
\phi(T(w)) = \hat T(\phi(w))
\]

for the relevant class of transformations, at least approximately.

This commutation condition is the core criterion. It says the internal state does not merely trigger action; it encodes relational structure from the world in a way that supports tracking, prediction, or evaluation across variations. Correctness conditions arise because, given \(w\), some internal states are structure-preserving images of \(w\) and others are not. So the state can be wrong, not merely ineffective.

The non-circularity comes from defining correctness relative to independently specifiable task structure, not by “what causes success” alone. If the agent must navigate spatial layout, for example, then topology, distance ordering, or transition constraints are independently relevant invariants. An internal state counts as a map only if those invariants are conserved well enough that internal operations mirror possible world changes. If no such invariant-preserving correspondence exists, then the state is just part of a control loop, however useful.

The crucial distinction is this:

- In causal control, performance may depend only on reliable covariance between input and action.
- In modeling, performance depends on an internal organization whose admissible transformations systematically correspond to world transformations.

A thermostat may correlate internal and external states, but unless its internal state supports a structured correspondence over a domain of counterfactual environmental changes, it lacks correctness conditions beyond brute functional success/failure. By contrast, a cognitive map, state estimator, or simulator qualifies because its internal transitions can be assessed for fidelity to world transitions.

So the criterion is: modelhood begins where there is an error-sensitive homomorphism between world structure and internal structure. The possibility of misrepresentation is not added afterward; it is mathematically induced by failure of the homomorphism to preserve the relevant invariants. That is what makes “correctness” precise rather than metaphorical.

## Questions

1. 1. Is the claim that there must be a nontrivial structure-preserving map from task-relevant world states to internal states necessary for the conjecture to distinguish genuine modelhood from a merely useful policy over internal states? — **yes**
2. 2. Is the requirement that system success depend on preserving specific invariants under counterfactual variation necessary for the conclusion that correctness conditions are more than post hoc descriptions of successful control? — **yes**
3. 3. If the commutation condition between world transformations and internal transformations were removed, would the conjecture lose its basis for saying that internal operations mirror possible world changes rather than merely correlate with them? — **yes**
4. 4. Is the appeal to independently specifiable task structure necessary to keep the account non-circular, so that correctness is not defined only by whatever internal state happens to produce success? — **yes**
5. 5. Does the conjecture imply that two systems with equal behavioral success can differ in whether they genuinely model, depending on whether one preserves world structure across counterfactual changes and the other does not? — **yes**
6. 6. Does the criterion extend beyond navigation to cases like state estimation or mental simulation by predicting that internal transitions in those systems should be assessable for fidelity to corresponding world transitions? — **yes**
7. 7. Does the conjecture illuminate why a thermostat can be reliable yet still fail to have full correctness conditions, by predicting that its internal state will break down under richer classes of environmental transformations than a cognitive map would handle? — **yes**
8. 8. If a counterexample showed a system with genuine correctness conditions but no identifiable family of world and internal transformations satisfying approximate commutation, would saving the conjecture require abandoning the commutation condition rather than adding a minor exception? — **yes**
9. 9. If a purely reactive controller were found to succeed across many environments without any independently specifiable invariant-preserving map, would defending the conjecture force a redefinition of modelhood so broad that the causal control versus modeling distinction collapses? — **yes**
10. 10. If some internal state preserved only arbitrary or task-irrelevant structure while still correlating with success, would rescuing the conjecture require dropping the claim that relevant invariants must be independently tied to the task rather than merely appending a small qualification? — **yes**

## Candidate Problems

- How can the 'relevant' world structure and invariants be specified non-circularly and uniquely enough to ground correctness conditions? The conjecture appeals to independently specifiable task structure, but it remains open how to choose W, T, and the privileged invariants without smuggling in the agent’s success criteria, observer interests, or semantic assumptions. A key problem is whether there is a principled procedure for individuating the structure that makes an internal state a model rather than an arbitrary homomorphic redescription. (score: 0.97)
- Is the existence of an approximate structure-preserving map actually sufficient for genuine representation, or only necessary? Many simple controllers, dynamical systems, and even thermostats may admit some nontrivial homomorphism under a cleverly chosen state space and transformation family. The unresolved tension is how to prevent trivialization: what additional constraints on counterfactual richness, decoupled use, explanatory indispensability, or action-guiding generalization are needed so that the criterion distinguishes robust model-based representation from mere useful covariance. (score: 0.99)
- What is the right formal treatment of approximation, granularity, and misrepresentation in real systems? The conjecture relies on approximate commutation and invariant preservation, but leaves open how much distortion, abstraction, or lossy compression is compatible with correctness conditions. This raises a new problem: defining metrics or equivalence classes under which internal states can be systematically wrong while still counting as models, especially in stochastic, partial, hierarchical, or learned representations where exact homomorphism is unavailable. (score: 0.94)
