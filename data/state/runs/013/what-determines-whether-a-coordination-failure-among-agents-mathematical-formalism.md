# Generated: what-determines-whether-a-coordination-failure-among-agents × mathematical-formalism

## Conjecture

Conjecture: A coordination failure persists exactly when the system’s update dynamics contain a self-reinforcing invariant set — a region of joint strategy space in which each agent’s locally rational response reproduces the aggregate conditions that made the failure rational in the first place. It resolves spontaneously when noise, learning, or changing beliefs make that set non-invariant or unstable.

Translate the problem into structure:

- Let agents \(i=1,\dots,n\) choose actions \(a_i\).
- Let the joint state be \(x \in X\), where \(x\) includes actions, beliefs, expectations, and possibly network position.
- Let payoffs be \(u_i(a_i, x_{-i})\), with strategic complementarities: the payoff to a “cooperative” or “high-coordination” action rises as more others choose it.
- Let dynamics be \(x_{t+1}=F(x_t)\), where \(F\) is induced by best response, boundedly rational updating, imitation, or Bayesian revision.

The key mathematical object is not a single bad action profile but a basin of attraction. A coordination failure persists if:

1. there exists a low-performance fixed point or recurrent class \(B\subset X\),
2. \(F(B)\subseteq B\) (invariance),
3. small perturbations inside \(B\) decay back toward it (local stability),
4. no admissible unilateral deviation yields enough gain to move the system outside the basin.

In plain terms: persistence is determined by the geometry of the basin and the eigenstructure of the dynamics near the bad equilibrium. If the dominant feedback loops are contracting toward failure, the failure reproduces itself indefinitely.

What resolves it spontaneously? A spontaneous resolution occurs when the “bad” set is only metastable, not truly stable. That can happen if:

- stochastic shocks are large enough to cross the separatrix between basins,
- learning changes the effective map \(F\) over time,
- belief updating amplifies rare successful deviations rather than suppressing them,
- network diffusion creates a critical cluster that flips best responses elsewhere.

So the decisive quantity is not merely payoff superiority of the good equilibrium, but the transition structure between equilibria. A Pareto-superior coordinated state can still be practically unreachable if the path requires too many simultaneous deviations. Conversely, even a severe failure can collapse quickly if one deviation changes expectations enough to alter many best responses at once.

An illuminating invariant is threshold structure. If each agent switches to coordination only when expected participation exceeds \(\theta_i\), then persistence depends on whether the realized participation level stays below the critical percolation threshold needed for a cascade. Below threshold, failure is absorbing; above threshold, resolution propagates.

Thus the conjecture is: coordination failure endures when the system contains an absorbing or strongly attracting low-coordination set separated from better states by a deviation threshold larger than endogenous variation. It resolves spontaneously when fluctuations or structural change exceed that threshold, causing the low-coordination set to lose invariance and triggering a cascade into a different attractor.

## Questions

1. 1. Is the claim that persistence requires an invariant low-coordination basin necessary, or could indefinite failure still be explained if the bad state were revisited often without any invariant set? — **no**
2. 2. Is local stability of the bad set necessary for the conjecture's conclusion about persistence, or would invariance alone already make the explanation work? — **no**
3. 3. Is the claim that no admissible unilateral deviation can move the system outside the basin necessary, or could the explanation survive if single agents sometimes could escape but others pulled the system back? — **no**
4. 4. Is strategic complementarity necessary to the conjecture's mechanism, or could the same explanation of persistent coordination failure hold in environments without complementarity? — **no**
5. 5. Does the conjecture imply that a Pareto-superior equilibrium can remain unrealized indefinitely even when every agent would prefer it once reached? — **yes**
6. 6. Does the threshold and basin account extend to networked diffusion cases where a small critical cluster flips expectations and triggers a cascade not mentioned in the bare problem statement? — **yes**
7. 7. Does the conjecture illuminate why two systems with the same static payoffs can differ sharply in persistence because their transition structures and basin geometry differ? — **yes**
8. 8. If a coordination failure persists in a system with continual random shocks that never settle into a fixed or recurrent class, would saving the conjecture require abandoning its core focus on invariant or attracting sets? — **yes**
9. 9. If spontaneous resolution occurs through a one-time public signal that changes beliefs instantly without any gradual learning or noise accumulation, would the conjecture need a major rewrite rather than a minor qualification? — **no**
10. 10. If a bad equilibrium is locally stable and invariant yet disappears after a coordinated policy intervention external to the agents' update dynamics, would incorporating that case force the conjecture beyond its core claim about spontaneous resolution? — **yes**

## Candidate Problems

- How general is the claimed equivalence between persistent coordination failure and the existence of a self-reinforcing invariant set? The conjecture says failure persists exactly when such a set exists, but this may break under nonstationary dynamics, heterogeneous agents, history-dependent strategies, partial observability, or continual entry/exit. A key open question is to characterize necessary and sufficient conditions on F, information structure, and admissible deviations under which the equivalence is actually true rather than only a useful metaphor. (score: 0.97)
- What is the right notion of invariance and stability when noise, learning, and belief revision are endogenous parts of the system rather than external perturbations? The conjecture treats spontaneous resolution as loss of invariance or instability, but in stochastic or adaptive systems the relevant object may be metastable sets, quasi-stationary distributions, or changing attractors instead of fixed invariant basins. The unresolved tension is whether the deterministic basin picture survives once the update rule itself evolves. (score: 0.94)
- Can one derive actionable, low-dimensional diagnostics that predict when a low-coordination basin will collapse, especially in threshold and networked settings? The conjecture points to eigenstructure, separatrices, percolation thresholds, and critical clusters, but it remains open which quantities are actually sufficient, observable, and robust across models. This matters because without such diagnostics the conjecture has limited explanatory or intervention value. (score: 0.91)
