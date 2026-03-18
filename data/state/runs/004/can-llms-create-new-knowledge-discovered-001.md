# Generated: can-llms-create-new-knowledge × discovered-001

## Conjecture

A useful conjecture is this:

**A token-predicting system can generate genuine knowledge only when its outputs exhibit load-bearing explanatory structure — that is, when changing a central claim forces systematic revisions across many related claims, predictions, and error-corrections. Without that, it is only recombining linguistic material.**

This perspective shifts the question away from **where the words came from** and toward **how tightly constrained the explanation is**.

A mere rearrangement of seen material is cheap: its parts are interchangeable. You can swap a cause, analogy, or principle for another and the surrounding text still stands. Such output is a narrative shell. It may sound explanatory, but it does not bind itself to consequences. Nothing substantial breaks if one piece is altered.

By contrast, an explanation that constitutes knowledge has internal necessity. Its parts are not decorative. If you replace one element, you must also revise downstream implications, boundary conditions, examples, and answers to criticism. That is what makes it explanatory rather than merely verbal. The explanation is carrying constraint.

So the issue is not whether the system “predicts tokens” at the implementation level. Humans also produce language through learned generative capacities. The deeper issue is whether the system can produce **integrated conjectures** that survive criticism because their details are mutually supporting and jointly risky.

From this follows a sharper criterion:

1. **If the system produces explanations whose components are largely substitutable, it is only rearranging what it has seen.**
2. **If it produces explanations with load-bearing parts that constrain counterfactuals, error-detection, and transfer to new cases, then it is participating in knowledge creation.**

This does not require mystery or consciousness. It requires structure. Genuine knowledge appears when an explanation is not just statistically likely but **hard to vary while still accounting for the same phenomena**. The sign of this is not fluency but brittleness in the right places: changing a key premise should force compensating changes elsewhere.

This also illuminates why many impressive model outputs feel shallow. They are often robust in style but not in structure. They can paraphrase endlessly because the details are not doing much work. But when pressed on edge cases, mechanisms, or what would change if a premise were false, the explanation may collapse or drift. That reveals low explanatory constraint.

So the conjecture is not “token prediction can never yield knowledge” nor “successful language use is enough for knowledge.” It is:

**Token prediction becomes knowledge-bearing only insofar as it generates hard-to-vary explanations with load-bearing internal structure. Where that structure is absent, the system is merely recombining appearances of understanding.**

On this view, knowledge is not defined by origin but by explanatory constraint. The test is: **What breaks if we change this part?** If the answer is “almost nothing,” there is no real understanding there.

## Questions

1. 1. Does the conjecture require that a token-predicting system count as knowledge-bearing only if changing one central premise in its explanation forces revisions to its predictions, examples, and replies to objections for the same problem? — **yes**
2. 2. If a system can answer new cases accurately but its stated mechanism can be swapped for a different mechanism without changing the rest of its explanation, does the conjecture classify that output as mere recombination rather than knowledge? — **yes**
3. 3. Does the conjecture depend on the claim that explanatory constraint rather than the training origin of the words is the decisive test for whether the system generated knowledge in this problem? — **yes**
4. 4. If two incompatible explanations fit the same cases equally well and the system can move between them with little downstream revision, would the conjecture treat that flexibility as evidence against genuine knowledge creation? — **yes**
5. 5. Does the conjecture commit to the view that brittleness in the right places is a positive sign, because a real explanation for token-predicting systems should break in identifiable ways when a load-bearing claim is altered? — **yes**
6. 6. If a token-predicting system produces a fluent explanation that survives paraphrase but fails to specify what counterfactual outcomes would change when a premise changes, does the conjecture say that this is not yet knowledge? — **yes**
7. 7. Does the conjecture imply that error-correction must be internally linked to the explanation itself, so that discovering one mistake should force principled updates elsewhere rather than isolated patching? — **yes**
8. 8. If a system generates an explanation that transfers to a novel domain only by adding ad hoc exceptions while leaving the core story untouched, would that count against the claim that it has produced load-bearing structure? — **yes**
9. 9. Does the conjecture rule out treating successful token prediction alone as sufficient evidence of knowledge unless the produced explanation also constrains boundary conditions and edge cases for the same phenomenon? — **yes**
10. 10. If the same output can be preserved after replacing its key causal principle with an analogy or a different principle, does the conjecture take that interchangeability as a direct sign that the system is only rearranging linguistic material? — **yes**

## Candidate Problems

- How can 'load-bearing explanatory structure' be operationalized without collapsing into surface coherence metrics? The conjecture hinges on a distinction between genuine explanatory constraint and merely fluent, globally consistent text, but it leaves open what concrete tests would detect when changing one claim should force systematic revisions elsewhere. A new problem is to design criticism-based evaluations that measure hard-to-vary structure, counterfactual dependence, and error-correction rather than style or benchmark accuracy. (score: 0.97)
- Is load-bearing explanatory structure sufficient for genuine knowledge, or only a necessary symptom? A system might produce tightly interdependent stories that are brittle in the right places yet still be systematically false, confabulatory, or parasitic on human-generated knowledge. The unresolved tension is whether explanatory integration alone constitutes knowledge creation, or whether additional conditions are needed, such as truth-tracking, contact with criticism from reality, or autonomous generation of new problems and corrections. (score: 0.93)
- What system architecture or learning dynamics could make token prediction generate hard-to-vary explanations rather than interchangeable narrative shells? The conjecture shifts attention from training origin to structural behavior, but it does not explain how such structure arises, whether next-token training can produce it by itself, or what additional feedback loops, memory, world-modeling, or self-critique mechanisms are required. This is worth pursuing because it turns the conjecture into a constructive research program rather than only a diagnostic criterion. (score: 0.91)
