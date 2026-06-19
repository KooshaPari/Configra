# ABSORBED-FROM: `Conft`

**Original location:** https://github.com/KooshaPari/Conft
**Status:** ARCHIVED
**Absorbed into Configra:** [Configra PR #47](https://github.com/KooshaPari/Configra/pull/47) (drain Conft unique content into Configra — config-schema + config-ts)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| `config-schema` (TypeScript config schema definition) | `KooshaPari/Conft:typescript/config-schema/` | [`Configra:typescript/`](../../typescript/) (preserved) |
| `config-ts` (TypeScript config loader) | `KooshaPari/Conft:typescript/config-ts/` | [`Configra:typescript/`](../../typescript/) (preserved) |
| Documentation (AGENTS.md, charter.md, intent.md, SOTA.md, etc.) | `KooshaPari/Conft:{AGENTS.md,charter.md,intent.md,SOTA.md}` | preserved in Conft (ARCHIVED) |

## Migration status

- [x] `typescript/config-schema/` drained into `Configra:typescript/` via [Configra PR #47](https://github.com/KooshaPari/Configra/pull/47) (MERGED 2026-06-19 01:10)
- [x] `typescript/config-ts/` drained into `Configra:typescript/` via [Configra PR #47](https://github.com/KooshaPari/Configra/pull/47) (MERGED 2026-06-19 01:10)
- [x] Migration log at `Configra:docs/migrations/2026-06-18-from-conft.md` (MERGED)
- [x] Conft archived (read-only marker; no further pushes possible)

## Notes

- Conft was the TS edge binding for the `phenotype-config` substrate per ADR-022.
- Per ADR-031, the TS edge is now preserved in `Configra:typescript/` (the canonical Configra repo) rather than in a sister repo.
- The 4 local-only Conft worktrees (`Conft-hygiene`, `Conft-4th`, `Conft-5th`, `Conft-6th`) are preserved in the local monorepo but not pushed to GitHub.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
