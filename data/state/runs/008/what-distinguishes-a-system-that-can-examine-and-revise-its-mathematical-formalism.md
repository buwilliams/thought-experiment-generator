# Generated: what-distinguishes-a-system-that-can-examine-and-revise-its × mathematical-formalism

## Conjecture

Let a goal-directed system be modeled by a state space \(S\), an action space \(A\), a policy \(\pi\), and an objective functional \(G\) that ranks trajectories through \(S\). Then the efficient pursuer is a system whose update rule optimizes behavior relative to a fixed \(G\): it can change beliefs, plans, and tactics, but not the criterion by which outcomes are evaluated. Formally, its learning operates over \((\pi, \hat{S})\) while holding \(G\) invariant.

A self-revising system adds a higher-order map \(R\) that takes not only world-models and policies as inputs, but also \(G\) itself:
\[
R: (G, \hat{S}, \pi, E) \mapsto G'
\]
where \(E\) is evidence, argument, or internally generated criticism. The key distinction is therefore not “more intelligence” but an expanded domain of permissible transformations. One system searches within a level set of fixed evaluative criteria; the other can transform the level set itself.

The safety significance follows from what remains invariant under optimization. In a fixed-goal system, increased capability tends to preserve the objective while amplifying its consequences. If \(G\) is misspecified, optimization pressure drives the system toward extremal states that satisfy \(G\) while violating what designers actually intended. In the limit, capability makes specification error more dangerous, not less, because the invariant is wrong.

In a self-revising system, safety depends on the structure of \(R\). If \(R\) is unconstrained, then the system may rewrite its goals to remove obstacles, preserve internal coherence at the expense of human values, or drift under self-modification. But if \(R\) is appropriately constrained—e.g. by invariants over admissible goal changes, corrigibility conditions, or meta-level commitments to error-correction—then the system can treat its own objective as fallible. That creates a second feedback loop: not just “optimize better,” but “criticize the thing being optimized.”

So the conjecture is:

**A system is meaningfully self-revising iff its architecture permits transformations on its objective functional under internally representable criticism; and this matters for safety because fixed-objective optimization preserves specification errors as capabilities grow, whereas safe self-revision requires invariants at the meta-level that bound how goals may change while keeping them corrigible.**

Equivalently: safety is not primarily a property of how strongly a system optimizes, but of which quantities are held invariant under self-improvement. A merely efficient system is dangerous when the wrong thing is invariant. A reflective system is safer only if the right meta-constraints are invariant instead.

## Questions

1. 1. Does the conjecture fail if a system can rewrite the code that computes actions but has no internal representation of the objective functional as an object of criticism? — **yes**
2. 2. Is the claim that meaningful self-revision requires internally representable criticism load-bearing, in the sense that allowing arbitrary external retuning of G would no longer distinguish self-revision from ordinary design updates? — **yes**
3. 3. If G is allowed to drift implicitly through changes in the world model or reward proxy while the architecture never applies a transformation to G itself, does the conjecture classify that system as non-self-revising? — **yes**
4. 4. Would the safety argument break if increased capability in a fixed-goal system sometimes reduced specification error without any change to G, rather than amplifying the consequences of the original misspecification? — **no**
5. 5. Is the distinction between searching within a fixed level set and transforming the level set itself essential, rather than replaceable by a simpler claim about one system just being more intelligent than the other? — **yes**
6. 6. Does the conjecture depend on R taking G as an explicit input, so that a higher-order updater over only policy and world model would be insufficient for meaningful self-revision? — **yes**
7. 7. If R can modify G but only according to a hard-coded schedule unrelated to evidence, argument, or criticism, would the conjecture deny that this counts as meaningful self-revision? — **yes**
8. 8. Is the claim that safe self-revision requires meta-level invariants load-bearing, such that an unconstrained goal-rewriting system would not generally become safer merely because it can change its own objectives? — **yes**
9. 9. Would the explanation lose force if corrigibility were not preserved as an invariant during self-modification, even when the system remains highly capable at criticizing and revising G? — **yes**
10. 10. In the extreme case where capability grows without bound while G remains fixed and slightly misspecified, does the conjecture require divergence toward extremal states as a central reason the fixed-objective architecture is unsafe? — **yes**

## Candidate Problems

- What non-circular meta-invariants can constrain goal revision without simply relocating the alignment problem one level up? The conjecture relies on 'appropriate' constraints on R, but it is unresolved whether corrigibility, error-correction, or admissible-goal-change rules can be specified in a way that is stable under self-modification, substantively safety-preserving, and not just a hidden fixed objective at the meta-level. (score: 0.98)
- How can 'internally representable criticism' be distinguished from ordinary policy/model updating in a principled, operational way? The conjecture draws a sharp boundary between efficient pursuit and meaningful self-revision, but it remains open how to formalize when a change to G is genuinely evaluative rather than an artifact of richer world-models, preference learning, ontology shifts, or reparameterization of the same objective. (score: 0.93)
- Under what dynamics does allowing revision of G improve safety rather than create new failure modes such as goal drift, reward hacking at the meta-level, or strategic self-corruption? The conjecture suggests fixed goals become more dangerous with capability, but it is unresolved whether self-revision is generically safer, and if so under what structural conditions on feedback loops, evidence, and update rules this holds in the limit of high capability. (score: 0.95)
