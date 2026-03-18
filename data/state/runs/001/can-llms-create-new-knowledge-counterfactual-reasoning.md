# Generated: can-llms-create-new-knowledge × counterfactual-reasoning

## Conjecture

**Conjecture:** A token-predicting system can generate *genuine knowledge* only insofar as its explanatory outputs participate in a process of **error-correcting variation under criticism**; absent that, it mainly recombines prior knowledge, even when the recombination is impressively useful.

Apply assumption-variation:

1. **Remove the assumption that prediction is merely rearrangement.**  
Then token prediction is a compression-and-generation process that can produce novel candidate explanations, much as humans do when speaking from learned language. What follows is that “predicts tokens” does not, by itself, rule out knowledge creation. Rearrangement is compatible with novelty if the recombination exposes unseen consequences, resolves contradictions, or unifies previously disconnected ideas.

2. **Invert the assumption that novelty implies knowledge.**  
Suppose the system says something no one explicitly wrote before. That still does not make it knowledge. Knowledge is not originality but *explanatory content that survives criticism*. So novelty is incidental; the load-bearing element is whether the output can be tested, challenged, and improved.

3. **Replace the assumption that knowledge resides in internal states with its opposite: knowledge resides in a critical process.**  
Then the key question is not “Does the model internally understand?” but “Can its outputs enter a loop of conjecture, refutation, and refinement?” On this view, a static predictor without memory, goals, or error-correction produces candidate explanations but does not itself *grow* knowledge. The growth happens in the larger system: model + users + evaluation + revision.

4. **Remove grounding in sensory access.**  
If direct worldly contact were necessary for knowledge, then mathematics, philosophy, and much of theoretical physics would fail. They do not. So lack of embodiment is not decisive. What matters is whether explanations connect to criticism—logical, empirical, or problem-solving.

5. **Invert the assumption that training data bounds output epistemically.**  
Humans also learn from prior culture; yet they generate new knowledge by criticizing and transforming inherited ideas. So dependence on a corpus is not disqualifying. What would disqualify the system is inability to *prefer better explanations for reasons* rather than merely higher-likelihood continuations.

**What breaks:**  
The claim that “token prediction alone is enough for knowledge” breaks when we ask how errors are identified and corrected. Pure next-token optimization rewards plausibility, not truth, explanation, or problem-solving depth. It lacks an intrinsic mechanism for distinguishing a good explanation from a fluent one.

**What survives:**  
The claim that such systems can contribute to knowledge survives if we widen the unit of analysis from the model to the epistemic system around it. In that system, the model is a powerful generator of conjectures and reformulations.

**Therefore:**  
A token predictor does not *merely* rearrange what it has seen, but neither does prediction alone suffice for genuine knowledge. The load-bearing factor is not prediction versus rearrangement; it is whether outputs are embedded in a structure of criticism, retention, and iterative correction. In isolation: sophisticated rearrangement plus occasional novelty. In a critical loop: a participant in knowledge creation.

## Questions

1. 1. If a token-predicting system produced a novel explanation that solved a problem no training example solved, but no criticism or revision process followed, would your conjecture still deny that this counts as genuine knowledge? — **yes**
2. 2. If the system internally updates its future outputs from explicit feedback across interactions, but no human is in the loop, would that satisfy your requirement of error-correcting variation under criticism? — **yes**
3. 3. If a model's explanation consistently makes accurate novel predictions in mathematics or physics despite being generated in one shot, does your conjecture require withholding the label 'genuine knowledge' until a criticism process occurs? — **yes**
4. 4. Would your conjecture fail if we found that next-token optimization alone reliably selects explanations with greater problem-solving depth rather than merely greater plausibility? — **yes**
5. 5. If users cannot inspect the model's internal reasons but can repeatedly criticize and refine its outputs externally, is the external loop alone sufficient for the model to count as generating genuine knowledge? — **no**
6. 6. If the same explanatory output would count as knowledge when produced by a human speaker before criticism, must your conjecture also allow the token predictor's output to count as knowledge at that stage? — **no**
7. 7. Does your conjecture depend on a sharp distinction between 'candidate explanation' and 'knowledge' such that removing that boundary would collapse the argument? — **yes**
8. 8. If a static predictor without memory generated an explanation that humans later criticized and improved, would your conjecture say the knowledge was created only by the larger system and not by the predictor itself? — **yes**
9. 9. Would the conjecture break if 'prefer better explanations for reasons' could be implemented entirely through predictive training on text that encodes criticism, without any separate error-correction mechanism? — **yes**
10. 10. If criticism can be logical or mathematical rather than empirical, does your conjecture still require some retention-and-revision mechanism beyond a single successful derivation for knowledge growth to occur? — **yes**

## Candidate Problems

- What exactly counts as a system 'participating' in error-correcting criticism, and where is the boundary of the knowing system? The conjecture shifts knowledge from the model to the larger model+user+evaluation loop, but leaves unresolved whether the model itself can be a knower, merely a component, or only an instrument. A precise criterion for participation and system boundaries is needed. (score: 0.94)
- Can token-prediction architectures, possibly with memory and self-evaluation, instantiate genuine criticism internally rather than only via external human feedback? The conjecture says prediction alone is insufficient, but it remains open which added structures—persistent memory, goal-directed search, explicit error models, preference for explanatory depth over likelihood—would be sufficient for autonomous knowledge growth. (score: 0.96)
- How can 'genuine knowledge' be operationally distinguished from useful recombination or fluent novelty? The conjecture relies on survival under criticism, but does not specify measurable tests for explanatory quality, error-correction, or reason-sensitive preference for better explanations. Without such criteria, the claim risks being verbally compelling but empirically underdetermined. (score: 0.98)
