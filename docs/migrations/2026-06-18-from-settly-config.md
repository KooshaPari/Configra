# Migration: `settly-config` (deprecated federated service) → `Configra`

**Date:** 2026-06-18
**ADRs:**
- [ADR-031 — `KooshaPari/Configra` is the canonical config substrate](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md)
- [ADR-017 — `settly-*` full deprecation (V6 Track 5 closure)](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-017-settly-archive.md)

**Source absorb PR:** [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44)
**Companion migration file:** `Settly/MIGRATION_TO_CONFIGRA.md`

---

## What moved

`settly-config` was the federated Rust config service published as the
`settly` crate from `KooshaPari/Settly`. The package-name `settly` is
unchanged; only the source-of-truth repository moved.

| Was                                                       | Is now                                                                              |
|-----------------------------------------------------------|-------------------------------------------------------------------------------------|
| `KooshaPari/Settly` (Rust crate `settly` 0.1.0, hexagonal) | [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) → `crates/settly/` |
| `KooshaPari/Settly/src/{adapters,application,domain,infrastructure,lib.rs}` | `KooshaPari/Configra/crates/settly/src/...` (verbatim)                              |
| `KooshaPari/Settly/benches/perf.rs`                        | `KooshaPari/Configra/crates/settly/benches/perf.rs`                                 |
| `KooshaPari/Settly/fuzz/`                                  | `KooshaPari/Configra/crates/settly/fuzz/`                                           |
| `KooshaPari/Settly/docs/{research,journeys,stories,traceability}/` | `KooshaPari/Configra/crates/settly/docs/{research,journeys,stories,traceability}/` |
| `KooshaPari/Settly/Cargo.toml` (package `settly`, deps)    | `KooshaPari/Configra/crates/settly/Cargo.toml` (workspace member)                   |

Public API (`Settings`, `SettingsRepository`, validator derives,
`PostgresAdapter`, `RedisCache`, `Combine`, etc.) is preserved verbatim.

## Where the canonical code lives now

- **Canonical repo**: [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra)
- **Crate path**: `crates/settly/`
- **Workspace member**: listed in `KooshaPari/Configra/Cargo.toml` as `crates/settly`
- **Default branch**: `main`
- **Absorb PR**: [#44](https://github.com/KooshaPari/Configra/pull/44)

## How to migrate consumers

### Cargo dependency

```diff
- [dependencies]
- settly = { git = "https://github.com/KooshaPari/Settly", tag = "v0.1.0" }
+ [dependencies]
+ configra = { git = "https://github.com/KooshaPari/Configra", tag = "settly-v0.1.0" }
```

### Imports

```diff
- use settly::{Settings, SettingsRepository, PostgresAdapter, RedisCache};
+ use configra::settly::{Settings, SettingsRepository, PostgresAdapter, RedisCache};
```

### Workspace member

```diff
  # Cargo.toml [workspace]
  members = [
-     "../Settly",
+     "../Configra/crates/settly",
  ]
```

### Federated-service consumers (the most important migration)

If you consumed `Settly` as a **running service** (HTTP/RPC) rather than a
library, migrate to **in-process library calls**. Per ADR-023 substrate
placement, `Configra` is a `pheno-*-lib` substrate, not a federated service.

```diff
- // Old: HTTP client to Settly service
- let client = settly_client::Client::new("https://settly.example.com");
- let settings = client.get("feature-flags").await?;
+ // New: in-process library call
+ let settings = configra::settly::Settings::load("feature-flags")?;
```

This eliminates a network hop, removes a deployment concern, and brings the
config layer into the same compilation unit as the consumer (catching type
mismatches at compile time, not runtime).

### CI / tooling

- Any `pheno-ci-templates` references to `Settly`-specific paths move to
  `Configra/crates/settly/`-specific paths.
- Any `pheno-context` port referencing `settly::*` paths from the old repo
  now references `configra::settly::*`.

## What does NOT move

- **`Settly` repo itself** — preserved (read-only) until archive per
  ADR-017 + ADR-031 (28-day grace).
- **Issue tracker, PR history, security advisories** — stay in `Settly` for
  traceability.
- **Federated deployment configs** (k8s manifests, helm charts, terraform)
  — those deployments are being decommissioned; consumers migrate to
  in-process usage.

## Timeline

| Date          | Event                                                                 |
|---------------|-----------------------------------------------------------------------|
| ~2025 (pre-ADR) | `KooshaPari/Settly` created as federated config service.            |
| 2026-06-15    | ADR-017 accepted — `settly-*` full deprecation (V6 Track 5 closure). |
| 2026-06-17    | ADR-031 accepted — Configra named canonical absorb target.           |
| 2026-06-17    | Configra PR #44 opened.                                               |
| 2026-06-18    | Absorb PR #44 merged.                                                 |
| 2026-06-18    | This migration doc authored (T10.6 of v8 DAG).                       |
| 2026-07-15    | `Settly` archive (28-day grace after ADR-031).                       |

## Cross-references

- **ADR-031** — `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- **ADR-017** — `settly-*` full deprecation (V6 Track 5 closure)
- **ADR-022** — config consolidation (Rust/TS split)
- **ADR-023** — substrate placement (Rule 3 — `pheno-*-lib` substrate)
- Configra PR [#44](https://github.com/KooshaPari/Configra/pull/44) — the absorb
- `Settly/MIGRATION_TO_CONFIGRA.md` — companion source-repo notice
- `findings/2026-06-17-L5-104-7-configra-absorb-plan.md`
- `plans/2026-06-18-v8-dag-stable.md` § 3.2 (T10)

L5-104.7 — T10.6 / 2026-06-18-from-settly-config
