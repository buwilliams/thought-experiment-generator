# Thought Experiment Generator

A system for exploring topics by running a depth-bounded branching search over explanation space. It collides structured knowledge fragments from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, and filters them through a coherence and Deutsch score pipeline to surface genuinely novel insights.

## Status

Design phase — no source code yet.

## Documents

- [LLMs as Universal Explainer](docs/llms-as-universal-explainer.md) — Motivation and theoretical argument for the project
- [Design Document](docs/thought-experiment-generator-design-doc.md) — Full architecture spec: data structures, LLM prompt templates, filter stack, tree structure, CLI parameters, and build order
## Inspiration

- [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) — Brett Hall's video discussing whether LLMs can create new knowledge, drawing on David Deutsch's epistemology
