# Configra — 1-Page Specification

**Version:** 0.1.0 (pre-1.0)
**Date:** 2026-06-19
**Status:** ACTIVE (canonical per ADR-031)

---

## What

Configra is the canonical Rust configuration substrate for the Phenotype
organization. Single crate workspace, four members:

- **`pheno-config`** — Core `Config` + `ConfigBuilder` + layered loader
- **`config-schema`** — JSON-Schema-style validation
- **`phenotype-config-loader`** — Generic TOML/JSON loaders (absorbed 2026-06-18)
- **`settly`** — Legacy config crate (absorbed 2026-06-18, back-compat)

Plus TypeScript edge layer: `@phenotype/config-ts` in `typescript/`.

## Why

Per ADR-031, Configra is the canonical name (supersedes `phenotype-config`)
for all Rust-side config substrate. Absorbs `pheno-config`, `settly-*`, and
`Conft/crates/`. Eliminates 7-repo config sprawl.

## How

**Layered loading (last-wins):**

```
compile-time defaults → TOML/JSON file → env vars (CONFIGRA_*) → CLI → builder
```

**Public API:**

```rust
let cfg = ConfigBuilder::new()
    .with_file("configra.toml")?
    .with_env_prefix("CONFIGRA")?
    .build()?;
```

**CLI:**

```bash
phenoctl config set app.name "My App"
phenoctl flags create dark-mode
phenoctl secrets set API_KEY
```

## When to use

- 12-factor layered config with validation
- Encrypted secrets at rest (AES-256-GCM)
- Audit trail with point-in-time restore
- Type-safe config + JSON Schema validation
- Feature flag lifecycle

## When NOT to use

- Distributed runtime sync → use `pheno-events`
- Hot-reload across processes → post-1.0
- Config-as-a-service → deferred

## Constraints

- Rust 2021, MSRV 1.75
- Coverage ≥80% (ADR-040)
- License: MIT OR Apache-2.0 (dual)
- Worklog schema: ADR-015 v2.1

## Owner

configra-circle

## Related

- Full spec: `SPEC.md`
- SSOT: `SSOT.md`
- Architecture: `docs/ARCHITECTURE.md`
- Meta-config: `docs/CONFIG.md`
- SLSA: `docs/slsa.md`
- ADRs: 022, 031, 035, 040