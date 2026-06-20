# AGENTS.md — Configra

**Date:** 2026-06-19
**Status:** ACTIVE
**Template:** v8.1

---

## Project Overview

Configra is the **canonical Rust configuration substrate** for the Phenotype
organization (per ADR-031, superseding `phenotype-config` and absorbing
`pheno-config`, `settly-*`, and `Conft/crates/`). It provides:

- Layered configuration loading (compile-time defaults → TOML/JSON files →
  environment variables → CLI overrides, last-wins)
- Type-safe schema validation
- Feature flags with lifecycle management
- AES-256-GCM encrypted secrets
- Point-in-time audit trail + restore
- SQLite persistence with auto-migration
- `phenoctl` CLI (clap) and Ratatui TUI
- TypeScript bindings via `@phenotype/config-ts` (in `typescript/`)

## Scope

**IN SCOPE:** config loading, validation, secrets at rest, feature flags,
audit trail, CLI, TUI, TS bindings.

**OUT OF SCOPE:** distributed config sync (use `pheno-events`), runtime hot-
reload across processes (planned post-1.0), config-as-a-service (deferred).

## Stack

- **Language:** Rust 2021 edition, MSRV 1.75
- **Build:** Cargo workspace, resolver 2
- **Persistence:** SQLite + `rusqlite`
- **Crypto:** `aes-gcm` (AES-256-GCM)
- **CLI:** `clap` v4
- **TUI:** `ratatui` + `crossterm`
- **Bindings:** TypeScript via `typescript/` package

## Key Commands

```bash
# Build & test
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo deny check
cargo audit

# Install CLI
cargo install --path crates/pheno-cli

# CLI usage
phenoctl config set app.name "My Application"
phenoctl flags create dark-mode --description "Enable dark mode"
phenoctl secrets set API_KEY
```

## Architecture

Workspace members:

- `crates/pheno-config` — core config types + loader
- `crates/config-schema` — JSON-Schema-style validation
- `crates/phenotype-config-loader` — generic TOML/JSON loaders (absorbed
  2026-06-18, ADR-031)
- `crates/settly` — legacy config crate (absorbed 2026-06-18, ADR-031)
- `typescript/` — `@phenotype/config-ts` edge layer (formerly `Conft/typescript/`)

## Related

- ADR-022: Rust/TS split (canonical)
- ADR-031: Configra canonical name + `phenotype-config` deprecation
- ADR-035: Configra migration gates
- `docs/SPEC.md` — 1-page spec
- `docs/slsa.md` — SLSA provenance policy

## Conventions

- Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`)
- Branch naming: `feat/<req-id>-<slug>-<date>` / `chore/<req-id>-<slug>-<date>`
- Worklog schema: ADR-015 v2.1 (11 columns including `device:`)
- Coverage gate: ≥80% for libs (per ADR-040)
- License: MIT OR Apache-2.0 (dual)