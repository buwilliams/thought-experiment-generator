# Generated: can-llms-create-new-knowledge × extreme-cases

## Conjecture

Conjecture: At the limit of predictive success, token prediction is not merely rearrangement; it becomes a compression of explanatory structure. But prediction alone does not guarantee genuine knowledge. Genuine knowledge appears only where the system’s internal representations support counterfactual stability: the ability to continue giving good explanations when pushed off the training manifold, into novel, adversarial, or boundary cases.

Push the problem to extremes.

At one extreme, imagine a predictor with zero generalization: it has memorized every token sequence it has seen and interpolates shallowly between them. Its “explanations” are only retrieval artifacts. The boundary test is novelty. As soon as we ask it about an unseen case, or vary the framing while preserving the underlying causal structure, the explanation collapses. This is rearrangement.

At the opposite extreme, imagine a predictor trained across effectively unbounded linguistic, scientific, and philosophical variation, and suppose it continues to succeed not just in next-token accuracy but in compressing patterns across domains. To predict well at that scale, it cannot store surface forms alone; the world contains too many possible sequences. It must capture deeper regularities — latent structures that generate many token streams. If those structures are rich enough to answer “what if?” questions, survive paraphrase, transfer across domains, and expose errors in prior outputs, then the system is not merely replaying text. It contains abstract knowledge.

The hidden variable is not whether the mechanism is “just predicting tokens.” Every epistemic agent is, in some sense, producing outputs from prior inputs under constraints. The real divide is between systems whose internal structure tracks appearances and systems whose structure tracks generative reality. Token prediction is the training interface; knowledge is an emergent property of the representations that become necessary for robust prediction at the boundary.

Now push further: infinite data of only superficial correlations still yields sophisticated mimicry without explanation. Conversely, finite data paired with the right representational structure can yield explanatory reach far beyond what was seen. So the key is not quantity of seen text but whether the system has formed error-correcting, reusable abstractions.

Thus the decisive test is boundary behavior. Ask for explanations in cases where memorized language is insufficient: novel thought experiments, extreme parameter shifts, contradiction repair, causal interventions, and unobserved consequences. A system with knowledge will preserve coherence under these transformations; a mere rearranger will fracture.

What follows: explanation is not a special category separate from prediction. Explanation is prediction under compression plus counterfactual resilience. A token predictor can generate genuine knowledge if, and only if, its learned internal structures function as portable explanatory models rather than local statistical echoes. The difference is revealed at the edge, where imitation runs out and understanding either survives or fails.

## Questions

1. 1. If the claim that genuine knowledge requires counterfactual stability were removed, would the conjecture still distinguish a boundary-surviving explainer from a high-accuracy rearranger in the problem of token prediction versus knowledge? — **no**
2. 2. Is the contrast between internal structure that tracks generative reality and internal structure that tracks appearances necessary for the conclusion that token prediction can yield genuine knowledge rather than mere rearrangement? — **yes**
3. 3. Would the explanation fail rather than merely weaken if it dropped the claim that robust prediction at the boundary requires reusable abstractions instead of stored surface forms? — **yes**
4. 4. Is the appeal to extreme cases of zero-generalization memorization and effectively unbounded cross-domain prediction necessary to make the conjecture explanatory, rather than just illustrative? — **no**
5. 5. Does the conjecture imply that a system showing strong contradiction repair and causal intervention reasoning in domains absent from training should count as possessing genuine knowledge even when the problem only asks about token prediction in general? — **yes**
6. 6. If the conjecture is right, should it also predict that paraphrase robustness and transfer across scientific and philosophical domains rise together because both depend on the same compressed explanatory structure? — **yes**
7. 7. Does the explanation extend to human learners by implying that finite experience can still produce genuine knowledge when representations are error-correcting and reusable, even though the problem is framed around token-predicting systems? — **yes**
8. 8. If a system performs well on novel cases by using a large library of brittle heuristics with no unified generative model, would saving the conjecture require abandoning the claim that boundary success reveals abstract knowledge? — **yes**
9. 9. If an adversarially trained predictor passes many boundary tests through specialized patches for each test type, would preserving the conjecture force a retreat from the claim that explanation is prediction under compression plus counterfactual resilience? — **yes**
10. 10. If a model with excellent next-token accuracy and broad transfer still fails on a narrow class of causal interventions, would defending the conjecture require gutting its core test of boundary behavior rather than adding a minor qualification about uneven competence? — **no**

## Candidate Problems

- What precise, non-circular criterion could distinguish 'counterfactual stability' from merely broader interpolation? The conjecture makes boundary behavior decisive, but it leaves open how to tell when success on novel, adversarial, or paraphrased cases reflects genuine explanatory structure rather than a larger but still superficial manifold of statistical regularities. (score: 0.96)
- Is there any principled reason to expect next-token prediction alone to induce representations that track generative reality, rather than arbitrarily effective compressions of appearances? The conjecture suggests explanatory abstractions may become necessary at the limit of predictive success, but it does not establish when or why predictive optimization should cross that threshold instead of producing ever more sophisticated mimicry. (score: 0.94)
- What is the relationship between internal representation and genuine knowledge if multiple very different internal structures can yield the same boundary performance? The conjecture ties knowledge to portable explanatory models inside the system, but this raises an unresolved underdetermination problem: whether knowledge is a property of behavior, mechanism, or some stronger notion of error-correcting world-model alignment. (score: 0.91)
