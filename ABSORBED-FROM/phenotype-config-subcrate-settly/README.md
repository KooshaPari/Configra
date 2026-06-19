# ABSORBED-FROM: `phenotype-config/crates/settly` (sub-crate)

**Original location:** https://github.com/KooshaPari/phenotype-config/tree/main/crates/settly
**Status:** Preserved in `phenotype-config` (NOT deleted); absorbed into Configra
**Absorbed into Configra:** [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (4,788 LoC, 89 files)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| `settly` domain (`Config`, `ConfigError`, `ConfigKitError`, `ConfigValue`, `ConfigSource`, `ConfigFormat`, etc.) | `KooshaPari/phenotype-config:crates/settly/src/domain/` | [`Configra:crates/settly/src/domain/`](../../crates/settly/src/domain/) |
| `settly` application (use cases, ports — `Loader` port, `ConfigPort`, `SecretPort`, etc.) | `KooshaPari/phenotype-config:crates/settly/src/application/` | [`Configra:crates/settly/src/application/`](../../crates/settly/src/application/) |
| `settly` adapters (file sources, env sources, TOML/JSON adapters) | `KooshaPari/phenotype-config:crates/settly/src/adapters/` | [`Configra:crates/settly/src/adapters/`](../../crates/settly/src/adapters/) |
| `settly` infrastructure (logging, secret gating, caching) | `KooshaPari/phenotype-config:crates/settly/src/infrastructure/` | [`Configra:crates/settly/src/infrastructure/`](../../crates/settly/src/infrastructure/) |
| `CANONICAL_FROM_PHENO_SHARED_CONFIG.md` (stale, re-pointed) | `KooshaPari/phenotype-config:crates/settly/CANONICAL_FROM_PHENO_SHARED_CONFIG.md` | superseded by [Configra:crates/settly/CANONICAL.md](../../crates/settly/CANONICAL.md) |

## Migration status

- [x] Full `settly` hexagonal crate (4,788 LoC, 89 files) absorbed into `Configra:crates/settly/` via [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (MERGED 2026-06-18 07:09)
- [x] Consolidated into `Configra:crates/settly/` with all pheno-config / settly migrations via [Configra PR #46](https://github.com/KooshaPari/Configra/pull/46) (MERGED 2026-06-19 01:10)
- [x] Stale `CANONICAL_FROM_PHENO_SHARED_CONFIG.md` marker superseded

## Notes

- The `settly` hexagonal crate is the canonical config framework per ADR-022 (preserved in ADR-031).
- The `settly` source code is now duplicated in two places: `KooshaPari/phenotype-config:crates/settly/` (preserved, DEPRECATED repo) and `Configra:crates/settly/` (canonical). The duplication is intentional during the 28-day archive grace period (2026-07-15).
- The `phenotype-config` repo will be archived on 2026-07-15; the `phenotype-config:crates/settly/` copy will be effectively historical after that date.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
