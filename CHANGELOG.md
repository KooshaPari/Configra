# CHANGELOG — Configra

All notable changes to this project are documented here. Format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Versioning
follows [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Workspace version bumped to 0.4.0 per T16 substrate audit.

## [0.4.0] — 2026-06-20

### Added — Tier-2 substrate enforcement (T16, L5-110)
- **crates/phenotype-config-loader/README.md** — first-class crate-level
  README (was previously inherited from the workspace root only).
- **crates/phenotype-config-loader/CHANGELOG.md** — per-crate changelog
  covering the 0.1.0 → 0.4.0 transition.
- **crates/phenotype-config-loader/AGENTS.md** — agent contract for
  extending the loader surface (one fn per format; one test per fn).
- **crates/config-schema/README.md** — first-class crate-level README.
- **crates/config-schema/CHANGELOG.md** — per-crate changelog.
- **crates/config-schema/AGENTS.md** — agent contract for the field-shape
  validator (no nested schemas; no I/O deps).
- Root `Cargo.toml` — tier-2 quality-bar comment block linking the four
  canonical sub-crates to ADR-022 / ADR-031 / ADR-040.
- Root `README.md` — new "Tier-2 Substrate Layout" section with the
  four-crate split diagram.

### Changed
- Workspace `[workspace.package] version` 0.1.0 → **0.4.0** (T16 bump).
- Sub-crates that pin their own `version` (`pheno-config` at 0.2.0,
  `config-schema` at 0.1.0) are unchanged; sub-crates that derive from
  `version.workspace = true` (`settly`, `phenotype-config-loader`) follow
  the bump.

### Notes
- This is a **docs + version-bump release**. No source-code changes;
  no API changes; no `Cargo.lock` churn beyond version metadata.
- Closes the L5-110 substrate-audit gap: every sub-crate now ships a
  README + CHANGELOG + AGENTS.md + tier-2 coverage gate.
- ADR-023 (agent-effort governance) + ADR-040 (test-coverage gates per
  tier) + ADR-031 (Configra absorb) are the policy basis.

## [0.3.0] — 2026-06-18

### Added
- **crates/phenotype-config-loader** — Absorbed from `KooshaPari/phenotype-config`
  (L5-110, ADR-031). Generic, type-safe JSON and TOML file loaders
  (`load_json<T>`, `load_toml<T>`, `ConfigLoadError`). 67 LoC, 3 unit tests.
- Workspace member `crates/phenotype-config-loader` registered in root `Cargo.toml`.

### Notes
- Source: `KooshaPari/phenotype-config/crates/phenotype-config-loader/`
  (commit `f86f8e9` on `main`, 2026-06-17).
- PR-#52: `feat(configra): absorb phenotype-config-loader crate (ADR-031, L5-110)`.
- See `docs/migrations/2026-06-18-from-phenotype-config.md` for the migration matrix.
- `phenotype-config` is DEPRECATED; archive scheduled 2026-07-15.

## [0.1.0] — 2026-06-18

### Added
- **crates/pheno-config** — Type-gated `Config` + `ConfigBuilder` (645 LoC, byte-identical
  from `repos/pheno-config/`). PR-#45.
- **crates/settly** — Hexagonal config (domain/application/adapters/infrastructure).
  Absorbed from `phenotype-config/crates/settly/` (PR-#44).
- **crates/config-schema** — JSON schema generation. Drained from Conft (PR-#47).
- **typescript/packages/conft/** — TypeScript edge layer drained from Conft (PR-#47).
- `ABSORBED-FROM/` index documenting the 8 source repos (PR-#51).
- `docs/migrations/` with per-source migration notes (5 files).
- Examples: `crates/pheno-config/examples/{cascade,quickstart,validation}.rs` (PR-#48).
- `crates/pheno-config/tests/tracing_test.rs` (PR-#48).
- WORKLOG.md migrated to v2.1 schema (11-col `device:`) (PR-#49).
- Repository URL fix in `crates/settly/Cargo.toml` to point at Configra (PR-#50).
- `feat/migration-docs-batch-2026-06-18` consolidated migration notes (PR-#46).
