# ABSORBED-FROM: `Settly`

**Original location:** https://github.com/KooshaPari/Settly
**Status:** ARCHIVED
**Absorbed into Configra:** [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (settly absorb — the `settly` crate in Configra supersedes the Settly standalone repo)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| Universal config management (layered configs, validation, environment-aware settings) | `KooshaPari:Settly:src/`, `benches/`, `fuzz/` | superseded by [`Configra:crates/settly/`](../../crates/settly/) (the canonical hexagonal `settly` crate) |
| Documentation (ADR.md, AGENTS.md, charter.md, intent.md, FUNCTIONAL_REQUIREMENTS.md, PRD.md, SPEC.md, etc.) | `KooshaPari/Settly:{ADR.md,AGENTS.md,charter.md,intent.md,FUNCTIONAL_REQUIREMENTS.md,PRD.md,SPEC.md,...}` | preserved in Settly (ARCHIVED) |

## Migration status

- [x] `Settly:src/` (universal config management) superseded by `Configra:crates/settly/` via [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) (MERGED 2026-06-18 07:09)
- [x] Migration log at `Configra:docs/migrations/2026-06-18-from-settly-config.md` (MERGED)
- [x] Settly archived (read-only marker; no further pushes possible)

## Notes

- Settly was the original "universal configuration management" framework with layered configs and validation.
- Per ADR-031, the canonical substrate is `Configra:crates/settly/` (hexagonal architecture: domain/application/adapters/infrastructure).
- The 4 settly-* repos mentioned in ADR-032 ("settle-adapter deprecation — 4 settly-* repos archived") do not exist as separate GitHub repos on KooshaPari; they were either sub-crates within Settly (and superseded by `Configra:crates/settly/`) or planned but never created.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
