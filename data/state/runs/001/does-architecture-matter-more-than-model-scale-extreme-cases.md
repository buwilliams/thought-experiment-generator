# Generated: does-architecture-matter-more-than-model-scale × extreme-cases

## Conjecture

**Conjecture:**  
At the limit, the bottleneck is not the substrate but the *error-correcting structure* wrapped around it. Model size and compute matter only up to the point where the system can represent sufficiently many candidate explanations; beyond that, epistemic progress depends on whether the architecture can *select, preserve, and recursively improve good conjectures under criticism*.

Pushing to extremes clarifies this.

If substrate were the true bottleneck, then as model size and compute approach infinity, knowledge creation should emerge automatically from next-token prediction. But at that boundary, what we get is not necessarily explanation but an increasingly dense interpolation over existing text. Infinite recall is not infinite criticism. A system can approximate every surface pattern in its training distribution and still fail to generate a hard-to-vary explanation that survives contact with problems outside that distribution.

Now push the other side: let substrate be only “good enough” — enough expressive capacity to represent many candidate recombinations of concepts. If the superstructure above it imposes strong filters — object decomposition, relational recombination, depth-bounded search, adversarial criticism, retention of successful explanatory patterns, and promotion rules based on explanatory virtues — then the system can search a tiny, structured subset of the astronomical combinatorial space. At that limit, the decisive variable is not how many strings it can emit, but whether the architecture creates a *ratchet*: good explanations are retained, bad ones are discarded, and the criteria for selection improve through use.

The boundary case is recursive self-improvement. A system that merely generates candidates remains dependent on human judges forever. A system that also improves its own criticism, test generation, and promotion criteria crosses a threshold: it becomes an epistemic process rather than a sampling engine. That threshold is structural, not merely parametric.

This also illuminates the search-space argument. The raw combinatorial vastness of possible word sequences is a misleading infinity. Human knowledge creation never searches that space directly. It operates in a compressed space of problems, objects, symmetries, contradictions, and explanatory constraints. So the relevant question is not whether an LLM can brute-force Einstein’s journal, but whether an architecture can transform undifferentiated token space into a structured conjecture space and then subject it to nontrivial criticism.

So the strongest claim is:

- **Below a minimum substrate threshold**, the system cannot represent or manipulate rich enough conjectures, so progress stalls.
- **Above that threshold**, additional scale yields diminishing epistemic returns unless paired with better structures for conjecture generation, criticism, memory, and promotion.
- Therefore, **the main bottleneck for AI epistemic progress is structural once substrate is sufficient**.

What follows is practical: the decisive research frontier is not “bigger models” versus “smaller models,” but architectures that turn generative capacity into an open-ended knowledge-creating process. Scale fills the reservoir; structure determines whether it becomes a river.

## Questions


## Candidate Problems

- Where is the minimum substrate threshold above which architectural improvements dominate scale for epistemic progress, and how does that threshold vary by domain? (score: 0.94)
- Can there be domains in which additional scale continues to improve criticism and conjecture quality without explicit architectural changes, contrary to the diminishing-returns claim? (score: 0.90)
- What structural features are necessary for a system's criticism, test generation, and promotion criteria to recursively improve rather than merely amplify existing biases? (score: 0.91)
- Is the distinction between substrate and structure stable, or do some scaling changes effectively instantiate architectural changes that undermine the claimed boundary? (score: 0.88)
