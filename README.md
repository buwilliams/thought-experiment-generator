# Thought Experiment Generator

Can LLMs create new knowledge? The conjecture: yes, through a narrowing search architecture algorithm that gets better with each pass. Given a topic, the system draws structured knowledge fragments from three pools (background, universal, novel), generates hypothetical thought experiments via LLM, and filters them for coherence and explanatory quality using Deutschian criteria. Humans guide the process, but at a critical threshold of search quality, the system's outputs become reliable enough to judge its next outputs, enabling recursive self-improvement without scaling compute.

## Status

Design phase — no source code yet.

## Documents

- [LLMs as Universal Explainer](docs/llms-as-universal-explainer.md) — Motivation and theoretical argument for the project
- [Design Document](docs/thought-experiment-generator-design-doc.md) — Full architecture spec: data structures, LLM prompt templates, filter stack, tree structure, CLI parameters, and build order
## Inspiration

- [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) ([transcript](docs/reaction-to-vishal-misra-transcript.md)) — Brett Hall's video discussing whether LLMs can create new knowledge, drawing on David Deutsch's epistemology
