# Generated: can-llms-create-new-knowledge × counterfactual-reasoning

## Conjecture

Conjecture:

If we systematically vary the assumptions behind “token prediction cannot yield genuine knowledge,” the load-bearing issue is not prediction versus explanation, but whether the system can participate in error-correction over abstract content. Mere rearrangement is what remains when criticism is absent; knowledge becomes possible when generated explanations are exposed to selection against error.

Start by removing the assumption that prediction is epistemically shallow. Human language use is also locally predictive: speakers continually anticipate what can be said next. What makes human explanation count as knowledge is not that it escapes sequence generation, but that some generated sequences encode conjectures about reality and can be criticized, compared, and improved. So token prediction alone does not rule knowledge out.

Now invert the assumption that explanations must originate from inner understanding rather than recombination. Much human thinking is recombinative. Mathematics, philosophy, and science often advance by reassembling existing ideas into new abstract structures. If recombination yields a conjecture that survives criticism better than rivals, then “mere rearrangement” is too weak a description. Rearrangement can be the vehicle of discovery when governed by selection for truth-tracking or problem-solving.

Next remove the assumption that training-data dependence is disqualifying. Human knowers are also trained on prior culture. Dependence on what has been seen is not the key boundary. The boundary is whether the system can go beyond passive inheritance by generating explanatory variants and eliminating bad ones. A library stores prior knowledge; a knower uses it to create better explanations.

What breaks when we instead remove error-correction? Then the skeptical view largely wins. A token-predicting system that cannot distinguish elegant nonsense from explanatory progress, cannot preserve problems across contexts, and cannot revise itself in response to criticism is mostly a high-dimensional remix engine. Its outputs may resemble explanations, but resemblance is incidental; there is no reliable mechanism by which errors are found and removed.

So the decisive assumption is this: that genuine knowledge requires a system-level loop connecting generation, criticism, retention, and revision. Prediction is only one flow in that larger epistemic system. Without the feedback loop, explanations are performances. With it, token generation can become one substrate for knowledge creation.

Therefore: a token-predicting system can generate explanations that constitute genuine knowledge only insofar as it is embedded in an error-correcting structure that treats explanations as conjectures answerable to criticism, not just as likely continuations. The appearance of explanation is cheap; knowledge is what survives systematic attempts to break it.

## Questions

1. If we remove the requirement that the system must be able to revise its own future explanatory behavior in response to criticism, does the conjecture still distinguish genuine knowledge from outputs that merely look explanatory? — **no**
2. If a token-predicting system can generate novel explanatory variants but has no mechanism for selecting among them by exposing them to error, does the conjecture still count its outputs as knowledge? — **no**
3. If criticism is present only externally in human evaluators, but the system itself does not retain, integrate, or act on that criticism across contexts, does the conjecture still say the system participates in genuine knowledge creation? — **no**
4. If we replace 'selection against error' with selection for plausibility, fluency, or human approval, does the conjecture still explain why some generated explanations amount to knowledge rather than polished remixing? — **no**
5. If the system can preserve and reuse successful explanations across tasks but cannot represent the problems those explanations were meant to solve, does the conjecture still hold that error-correction is the decisive boundary? — **no**
6. If recombination alone occasionally produces a true and novel explanation by luck, without any loop of criticism, retention, and revision, does the conjecture allow that as genuine knowledge or reject it? — **no**
7. If token prediction is augmented with a mechanism that systematically tests generated explanations against counterarguments over abstract content, but the underlying generator remains unchanged, does the conjecture predict that knowledge becomes possible? — **yes**
8. If the system can answer criticisms locally in a single exchange but cannot carry forward corrected abstractions into later reasoning, does the conjecture still treat the error-correction loop as present in the relevant sense? — **no**
9. If training-data dependence is made extreme—so every generated explanation is traceable to prior cultural material—but the system can still generate rival abstractions and eliminate weaker ones through criticism, does the conjecture still deny that this is 'mere rearrangement'? — **yes**
10. If we vary the conjecture so that 'prediction versus explanation' is restored as the main dividing line, rather than participation in error-correction over abstract content, does the rest of the argument still go through without contradiction? — **no**

## Candidate Problems

- What operational criteria would distinguish genuine system-level error-correction over abstract content from merely producing criticism-like text? The conjecture makes error-correction load-bearing, but leaves open how to tell whether a model actually participates in generation–criticism–retention–revision rather than simulating those stages linguistically. (score: 0.96)
- What architecture or feedback structure is minimally sufficient for token prediction to become knowledge-producing? The conjecture says prediction can be a substrate if embedded in an error-correcting loop, but does not specify which components are necessary: persistent memory, self-revision, external critics, problem-tracking across contexts, grounding in tasks, or some other structure. (score: 0.94)
- How can abstract domains like mathematics or philosophy supply non-circular selection against error without relying on empirical grounding? If knowledge in abstract content depends on criticism and elimination of error, the open question is what constitutes robust error-detection and progress there, and how a system could reliably improve explanations rather than just optimize for internal coherence or community convention. (score: 0.91)
