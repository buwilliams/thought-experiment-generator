# Generated: how-do-dominant-power-structures-self-perpetuate-without-exp × mathematical-formalism

## Conjecture

Conjecture: dominant power structures can self-perpetuate without explicit collusion because they are dynamically stable equilibria in a high-dimensional social system: many local optimization rules, acting independently, reproduce the same global inequality pattern when the transition rules preserve advantage as an invariant.

Mathematically, let agents \(i=1,\dots,n\) occupy positions in a network with state variables such as wealth \(w_i\), status \(s_i\), institutional access \(a_i\), and perceived legitimacy \(l_i\). Let institutions implement update functions
\[
(w,s,a,l)_{t+1} = F((w,s,a,l)_t,\theta,\varepsilon),
\]
where \(\theta\) encodes hiring rules, school admissions, zoning, inheritance, social trust, policing thresholds, credential requirements, etc. No conspiracy is required if \(F\) has three structural properties.

First, positive feedback: access increases the probability of gaining further access. Formally,
\[
\frac{\partial a_{t+1}}{\partial a_t} > 0,\quad \frac{\partial w_{t+1}}{\partial w_t} > 0,\quad \frac{\partial s_{t+1}}{\partial s_t} > 0.
\]
This creates path dependence: small initial asymmetries amplify.

Second, thresholding and selection under uncertainty: when decision-makers lack full information, they use proxies. If “merit” is estimated through variables already correlated with advantage, then selection functions preserve existing distributions. For candidate \(i\),
\[
P(\text{selected}_i) = g(x_i, p_i),
\]
where proxy \(p_i\) is partly a function of prior privilege. Then even individually rational, decentralized choices generate aggregate bias.

Third, legitimacy as a stabilizer: beliefs update to interpret outcomes as deserved. If observed success raises perceived legitimacy of the rules,
\[
\frac{\partial l_{t+1}}{\partial s_t} > 0,
\]
then inequality reproduces not only materially but epistemically. The system protects itself by making its outputs appear natural.

The key invariant is not any one person’s power, but the rank-order structure of advantage across groups. Individual members may enter or exit elite positions, yet the distributional form persists. This is like a Markov process with mobility at the micro level but a stationary macro-distribution.

An extreme-case test clarifies the structure: if all explicit coordination vanished overnight, but inheritance, network referral, unequal schooling, and reputation-based selection remained, would the hierarchy persist? Likely yes. But if those transmission channels were neutralized, the hierarchy would decay even if beneficiaries privately preferred its continuation. Therefore the persistence mechanism lies less in shared intention than in conserved transmission pathways.

So the illuminating claim is: dominant power structures are reproduced when institutions act as advantage-preserving operators. Beneficiaries need not collude because the system converts local incentive-following into global continuity. The real object of analysis is the transformation rule, not the motives of the occupants.

## Questions

1. 1. Is the claim that transition rules preserve advantage as an invariant necessary for explaining persistent group hierarchy, rather than merely repeated short term inequality? — **yes**
2. 2. Would the explanation fail if positive feedback in wealth, status, and access were removed while threshold based selection and legitimacy updating remained? — **no**
3. 3. Is the claim that proxies used under uncertainty are already correlated with prior privilege required for decentralized rational choices to reproduce the same inequality pattern? — **yes**
4. 4. Would removing legitimacy as a stabilizer destroy the account of long run self perpetuation, rather than just making the hierarchy more contested? — **no**
5. 5. Does the conjecture imply that high turnover among individual elites can coexist with a stable group level rank order of advantage? — **yes**
6. 6. Does the explanation extend to domains not named in the problem, such as algorithmic hiring or credit scoring, when those systems use proxies tied to prior advantage? — **yes**
7. 7. If explicit collusion disappeared but inheritance, network referral, and unequal schooling remained, does the conjecture predict that hierarchy would still persist? — **yes**
8. 8. If a counterexample showed durable inequality in a setting with little legitimacy belief, would saving the conjecture require abandoning the claim that epistemic stabilization is one of its core structural properties? — **no**
9. 9. If some institutions use random selection or blind review and inequality still persists, would preserving the explanation require gutting the claim that proxy based thresholding is a central mechanism? — **no**
10. 10. If a society had strong positive feedback and proxy based selection but no stable group rank order over time, would rescuing the conjecture force a revision of its core invariant rather than a minor qualification? — **yes**

## Candidate Problems

- What are the necessary and sufficient structural conditions on the update operator F for 'advantage preservation' to produce a stable macro-inequality distribution rather than convergence, cycling, or fragmentation? The conjecture names positive feedback, proxy-based selection, and legitimacy, but it remains unresolved whether these are jointly sufficient, merely common correlates, or one subset among many equivalent mechanisms. Formalizing the invariant, identifying phase transitions, and distinguishing stability from metastability is a high-value open problem. (score: 0.96)
- How can the conjecture be empirically distinguished from alternative explanations such as explicit collusion, preference-based discrimination, productivity differences, or exogenous shocks when all can generate similar observed inequality patterns? The central unresolved tension is one of identification: what observable signatures would uniquely indicate decentralized equilibrium reproduction via transmission pathways rather than intentional coordination or other mechanisms? Without this, the conjecture risks explanatory breadth without discriminating power. (score: 0.94)
- What exactly is the causal role of perceived legitimacy l in reproducing hierarchy: is it an independent stabilizing state variable, a downstream epiphenomenon of material inequality, or a feedback channel that changes the basin of attraction of the whole system? The conjecture treats legitimacy as epistemic glue, but it is unclear when belief-updating materially alters selection, compliance, and reform resistance versus merely narrating outcomes after the fact. Resolving this would clarify whether ideological reproduction is structurally indispensable or only contingently reinforcing. (score: 0.88)
