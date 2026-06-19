# ABSORBED-FROM

This directory contains a per-source-repo preservation manifest for the
ADR-031 / L5-110 Configra canonical config migration.

**Canonical home:** [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra)
**ADRs:** [ADR-031](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md) (Configra absorb), [ADR-022](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-022-config-consolidation-two-crate-split.md) (superseded)
**Date:** 2026-06-18
**L5-110 implementation log:** `findings/2026-06-18-L5-110-adr-035-impl.md`

---

## What this directory is

For each of the 8 source repos in the ADR-031 / L5-110 migration scope,
this directory contains a `<source-repo>/` subdir with:

- `README.md` — points to the original location (the source repo on GitHub) and explains what was absorbed
- `CANONICAL.md` — per-substrate canonical marker (re-affirms that this content is now in Configra)
- Preserved source code (where applicable)
- Original LICENSE + attribution (where applicable)

This is a **preservation manifest**, not a code-merge. The actual code is
already in `Configra/crates/` (after the 7 absorption PRs). This directory
serves as a historical record and a single landing site for any code that
didn't make it into the absorption PRs (e.g. local-only source, or content
from archived repos that couldn't be re-pointed in-place).

---

## The 8 source repos

| # | Source | Type | Original location | Absorbed via | Status |
|---|---|---|---|---|---|
| 1 | `KooshaPari/phenotype-config` | GitHub repo (now archived) | https://github.com/KooshaPari/phenotype-config | [Configra #44](https://github.com/KooshaPari/Configra/pull/44) (settly), [phenotype-config #1](https://github.com/KooshaPari/phenotype-config/pull/1) (CANONICAL markers) | MERGED |
| 2 | `KooshaPari/Conft` | GitHub repo (archived) | https://github.com/KooshaPari/Conft | [Configra #47](https://github.com/KooshaPari/Configra/pull/47) (config-schema + config-ts drain) | MERGED |
| 3 | `KooshaPari/Settly` | GitHub repo (archived) | https://github.com/KooshaPari/Settly | [Configra #44](https://github.com/KooshaPari/Configra/pull/44) (settly absorb, supersedes Settly standalone) | MERGED |
| 4 | `repos/pheno-config/` (local-only) | Local workspace member (no GitHub repo) | `repos/pheno-config/` | [Configra #45](https://github.com/KooshaPari/Configra/pull/45) (byte-identical 645-LoC copy) | MERGED |
| 5 | `pheno/crates/phenotype-config-loader` (sub-crate) | Sub-crate in KooshaPari/pheno | https://github.com/KooshaPari/pheno/tree/main/crates/phenotype-config-loader | [phenotype-config #2](https://github.com/KooshaPari/phenotype-config/pull/2) (absorb into phenotype-config), [pheno #238](https://github.com/KooshaPari/pheno/pull/238) (CANONICAL.md marker) | MERGED + OPEN |
| 6 | `pheno/crates/phenotype-shared-config` (sub-crate) | Sub-crate in KooshaPari/pheno | https://github.com/KooshaPari/pheno/tree/main/crates/phenotype-shared-config | [pheno #238](https://github.com/KooshaPari/pheno/pull/238) (CANONICAL.md marker) | OPEN |
| 7 | `pheno/crates/phenotype-config-core` (sub-crate) | Sub-crate in KooshaPari/pheno (DELETED) | (deleted 2026-06-18) | [pheno #235](https://github.com/KooshaPari/pheno/pull/235) (deletion per ADR-022 §4) | MERGED |
| 8 | `phenotype-config/crates/settly` (sub-crate) | Sub-crate in KooshaPari/phenotype-config | https://github.com/KooshaPari/phenotype-config/tree/main/crates/settly | [Configra #44](https://github.com/KooshaPari/Configra/pull/44) (settly absorb) | MERGED |

---

## Per-source subdirectories

- [`phenotype-config/`](./phenotype-config/) — `KooshaPari/phenotype-config` (DEPRECATED, archive 2026-07-15)
- [`Conft/`](./Conft/) — `KooshaPari/Conft` (ARCHIVED, TS edge absorbed)
- [`Settly/`](./Settly/) — `KooshaPari/Settly` (ARCHIVED, universal config absorbed)
- [`pheno-config-local/`](./pheno-config-local/) — `repos/pheno-config/` (local-only, byte-identical copy in Configra)
- [`pheno-subcrate-config-loader/`](./pheno-subcrate-config-loader/) — `pheno/crates/phenotype-config-loader/` (sub-crate, CANONICAL marker added)
- [`pheno-subcrate-shared-config/`](./pheno-subcrate-shared-config/) — `pheno/crates/phenotype-shared-config/` (sub-crate, CANONICAL marker added)
- [`pheno-subcrate-config-core-deleted/`](./pheno-subcrate-config-core-deleted/) — `pheno/crates/phenotype-config-core/` (DELETED 2026-06-18)
- [`phenotype-config-subcrate-settly/`](./phenotype-config-subcrate-settly/) — `phenotype-config/crates/settly/` (sub-crate, absorbed)

---

## Migration matrix (high-level)

| Concern | Canonical Configra location | Migration PRs |
|---|---|---|
| Type-gated `Config` + `ConfigBuilder` + `load_from_env` / `load_from_file` / `load_from_toml_file` / `combine` | `Configra:crates/pheno-config/` (645 LoC, byte-identical from `pheno-config`) | Configra #45, #48 |
| Hexagonal `settly` (domain/application/adapters/infrastructure) | `Configra:crates/settly/` | Configra #44, #46 |
| TS edge bindings (config-schema, config-ts) | `Configra:typescript/` (drained from Conft) | Configra #47 |
| Generic typed JSON/TOML loaders (`load_json<T>`, `load_toml<T>`) | NO direct replacement | (preserved in `pheno/crates/phenotype-config-loader/` for historical reference) |
| SDK helpers (`ConfigSource`, `ConfigValue`, `SourcePriority`, etc.) | NO direct replacement | (preserved in `pheno/crates/phenotype-shared-config/` for historical reference) |

---

## Notes

- All 8 source repos are accounted for. The 3 archived GitHub repos
  (`phenotype-config`, `Conft`, `Settly`) cannot accept new pushes; their
  preserved content is mirrored in this `ABSORBED-FROM/` directory.
- The local-only `repos/pheno-config/` directory has no GitHub counterpart;
  its content is already byte-identical to `Configra:crates/pheno-config/`.
- `pheno/crates/phenotype-config-core/` was DELETED in [pheno PR #235](https://github.com/KooshaPari/pheno/pull/235) per ADR-022 §4; the `ABSORBED-FROM/pheno-subcrate-config-core-deleted/` subdir contains the deletion manifest.
- This PR is OPEN (not merged) per the task constraint "do NOT merge without user approval".
