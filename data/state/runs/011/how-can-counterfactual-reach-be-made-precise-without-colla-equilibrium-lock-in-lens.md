# Generated: how-can-counterfactual-reach-be-made-precise-without-colla × equilibrium-lock-in-lens

## Conjecture

**Conjecture:** *Counterfactual reach* should be made precise as a model’s **ability to generate and coordinate support across a large space of mutually dependent, unrealized interventions**, where each intervention-specific answer is individually costly to derive but jointly enabled by a compact structural representation. Its distinctive criterion is not accuracy on observed data, compression alone, causal identifiability alone, or invariance alone, but whether the model overcomes a **public-goods-style underprovision problem in explanation**.

More precisely: in many domains, answers to individual counterfactuals are **non-excludable and diffuse in benefit**. Any one query—“what if X had been different?”—can often be answered by local heuristic patching. But a system that can answer *many* such questions coherently requires a shared stock of structure: variables, dependencies, intervention semantics, constraints, and composition rules. Building that stock is costly, while the benefit of any one component is spread across many future queries. So locally rational modeling tends toward a stable **no-provision equilibrium**: ad hoc predictors, narrow causal estimates, and task-specific invariances, each good enough for immediate use, but collectively failing to fund the deeper representational structure that would support broad counterfactual competence.

On this view, **counterfactual reach** is the extent to which a representation provisions this shared structure. A model has greater reach when adding one more counterfactual query requires little bespoke repair because prior structure already bears the load. This gives a structural criterion:

- A representation has high counterfactual reach iff  
  **(1)** it supports a wide range of interventions not explicitly trained or encoded one by one,  
  **(2)** answers to those interventions are mutually constrained by shared internal structure, and  
  **(3)** removing or altering parts of that structure degrades performance across many counterfactuals at once, not just isolated tasks.

This distinguishes it from nearby notions:

- **Prediction:** can succeed in the observed regime without provisioning intervention-supporting structure.
- **Compression:** can summarize regularities while omitting the semantics of manipulable alternatives.
- **Causal modeling:** provides part of the needed structure, but counterfactual reach additionally concerns the **breadth and compositional generativity** of supported unrealized interventions.
- **Invariance:** identifies stable relations, but not necessarily a system’s ability to extend them across a broad intervention space.

The load-bearing test is crucial: if the model’s “counterfactual answers” are replaceable by independent patches, it has low reach. If instead one structural core constrains many answers, then the representation has genuinely provisioned a public good.

So the illuminating collision is this: counterfactual reach is not another predictive metric but a measure of whether a model escapes the structural underprovision equilibrium and invests in reusable explanatory goods. It is the **fundedness of unrealized alternatives** within a representation.

## Questions

1. 1. If the public-goods-style underprovision problem is removed from the conjecture, does the proposed criterion lose its basis for distinguishing counterfactual reach from prediction, compression, causal modeling, and invariance? — **yes**
2. 2. Is the claim that intervention-specific answers must be individually costly to derive necessary for explaining why a compact shared representation marks high counterfactual reach rather than mere convenience? — **yes**
3. 3. If the requirement that answers be mutually constrained by shared internal structure is dropped, does the conjecture cease to explain why independent patches count as low reach? — **yes**
4. 4. Is the load-bearing condition that altering one structural component should degrade many counterfactual answers at once necessary for the conclusion that the representation has provisioned a genuine explanatory public good? — **yes**
5. 5. Does the conjecture imply that a model trained on only a narrow set of interventions can still exhibit high counterfactual reach if its internal structure supports many untrained interventions with little bespoke repair? — **yes**
6. 6. Does framing reach as fundedness of unrealized alternatives illuminate why two models with similar observed predictive accuracy could differ sharply in their ability to answer novel what-if questions coherently? — **yes**
7. 7. Does the conjecture extend to cases where no single counterfactual query is especially valuable on its own, yet a shared representational stock would still enable broad future intervention reasoning? — **yes**
8. 8. If a counterexample is a model that answers many counterfactuals accurately by storing a large library of case-specific heuristics, would saving the conjecture require abandoning the claim that compact shared structure is central rather than adding a minor qualification? — **yes**
9. 9. If a causal model is identifiable and intervention-ready for one family of manipulations but brittle outside that family, would defending the conjecture against this case force a major revision of the claim that breadth and compositional generativity distinguish reach from causal modeling? — **no**
10. 10. If a highly compressed representation happens to support many counterfactual answers but those answers are not jointly degraded when one structural element is changed, would preserving the conjecture require gutting the load-bearing test rather than making a small exception? — **yes**

## Candidate Problems

- How can counterfactual reach be operationalized in a way that cleanly distinguishes it from neighboring properties like compression, causal identifiability, invariance, and generalization? The conjecture gives an intuitive criterion based on shared structure and load-bearing degradation across many interventions, but it remains open what formal metric, benchmark, or theorem would capture this without collapsing into existing notions or rewarding superficial patchwork. (score: 0.97)
- Is the 'public-goods underprovision' framing a genuine explanatory mechanism for why broad counterfactual structure is rare, or only an analogy? The open question is whether one can build a model of incentives, learning dynamics, or search costs showing a real no-provision equilibrium in which locally rational optimization systematically favors ad hoc predictors over reusable intervention-supporting structure. (score: 0.91)
- What structural representations actually maximize counterfactual reach, and under what constraints? The conjecture says reach comes from compact shared structure that coordinates many unrealized interventions, but it is unresolved whether the right substrate is causal graphs, mechanistic simulators, program-like world models, object-centric abstractions, or something else, and what tradeoffs govern breadth, fidelity, and repair cost when extending to new interventions. (score: 0.89)
