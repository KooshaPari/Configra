# Migration: `phenotype-config` → `Configra`

**Date:** 2026-06-18
**ADR:** [ADR-031 — `KooshaPari/Configra` is the canonical config substrate](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md)
**Source absorb PR:** [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (`feat(Configra): absorb phenotype-config/crates/settly (ADR-031)`)
**Companion migration files:** `phenotype-config/MIGRATE_TO_CONFIGRA.md` (short-form) · `phenotype-config/CANONICAL_REDIRECT.md` (consumer redirect table)

---

## What moved

| Was                                                                  | Is now                                                                              |
|----------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| `phenotype-config/crates/settly/` (hexagonal Rust, ~64 LoC + adapters) | [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) → `crates/settly/`  |
| `phenotype-config/crates/settly/docs/*` (research, journeys, stories) | `KooshaPari/Configra/crates/settly/docs/*` (verbatim)                                |
| `phenotype-config/crates/settly/Cargo.toml` (workspace member)       | `KooshaPari/Configra/Cargo.toml` (workspace member `crates/settly`)                  |
| `phenotype-config/crates/settly/AGENTS.md`, `WORKLOG.md`, etc.       | `KooshaPari/Configra/crates/settly/` (verbatim)                                      |
| `phenotype-config/crates/settly/.github/workflows/*`                  | `KooshaPari/Configra/crates/settly/.github/workflows/*` (verbatim)                  |

The `settly` crate's public API (`Settings`, `SettingsRepository`,
`PostgresAdapter`, `RedisCache`, validator derives, etc.) is **preserved
verbatim**. Consumers continue to depend on `configra::settly::...` exactly as
they depended on `phenotype_config::settly::...`.

## Where the canonical code lives now

- **Canonical repo**: [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra)
- **Crate path**: `crates/settly/`
- **Workspace member**: listed in `KooshaPari/Configra/Cargo.toml` as `crates/settly`
- **Default branch**: `main`
- **Absorb PR**: [#44](https://github.com/KooshaPari/Configra/pull/44)

## How to migrate consumers

### Cargo dependency (direct git)

```diff
- [dependencies]
- phenotype-config = { git = "https://github.com/KooshaPari/phenotype-config", tag = "v0.1.0" }
+ [dependencies]
+ configra = { git = "https://github.com/KooshaPari/Configra", tag = "settly-v0.1.0" }
```

### Imports

```diff
  // src/lib.rs / src/main.rs
- use phenotype_config::settly::{Settings, SettingsRepository, PostgresAdapter, RedisCache};
+ use configra::settly::{Settings, SettingsRepository, PostgresAdapter, RedisCache};
```

The crate's external surface is unchanged; only the host repo and Cargo
registry key move.

### Workspace members

```diff
  # Cargo.toml [workspace]
  members = [
-     "../phenotype-config/crates/settly",
+     "../Configra/crates/settly",
  ]
```

### CI / tooling

- `pheno-ci-templates` references to `phenotype-config`-specific paths move to
  `Configra`-specific paths.
- `pheno-context`, `phenotype-mcp-router`, `pheno-registry`, and other
  adapters that referenced `phenotype-config::*` ports move to
  `configra::settly::*` ports.

## What does NOT move

- **`phenotype-config` repo itself** — preserved (read-only) until 2026-07-15
  (28-day grace), then archived per ADR-031.
- **Issue tracker, PR history, security advisories** — stay in
  `phenotype-config` for traceability.
- **GitHub Actions, hooks, secrets, CODEOWNERS** — stay with
  `phenotype-config` until archive.

## Timeline

| Date          | Event                                                                                |
|---------------|--------------------------------------------------------------------------------------|
| 2026-03-25    | `KooshaPari/Configra` created (original config framework intent).                    |
| 2026-06-15    | ADR-022 accepted (Rust/TS split: `phenotype-config` + `Conft`).                       |
| 2026-06-17    | ADR-031 accepted (rename + absorb: `phenotype-config` → `Configra`).                  |
| 2026-06-17    | Configra PR #44 opened.                                                              |
| 2026-06-18    | Absorb PR #44 merged.                                                                |
| 2026-06-18    | This migration doc authored (T10.6 of v8 DAG).                                      |
| **2026-07-15** | **`phenotype-config` archive (28-day grace period expires).**                       |

## Cross-references

- **ADR-031** — `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- **ADR-022** — config consolidation (Rust/TS split; superseded for naming only)
- **ADR-023** — substrate placement (Rule 3)
- **ADR-017** — `settly-*` deprecation
- Configra PR [#44](https://github.com/KooshaPari/Configra/pull/44) — the absorb
- `phenotype-config/MIGRATE_TO_CONFIGRA.md` — earlier short-form notice
- `phenotype-config/CANONICAL_REDIRECT.md` — consumer redirect table
- `findings/2026-06-17-L5-104-7-configra-absorb-plan.md`
- `plans/2026-06-18-v8-dag-stable.md` § 3.2 (T10)

L5-104.7 — T10.6 / 2026-06-18-from-phenotype-config
