# Rumble LM

**Layer:** Rumble — Product  
**Role:** sovereign collaborative learning platform  
**Mission:** help groups learn from source-grounded AI content in reliable, real-time sessions.

---

## Purpose

`rumble-lm` is the learning product of the ecosystem. It supports collaborative sessions, AI-assisted study content, source grounding, and facilitation for groups.

It is not a generic chat application; the product outcome is learning.

## Owns

- Learning session UX and facilitation workflows.
- Source-grounded study content presentation.
- Collaboration features for learners and facilitators.
- Sovereign/BYO-key product experience and RGPD-aware operation.

## Does Not Own

- Generic model hosting or provider abstraction as infrastructure.
- Agentic orchestration internals: belongs to `cos-matic`.
- Raw ingestion/extraction: belongs to `wrench-loader`.
- Memory/storage/distribution primitives: belongs to Gear.

## Allowed Dependencies

- Uses Bolt for orchestration when learning workflows need planning or agents.
- Uses Wrench for document ingestion and validation.
- Uses Gear for memory, artifact integrity, and reproducible deployment paths.

## Product Vision Challenge

`rumble-lm` must be judged by learning outcomes, groundedness, session reliability, and group adoption — not by being yet another LLM chat UI.
