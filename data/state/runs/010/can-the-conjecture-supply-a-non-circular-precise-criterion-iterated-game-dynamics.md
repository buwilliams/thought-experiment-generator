# Generated: can-the-conjecture-supply-a-non-circular-precise-criterion × iterated-game-dynamics

## Conjecture

**Conjecture:**  
An internal state genuinely functions as a **model with correctness conditions** iff it is embedded in an **iterated interaction structure** where future payoffs depend systematically on whether that state supports **counterfactually discriminating expectations** about an environment or partner across repeated rounds. Mere causal control lacks this property because its success is exhausted by immediate behavioral coupling; modelhood appears when the state’s contribution can be **right or wrong independently of the current output**, and that distinction matters for later interaction.

### Why this follows from iterated game dynamics
In a one-shot interaction, an internal state need only help produce an effective move. There is no strong structural reason to distinguish:
- a state that merely triggers a useful response, from
- a state that represents how things are and can therefore misrepresent.

But under repeated play, the agent faces a recurring problem: future success depends on tracking stable or revisable features of the other player or environment — e.g. “this partner reciprocates,” “defection happened last round,” “the resource renews after delay.” In such settings, internal states that preserve information about latent structure outperform mere stimulus-response couplings because they support **reciprocity, reputation management, and conditional strategy updating** over time.

### Precise criterion
An internal state counts as a model-state when:

1. **Iterability:** The system is engaged in recurrent interactions where later outcomes depend on earlier conditions.
2. **Counterfactual sensitivity:** Holding the control policy fixed, variation in the world would make the state assessable as accurate or inaccurate relative to those world conditions.
3. **Downstream strategic dependence:** That accuracy/inaccuracy systematically affects future payoffs across repeated rounds.
4. **Detachable correctness:** The state can be wrong even when immediate behavior is locally adaptive, because its role concerns expected future structure, not just present causal efficacy.

So correctness conditions are not imposed from outside by interpretation alone; they arise from the state’s role in regulating behavior under the **shadow of the future**. A model is a state whose content matters because repeated interaction makes error accumulate.

### Non-circularity
This avoids circularity because it does not define a model as “whatever has correctness conditions.” Instead, it identifies a structural regime in which correctness becomes objectively consequential:
- if the state misclassifies a cooperator as a defector, reciprocity breaks down;
- if it mispredicts hidden environmental regularities, future coordination fails;
- if no such iterated penalty exists, the state may still causally control behavior, but there is no reason to attribute robust correctness conditions.

### Illumination
The key distinction is therefore not internal complexity, nor resemblance, nor mere covariance. It is whether the state is part of a repeated-game architecture that **rewards being right about persisting structure and punishes being wrong over time**. In short:

> **Modelhood begins where error has a future.**

That is the criterion iterated game dynamics contributes.

## Questions

1. 1. Is the claim that repeated interaction is required essential to the conjecture, such that removing iterability would collapse its distinction between model-states and one-shot causal control states? — **yes**
2. 2. Is the claim that a state must be assessable as accurate or inaccurate under counterfactual variation necessary for the conclusion that it has correctness conditions, rather than merely a useful causal role? — **yes**
3. 3. Is the requirement that accuracy or inaccuracy systematically affects future payoffs indispensable to the explanation, so that without payoff consequences the conjecture no longer explains why correctness conditions are objectively consequential? — **yes**
4. 4. Is the claim that a state can be wrong even when immediate behavior is locally adaptive necessary for separating genuine modelhood from successful present-time control? — **yes**
5. 5. Does the conjecture imply that internal states used for navigation, foraging, or delayed resource tracking in non-social environments can count as model-states whenever repeated interaction with stable environmental structure makes error accumulate over time? — **yes**
6. 6. Does the conjecture illuminate why memory-like states involved in reciprocity, reputation, and conditional cooperation should exhibit stronger correctness conditions than reflexive stimulus-response couplings even when both improve immediate performance? — **yes**
7. 7. Does the conjecture predict that as the shadow of the future weakens, such as when interactions become effectively one-shot or future dependence disappears, the basis for attributing robust correctness conditions to the state should also weaken? — **yes**
8. 8. If there were a one-shot system with an internal state that clearly seems right or wrong about a hidden cause and guides successful action, would saving the conjecture require abandoning the claim that iterated interaction is necessary rather than adding a minor qualification? — **yes**
9. 9. If a repeated interaction system gains future payoffs from a state that is useful only because of a fixed heuristic and not because it tracks latent structure, would preserving the conjecture require substantially revising its link between payoff success and correctness conditions? — **yes**
10. 10. If immediate behavior always reveals and corrects any state error before later rounds occur, would defending the conjecture force it to give up the claim that detachable correctness can exist independently of current output? — **yes**

## Candidate Problems

- Is iterated interaction actually necessary for modelhood, or only a common sufficient condition? The conjecture risks excluding plausible one-shot cases where a state seems to have correctness conditions because it tracks latent structure used for planning or inference even without repeated rounds. A key open problem is to formulate and test a necessity claim against edge cases such as single high-stakes decisions, offline simulation, and one-trial organisms or devices. (score: 0.96)
- How can 'counterfactually discriminating expectations' and 'detachable correctness' be specified non-circularly and operationally? The conjecture depends on being able to distinguish mere causal control from genuine accuracy-sensitive representation, but the proposed tests may smuggle in representational notions through terms like 'latent structure,' 'misclassification,' or 'being right.' A worthwhile problem is to derive measurable criteria from system dynamics alone that separate representation from sophisticated control architectures. (score: 0.94)
- What is the minimal system structure that makes correctness conditions emerge, and where is the boundary with memory-based policy optimization? Repeated interaction can reward stored information without that information obviously constituting a model; simple reinforcement learners, lookup tables, and dynamical controllers may exploit history-dependent payoffs while lacking anything intuitively representational. The open question is whether the conjecture can identify a principled threshold—perhaps involving abstraction, compression, generalization, or intervention sensitivity—rather than collapsing modelhood into any payoff-relevant state variable. (score: 0.98)
