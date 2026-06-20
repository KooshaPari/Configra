# AGENTS.md — Configra

**Date:** 2026-06-20
**Status:** ACTIVE
**Template:** v8.1

---

## Project Overview

Configra is the canonical Rust configuration substrate for the Phenotype
organization. It provides local-first configuration management, feature flags,
secrets, and version tracking with auditable change history and CLI-first
workflows.

## Stack

- **Language:** Rust (primary)
- **Build system:** Cargo workspace, resolver 2
- **Key crates:** pheno-config, config-schema, settly (absorbed from standalone repo)
- **Frameworks:** Clap (CLI), Ratatui (TUI)
- **Persistence:** SQLite with auto-migration
- **Cryptography:** AES-256-GCM for secrets
- **Bindings:** TypeScript (`@phenotype/config-ts` via Conft)

## Key Commands

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace

# Audit
cargo deny check
cargo audit

# Install CLI
cargo install --path crates/pheno-cli

# CLI usage
phenoctl config set app.name "My Application"
phenoctl flags create dark-mode --description "Enable dark mode"
phenoctl secrets set API_KEY
```

## Conventions

- **Branch naming:** `chore/<slug>-<date>` for chore work;
  `feat/<slug>-<date>` for features
- **Commit messages:** Conventional Commits (`feat:`, `fix:`, `chore:`,
  `docs:`, `refactor:`, `test:`, `build:`, `ci:`)
- **PR labels:** `governance` for cleanup
- **Quality gates:** lint + test must pass before push

## Related Repos

- **Conft** (`../Conft`) — TypeScript edge layer (Zod-validated bindings)
- **pheno-config** (`../pheno-config`) — legacy config crate (deprecated)
- **phenotype-config** (`../phenotype-config`) — legacy config repo (ARCHIVED)
- **Settly** (`../Settly`) — standalone settly repo (ARCHIVED; absorbed into `crates/settly/`)

## Architecture

See `docs/ARCHITECTURE.md` for crate relationships and hexagonal port-adapter
design.

## Active ADRs

- ADR-022 — config consolidation (Rust/TS edge split)
- ADR-031 — Configra is the canonical config substrate
- ADR-035 — Configra migration gates
