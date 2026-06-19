# ABSORBED-FROM: `pheno/crates/phenotype-config-core` (DELETED)

**Original location:** ~~https://github.com/KooshaPari/pheno/tree/main/crates/phenotype-config-core~~ (DELETED 2026-06-18)
**Status:** DELETED (per ADR-022 §4)
**Absorbed into Configra:** [pheno PR #235](https://github.com/KooshaPari/pheno/pull/235) (`chore(pheno): remove phenotype-config-core (ADR-012 config consolidation)`)

---

## What was deleted

| Concern | Original location | Reason for deletion |
|---|---|---|
| `phenotype-config-core` (initial typed config core, 75 LoC) | `KooshaPari/pheno:crates/phenotype-config-core/src/lib.rs` | Superseded by `pheno-config` standalone + `phenotype-config-loader` sub-crate; the `pheno*` fleet converged on the canonical `Configra:crates/pheno-config/` substrate |
| `phenotype-config-core/CANONICAL.md` (stale, pointed to `phenoShared`) | `KooshaPari/pheno:crates/phenotype-config-core/CANONICAL.md` | Stale redirect (per ADR-022 RFC 002, the canonical substrate moved from `phenoShared` to `phenotype-config`); the CANONICAL.md was a leftover artifact |

## Migration status

- [x] `phenotype-config-core/src/lib.rs` (75 LoC) DELETED via [pheno PR #235](https://github.com/KooshaPari/pheno/pull/235) (MERGED 2026-06-18)
- [x] `phenotype-config-core/CANONICAL.md` (stale) DELETED via [pheno PR #235](https://github.com/KooshaPari/pheno/pull/235) (MERGED 2026-06-18)
- [x] `phenotype-config-core` directory removed from `pheno/Cargo.toml` workspace members (MERGED 2026-06-18)

## Notes

- Per ADR-022 §4, the `phenotype-config-core` sub-crate was identified as a DELETION target because it duplicated intent already covered by the standalone `pheno-config` crate and the `phenotype-config-loader` sub-crate.
- The deletion is a one-way migration; the `phenotype-config-core` directory does not exist in `pheno` after PR #235.
- This `ABSORBED-FROM/pheno-subcrate-config-core-deleted/` subdir contains the deletion manifest for historical record.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
