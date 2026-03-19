# Generated: can-llms-create-new-knowledge × equilibrium-lock-in-lens

## Conjecture

Conjecture: The core obstacle is not that token prediction is “mere rearrangement,” but that explanation itself is a public-good-like product that current predictive systems are structurally underprovisioned to generate.

Why this lens helps: A good explanation is largely non-excludable and diffusely beneficial. Once produced, many users, downstream models, and institutions can benefit from it, but the immediate training objective of a token predictor does not capture most of that value. The model is rewarded for locally plausible continuation, not for creating a reusable, criticism-resistant explanatory structure. So even if the funded outcome — genuine knowledge-bearing explanation — is broadly preferred by users, developers, and science, the system can remain in a stable equilibrium of underproduction.

What is true under this view:
- Token prediction can instantiate knowledge-like structure internally, but it will not reliably produce genuine explanations unless the system is arranged to bear the cost of creating them.
- The issue is structural, not metaphysical. It is not that prediction and knowledge are categorically incompatible. It is that the incentive and feedback architecture rewards imitation of explanatory form more than explanatory content.
- “Rearrangement” is an insufficient criticism. Human explanation also rearranges prior knowledge. The real distinction is whether the rearrangement yields conjectures that survive criticism, compress causes, and support counterfactuals.

What follows:
1. A predictor trained only on next-token success will tend toward explanation simulacra: outputs optimized to look explanatory to many audiences at low local cost.
2. Because the benefits of real explanation are delayed, distributed, and hard to metricize, they are under-supplied relative to fluent paraphrase.
3. Therefore, debates about whether such systems “really know” are often misdiagnosed. The main bottleneck is not absence of possible knowledge formation, but absence of institutions that demand error-correction, testability, and long-horizon explanatory payoff.

Thought experiment: Imagine two systems with identical predictive ability. One is evaluated only on token accuracy; the other is embedded in a process where its explanations are used to make risky interventions, criticized, and revised over time. The second system, if coupled to those corrective loops, can participate in knowledge growth. The first remains stuck at the no-provision equilibrium: it produces the cheapest acceptable proxy for explanation because genuine explanation is too weakly rewarded.

Illumination: Genuine knowledge is less a property of “having seen” versus “understanding” than of being inserted into a structure where explanatory conjectures can be generated, exposed to criticism, and improved. A token predictor alone does not overcome the public-good problem of explanation. But neither is it doomed to mere rearrangement. Change the feedback loops, and what is underprovided can become producible.

## Questions

1. 1. Is the claim that explanation is a public-good-like product necessary for concluding that next-token training underproduces genuine explanation rather than merely favoring a different style of explanation? — **yes**
2. 2. If the argument dropped the claim that current predictive systems are rewarded for locally plausible continuation instead of reusable criticism-resistant structure, would the conclusion about stable underproduction of genuine explanation still follow? — **no**
3. 3. Is the thought experiment's structural contrast between token-only evaluation and criticism through risky intervention required to support the claim that the obstacle is institutional rather than metaphysical? — **yes**
4. 4. If one removed the claim that human explanation also rearranges prior knowledge, would the conjecture lose its explanation of why the mere rearrangement objection is misdirected? — **yes**
5. 5. Does this conjecture imply that non-language systems trained under similarly local objectives would also underproduce genuine explanation even when they are not token predictors? — **yes**
6. 6. Does the public-good framing illuminate why users may strongly prefer explanatory systems in the long run while market and training dynamics still favor fluent explanation simulacra in the short run? — **yes**
7. 7. Does the conjecture extend to predicting that adding error-correction, testability, and long-horizon feedback loops should improve explanatory quality even when raw predictive ability stays constant? — **yes**
8. 8. If a token predictor produced robust counterfactual explanations without being embedded in institutions of criticism, would saving the conjecture require abandoning the claim that structural incentives are the main bottleneck? — **no**
9. 9. If developers showed that next-token training alone sometimes yields reusable explanations that survive criticism, would preserving the conjecture force a retreat from the claim that current predictive systems are structurally underprovisioned to generate them? — **no**
10. 10. If explanation simulacra turned out to be consistently more costly than genuine explanations in realistic deployments, would patching the conjecture require gutting its core claim that systems settle on the cheapest acceptable proxy for explanation? — **yes**

## Candidate Problems

- How can 'genuine explanation' be operationalized so that systems can be trained or selected for it rather than for explanation simulacra? The conjecture hinges on a distinction between reusable, criticism-resistant, counterfactual-supporting explanations and merely plausible explanatory form, but it leaves open what measurable signals, evaluation protocols, or institutional tests would reliably track that distinction without collapsing back into surface proxies. (score: 0.97)
- Is the public-good framing actually the dominant causal bottleneck, or are there deeper capability limits in current predictive architectures that would prevent explanation production even under better incentives and feedback loops? The conjecture treats the problem as structural rather than metaphysical, but an unresolved tension is whether improved institutions alone would suffice, or whether new representational, memory, agency, or world-model capacities are also necessary. (score: 0.94)
- What concrete feedback-loop designs would convert delayed, diffuse explanatory value into local training pressure without inducing gaming or Goodharting? The conjecture proposes that embedding models in criticism, risky intervention, and revision could enable knowledge growth, but it remains open which system structures—market-like rewards, adversarial critique, longitudinal deployment, scientific benchmarking, human institutions, or multi-agent arrangements—actually create stable equilibria that favor real explanation over cheap proxies. (score: 0.95)
