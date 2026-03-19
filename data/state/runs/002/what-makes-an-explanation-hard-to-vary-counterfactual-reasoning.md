# Generated: what-makes-an-explanation-hard-to-vary × counterfactual-reasoning

## Conjecture

Conjecture: The load-bearing property of an explanation is not primarily about whether its words are novel, nor whether its search space is large, but whether the explanation sits inside a tightly constrained network of counterfactual dependencies such that varying one key part breaks many consequences at once.

Apply the assumption-variation test:

1. Remove novelty of vocabulary.  
Nothing important breaks. Einstein did not need new words; an explanation can be revolutionary while reusing existing language. So lexical novelty is incidental.

2. Invert the search-space assumption: suppose explanation-generation were just sequence search over words.  
What breaks is explanatory necessity. A huge combinatorial space tells us only that brute force is infeasible, not what makes one account good. The load-bearing structure is therefore not at the token level. Search space over sentences is the wrong ontology.

3. Replace “generate explanations” with “generate structures of constraints.”  
Now the problem sharpens. Einstein’s train-and-lightning case is not valuable because it is a clever story, but because its elements are mutually locked:
- constancy of light speed,
- observer-relative simultaneity,
- altered time measurement,
- altered spatial measurement.

Swap one freely and the rest fail to follow. This is what “hard to vary” cashes out to structurally: low degrees of freedom under criticism.

4. Remove criticism from the system.  
Then nothing distinguishes a merely coherent narrative from an explanation whose parts are necessary. So criticism is load-bearing. A system that only proposes cannot create knowledge in the relevant sense; it must also expose which substitutions destroy reach.

5. Invert “details can be swapped freely” into “details are interchangeable if the same outputs remain.”  
This reveals the distinction: incidental details are those whose variation leaves the explanatory consequences invariant. Load-bearing details are those whose variation changes downstream predictions, problem-solutions, or compatibility with other explanations.

So the structural property you are looking for is something like dependency density plus fragility under principled variation. A load-bearing explanation has:
- high consequence coupling: many conclusions depend on a small core;
- low interchangeable redundancy: few central parts can be replaced without loss;
- strong cross-domain reach: the same core constrains multiple phenomena;
- failure under nearby alternatives: plausible variations stop solving the problem.

A freely swappable account has the opposite profile:
- local coherence but weak necessity,
- many paraphrasable or replaceable central moves,
- little loss when assumptions are altered,
- minimal reach beyond the immediate case.

This also illuminates the LLM question. If an architecture can search over structured conceptual relations and systematically test what breaks when assumptions are removed, inverted, or replaced, then it is not merely sampling language; it is probing explanatory load-bearingness. That would be closer to knowledge creation than token generation. The threshold is not “produces new text,” but “discovers structures whose central parts survive criticism while their nearby variants fail.”

## Questions


## Candidate Problems

- Can dependency density, interchangeable redundancy, and failure under nearby alternatives be operationalized into a test that distinguishes hard-to-vary explanations from merely coherent narratives? (score: 0.93)
- In what kinds of explanations do cross-domain reach and low degrees of freedom come apart, producing accounts that are tightly coupled locally but weak globally? (score: 0.82)
- Can an AI system systematically discover load-bearing explanatory parts by structured assumption-variation better than human evaluators, and on what benchmark cases? (score: 0.85)
