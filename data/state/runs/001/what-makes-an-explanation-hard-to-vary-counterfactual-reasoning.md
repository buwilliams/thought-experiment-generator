# Generated: what-makes-an-explanation-hard-to-vary × counterfactual-reasoning

## Conjecture

**Conjecture:**  
The key structural difference is **counterfactual tightness**: in an explanation whose parts are all load-bearing, each part participates in a network of dependencies such that altering, removing, or inverting any one of them changes what the explanation predicts, entails, or rules out. In an explanation with freely swappable details, many parts are merely **descriptive decorations**: changing them leaves the explanatory work intact because they are not coupled to the mechanism that generates the phenomenon.

To see this, vary assumptions systematically.

- **Remove a part.**  
  If the explanation still accounts for the phenomenon with no loss of scope, constraint, or predictive power, that part was not load-bearing. If removal creates a gap—something no longer follows, a contrast can no longer be explained, or an intervention would no longer be expected to work—the part was structurally necessary.

- **Invert a part.**  
  In a tightly structured explanation, replacing “X increases Y” with “X decreases Y,” or “the system is stabilized by negative feedback” with “positive feedback,” should propagate through the account and force broad revision. If inversion changes only the wording while the explanation still stands, the original claim was not doing real explanatory work.

- **Swap a part with an opposite or near-equivalent.**  
  Where details are freely swappable, the explanation is operating at a more abstract level than those details. That is not always a flaw: sometimes the abstraction is the real explanation, and the details are only implementation variants. But if supposedly concrete details can be exchanged arbitrarily, then the account is mislocating its explanatory burden. The load-bearing structure lies elsewhere.

What breaks reveals the explanation’s **constraint architecture**. Good explanations are not distinguished by having many details, but by having details that are **mutually constraining**. Their parts narrow the space of possible worlds in coordinated ways. They explain not just why the phenomenon occurs, but why nearby alternatives do not.

So the distinguishing properties are:

1. **Dependency density:** parts support one another rather than sitting independently.  
2. **Counterfactual sensitivity:** changing a part changes downstream consequences.  
3. **Exclusion power:** the explanation rules out rival outcomes, not merely redescribes the actual one.  
4. **Compression with necessity:** it contains little that can be removed without explanatory loss.

What survives variation is incidental; what resists variation is structural. Thus, an all-load-bearing explanation is one with **high fragility to arbitrary substitution but high robustness to criticism**, because each part is there under pressure from the phenomenon and from competing explanations. A loosely load-bearing explanation is robust to swapping because its details are not doing the causal or logical work. In short: **load-bearing parts are those whose alteration deforms the explanation’s consequences, not merely its surface.**

## Questions

1. 1. If the claim that the key difference is specifically counterfactual tightness were replaced with mere descriptive richness, would the conjecture still distinguish load-bearing explanations from ones with swappable details? — **no**
2. 2. If removing the notion of a network of dependencies left the conjecture’s test for load-bearing parts unchanged, would that show the dependency claim is not itself load-bearing? — **yes**
3. 3. If altering one part of an explanation changed none of its predictions, entailments, or ruled-out alternatives, does this conjecture force the verdict that the part was only decorative rather than structural? — **yes**
4. 4. If inversion of a relation such as negative feedback to positive feedback required no broader revision of the explanation, would that count against the conjecture’s criterion of counterfactual tightness? — **yes**
5. 5. If concrete details in an explanation can be swapped for near-equivalents while the same mechanism still generates the phenomenon, does the conjecture say the real explanatory burden lies at a more abstract level? — **yes**
6. 6. If an explanation has high dependency density but changing one part does not alter downstream consequences, would that undermine the conjecture’s claim that dependency density and counterfactual sensitivity jointly mark load-bearing structure? — **yes**
7. 7. If an account explains why the phenomenon occurs but does not rule out nearby alternatives, does the conjecture classify it as lacking the exclusion power characteristic of an all-load-bearing explanation? — **yes**
8. 8. If a detail can be removed without any loss of scope, constraint, predictive power, or intervention guidance, does the conjecture commit to treating that detail as non-load-bearing? — **yes**
9. 9. If an explanation is highly fragile to arbitrary substitution but also fragile to criticism in the sense that small objections collapse it, would that contradict the conjecture’s claim that good all-load-bearing explanations are substitution-fragile yet criticism-robust? — **yes**
10. 10. If what survives systematic variation is taken to be structural rather than incidental, would that reverse the conjecture’s stated rule that what resists variation is structural and what survives variation is incidental? — **yes**

## Candidate Problems

- How can counterfactual tightness be made precise without collapsing into triviality or observer-relativity? The conjecture relies on notions like 'changes what the explanation predicts, entails, or rules out,' but it is unresolved how to formally individuate parts of an explanation, specify the allowed interventions/substitutions, and measure explanatory loss across different domains. Without such a criterion, 'load-bearing' risks depending on arbitrary decomposition or the evaluator's background assumptions. (score: 0.95)
- How do we distinguish genuinely load-bearing detail from merely implementation-specific detail when multiple realizations share the same abstract mechanism? The conjecture notes that swappable details may indicate the real explanation is more abstract, but leaves open a principled method for locating the correct explanatory level. This raises a major tension between explanatory compression and mechanistic concreteness: when does abstraction capture the true constraint architecture, and when does it hide missing structure? (score: 0.92)
- What is the relationship between fragility to substitution and robustness to criticism, and are they actually compatible in practice? The conjecture claims good explanations are highly sensitive to arbitrary alteration yet robust under criticism, but this may come apart: some brittle explanations are simply overfit, while some powerful explanations tolerate reformulation or modular replacement. The open question is what structural features separate productive counterfactual tightness from dogmatic rigidity or over-parameterization. (score: 0.90)
