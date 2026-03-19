# Generated: under-what-conditions-does-next-token-prediction-actually-fo × norm-maintenance-public-goods

## Conjecture

**Conjecture:** Next-token prediction learns latent variables aligned with real domain invariants only when those invariants function like *enforced coordination constraints* in the data-generating process—i.e., when many local token choices are jointly disciplined by a shared hidden structure such that “defecting” from that structure causes broad predictive failure. When the hidden structure is weakly enforced, cheaply faked, or institutionally optional, prediction can instead succeed via surface conventions, stylistic mimicry, or short-horizon heuristics without representing the invariant itself.

From the collective-action perspective, a latent variable becomes learnable when it is not merely present but *maintained* by the process producing sequences. Ask: who “pays” to enforce consistency? In language or data, the payers are the mechanisms that make later observations depend on earlier hidden state: physical law, game rules, code semantics, accounting identities, persistent objects, social roles, database schemas, etc. These create a regime where many tokens must cohere with the same underlying state. The benefit of tracking that state accrues to the predictor as reduced loss across many downstream predictions.

The key condition is therefore not prediction alone, but **whether the environment makes invariant-respecting generation unavoidable**. If generators can privately “defect” by producing locally plausible continuations without preserving the hidden state—and suffer little penalty in the observed data—then next-token training rewards imitation of regularities rather than discovery of invariants. In social-norm terms, this is a cheap-talk equilibrium: appearances are enough. Latents need not be real; they need only compress frequent surface patterns.

By contrast, real invariants are learned when the data source imposes **distributed punishment for inconsistency**. A hidden variable is forced into the model when:
1. it has persistent downstream effects,
2. those effects are sampled densely enough in the training signal,
3. alternative shortcuts break across contexts,
4. and the cost of inconsistency is borne repeatedly, not just at isolated tokens.

Thought experiment: compare two corpora about chess. In one, text consists mostly of commentary using stock phrases (“strong attack,” “positional edge”). In the other, continuations must match legal move sequences, board states, and tactical consequences. In the first, “chess understanding” is a weak norm with low enforcement; style can substitute for state tracking. In the second, legality and board consistency are hard-to-defect constraints maintained by the game itself. Next-token prediction there is pressured to encode latent board variables because too many future tokens depend on them.

So the collision yields this illumination: **prediction learns reality-corresponding latents only when reality is the cheapest way to satisfy the coordination constraints embedded in the sequence distribution**. Sufficiency fails in domains where institutional enforcement of consistency is absent, sparse, or bypassable. It succeeds where hidden structure is a bottleneck through which many correct continuations must pass.

## Questions

1. 1. Is the claim that many local token choices must be jointly constrained by one shared hidden state necessary for the conjecture to conclude that next-token prediction learns real invariants rather than just useful features? — **yes**
2. 2. If the requirement that inconsistency causes broad downstream predictive failure were removed, would the chess legality example still support the conclusion that board state must be learned? — **no**
3. 3. Is the claim that the hidden structure is maintained by mechanisms such as game rules, code semantics, or accounting identities necessary to distinguish real invariant learning from mere stylistic imitation in this conjecture? — **yes**
4. 4. Does the conclusion depend essentially on the claim that the cost of inconsistency is paid repeatedly across many tokens rather than at isolated points? — **yes**
5. 5. Does the conjecture imply that next-token models should learn latent variables for source code execution state more reliably than for persuasive essay style, because code semantics enforce cross-token consistency? — **yes**
6. 6. Does the explanation extend to multimodal sequence data by predicting stronger latent learning when visual or sensor observations are tied to persistent hidden state with dense downstream effects? — **yes**
7. 7. Does the conjecture illuminate why models may appear competent in domains with rich jargon and stable surface conventions yet fail when asked to maintain long-range world consistency? — **yes**
8. 8. If a model learned real board-state variables from the commentary-only chess corpus, would saving the conjecture require abandoning the claim that weakly enforced domains permit success through surface mimicry? — **no**
9. 9. If latent variables were learned in a domain where generators can defect freely with little observed penalty, would rescuing the explanation force rejection of the claim that distributed punishment for inconsistency is the key driver of learnability? — **yes**
10. 10. If sparse but strategically placed tokens were enough to force invariant learning without dense sampling of downstream effects, would preserving the conjecture require revising its core bottleneck picture rather than adding a minor exception? — **yes**

## Candidate Problems

- How can 'enforced coordination constraint' be formalized in a way that yields testable predictions about when next-token training must recover a latent invariant rather than a surface shortcut? The conjecture is suggestive but currently lacks a precise quantity or criterion—e.g., involving mutual information over long horizons, intervention-based defectability, counterfactual predictive regret, or the density of downstream dependencies—that distinguishes genuinely enforced invariants from merely correlated patterns. (score: 0.97)
- Is the conjecture actually true as a necessary condition, or can models learn reality-aligned latent variables even when enforcement is weak because architecture, pretraining diversity, scale, or simplicity biases favor invariant-based world models anyway? This is a major open tension between data-distribution pressure and model-side inductive bias: perhaps invariants are sometimes learned not because the environment punishes inconsistency strongly enough, but because invariant representations transfer better across many loosely coupled contexts. (score: 0.93)
- What are the boundary cases where apparent enforcement is present but still does not produce invariant learning—for example when the hidden state is too expensive to track, supervision is too sparse, shortcuts remain globally competitive, or the model represents a simulacrum of the invariant rather than the invariant itself? This raises the unresolved question of what additional conditions besides enforcement—capacity, optimization accessibility, observability, horizon length, and identifiability—are required for latent alignment to emerge in practice. (score: 0.91)
