# CHANGELOG — Configra

All notable changes to this project are documented here. Format follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Versioning
follows [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html).

## [Unreleased] — 2026-06-19

### Added

- Meta-bundle per preflight Gate 1 (T10.1 remediation):
  - `AGENTS.md` (v8.1 template, substrate + scope)
  - `llms.txt` (LLM context index)
  - `WORKLOG.md` (ADR-015 v2.1 schema with `device:` column)
  - `SSOT.md` (single source of truth — file layout + conventions)
  - `LICENSE-MIT` + `LICENSE-APACHE` (dual licensing)
  - `docs/SPEC.md` (1-page specification)
- SLSA provenance scaffolding per preflight Gate 3 (T10.1 remediation):
  - `docs/slsa.md` (SLSA provenance policy)
  - `.github/workflows/release-attestation.yml` (stub: cargo build + cosign sign)
  - `.github/workflows/slsa-provenance.yml` (stub: slsa-github-generator)

### Notes

- Preflight gate remediation PR (T10.1, v8 batch 11E)
- Branch: `wip-2026-06-19-configra-gate-remediation`
- See `docs/SPEC.md` and `docs/slsa.md` for normative policy.

## [Unreleased] — 2026-06-18

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
