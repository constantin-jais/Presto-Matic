# Presto-Matic

> Sovereign, self-hostable collaborative learning platform — **NotebookLM × Kahoot**: AI-generated, source-grounded study content delivered in real-time sessions for 200+ participants.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/Rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/constantin-jais/Rumble-LM/actions/workflows/ci.yml/badge.svg)](https://github.com/constantin-jais/Rumble-LM/actions/workflows/ci.yml)

> **Status:** `v0.1` — backend/RAG/live-session stable baseline. The live-session tracer bullet is implemented and gated (Biscuit join link, 200 participants, grounded generation, real-time aggregation, leaderboard/load SLOs). Product-complete front, RGPD erasure/audit, and production AI-latency work remain tracked in `docs/`.

## Why it exists

Study platforms either lock you into a vendor's AI stack or lack live collaboration. Presto-Matic is self-hostable with your own AI keys, generates study content grounded in your sources (every item is traceable and agentic-verified), and supports real-time sessions — without compromise on data sovereignty or RGPD compliance.

## Ecosystem

```mermaid
graph TB
    subgraph product["🎯 Product"]
        RL["Presto-Matic · rumble-lm<br/>Collaborative Learning App"]
    end
    subgraph agentic["🤖 Agentic Tools"]
        AOM["agent-o-matic<br/>Config Compiler + Orchestrator"]
        DL["disc-loader<br/>Document Ingestion Worker"]
        MC["memory-card<br/>Local Agent Context"]
    end
    subgraph devops["🔧 DevOps Tools"]
        LC["link-cable<br/>Distribution Substrate"]
        SD["supply-depot<br/>Registry Proxy / Cache"]
        VI["vault-inspector<br/>Postgres Security Audit"]
    end
    RL --> DL
    RL --> MC
    RL --> VI
    RL --> SD
    RL --> LC
    AOM --> LC
    DL --> MC
    style RL fill:#dbeafe,stroke:#2563eb,stroke-width:2px
```

Adjacent tooling lives in separate repos to keep Presto-Matic's runtime boundary tight. See [`docs/adr/0003-companion-repositories.md`](docs/adr/0003-companion-repositories.md).

## Key properties

- **Sovereign / BYO** — self-host on your own infrastructure with your own AI keys. Defaults to Clever Cloud + Clever AI (EU, RGPD).
- **Grounded** — every generated item (quiz, flashcard, summary) is traceable to its source and verified by an agentic grounding checker.
- **Live** — host a session, participants join by link, answer grounded quizzes, watch a real-time leaderboard and comprehension heatmap.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Presto-Matic                        │
├──────────────┬──────────────────┬───────────────────────┤
│  crates/core │   crates/rag     │    crates/server      │
│  Protocol    │  Ingestion ·     │  axum HTTP/WS ·       │
│  API client  │  Retrieval ·     │  Auth (Biscuit/OIDC)  │
│  WASM        │  Grounded gen ·  │  Session engine ·     │
│  bindings    │  Verification    │  Fanout · Stores      │
└──────────────┴──────────────────┴───────────────────────┘
       │                │                    │
    UniFFI          pgvector             PostgreSQL
    WASM             Redis               Pulsar
                     S3/Cellar
```

## Quick start

```bash
# Clone and build
cargo build --release

# Run backend (requires PostgreSQL, Redis, Pulsar, S3-compatible storage)
cargo run -p server -- --config config/local.toml
```

See [`docs/`](docs/) for full setup instructions including Clever Cloud deployment.

## Tech stack

| Component        | Choice                                       |
| ---------------- | -------------------------------------------- |
| Language         | Rust 2024, edition MSRV 1.95+                |
| HTTP / WebSocket | axum + tokio                                 |
| Database         | PostgreSQL + pgvector                        |
| Object storage   | S3-compatible (Cellar/Clever Cloud default)  |
| Messaging        | Pulsar                                       |
| Cache / KV       | Redis / Materia KV                           |
| Auth             | Biscuit tokens + OIDC / Keycloak             |
| AI               | OpenAI-compatible (Clever AI default)        |
| UI               | Dioxus 0.7 (WASM + native via `crates/core`) |

## Related projects

| Repo                                                                  | Role                                                     |
| --------------------------------------------------------------------- | -------------------------------------------------------- |
| [disc-loader](https://github.com/constantin-jais/disc-loader)         | Xberg-backed document ingestion worker for RAG           |
| [memory-card](https://github.com/constantin-jais/memory-card)         | Local agent context layer — code map and repo memory     |
| [vault-inspector](https://github.com/constantin-jais/vault-inspector) | Scythe-backed SQL audit and Postgres security inspection |
| [supply-depot](https://github.com/constantin-jais/supply-depot)       | Starmetal-backed sovereign registry proxy                |
| [link-cable](https://github.com/constantin-jais/link-cable)           | Multi-platform distribution substrate                    |
| [agent-o-matic](https://github.com/constantin-jais/Agent-O-Matic)     | Config compiler and autonomous CI/CD orchestrator        |

## License

MIT © Constantin Jais
