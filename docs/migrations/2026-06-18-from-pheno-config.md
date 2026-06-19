# Migration: `pheno-config` → `Configra`

**Date:** 2026-06-18
**ADR:** [ADR-031 — `KooshaPari/Configra` is the canonical config substrate](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md)
**Source absorb PR:** [Configra PR #45](https://github.com/KooshaPari/Configra/pull/45) (`feat(config): absorb pheno-config (Rust crate) into Configra canonical (L5-104.7)`)

---

## What moved

| Was                                       | Is now                                                                              |
|-------------------------------------------|-------------------------------------------------------------------------------------|
| `pheno-config` Rust crate (12-factor config: env + JSON + TOML + `combine()` overlay) | [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) → `crates/pheno-config/` |
| Package name on crates.io: `pheno-config` | **Unchanged** — `pheno-config` package continues to exist; only the source-of-truth repo moved from the meta-repo `repos/pheno-config/` subdir to `KooshaPari/Configra/crates/pheno-config/` |
| Public API (`Config`, `ConfigBuilder`, `combine()`) | **Unchanged verbatim** — consumers continue to depend on `pheno-config` exactly as before |

The crate was already a leaf library; nothing in its public surface changed.
Only the source-of-truth repo changed.

## Where the canonical code lives now

- **Canonical repo**: [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra)
- **Crate path**: `crates/pheno-config/`
- **Workspace member**: listed in `KooshaPari/Configra/Cargo.toml` as `crates/pheno-config` (alongside `crates/settly`)
- **Default branch**: `main`
- **Absorb PR**: [#45](https://github.com/KooshaPari/Configra/pull/45)

## How to migrate consumers

### For meta-repo `pheno-*` consumers (most common)

If your `Cargo.toml` references `pheno-config` as a **path** dependency to the
meta-repo subdir:

```diff
- [dependencies]
- pheno-config = { path = "../pheno-config" }
+ [dependencies]
+ pheno-config = { git = "https://github.com/KooshaPari/Configra", tag = "pheno-config-v0.2.0" }
```

If your `Cargo.toml` already uses `pheno-config` as a crates.io or git
dependency, **no change is required** — the package name and version are
unchanged. Only the source repo that publishes the crate has moved.

### For external consumers (crates.io users)

```diff
- [dependencies]
- pheno-config = "0.2"
+ [dependencies]
- # unchanged: still resolves to crates.io as `pheno-config` 0.2.x
- pheno-config = "0.2"
```

The crate continues to publish to crates.io. The source-of-truth moved to
`KooshaPari/Configra`, but the published artifact is bit-for-bit identical for
the 0.2.x line.

### Imports

```diff
  // unchanged — public API preserved
  use pheno_config::{Config, ConfigBuilder};
```

### Workspace members

If your workspace previously listed `crates/pheno-config` as a local member
pointing into the meta-repo, point it at Configra instead:

```diff
  # Cargo.toml [workspace]
  members = [
-     "../pheno-config",
+     "../Configra/crates/pheno-config",
  ]
```

## Timeline

| Date          | Event                                                                                |
|---------------|--------------------------------------------------------------------------------------|
| 2026-06-12    | `pheno-config` 0.1.0 — initial release (env, JSON, ConfigBuilder).                  |
| 2026-06-15    | `pheno-config` 0.2.0 — TOML loading, `Config::merge`, `combine()` (ADR-012 PR-6).  |
| 2026-06-17    | ADR-031 accepted.                                                                    |
| 2026-06-18    | Absorb PR #45 merged.                                                               |
| 2026-06-18    | This migration doc authored (T10.6 of v8 DAG).                                      |
| 2026-07-16    | 28-day grace period for any straggler consumers (per ADR-031).                      |

## Cross-references

- **ADR-031** — `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- **ADR-022** — config consolidation (Rust/TS split)
- **ADR-023** — substrate placement (Rule 3)
- Configra PR [#45](https://github.com/KooshaPari/Configra/pull/45) — the absorb
- `findings/2026-06-17-L5-104-7-configra-absorb-plan.md`
- `plans/2026-06-18-v8-dag-stable.md` § 3.2 (T10)

L5-104.7 — T10.6 / 2026-06-18-from-pheno-config
