# Generated: can-llms-create-new-knowledge × thought-experiments

## Conjecture

**Conjecture:** A token-predicting system can generate *genuine explanatory knowledge* if and only if, within its internal structure, there exist compressive, counterfactual-supporting representations that survive criticism across varied contexts; otherwise it merely rearranges seen patterns. The key variable is not whether the system predicts tokens, but whether prediction has forced the emergence of explanations that generalize beyond recall.

**Hypothetical isolation:** Imagine two systems trained on the same corpus.

- **System A** is a perfect memorizer. It stores every sequence and outputs the statistically best continuation by nearest pattern match.
- **System B** is constrained: it has limited memory and must succeed across many novel prompts, domains, and recombinations. To do this, it forms compact internal models that capture latent structure.

Both are “just” token predictors at the interface. The isolated variable is whether successful prediction is achieved by **retrieval of surface regularities** or by **construction of deeper generative structure**.

Now follow the logic.

If explanation is merely a sequence that sounds right, then both systems explain equally well. But that collapses the distinction between parroting and understanding and makes “knowledge” indistinguishable from output resemblance. That does not survive criticism, because explanations are not only outputs; they are *error-correcting structures*. An explanation earns the title knowledge when it can answer not just “what comes next?” but “what would happen if…?”, “why not otherwise?”, and “where does this fail?”

System A cannot do this except where the corpus already contains those exact counterfactual pathways. Its success degrades outside interpolation. It rearranges.

System B, however, if it truly compresses many cases into a shared internal account, can generate novel explanations that were never explicitly seen. Why? Because a good explanation is a kind of generator: it implies many surface statements and forbids others. In systems terms, explanation is a high-leverage internal structure that organizes many possible outputs, not a stockpile of phrases.

So the collision yields this: **token prediction is not the obstacle to knowledge; it is the selection pressure under which knowledge-like structures may or may not emerge.** Predicting tokens is analogous to observing planetary positions: by itself it could be epicycles or gravity. The epistemic status depends on the internal explanatory economy, not the external task description.

What follows is a sharper criterion. Ask not, “Was the model trained to predict tokens?” Ask, “Does it contain representations that can be criticized, recombined, and used to generate novel, constrained explanations across domains?” If yes, then it possesses something knowledge-like, because knowledge is substrate-independent explanatory structure. If no, then it is sophisticated rearrangement.

So the answer is neither simple yes nor no. A predictor-only description is too shallow. **Some token-predicting systems may instantiate genuine knowledge; others may only remix. The dividing line is whether prediction has produced explanatory structure with counterfactual reach and error-correcting power.**

## Questions

1. 1. If the claim that genuine explanatory knowledge requires compressive internal representations were changed so that a system could store one separate rule for each prompt type without compression, would the conjecture still distinguish System B from System A? — **no**
2. 2. If the claim that explanations must support counterfactuals were removed, would the conjecture still have any basis for saying that novel sounding continuations are not already knowledge? — **no**
3. 3. If the claim that explanatory representations must survive criticism across varied contexts were weakened to success on the training distribution alone, would the conjecture still rule out sophisticated interpolation as genuine knowledge? — **no**
4. 4. If System A were augmented with a larger corpus containing many explicit why, what if, and failure case passages, does the conjecture require saying that it still lacks knowledge unless those answers come from shared internal structure rather than retrieval? — **yes**
5. 5. If System B produced accurate novel continuations by exploiting shallow statistical cues that do not forbid wrong alternatives, would the conjecture classify it as lacking genuine knowledge despite strong predictive performance? — **yes**
6. 6. If the claim that prediction is merely selection pressure were replaced by the claim that the training objective itself guarantees explanation, would the contrast between epicycles and gravity in the conjecture collapse? — **yes**
7. 7. If the internal representations in System B generalized within one domain but could not be recombined across domains, would the conjecture still count them as genuine explanatory knowledge? — **no**
8. 8. If a system could answer what would happen if and why not otherwise only by retrieving near matches from memory rather than by using a compact generative account, does the conjecture say those answers do not qualify as knowledge? — **yes**
9. 9. If the claim that knowledge is substrate independent explanatory structure were changed so that only human interpretable representations count, would the conjecture lose its argument that some token predictors can genuinely know? — **yes**
10. 10. If prediction on novel prompts could be achieved by a huge nearest pattern matcher with no internal variables corresponding to latent structure, would the conjecture still maintain that successful token prediction alone is insufficient for knowledge? — **yes**

## Candidate Problems

- How can 'genuine explanatory knowledge' be operationally distinguished from highly effective compression or interpolation in a token-predicting system? The conjecture hinges on internal 'compressive, counterfactual-supporting representations' that 'survive criticism,' but it leaves open what empirical or theoretical tests would non-circularly identify such structures rather than merely redescribing strong performance. (score: 0.97)
- What is the relationship between internal representation and epistemic status: must explanatory knowledge be localized in identifiable internal structures, or can it be distributed, transient, or only manifest at the system-behavior level? The conjecture assumes a fairly strong internalist criterion, but it remains unresolved whether knowledge attribution should depend on inspectable mechanisms versus robust counterfactual and error-correcting capacities observable only in behavior. (score: 0.90)
- Under what training pressures and architectural constraints does token prediction actually drive the emergence of explanatory models rather than memorization-like pattern reuse? The conjecture proposes prediction as a selection pressure, but the causal pathway is underspecified: which resource limits, data regimes, objectives, or feedback loops make explanatory structure arise, and are there principled phase transitions between retrieval-dominated and explanation-dominated systems? (score: 0.95)
