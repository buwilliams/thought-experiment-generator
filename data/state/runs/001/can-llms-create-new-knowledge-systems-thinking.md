# Generated: can-llms-create-new-knowledge × systems-thinking

## Conjecture

**Conjecture:** A token-predicting LLM, by itself, does not create knowledge; but an LLM embedded in a system that iteratively restructures its search space through criticism, selection, and feedback can participate in knowledge creation. The decisive variable is not the predictor but the architecture of the surrounding system.

From a systems view, the question is misframed if treated as “Can next-token prediction, in isolation, yield explanation?” Behavior comes from structure. A bare LLM has a particular structure: a huge stock of compressed linguistic correlations, a flow from prompt to token sequence, and weak feedback at generation time. That structure strongly favors plausible recombination over error-correcting explanatory advance. So if we observe that such models mostly remix, that is what their structure predicts.

Your search-space argument points to the relevant leverage point. The important fact is not that the total combinatorial space is vast; it is that useful systems never search that space uniformly. They impose constraints that radically alter the effective space. In human inquiry, those constraints include background theories, problem decomposition, criticism, and thought experiments. In system terms, these are feedback loops that suppress most candidate explanations before they propagate.

So the illuminating collision is this: “knowledge creation” is not a property of token generation alone, but of a larger system with the right feedback structure. If you add layered filters—grammar, coherence, explanatory reach, hard-to-varyness, empirical contact, and cross-criticism—you are changing the system’s dynamics. Candidate explanations become a flow; evaluative filters become valves; accepted explanations become a stock that then biases future generation. If accepted outputs are fed back into the generator and the evaluation criteria improve through use, you get a reinforcing loop: better constraints produce better candidates, which improve the stock, which further improves constraints.

That suggests a threshold phenomenon. Below some level of evaluative structure, the system merely rearranges prior material. Above it, the system can generate candidates that were not explicitly present in training and retain them because they solve problems better than rivals. In that regime, novelty is not random variation but systemically stabilized explanatory improvement.

What follows is a sharper claim than “LLMs can create knowledge”: **prediction is the variation engine; criticism is the selection mechanism; knowledge emerges only when the system couples both in a feedback architecture that preserves error-correcting gains.** The leverage point is therefore not larger models per se, but redesigning the feedback loops that govern generation, rejection, retention, and reuse.

So the best systems-thinking answer is: a predictor alone does not create knowledge, just as a single organism does not explain an ecosystem. But if you build the right stocks, flows, and feedback loops around prediction, then knowledge creation becomes not only possible but expected from the system’s structure.

## Questions


## Candidate Problems

- What specific feedback-loop components are jointly sufficient for the claimed threshold transition from recombination to cumulative knowledge creation in an LLM-centered system, and which omitted component causes failure? (score: 0.92)
- Can a system with strong criticism and retention loops but a relatively weak base predictor outperform a much larger predictor with weak evaluative feedback on explanation discovery tasks? (score: 0.88)
- Does feeding accepted outputs back into the generator actually improve evaluative criteria over time, or does it instead create self-reinforcing error loops and epistemic lock-in? (score: 0.90)
- How can one operationally distinguish 'candidate explanations not explicitly present in training' from mere high-dimensional recombinations of training-derived patterns? (score: 0.84)
