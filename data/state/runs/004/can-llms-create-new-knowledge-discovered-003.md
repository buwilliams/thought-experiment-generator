# Generated: can-llms-create-new-knowledge × discovered-003

## Conjecture

**Conjecture:** A token-predicting system can generate explanations that count as genuine knowledge **iff** its internal and external use preserve the domain’s load-bearing abstract structures — the invariants that explanations must track to support compression, counterfactuals, and error-correction. Where it merely reproduces surface regularities, it rearranges seen material; where it latches onto those deeper constraints, it can produce knowledge even if trained by token prediction alone.

The key collision here is between **prediction over sequences** and **explanation as structure-sensitive understanding**. If explanation were just a more fluent summary of past tokens, then token prediction would indeed only remix precedent. But in many domains, the same explanatory content can be expressed in many different token sequences, while bad explanations can be rhetorically polished. So the load-bearing part of explanation is not the wording but the **abstract relation preserved across reformulations**: causal dependency, mathematical necessity, compositional structure, symmetry, mechanism, or lawful constraint.

This reframes the question. The issue is not whether the system was trained on tokens, but whether token prediction can induce models of the underlying invariants that generated those tokens. In physics, mathematics works not by miracle but because both mathematics and physical theory encode the same stable constraints. Likewise here: if a model learns the invariant structures that make explanations explanatory, then its outputs are not mere rearrangements. They are new instantiations of abstract knowledge.

What follows:

1. **Training objective is not destiny.** A system optimized for token prediction may still discover latent structures because predicting well often requires representing them. Compression pressure can favor deeper generative regularities over memorized surface forms.

2. **But success is uneven by domain.** Genuine knowledge is more likely where the domain has strong reusable invariants — logic, mathematics, code, some sciences. It is less likely where discourse is dominated by convention, rhetoric, or weakly constrained social patterns.

3. **Explanation requires counterfactual robustness.** A real explanation should survive paraphrase, support “what if” variation, and help detect its own failure conditions. Outputs that collapse under these tests are signs of rearrangement without understanding.

4. **Knowledge is not located solely inside the model.** A system plus critique loop — tools, experiments, dialogue, error correction — is more knowledge-generating than the isolated predictor. The load-bearing structure includes the feedback mechanisms that expose and eliminate error.

So the sharp distinction is not **prediction vs knowledge**, but **surface correlation vs invariant-tracking structure**. Token prediction is a route by which knowledge can emerge, not a guarantee of it. The system generates genuine knowledge when its explanations are constrained by, and answerable to, the abstract structures that make explanation possible in the first place.

## Questions

1. 1. Would the conjecture still hold if preserving the domain's load-bearing abstract structures were replaced by matching human judgments of explanatory quality on benchmark prompts? — **no**
2. 2. Does the claim that token prediction can yield genuine knowledge depend on the same explanation remaining valid across many paraphrases that change wording but preserve causal or mathematical structure? — **yes**
3. 3. If a system produced fluent answers that compressed past text well but failed on counterfactual variations within the domain, would the conjecture classify those outputs as lacking genuine knowledge? — **yes**
4. 4. Would the conjecture break if the relevant invariants were taken to be statistical regularities in token sequences rather than domain-level constraints such as mechanism, symmetry, or necessity? — **yes**
5. 5. Is the external critique loop a necessary part of the explanation in the conjecture, such that an isolated predictor with the same internal representations would often fail to count as knowledge-generating? — **yes**
6. 6. Does the conjecture require that the same internal structures support both successful prediction and successful error-correction, rather than allowing separate mechanisms for each? — **no**
7. 7. If two domains had equal predictive regularity in text but only one had strong reusable invariants, does the conjecture predict genuine knowledge in one and mere rearrangement in the other? — **yes**
8. 8. Would the conjecture survive if compression pressure in training were removed or weakened so that memorizing surface forms predicted tokens nearly as well as modeling deeper generative structure? — **no**
9. 9. Does the claim that token prediction is not destiny rely on there being many token sequences that express the same explanatory content, making wording non-load-bearing relative to abstract relations? — **yes**
10. 10. If a model tracked domain invariants internally but could not expose failure conditions or revise outputs under criticism, would the conjecture still count its explanations as genuine knowledge? — **no**

## Candidate Problems

- How can 'load-bearing abstract structures' be defined and operationalized non-circularly across domains so that we can tell when an explanation tracks genuine invariants rather than merely fitting evaluation proxies? The conjecture depends on this distinction, but it leaves open what counts as an invariant, how domain-relative that notion is, and how to test preservation without already presupposing the explanatory knowledge we are trying to detect. (score: 0.97)
- Under what conditions does next-token prediction actually induce internal representations of domain invariants rather than high-performing surface heuristics, and what properties of data, architecture, and training dynamics govern that transition? This is the central unresolved mechanism in the conjecture: compression pressure may favor deeper structure, but it is unclear when that pressure is sufficient, when it fails, and whether there are principled thresholds by domain. (score: 0.95)
- Where is the boundary between knowledge generated by the token-predicting model itself and knowledge generated by the larger socio-technical critique loop in which it is embedded? The conjecture says knowledge is not solely inside the model, but this raises an open question about attribution, system boundaries, and whether 'genuine knowledge' should be assigned to the model, the interactive system, or only to the error-correcting process that links outputs to reality. (score: 0.91)
