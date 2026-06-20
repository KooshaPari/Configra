# Configra — Architecture

**Status:** ACTIVE  
**Last Updated:** 2026-06-19

## Overview

Configra is the canonical Rust configuration substrate for the Phenotype
organization. It follows a **hexagonal (ports-and-adapters) architecture** with
a clean separation between domain logic, application use-cases, and
infrastructure concerns.

## Crate Layout

```
Configra/
├── crates/
│   ├── config-schema/       # Schema validation layer (pure validation)
│   ├── configra-config/     # Meta-configuration for Configra itself
│   ├── pheno-config/        # Canonical typed-config loader (env, file, builder)
│   └── settly/              # Legacy config crate (absorbed, hexagonal design)
├── docs/
│   ├── migrations/          # Migration records from absorbed repos
│   └── phenotype-config-absorbed/ # Original source artifacts
├── ABSORBED-FROM/           # Absorbed repository snapshots
├── AGENTS.md                # Agent instructions
├── SPEC.md                  # Specification
└── ARCHITECTURE.md          # This file
```

## Crate Dependency Graph

```
configra-config
├── serde, serde_json, toml, thiserror

pheno-config
├── serde, serde_json, toml, thiserror
└── (optional) tracing

config-schema
├── serde_json, thiserror

settly (hexagonal)
├── domain/       # Pure business logic (no external deps beyond Rust std)
├── application/  # Use-cases (ConfigBuilder, SubmissionService)
├── adapters/     # I/O (file parsers, env sources, validators)
└── infrastructure/ # Cross-cutting (error handling)
```

## Design Principles

1. **Local-first** — All state is on disk; no network dependency for core
   operations.
2. **CLI-first** — Every operation is available through `phenoctl`.
3. **Auditable** — Changes are versioned with point-in-time restore.
4. **Hexagonal** — Domain logic never depends on I/O or external frameworks.
5. **Minimal dependencies** — Each crate pulls only what it needs.

## Ports and Adapters (settly)

The `settly` crate follows hexagonal architecture:

- **Domain** (`crates/settly/src/domain/`): Config entities, layers, validation,
  idempotency traits, error types. Pure business logic.
- **Application** (`crates/settly/src/application/`): ConfigBuilder,
  SubmissionService. Orchestrates domain logic.
- **Adapters** (`crates/settly/src/adapters/`): File format parsers (TOML,
  JSON, YAML), environment variable sources, idempotency stores
  (InMemoryDlq, InMemoryIdempotencyStore).
- **Infrastructure** (`crates/settly/src/infrastructure/`): Cross-cutting
  concerns — error wrapping, shared utilities.

## Configuration Loading (pheno-config)

The `pheno-config` crate provides three loading strategies:

1. **`load_from_env(prefix)`** — Reads `<PREFIX>_*` environment variables.
2. **`load_from_file(path)`** — Reads JSON or TOML config files.
3. **`ConfigBuilder`** — Programmatic construction with sensible defaults.

The **`combine(file, prefix)`** function implements the 12-factor pattern:
file values fill in gaps; env vars override when present.

## Security

- Secrets are encrypted with AES-256-GCM at rest.
- Feature flags support rollback with version history.
- All cryptographic operations use audited, well-known Rust crates.

## Related ADRs

- ADR-022 — Config consolidation (Rust/TS edge split)
- ADR-031 — Configra is the canonical config substrate
- ADR-035 — Configra migration gates
- ADR-036 — Tracing substrate (opt-in)
