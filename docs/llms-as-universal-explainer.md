# Can LLMs create knowledge?

## Argument

In the video, [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) by Brett Hall, he considers if LLMs can generate new explanations. It is an attractive argument, if you, like me, enjoy the work of David Deutsch, but there is a criticism I have which has kept me from accepting this argument as the best explanation. That reason is my explanation of search space.

1. Einstein didn't need to invent any new words to explain relativity. This is my first observation, LLMs have access to all the same words as Einstein, more actually (millions more). AI was able to estimate that Einstein knew 50k to 80k words given his multilingualism and intellectual breadth while most people actively use 20k to 35k words.
2. I asked AI to provide a parochial version of Einstein told in first-person as a continuous journal entry including his thought experiments. After a few attempts, you can see what I got below. The important point is that there were 686 total words in the explanation and 289 unique words.
	> June 1895 — Aarau, Switzerland
	> The question will not leave me. What would I see if I could ride alongside a beam of light? I have been turning it over for weeks and I cannot shake it. Classical mechanics says I would see the wave frozen — fields oscillating in place, perfectly still. But I have read Maxwell. There is no such solution in his equations. A frozen light wave does not exist. Which means one of two things: either I cannot reach the speed of light, or the very notion of speed is not what I think it is. I do not know which. But I know something is wrong.
	> March 1901 — Bern, Switzerland
	> Still no position. I write letters. I wait. In the meantime I think. The question about light has not left — it has deepened. I have been trying to understand what it would actually mean for two people moving differently to measure the same beam of light and get the same speed. It seems impossible. And yet Maxwell insists it must be so.
	> June 1905 — Bern, Patent Office
	> I have it — or I think I have it. I was on the tram home when it struck me. I imagined myself standing on a platform, midway between two points where lightning strikes simultaneously. The light from both bolts reaches me at the same moment. They are simultaneous — that much seems obvious. But then I asked: what does my colleague see, sitting in the middle of a moving train passing me at that instant? She is moving toward one strike and away from the other. The light from the forward bolt reaches her first. She says they were not simultaneous. She is not wrong. I am not wrong. We are both right — and simultaneity is simply not absolute. It depends on how you are moving.
	> I could not sleep. If simultaneity is not fixed, then time itself cannot be fixed. A clock on the moving train must tick differently than mine. Not because it is broken — because that is what time actually does. I am writing as fast as I can. Length must also change. And mass — mass and energy must be the same thing at bottom. E = mc². I will submit the paper this month.
	> November 1907 — Bern, Patent Office
	> I was sitting at my desk reviewing a patent when it came to me — completely unbidden. If I were to fall from a height, I would not feel my own weight. For a moment, falling, gravity would simply be gone. I sat very still. This is not a small observation. This is a clue about the nature of gravity itself. I wrote it down immediately: a freely falling observer feels no gravity. I think this is the happiest thought I have ever had. I do not yet know where it leads but I know it leads somewhere important.
	> I have been following it all afternoon. If a man is sealed in a box accelerating upward through empty space, he stands on the floor, feels his weight, drops a pencil and it falls — he cannot distinguish this from gravity. Not in principle. Not even in theory. Gravity and acceleration are the same thing. I am calling this the equivalence principle. Now I must ask what it means for light.
	> If light crosses my accelerating box, the floor moves up slightly while the light travels — so the light appears to bend toward the floor. But if acceleration and gravity are equivalent, then light must bend in a gravitational field. It must. And if light bends — and light always follows the straightest possible path — then the straightest path near a massive object is not what I thought. It is curved. The geometry itself is curved. Gravity is not a force. It is shape. Objects are not being pulled — they are following straight lines through a curved spacetime, and we feel gravity only because the ground is pressing us away from the free fall we would otherwise follow.
	> I need ten years to make this into mathematics. But the thought is there. It is solid. I can see it.
3. Some back of the envelope math using 1,000 words and 50,000 word choices is ~10^5000 possible combinations to reach the words above. That's a big number. There is an estimated ~10^80 atoms in the observable universe which is much smaller than our search space, by ~10^4920. So, pure iteration is practically intractable. We need to narrow the search space. This is one area where people excel! But, there are many ways to design this.
	- I asked AI to analyze the journal above to find the objects and concepts. It helpfully came back with 5 distinct thought experiments and each thought experiment has ~4 objects, ~3 relationships, and ~2 properties. From this, I've designed a sample project to achieve this goal.
4. Since we can narrow the search and have an LLM criticize the resulting explanations, it is possible for LLMs to create new knowledge this way. There are many details to work out: LLM bias, search pool composition (background pool, universal pool, and novel pool), etc. My point is that this is a tractable problem.

---

## The Level Distinction: Substrate vs. Architecture

Progress in physics happens in abstract space, not in the brain tissue of physicists. Einstein's relativity exists there — it is a fact about abstract space, not a fact about his neurons. The same is true of mathematical theorems, logical arguments, and any genuine explanation. Neurons are the substrate. Ideas are what travel through them. We do not say that Einstein's discovery lived in any particular neuron, or that it would have ceased to exist if his brain had been wired differently. The explanation has independent existence in the space of all possible explanations.

An LLM is analogous to a neuron cluster: it performs pattern completion at enormous scale. The token stream is to the LLM what neural firing is to the brain — the mechanism, not the meaning. When we ask whether LLMs can create knowledge, we are asking the wrong question if we are pointing at the model. The model is substrate.

The right question is whether an architecture built above the model can operate in the space where knowledge lives. This distinction matters because it changes what we should build. The answer to the search space problem is not a bigger model — it is a better structure for steering through that space. Scaling the substrate does not change the geometry of the problem; it only makes each step faster.

This is already visible in practice. Projects like OpenClaw, AI Research agents, and Scientific Researchers agents all follow this pattern: they use LLMs as execution substrate and organize reasoning at a higher level. They are existence proofs that the architecture layer is where the leverage is. None of them represent progress by making a better LLM. All of them represent progress by designing a better structure above the LLM.

This system is the generalization of that principle. Conjectures, problems, evaluations, and promotion cycles are not model features — they are the structure of how explanations grow in abstract space. The LLM executes one step at a time, at the token level. The architecture operates at the level of ideas: what to conjecture, what to test it against, what survives criticism, what gets promoted. That is the level where knowledge actually grows.

---

## Response to Brett

Brett, I think the "inductive closure" argument undersells what's possible. Not through scaling, but through architecture.

I asked an LLM to write Einstein's theory of relativity as a first-person journal. The result was 686 words, 289 unique. Einstein himself likely had a vocabulary of 50,000-80,000 words. So the search space for producing that journal, even at just 1,000 words from 50,000 choices, is ~10^4,699 combinations. The observable universe has ~10^80 atoms. We're short by 10^4,619. Pure iteration doesn't practically work.

But tractability doesn't require covering the space. It requires collapsing it, through layered filters: grammar, coherence, then explanatory quality judged by Deutschian criteria: hard to vary, reach beyond inputs, minimal assumptions. Once you operationalize the Deutsch score as a filter, you don't need the model to "understand" anything. You draw from a prior biased by background knowledge and reject poor candidates before they propagate.

I've designed a [sample experiment](https://github.com/buwilliams/thought-experiment-generator) to test this. The claim: given "why does light always travel at the same speed regardless of the observer," a depth-bounded branching search should surface a thought experiment structurally equivalent to Einstein's train and lightning bolts. Einstein is the benchmark, not the model.

That's not scaling. It's a different architecture entirely.

The key conjecture is that a narrowing search architecture algorithm, guided by human judgment, improves with each pass, and at a critical threshold begins producing genuine novelty. Layer filters that collapse the search space, compound discoveries back into the draw pool, and the results get better at every depth. Humans are still the judge, for now. But past that threshold, the system's own outputs become reliable enough to judge its next outputs. At that point, it can recursively self-improve, not by scaling compute, but by narrowing the space it searches until what remains is genuinely new.