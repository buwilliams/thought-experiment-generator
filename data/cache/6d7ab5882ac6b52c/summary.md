# # Can LLMs create knowledge?

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

## Response to Brett

Brett, I think the "inductive closure" argument undersells what's possible. Not through scaling, but through architecture.

I asked an LLM to write Einstein's theory of relativity as a first-person journal. The result was 686 words, 289 unique. Einstein himself likely had a vocabulary of 50,000-80,000 words. So the search space for producing that journal, even at just 1,000 words from 50,000 choices, is ~10^4,699 combinations. The observable universe has ~10^80 atoms. We're short by 10^4,619. Pure iteration doesn't practically work.

But tractability doesn't require covering the space. It requires collapsing it, through layered filters: grammar, coherence, then explanatory quality judged by Deutschian criteria: hard to vary, reach beyond inputs, minimal assumptions. Once you operationalize the Deutsch score as a filter, you don't need the model to "understand" anything. You draw from a prior biased by background knowledge and reject poor candidates before they propagate.

I've designed a [sample experiment](https://github.com/buwilliams/thought-experiment-generator) to test this. The claim: given "why does light always travel at the same speed regardless of the observer," a depth-bounded branching search should surface a thought experiment structurally equivalent to Einstein's train and lightning bolts. Einstein is the benchmark, not the model.

That's not scaling. It's a different architecture entirely.

The key conjecture is that a narrowing search architecture algorithm, guided by human judgment, improves with each pass, and at a critical threshold begins producing genuine novelty. Layer filters that collapse the search space, compound discoveries back into the draw pool, and the results get better at every depth. Humans are still the judge, for now. But past that threshold, the system's own outputs become reliable enough to judge its next outputs. At that point, it can recursively self-improve, not by scaling compute, but by narrowing the space it searches until what remains is genuinely new.

| # | Experiment | Reach | Novelty | Falsifiable | Total |
|---|-----------|-------|---------|-------------|-------|
| 1 | [091-experiment.txt](experiments/091-experiment.txt) | 0.63 | 0.82 | 0.78 | 2.23 |
| 2 | [045-experiment.txt](experiments/045-experiment.txt) | 0.63 | 0.82 | 0.58 | 2.03 |
| 3 | [021-experiment.txt](experiments/021-experiment.txt) | 0.42 | 0.74 | 0.81 | 1.97 |
| 4 | [019-experiment.txt](experiments/019-experiment.txt) | 0.31 | 0.78 | 0.84 | 1.93 |
| 5 | [061-experiment.txt](experiments/061-experiment.txt) | 0.42 | 0.78 | 0.71 | 1.91 |
| 6 | [071-experiment.txt](experiments/071-experiment.txt) | 0.42 | 0.79 | 0.68 | 1.89 |
| 7 | [076-experiment.txt](experiments/076-experiment.txt) | 0.42 | 0.76 | 0.71 | 1.89 |
| 8 | [017-experiment.txt](experiments/017-experiment.txt) | 0.31 | 0.72 | 0.84 | 1.87 |
| 9 | [096-experiment.txt](experiments/096-experiment.txt) | 0.34 | 0.71 | 0.82 | 1.87 |
| 10 | [014-experiment.txt](experiments/014-experiment.txt) | 0.46 | 0.74 | 0.63 | 1.83 |
| 11 | [007-experiment.txt](experiments/007-experiment.txt) | 0.31 | 0.72 | 0.78 | 1.81 |
| 12 | [023-experiment.txt](experiments/023-experiment.txt) | 0.34 | 0.68 | 0.79 | 1.81 |
| 13 | [088-experiment.txt](experiments/088-experiment.txt) | 0.42 | 0.78 | 0.61 | 1.81 |
| 14 | [073-experiment.txt](experiments/073-experiment.txt) | 0.42 | 0.76 | 0.61 | 1.79 |
| 15 | [050-experiment.txt](experiments/050-experiment.txt) | 0.36 | 0.79 | 0.62 | 1.77 |
| 16 | [063-experiment.txt](experiments/063-experiment.txt) | 0.42 | 0.74 | 0.61 | 1.77 |
| 17 | [035-experiment.txt](experiments/035-experiment.txt) | 0.43 | 0.71 | 0.62 | 1.76 |
| 18 | [051-experiment.txt](experiments/051-experiment.txt) | 0.42 | 0.76 | 0.58 | 1.76 |
| 19 | [093-experiment.txt](experiments/093-experiment.txt) | 0.39 | 0.72 | 0.63 | 1.74 |
| 20 | [013-experiment.txt](experiments/013-experiment.txt) | 0.34 | 0.72 | 0.66 | 1.72 |
| 21 | [031-experiment.txt](experiments/031-experiment.txt) | 0.34 | 0.72 | 0.66 | 1.72 |
| 22 | [069-experiment.txt](experiments/069-experiment.txt) | 0.34 | 0.72 | 0.66 | 1.72 |
| 23 | [009-experiment.txt](experiments/009-experiment.txt) | 0.29 | 0.68 | 0.74 | 1.71 |
| 24 | [001-experiment.txt](experiments/001-experiment.txt) | 0.28 | 0.67 | 0.74 | 1.69 |
| 25 | [003-experiment.txt](experiments/003-experiment.txt) | 0.31 | 0.72 | 0.66 | 1.69 |
| 26 | [072-experiment.txt](experiments/072-experiment.txt) | 0.34 | 0.72 | 0.63 | 1.69 |
| 27 | [090-experiment.txt](experiments/090-experiment.txt) | 0.31 | 0.72 | 0.66 | 1.69 |
| 28 | [004-experiment.txt](experiments/004-experiment.txt) | 0.34 | 0.71 | 0.62 | 1.67 |
| 29 | [011-experiment.txt](experiments/011-experiment.txt) | 0.31 | 0.72 | 0.64 | 1.67 |
| 30 | [029-experiment.txt](experiments/029-experiment.txt) | 0.34 | 0.72 | 0.61 | 1.67 |
| 31 | [099-experiment.txt](experiments/099-experiment.txt) | 0.34 | 0.72 | 0.61 | 1.67 |
| 32 | [070-experiment.txt](experiments/070-experiment.txt) | 0.33 | 0.62 | 0.71 | 1.66 |
| 33 | [057-experiment.txt](experiments/057-experiment.txt) | 0.34 | 0.72 | 0.58 | 1.64 |
| 34 | [020-experiment.txt](experiments/020-experiment.txt) | 0.34 | 0.71 | 0.58 | 1.63 |
| 35 | [058-experiment.txt](experiments/058-experiment.txt) | 0.34 | 0.68 | 0.61 | 1.63 |
| 36 | [089-experiment.txt](experiments/089-experiment.txt) | 0.31 | 0.74 | 0.58 | 1.63 |
| 37 | [092-experiment.txt](experiments/092-experiment.txt) | 0.34 | 0.71 | 0.58 | 1.63 |
| 38 | [005-experiment.txt](experiments/005-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 39 | [010-experiment.txt](experiments/010-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 40 | [038-experiment.txt](experiments/038-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 41 | [046-experiment.txt](experiments/046-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 42 | [049-experiment.txt](experiments/049-experiment.txt) | 0.24 | 0.58 | 0.79 | 1.61 |
| 43 | [059-experiment.txt](experiments/059-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 44 | [064-experiment.txt](experiments/064-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 45 | [077-experiment.txt](experiments/077-experiment.txt) | 0.31 | 0.72 | 0.58 | 1.61 |
| 46 | [085-experiment.txt](experiments/085-experiment.txt) | 0.28 | 0.71 | 0.62 | 1.61 |
| 47 | [065-experiment.txt](experiments/065-experiment.txt) | 0.34 | 0.68 | 0.57 | 1.59 |
| 48 | [033-experiment.txt](experiments/033-experiment.txt) | 0.34 | 0.68 | 0.56 | 1.58 |
| 49 | [037-experiment.txt](experiments/037-experiment.txt) | 0.29 | 0.68 | 0.61 | 1.58 |
| 50 | [039-experiment.txt](experiments/039-experiment.txt) | 0.31 | 0.68 | 0.57 | 1.56 |
| 51 | [042-experiment.txt](experiments/042-experiment.txt) | 0.31 | 0.68 | 0.57 | 1.56 |
| 52 | [100-experiment.txt](experiments/100-experiment.txt) | 0.31 | 0.68 | 0.57 | 1.56 |
| 53 | [054-experiment.txt](experiments/054-experiment.txt) | 0.31 | 0.68 | 0.56 | 1.55 |
| 54 | [041-experiment.txt](experiments/041-experiment.txt) | 0.22 | 0.58 | 0.74 | 1.54 |
| 55 | [080-experiment.txt](experiments/080-experiment.txt) | 0.34 | 0.74 | 0.46 | 1.54 |
| 56 | [002-experiment.txt](experiments/002-experiment.txt) | 0.29 | 0.68 | 0.56 | 1.53 |
| 57 | [056-experiment.txt](experiments/056-experiment.txt) | 0.31 | 0.68 | 0.54 | 1.53 |
| 58 | [043-experiment.txt](experiments/043-experiment.txt) | 0.42 | 0.78 | 0.31 | 1.51 |
| 59 | [008-experiment.txt](experiments/008-experiment.txt) | 0.24 | 0.68 | 0.58 | 1.50 |
| 60 | [024-experiment.txt](experiments/024-experiment.txt) | 0.31 | 0.62 | 0.56 | 1.49 |
| 61 | [053-experiment.txt](experiments/053-experiment.txt) | 0.31 | 0.62 | 0.56 | 1.49 |
| 62 | [060-experiment.txt](experiments/060-experiment.txt) | 0.31 | 0.72 | 0.46 | 1.49 |
| 63 | [095-experiment.txt](experiments/095-experiment.txt) | 0.33 | 0.72 | 0.44 | 1.49 |
| 64 | [006-experiment.txt](experiments/006-experiment.txt) | 0.34 | 0.71 | 0.42 | 1.47 |
| 65 | [067-experiment.txt](experiments/067-experiment.txt) | 0.31 | 0.72 | 0.44 | 1.47 |
| 66 | [098-experiment.txt](experiments/098-experiment.txt) | 0.34 | 0.72 | 0.41 | 1.47 |
| 67 | [052-experiment.txt](experiments/052-experiment.txt) | 0.18 | 0.56 | 0.72 | 1.46 |
| 68 | [086-experiment.txt](experiments/086-experiment.txt) | 0.28 | 0.61 | 0.57 | 1.46 |
| 69 | [068-experiment.txt](experiments/068-experiment.txt) | 0.24 | 0.63 | 0.58 | 1.45 |
| 70 | [022-experiment.txt](experiments/022-experiment.txt) | 0.34 | 0.68 | 0.41 | 1.43 |
| 71 | [075-experiment.txt](experiments/075-experiment.txt) | 0.34 | 0.68 | 0.41 | 1.43 |
| 72 | [079-experiment.txt](experiments/079-experiment.txt) | 0.31 | 0.68 | 0.44 | 1.43 |
| 73 | [027-experiment.txt](experiments/027-experiment.txt) | 0.28 | 0.52 | 0.61 | 1.41 |
| 74 | [026-experiment.txt](experiments/026-experiment.txt) | 0.31 | 0.68 | 0.42 | 1.41 |
| 75 | [040-experiment.txt](experiments/040-experiment.txt) | 0.31 | 0.68 | 0.42 | 1.41 |
| 76 | [062-experiment.txt](experiments/062-experiment.txt) | 0.31 | 0.68 | 0.42 | 1.41 |
| 77 | [094-experiment.txt](experiments/094-experiment.txt) | 0.28 | 0.72 | 0.41 | 1.41 |
| 78 | [074-experiment.txt](experiments/074-experiment.txt) | 0.31 | 0.68 | 0.39 | 1.38 |
| 79 | [082-experiment.txt](experiments/082-experiment.txt) | 0.18 | 0.63 | 0.56 | 1.37 |
| 80 | [015-experiment.txt](experiments/015-experiment.txt) | 0.28 | 0.67 | 0.41 | 1.36 |
| 81 | [025-experiment.txt](experiments/025-experiment.txt) | 0.28 | 0.62 | 0.46 | 1.36 |
| 82 | [047-experiment.txt](experiments/047-experiment.txt) | 0.28 | 0.67 | 0.41 | 1.36 |
| 83 | [055-experiment.txt](experiments/055-experiment.txt) | 0.28 | 0.62 | 0.46 | 1.36 |
| 84 | [028-experiment.txt](experiments/028-experiment.txt) | 0.18 | 0.56 | 0.62 | 1.36 |
| 85 | [044-experiment.txt](experiments/044-experiment.txt) | 0.18 | 0.61 | 0.57 | 1.36 |
| 86 | [036-experiment.txt](experiments/036-experiment.txt) | 0.31 | 0.72 | 0.28 | 1.31 |
| 87 | [081-experiment.txt](experiments/081-experiment.txt) | 0.31 | 0.58 | 0.42 | 1.31 |
| 88 | [030-experiment.txt](experiments/030-experiment.txt) | 0.22 | 0.58 | 0.46 | 1.26 |
| 89 | [032-experiment.txt](experiments/032-experiment.txt) | 0.28 | 0.67 | 0.31 | 1.26 |
| 90 | [016-experiment.txt](experiments/016-experiment.txt) | 0.18 | 0.42 | 0.61 | 1.21 |
| 91 | [048-experiment.txt](experiments/048-experiment.txt) | 0.28 | 0.62 | 0.31 | 1.21 |
| 92 | [078-experiment.txt](experiments/078-experiment.txt) | 0.31 | 0.72 | 0.18 | 1.21 |
| 93 | [084-experiment.txt](experiments/084-experiment.txt) | 0.31 | 0.68 | 0.22 | 1.21 |
| 94 | [083-experiment.txt](experiments/083-experiment.txt) | 0.31 | 0.62 | 0.24 | 1.17 |
| 95 | [097-experiment.txt](experiments/097-experiment.txt) | 0.31 | 0.62 | 0.24 | 1.17 |
| 96 | [012-experiment.txt](experiments/012-experiment.txt) | 0.28 | 0.62 | 0.24 | 1.14 |
| 97 | [066-experiment.txt](experiments/066-experiment.txt) | 0.22 | 0.58 | 0.34 | 1.14 |
| 98 | [034-experiment.txt](experiments/034-experiment.txt) | 0.24 | 0.58 | 0.31 | 1.13 |
| 99 | [087-experiment.txt](experiments/087-experiment.txt) | 0.22 | 0.58 | 0.31 | 1.11 |
| 100 | [018-experiment.txt](experiments/018-experiment.txt) | 0.18 | 0.56 | 0.34 | 1.08 |

## Top 5

**1. [091-experiment.txt](experiments/091-experiment.txt)**  
The critique: ontology-biased narrowing and recursive recycling can efficiently reconstruct self-consistent pseudo-discoveries, mistaking coherent artifacts for genuine historical reasoning.

**2. [045-experiment.txt](experiments/045-experiment.txt)**  
Necessary forgetting enables search, but curated benchmarks, recursive synthetic validation, and opaque truths make correct pruning structurally unverifiable and hazardous.

**3. [021-experiment.txt](experiments/021-experiment.txt)**  
Returning accepted abstractions creates path-dependent lock-in; without a stable evaluator, recursive self-improvement drifts, amplifying ontological bias and arbitrary early artifacts.

**4. [019-experiment.txt](experiments/019-experiment.txt)**  
Graph extraction can misreconstruct reasoning by imposing the model’s ontology, especially when source concepts lack stable, agreed causal grammar.

**5. [061-experiment.txt](experiments/061-experiment.txt)**  
Thought experiment: recycled abstractions and recursive synthetic Einstein priors entrench path-dependent familiarity; critique: apparent novelty masks inherited bias, blocking trustworthy discovery.

