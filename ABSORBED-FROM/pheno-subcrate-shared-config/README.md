# ABSORBED-FROM: `pheno/crates/phenotype-shared-config` (sub-crate)

**Original location:** https://github.com/KooshaPari/pheno/tree/main/crates/phenotype-shared-config
**Status:** Preserved in `pheno` (NOT deleted); CANONICAL.md marker re-pointed to Configra
**Absorbed into Configra:** [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238) (CANONICAL.md marker only — no code absorbed)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| SDK helpers (`ConfigSource`, `ConfigValue`, `SourcePriority`, `ConfigFormat`, `ConfigMeta`, `search_config_dirs`, `AppDirs`, `ConfigDir`, `ConfigError`) | `KooshaPari/pheno:crates/phenotype-shared-config/src/{lib,dirs,error,format,source}.rs` (33 LoC `lib.rs`) | NO direct replacement (intentionally not absorbed; the `pheno*` fleet has no current consumer of this API) |
| CANONICAL.md (re-pointed to Configra) | (none — new) | [`KooshaPari/pheno:crates/phenotype-shared-config/CANONICAL.md`](https://github.com/KooshaPari/pheno/blob/chore/l5-110-adr-031-configra-canonical-markers-2026-06-18/crates/phenotype-shared-config/CANONICAL.md) (added by [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238)) |

## Migration status

- [x] `phenotype-shared-config` source preserved in `pheno/crates/phenotype-shared-config/`
- [x] CANONICAL.md marker added in `pheno/crates/phenotype-shared-config/CANONICAL.md` via [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238) (OPEN 2026-06-18)

## Notes

- Despite the `phenotype-shared-config` name, this crate is NOT the canonical shared config. The name is misleading.
- The canonical Rust config is `KooshaPari/Configra:crates/pheno-config/` (the substrate absorbed via [Configra PR #45](https://github.com/KooshaPari/Configra/pull/45)).
- The canonical hexagonal config is `KooshaPari/Configra:crates/settly/` (absorbed via [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44)).
- The sub-crate is preserved as a historical artifact; the `pheno*` fleet has no current consumer of this API after the Configra migration. The new `CANONICAL.md` marker explicitly disambiguates.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
