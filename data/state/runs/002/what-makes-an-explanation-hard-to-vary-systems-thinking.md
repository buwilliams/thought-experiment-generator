# Generated: what-makes-an-explanation-hard-to-vary × systems-thinking

## Conjecture

**Conjecture:**  
What distinguishes a load-bearing explanation from a freely swappable one is not primarily the novelty of its words or even its local logical validity, but the **dependency structure** of the system of claims: in a load-bearing explanation, the parts participate in tight feedback and constraint loops such that changing one element forces compensating changes elsewhere or breaks the explanation’s ability to reproduce the phenomenon. In a swappable explanation, the parts are only loosely coupled; substitutions do not propagate meaningful consequences.

From a systems perspective, “hard to vary” can be made more precise as a structural property. An explanation is load-bearing when it forms a **high-constraint network** among objects, relationships, assumptions, and predicted observations. Its components are not merely present together; they regulate one another. The explanation compresses the search space because each part narrows what the others can be.

Einstein’s train-and-lightning case illustrates this. “Observer,” “simultaneity,” “constant light speed,” and “relative motion” are not independent decorations. If you alter one—say, make simultaneity absolute—you must also alter the treatment of light speed, time measurement, and the interpretation of observations. The explanation is structurally stiff. That stiffness is what makes its parts load-bearing.

So the practical distinction is this:

- **Freely swappable explanation:** low coupling, weak downstream consequences, many substitutions preserve apparent coherence.
- **Load-bearing explanation:** strong coupling, high consequence propagation, substitutions trigger contradictions, loss of reach, or ad hoc repair.

This suggests a way to test explanations in an LLM-based search architecture. Do not score candidates only for fluency or resemblance to known good explanations. Score them for **structural sensitivity**:

1. Identify core elements: objects, relations, assumptions, invariants, predictions.
2. Perturb one element at a time.
3. Measure how far the perturbation propagates through the explanatory system.
4. Penalize explanations that survive arbitrary substitutions with little consequence.
5. Reward explanations where small changes produce global failure or require principled reconstruction.

In systems terms, a good explanation has **high internal feedback, low redundancy of arbitrary parts, and strong constraint propagation**. A bad explanation has many degrees of freedom because its structure does not actually control its outputs.

What follows is important for the larger LLM question: if knowledge creation is framed as search, the key bottleneck is not generating candidate strings but building filters that detect **constraint-dense explanatory architectures**. The leverage point is therefore not bigger model scale but a representation and evaluation scheme that tracks interdependence among parts. If such a system can iteratively prefer explanations with stronger internal constraint and broader consequence, then “hard to vary” becomes an operational structural criterion rather than a philosophical slogan.

## Questions


## Candidate Problems

- How can 'constraint propagation' be operationalized so that an evaluator can distinguish principled reconstruction from ad hoc repair after perturbation? (score: 0.92)
- Are there important explanations with low apparent coupling at the verbal level but high underlying load-bearing structure, and how would the proposed test avoid misclassifying them? (score: 0.87)
- What representation of objects, relations, assumptions, and invariants is sufficient for perturbation tests to track explanatory stiffness rather than surface wording dependence? (score: 0.90)
