---
name: Novelty separate from ranking
description: Decision to keep novelty-check as a diagnostic tool, not part of conjecture composite score
type: project
---

Novelty scores are intentionally kept separate from conjecture ranking. The `novelty-check` command is a diagnostic tool only.

**Why:** A known lens that reliably generates high-scoring outputs is still useful. Penalizing it for being recognizable would conflate novelty with explanatory value. The two are worth tracking independently.

**How to apply:** Do not propose folding novelty into `composite()` or the ranking pipeline unless the user revisits this. If asked about ranking, note that novelty is a separate diagnostic, not a weight.
