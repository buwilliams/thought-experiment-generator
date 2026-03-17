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
| 1 | [021-experiment.txt](experiments/021-experiment.txt) | 0.56 | 0.79 | 0.88 | 2.23 |
| 2 | [076-experiment.txt](experiments/076-experiment.txt) | 0.42 | 0.79 | 0.88 | 2.09 |
| 3 | [029-experiment.txt](experiments/029-experiment.txt) | 0.42 | 0.76 | 0.84 | 2.02 |
| 4 | [066-experiment.txt](experiments/066-experiment.txt) | 0.42 | 0.74 | 0.86 | 2.02 |
| 5 | [082-experiment.txt](experiments/082-experiment.txt) | 0.39 | 0.71 | 0.88 | 1.98 |
| 6 | [040-experiment.txt](experiments/040-experiment.txt) | 0.42 | 0.74 | 0.81 | 1.97 |
| 7 | [036-experiment.txt](experiments/036-experiment.txt) | 0.39 | 0.72 | 0.86 | 1.97 |
| 8 | [045-experiment.txt](experiments/045-experiment.txt) | 0.36 | 0.74 | 0.86 | 1.96 |
| 9 | [046-experiment.txt](experiments/046-experiment.txt) | 0.31 | 0.78 | 0.86 | 1.95 |
| 10 | [037-experiment.txt](experiments/037-experiment.txt) | 0.39 | 0.74 | 0.82 | 1.95 |
| 11 | [031-experiment.txt](experiments/031-experiment.txt) | 0.42 | 0.78 | 0.74 | 1.94 |
| 12 | [070-experiment.txt](experiments/070-experiment.txt) | 0.34 | 0.76 | 0.84 | 1.94 |
| 13 | [027-experiment.txt](experiments/027-experiment.txt) | 0.34 | 0.71 | 0.88 | 1.93 |
| 14 | [034-experiment.txt](experiments/034-experiment.txt) | 0.34 | 0.76 | 0.83 | 1.93 |
| 15 | [044-experiment.txt](experiments/044-experiment.txt) | 0.31 | 0.78 | 0.84 | 1.93 |
| 16 | [094-experiment.txt](experiments/094-experiment.txt) | 0.31 | 0.72 | 0.90 | 1.93 |
| 17 | [048-experiment.txt](experiments/048-experiment.txt) | 0.34 | 0.72 | 0.86 | 1.92 |
| 18 | [050-experiment.txt](experiments/050-experiment.txt) | 0.34 | 0.72 | 0.86 | 1.92 |
| 19 | [077-experiment.txt](experiments/077-experiment.txt) | 0.34 | 0.76 | 0.82 | 1.92 |
| 20 | [098-experiment.txt](experiments/098-experiment.txt) | 0.31 | 0.74 | 0.86 | 1.91 |
| 21 | [047-experiment.txt](experiments/047-experiment.txt) | 0.38 | 0.71 | 0.82 | 1.91 |
| 22 | [005-experiment.txt](experiments/005-experiment.txt) | 0.34 | 0.72 | 0.83 | 1.89 |
| 23 | [016-experiment.txt](experiments/016-experiment.txt) | 0.31 | 0.72 | 0.86 | 1.89 |
| 24 | [039-experiment.txt](experiments/039-experiment.txt) | 0.31 | 0.72 | 0.86 | 1.89 |
| 25 | [001-experiment.txt](experiments/001-experiment.txt) | 0.42 | 0.76 | 0.71 | 1.89 |
| 26 | [091-experiment.txt](experiments/091-experiment.txt) | 0.39 | 0.72 | 0.78 | 1.89 |
| 27 | [067-experiment.txt](experiments/067-experiment.txt) | 0.31 | 0.74 | 0.83 | 1.88 |
| 28 | [072-experiment.txt](experiments/072-experiment.txt) | 0.34 | 0.68 | 0.86 | 1.88 |
| 29 | [020-experiment.txt](experiments/020-experiment.txt) | 0.34 | 0.72 | 0.81 | 1.87 |
| 30 | [028-experiment.txt](experiments/028-experiment.txt) | 0.34 | 0.71 | 0.82 | 1.87 |
| 31 | [032-experiment.txt](experiments/032-experiment.txt) | 0.31 | 0.72 | 0.84 | 1.87 |
| 32 | [035-experiment.txt](experiments/035-experiment.txt) | 0.31 | 0.74 | 0.82 | 1.87 |
| 33 | [038-experiment.txt](experiments/038-experiment.txt) | 0.31 | 0.72 | 0.84 | 1.87 |
| 34 | [054-experiment.txt](experiments/054-experiment.txt) | 0.31 | 0.72 | 0.84 | 1.87 |
| 35 | [085-experiment.txt](experiments/085-experiment.txt) | 0.34 | 0.72 | 0.81 | 1.87 |
| 36 | [012-experiment.txt](experiments/012-experiment.txt) | 0.28 | 0.72 | 0.86 | 1.86 |
| 37 | [003-experiment.txt](experiments/003-experiment.txt) | 0.34 | 0.72 | 0.79 | 1.85 |
| 38 | [056-experiment.txt](experiments/056-experiment.txt) | 0.28 | 0.71 | 0.86 | 1.85 |
| 39 | [081-experiment.txt](experiments/081-experiment.txt) | 0.34 | 0.68 | 0.83 | 1.85 |
| 40 | [083-experiment.txt](experiments/083-experiment.txt) | 0.34 | 0.72 | 0.78 | 1.84 |
| 41 | [010-experiment.txt](experiments/010-experiment.txt) | 0.31 | 0.68 | 0.84 | 1.83 |
| 42 | [018-experiment.txt](experiments/018-experiment.txt) | 0.31 | 0.68 | 0.84 | 1.83 |
| 43 | [026-experiment.txt](experiments/026-experiment.txt) | 0.34 | 0.71 | 0.78 | 1.83 |
| 44 | [051-experiment.txt](experiments/051-experiment.txt) | 0.29 | 0.68 | 0.86 | 1.83 |
| 45 | [057-experiment.txt](experiments/057-experiment.txt) | 0.34 | 0.71 | 0.78 | 1.83 |
| 46 | [059-experiment.txt](experiments/059-experiment.txt) | 0.29 | 0.68 | 0.86 | 1.83 |
| 47 | [064-experiment.txt](experiments/064-experiment.txt) | 0.34 | 0.71 | 0.78 | 1.83 |
| 48 | [073-experiment.txt](experiments/073-experiment.txt) | 0.31 | 0.68 | 0.84 | 1.83 |
| 49 | [075-experiment.txt](experiments/075-experiment.txt) | 0.29 | 0.68 | 0.86 | 1.83 |
| 50 | [011-experiment.txt](experiments/011-experiment.txt) | 0.24 | 0.68 | 0.90 | 1.82 |
| 51 | [007-experiment.txt](experiments/007-experiment.txt) | 0.42 | 0.81 | 0.58 | 1.81 |
| 52 | [008-experiment.txt](experiments/008-experiment.txt) | 0.28 | 0.67 | 0.86 | 1.81 |
| 53 | [013-experiment.txt](experiments/013-experiment.txt) | 0.34 | 0.68 | 0.79 | 1.81 |
| 54 | [042-experiment.txt](experiments/042-experiment.txt) | 0.28 | 0.63 | 0.90 | 1.81 |
| 55 | [043-experiment.txt](experiments/043-experiment.txt) | 0.28 | 0.71 | 0.82 | 1.81 |
| 56 | [049-experiment.txt](experiments/049-experiment.txt) | 0.28 | 0.72 | 0.81 | 1.81 |
| 57 | [065-experiment.txt](experiments/065-experiment.txt) | 0.34 | 0.68 | 0.79 | 1.81 |
| 58 | [068-experiment.txt](experiments/068-experiment.txt) | 0.31 | 0.72 | 0.78 | 1.81 |
| 59 | [080-experiment.txt](experiments/080-experiment.txt) | 0.34 | 0.68 | 0.79 | 1.81 |
| 60 | [084-experiment.txt](experiments/084-experiment.txt) | 0.28 | 0.71 | 0.82 | 1.81 |
| 61 | [086-experiment.txt](experiments/086-experiment.txt) | 0.29 | 0.68 | 0.84 | 1.81 |
| 62 | [093-experiment.txt](experiments/093-experiment.txt) | 0.31 | 0.72 | 0.78 | 1.81 |
| 63 | [096-experiment.txt](experiments/096-experiment.txt) | 0.28 | 0.72 | 0.81 | 1.81 |
| 64 | [099-experiment.txt](experiments/099-experiment.txt) | 0.29 | 0.68 | 0.84 | 1.81 |
| 65 | [100-experiment.txt](experiments/100-experiment.txt) | 0.29 | 0.68 | 0.84 | 1.81 |
| 66 | [062-experiment.txt](experiments/062-experiment.txt) | 0.28 | 0.67 | 0.84 | 1.79 |
| 67 | [015-experiment.txt](experiments/015-experiment.txt) | 0.34 | 0.76 | 0.68 | 1.78 |
| 68 | [014-experiment.txt](experiments/014-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 69 | [017-experiment.txt](experiments/017-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 70 | [041-experiment.txt](experiments/041-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 71 | [060-experiment.txt](experiments/060-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 72 | [078-experiment.txt](experiments/078-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 73 | [088-experiment.txt](experiments/088-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 74 | [095-experiment.txt](experiments/095-experiment.txt) | 0.31 | 0.68 | 0.79 | 1.78 |
| 75 | [074-experiment.txt](experiments/074-experiment.txt) | 0.29 | 0.67 | 0.82 | 1.78 |
| 76 | [079-experiment.txt](experiments/079-experiment.txt) | 0.24 | 0.63 | 0.90 | 1.77 |
| 77 | [025-experiment.txt](experiments/025-experiment.txt) | 0.28 | 0.71 | 0.76 | 1.75 |
| 78 | [033-experiment.txt](experiments/033-experiment.txt) | 0.28 | 0.62 | 0.84 | 1.74 |
| 79 | [089-experiment.txt](experiments/089-experiment.txt) | 0.28 | 0.62 | 0.84 | 1.74 |
| 80 | [004-experiment.txt](experiments/004-experiment.txt) | 0.28 | 0.63 | 0.82 | 1.73 |
| 81 | [023-experiment.txt](experiments/023-experiment.txt) | 0.31 | 0.74 | 0.68 | 1.73 |
| 82 | [097-experiment.txt](experiments/097-experiment.txt) | 0.28 | 0.63 | 0.82 | 1.73 |
| 83 | [087-experiment.txt](experiments/087-experiment.txt) | 0.34 | 0.72 | 0.66 | 1.72 |
| 84 | [052-experiment.txt](experiments/052-experiment.txt) | 0.31 | 0.62 | 0.79 | 1.72 |
| 85 | [053-experiment.txt](experiments/053-experiment.txt) | 0.31 | 0.68 | 0.72 | 1.71 |
| 86 | [063-experiment.txt](experiments/063-experiment.txt) | 0.34 | 0.71 | 0.66 | 1.71 |
| 87 | [002-experiment.txt](experiments/002-experiment.txt) | 0.22 | 0.64 | 0.83 | 1.69 |
| 88 | [019-experiment.txt](experiments/019-experiment.txt) | 0.31 | 0.72 | 0.66 | 1.69 |
| 89 | [090-experiment.txt](experiments/090-experiment.txt) | 0.24 | 0.58 | 0.86 | 1.68 |
| 90 | [058-experiment.txt](experiments/058-experiment.txt) | 0.29 | 0.71 | 0.66 | 1.66 |
| 91 | [030-experiment.txt](experiments/030-experiment.txt) | 0.28 | 0.63 | 0.74 | 1.65 |
| 92 | [055-experiment.txt](experiments/055-experiment.txt) | 0.18 | 0.63 | 0.84 | 1.65 |
| 93 | [092-experiment.txt](experiments/092-experiment.txt) | 0.24 | 0.58 | 0.83 | 1.65 |
| 94 | [022-experiment.txt](experiments/022-experiment.txt) | 0.22 | 0.64 | 0.78 | 1.64 |
| 95 | [071-experiment.txt](experiments/071-experiment.txt) | 0.24 | 0.68 | 0.72 | 1.64 |
| 96 | [061-experiment.txt](experiments/061-experiment.txt) | 0.24 | 0.58 | 0.79 | 1.61 |
| 97 | [009-experiment.txt](experiments/009-experiment.txt) | 0.27 | 0.58 | 0.74 | 1.59 |
| 98 | [024-experiment.txt](experiments/024-experiment.txt) | 0.31 | 0.68 | 0.56 | 1.55 |
| 99 | [006-experiment.txt](experiments/006-experiment.txt) | 0.24 | 0.58 | 0.71 | 1.53 |
| 100 | [069-experiment.txt](experiments/069-experiment.txt) | 0.18 | 0.62 | 0.57 | 1.37 |

## Top 5

**1. [021-experiment.txt](experiments/021-experiment.txt)**  
Complex search seems open, but human-trained filters enforce familiar scientific taste, discarding true alien decompositions and explanations as malformed.

**2. [076-experiment.txt](experiments/076-experiment.txt)**  
Fluency-filtered novelty search excels at thought experiments but systematically misses awkwardly expressed truths, especially measurement or protocol innovations.

**3. [029-experiment.txt](experiments/029-experiment.txt)**  
Human-biased hard-to-vary scoring plus recursive feedback and compression steers discovery systems toward self-reinforcing, elegant falsehoods over messy truth.

**4. [066-experiment.txt](experiments/066-experiment.txt)**  
Universal priors and ungrounded heuristic mimicry can smooth away rare truths; introspective justifications may sound coherent without tracking reality.

**5. [082-experiment.txt](experiments/082-experiment.txt)**  
Language-centered discovery handles salient or traceable problems, but fails at protocol invention, deep mathematics, and often recycles known ideas.

