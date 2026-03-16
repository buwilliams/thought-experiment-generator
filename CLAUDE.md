# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Status

This project is in the **design phase** — no source code exists yet. The repository contains design documents and essays that define the system to be built.

## What This Project Is

The Thought Experiment Generator is a system that explores a user-provided topic by running a **depth-bounded branching search over explanation space**. It collides structured knowledge fragments ("quads") from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, filters them through a coherence and Deutsch score pipeline, and builds a tree of progressively deeper insights.

The core claim: the ~10^4,699 search space of possible thought experiments is made tractable not by coverage but by **informed guessing** — pool composition biases draws toward regions likely to survive filtering, and the novel pool compounds prior discoveries into better future draws.

## Key Design Documents

- `docs/thought-experiment-generator-design-doc.md` — Full architecture spec including data structures, LLM prompt templates, filter stack, tree structure, CLI parameters, and a 13-step build order
- `docs/llms-as-universal-explainer.md` — Motivation and theoretical argument for the project
- `docs/reaction-to-vishal-misra-transcript.md` — Transcript with commentary on LLM limitations and knowledge creation (background context)

## Architecture (from design doc)

**Three-Pool Draw System:** Every draw samples from background (corpus facts), universal (50k-term vocabulary), and novel (high-scoring tree discoveries). The novel pool ratio grows as the tree runs.

**Atom Structure:** Each thought experiment is generated from a 4/3/2 draw — 4 objects, 3 relationships, 2 properties — derived empirically from Einstein's journal entries.

**Filter Stack (cheap → expensive):** Grammar → Coherence → Deutsch score → Survivor threshold.

**Tree:** N root branches, each running to depth limit (default 10) regardless of score. No pruning on score — only depth limit, circularity, or vocabulary exhaustion terminate a branch. Cross-pollination merges branches with complementary unresolved tensions.

**Build order** (from design doc): universal vocabulary → quad extractor → single TE generator → coherence filter → Deutsch scorer → tension extractor → single branch runner → background pool initializer → root branch generator → trajectory scorer → cross-pollination detector → parallel tree runner → CLI wrapper.

## Domain Terminology

- **Quad**: (object_a, relationship, object_b, property) — atomic knowledge unit
- **Deutsch score**: LLM judgment of explanatory quality — hard to vary, reach, minimal assumptions, tension resolution
- **Novel pool**: Quads earned by high-scoring survivors; makes the tree compound its own discoveries
- **Cross-pollination**: Merging branches with complementary unresolved tensions into a new branch
