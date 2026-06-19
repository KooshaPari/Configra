# ABSORBED-FROM: `pheno/crates/phenotype-config-loader` (sub-crate)

**Original location:** https://github.com/KooshaPari/pheno/tree/main/crates/phenotype-config-loader
**Status:** Preserved in `pheno` (NOT deleted); CANONICAL.md marker re-pointed to Configra
**Absorbed into Configra:** [phenotype-config PR #2](https://github.com/KooshaPari/phenotype-config/pull/2) (absorb into phenotype-config, the 2nd half of the absorb), [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238) (CANONICAL.md marker)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| Generic typed JSON/TOML loaders (`load_json<T: DeserializeOwned>`, `load_toml<T: DeserializeOwned>`, `ConfigLoadError`) | `KooshaPari/pheno:crates/phenotype-config-loader/src/lib.rs` (64 LoC) | NO direct replacement (generic `T` loading is intentionally not in the canonical substrate) |
| Tests (`test_load_json`, `test_load_toml`, `test_load_not_found`) | `KooshaPari/pheno:crates/phenotype-config-loader/src/lib.rs` (inline `mod tests`) | preserved in pheno |
| CANONICAL.md (re-pointed to Configra) | (none — new) | [`KooshaPari/pheno:crates/phenotype-config-loader/CANONICAL.md`](https://github.com/KooshaPari/pheno/blob/chore/l5-110-adr-031-configra-canonical-markers-2026-06-18/crates/phenotype-config-loader/CANONICAL.md) (added by [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238)) |

## Migration status

- [x] `phenotype-config-loader` source preserved in `pheno/crates/phenotype-config-loader/`
- [x] Absorbed (a duplicate copy) into `phenotype-config/crates/phenotype-config-loader/` via [phenotype-config PR #2](https://github.com/KooshaPari/phenotype-config/pull/2) (MERGED 2026-06-18 12:50)
- [x] CANONICAL.md marker added in `pheno/crates/phenotype-config-loader/CANONICAL.md` via [pheno PR #238](https://github.com/KooshaPari/pheno/pull/238) (OPEN 2026-06-18)

## Notes

- The sub-crate exposes generic `load_json<T>` / `load_toml<T>` helpers and a `ConfigLoadError` enum.
- The canonical Configra substrate (`Configra:crates/pheno-config/`) does NOT expose a generic `T` loader; it uses a typed `Config` struct instead. Consumers who need a generic loader can either:
  1. Define their own thin wrapper around `serde_json::from_str` / `toml::from_str`
  2. Open a `Configra` PR to upstream a generic helper (deferred; no current consumer in the `pheno*` fleet)
- The sub-crate is preserved as a historical artifact; the `pheno*` fleet has no current consumer of this exact API after the Configra migration.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
