<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/Configra/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/Configra?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/Configra?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
> **Work state:** ACTIVE · **Progress:** `█████░░░░░ 50%`
> Rust configuration framework (layered config/env/secrets); pre-1.0 · updated 2026-06-02

> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# Configra (phenotype-config)

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.83+-orange.svg?logo=rust&logoColor=white)](Cargo.toml)
[![Status](https://img.shields.io/badge/status-active-brightgreen.svg)](#)

Rust configuration framework for Phenotype projects — typed config loading,
settings lifecycle management, JSON schema validation, and encryption-at-rest.
Five-crate Cargo workspace; pre-1.0, active development.

> **Scope note (v37 integrity pass):** Earlier versions of this README described
> `pheno-db` (SQLite persistence), `pheno-cli` / `phenoctl` (TUI CLI), `pheno-core`,
> and `pheno-crypto` as separate crates. **Those crates do not exist in this
> workspace.** The actual crate split is documented below. AES-256-GCM
> encryption-at-rest is implemented inside `settly` (`src/crypto.rs`), not in a
> standalone `pheno-crypto` crate. The `configra-ops` crate ships a minimal
> health/version CLI binary (`configra-ops`), not a full `phenoctl` command suite.
> A `phenoctl`-style feature-flag/secrets CLI and SQLite persistence layer are
> on the roadmap but not yet implemented.

## Overview

Configra is a tier-2 library substrate for Phenotype projects. It consolidates
configuration loading, settings validation, JSON schema primitives, and
observability/ops utilities from eight absorbed upstream repos into a single
coherent workspace.

## What Is Actually Here

| Crate | What it does |
|---|---|
| `pheno-config` | Typed `Config` struct with env-var + TOML + builder loading, 5-layer 12-factor cascade, `SecretBox<str>` redacting wrapper |
| `settly` | Settings lifecycle (submission, validation, migration), AES-256-GCM encryption-at-rest via Argon2id KDF, hot-reload via `notify` |
| `config-schema` | JSON schema field/object validation primitives (`SchemaField`, `ConfigSchema`) |
| `phenotype-config-loader` | Generic `load_json<T>` / `load_toml<T>` file loaders with bounded error types |
| `configra-ops` | Observability + ops primitives (health/liveness/readiness checks, metrics, tracing, shutdown) + `configra-ops` health/version CLI |

## Technology Stack

- **Language**: Rust 1.75+ (MSRV), ES Modules TypeScript edge layer (`typescript/packages/conft/`)
- **Cryptography**: AES-256-GCM + Argon2id KDF (in `settly`, gated behind `encryption` feature flag)
- **Hot-reload**: `notify` v6 + tokio broadcast channel (in `settly`, gated behind `hot-reload` feature flag)
- **Supply chain**: `cargo-deny`, `cargo-audit` weekly, CycloneDX SBOM, TruffleHog scan
- **No external services required** for library use; `settly` optionally connects to PostgreSQL (via `sqlx`) and Redis when those features are enabled

## Roadmap (not yet implemented)

The following are planned but absent from the current codebase:

- `phenoctl` full feature-flag/secrets/version CLI (`clap` + `ratatui` TUI)
- SQLite-backed audit trail and point-in-time restore
- Shell completion generation
- Go/Python bindings

## Quick Start

```bash
git clone https://github.com/KooshaPari/Configra.git
cd Configra

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Health/version CLI (the only CLI currently available)
cargo run -p configra-ops -- health
cargo run -p configra-ops -- version

# Use pheno-config in your project
# Add to Cargo.toml:
#   pheno-config = { git = "https://github.com/KooshaPari/Configra" }
```

## Project Structure

```
Configra/
├── crates/
│   ├── pheno-config/              # Typed Config + env/TOML/builder loading
│   ├── settly/                    # Settings lifecycle + encryption-at-rest + hot-reload
│   ├── config-schema/             # JSON schema validation primitives
│   ├── phenotype-config-loader/   # Generic JSON/TOML file loaders
│   └── configra-ops/              # Observability primitives + health CLI binary
├── typescript/packages/conft/     # TS edge layer (drained from Conft)
├── ABSORBED-FROM/                 # Index of 8 source repos absorbed here
├── docs/migrations/               # Per-source migration notes
├── Cargo.toml                     # Workspace manifest
└── Cargo.lock                     # Dependency lock
```

## Tier-2 Substrate Layout (T16, 2026-06-20)

Configra is a **tier-2 library substrate** per ADR-023 (agent-effort
governance) + ADR-040 (test-coverage gates per tier). The canonical
four-crate split, each with its own README, CHANGELOG, AGENTS, and
≥ 80 % line coverage, is:

```
Configra/                                  v0.4.0 (workspace)
├── crates/
│   ├── pheno-config/                      typed runtime Config + ConfigBuilder
│   │   ├── README.md · CHANGELOG.md · AGENTS.md · llms.txt
│   │   └── (env-var cascade, TOML+env overlay, combine())
│   ├── settly/                            settings lifecycle (validation, migration)
│   │   ├── README.md · CHANGELOG.md · AGENTS.md
│   │   └── (hexagonal: domain / application / adapters / infrastructure)
│   ├── config-schema/                     JSON schema validation primitives
│   │   ├── README.md · CHANGELOG.md · AGENTS.md   ← T16 NEW
│   │   └── (SchemaField + ConfigSchema + SchemaError)
│   └── phenotype-config-loader/           generic JSON/TOML file loaders
│       ├── README.md · CHANGELOG.md · AGENTS.md   ← T16 NEW
│       └── (load_json<T>, load_toml<T>, ConfigLoadError)
├── typescript/packages/conft/             TS edge layer (drained from Conft, PR-#47)
├── ABSORBED-FROM/                         index of the 8 source repos drained here
└── docs/migrations/                       per-source migration notes
```

**Tier-2 quality bar (every sub-crate):**

| Artifact | Status (2026-06-20) | Owner |
| --- | --- | --- |
| `README.md` | ✅ all 4 sub-crates | T16 |
| `CHANGELOG.md` | ✅ all 4 sub-crates | T16 |
| `AGENTS.md` | ✅ all 4 sub-crates | T16 |
| Coverage ≥ 80 % | ✅ library tier (ADR-040) | ongoing |
| `cargo clippy -- -D warnings` | ✅ | CI |
| `cargo audit` | ✅ weekly | CI |

**Why four crates, not one?** Each sub-crate has a distinct concern:
runtime `Config` (pheno-config) vs. settings lifecycle (settly) vs.
field-shape validation (config-schema) vs. raw file loading
(phenotype-config-loader). Splitting keeps each surface minimal and
lets downstream consumers depend on the smallest primitive they need.

See `docs/migrations/` for per-source-repo migration notes and
`ABSORBED-FROM/` for the index of all 8 source repos drained into
Configra.

## Related Phenotype Projects

- **[AgilePlus](../AgilePlus)** — Specification and work tracking
- **[phenotype-shared](../phenotype-shared)** — Shared infrastructure
- **[thegent](../thegent)** — Dotfiles management

## Governance & Documentation

- **CLAUDE.md** — Development standards and AgilePlus mandate
- **docs/ARCHITECTURE.md** — System design and crate relationships
- **docs/QUICKSTART.md** — Getting started guide
- **docs/GUIDE.md** — CLI and feature usage
- **License**: MIT

---

**Status**: Active development  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-04-24

## License

MIT — see [LICENSE](./LICENSE).
