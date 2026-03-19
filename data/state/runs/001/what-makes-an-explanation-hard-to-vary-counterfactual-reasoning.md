# Generated: what-makes-an-explanation-hard-to-vary × counterfactual-reasoning

## Conjecture

**Conjecture:** An explanation is “all load-bearing” not because its words are rare or its search space is vast, but because its internal constraints are tightly coupled: if you remove, invert, or replace one central assumption, the explanation loses reach, coherence, or problem-solving power. By contrast, an explanation with swappable details is one whose parts are only locally fitted—changing components leaves the overall output fluent but does not alter what the account can actually explain.

Vary the assumptions behind the search-space argument.

First, **remove the assumption that combinatorial size is the key obstacle**. Then the important distinction is no longer between brute force and tractable search, but between **spaces structured by explanatory constraints** and spaces structured only by stylistic plausibility. Einstein did not search over word strings; he searched over conjectural structures constrained by contradictions between Maxwell, mechanics, and thought experiments. So what is load-bearing in relativity is not the wording of “train,” “lightning,” or “platform,” but the necessity that simultaneity become frame-relative if light speed is invariant.

Second, **invert the assumption that novelty comes from generating many candidate combinations and filtering them**. Suppose instead that novelty comes from exposing and resolving conflict among existing ideas. Then an explanation is load-bearing when its parts participate in a network of mutual necessity: alter one piece and the conflict resolution fails. In the train thought experiment, swap “light speed is invariant” for “light speed adds classically,” and relativity of simultaneity no longer follows. Swap “simultaneity is observer-dependent” back to “absolute,” and Maxwell plus observation become inconsistent. That is a strong indicator of load-bearing structure.

Third, **replace the assumption that linguistic recombination is the relevant search space with the opposite: the relevant space is a space of problems, constraints, and transformations.** On this view, details are incidental when they can be exchanged without changing the dependency structure. A train can become a spaceship; lightning can become synchronized lamps. If the explanatory consequences remain intact, those surface elements are not load-bearing. But if changing a component breaks the chain of necessity, that component is structural.

So the distinguishing property is:

1. **Counterfactual fragility at the principle level** — changing a core assumption breaks the explanation’s success.
2. **Counterfactual robustness at the representational level** — changing surface imagery or wording does not.
3. **High dependency density** — many consequences flow from few principles, and those principles constrain one another.
4. **Error sensitivity** — the explanation forbids nearby alternatives rather than merely coexisting with them.

This suggests a sharper test for whether LLM systems can create knowledge. Not: can they produce novel-looking accounts from a huge space? But: **can they generate structures whose core assumptions are hard to vary without collapse, while their surface realizations remain freely variable?** If yes, they are not merely sampling prose; they are participating in explanatory discovery.

## Questions


## Candidate Problems

- Can dependency density be operationalized so that two explanations of the same phenomenon can be compared for how many consequences flow from how few mutually constraining principles? (score: 0.92)
- How can one distinguish principle-level counterfactual fragility from mere overfitting, where an explanation breaks under variation because it is brittle rather than because it is deeply constrained? (score: 0.91)
- Can an automated system reliably separate representational substitutions that preserve dependency structure from principle substitutions that destroy explanatory necessity? (score: 0.90)
- Are there domains where genuinely good explanations are not highly hard-to-vary, and if so, what structural feature replaces dependency density there? (score: 0.86)
