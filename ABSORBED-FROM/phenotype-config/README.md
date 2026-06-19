# ABSORBED-FROM: `phenotype-config`

**Original location:** https://github.com/KooshaPari/phenotype-config
**Status:** DEPRECATED 2026-06-17, scheduled archive 2026-07-15 (28-day grace)
**Absorbed into Configra:** [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (settly absorb), [phenotype-config PR #1](https://github.com/KooshaPari/phenotype-config/pull/1) (CANONICAL markers), [phenotype-config PR #2](https://github.com/KooshaPari/phenotype-config/pull/2) (absorb phenotype-config-loader from phenoShared)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| Hexagonal config (`settly` domain/application/adapters/infrastructure) | `KooshaPari/phenotype-config:crates/settly/` | [`Configra:crates/settly/`](../../crates/settly/) |
| `phenotype-config-loader` (typed JSON/TOML loader) | `KooshaPari/phenotype-config:crates/phenotype-config-loader/` | preserved in phenotype-config (DEPRECATED); canonical Configra has `Configra:crates/pheno-config/` (typed `Config`) |
| TS edge bindings | `KooshaPari/phenotype-config:typescript/` | [`Configra:typescript/`](../../typescript/) |
| Documentation (charter.md, intent.md, okf/, docs/, SOTA.md, review.md) | `KooshaPari/phenotype-config:{charter.md,intent.md,okf/,docs/,SOTA.md,review.md}` | preserved in `phenotype-config` (DEPRECATED) |

## Migration status

- [x] `crates/settly/` absorbed into `Configra:crates/settly/` via [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (MERGED 2026-06-18 07:09)
- [x] `crates/phenotype-config-loader/` absorbed from phenoShared via [phenotype-config PR #2](https://github.com/KooshaPari/phenotype-config/pull/2) (MERGED 2026-06-18 12:50)
- [x] CANONICAL.md markers + SLSA doc ported via [phenotype-config PR #1](https://github.com/KooshaPari/phenotype-config/pull/1) (MERGED 2026-06-18 04:45)
- [x] `CANONICAL_REDIRECT.md` + `MIGRATE_TO_CONFIGRA.md` + README deprecation header (merged 2026-06-17)
- [x] L5-110 `CANONICAL.md` + `DEPRECATED.md` + `CHANGELOG.md` ([phenotype-config PR #3](https://github.com/KooshaPari/phenotype-config/pull/3), OPEN 2026-06-18)
- [ ] `phenotype-config` archive on 2026-07-15 (28-day grace period)

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
