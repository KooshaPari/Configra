# Top-level governance files (absorbed)

> **Absorbed from `KooshaPari/phenotype-config` per ADR-031 (L5-110).**
> Original commit: `f86f8e9` on `main`, 2026-06-17.
> Per ADR-031, all `phenotype-config` content migrates to Configra.
> These 4 top-level governance files (SOTA, charter, intent, review) were
> template placeholders in `phenotype-config`; they are preserved here
> for historical reference and so the methodology is not lost.
>
> The actual Configra governance lives at the Configra repo root
> (no equivalent top-level files at this point — see ADR-031 and the
> Configra README.md).

This directory contains:

| File | Original (phenotype-config) | Status |
|------|------------------------------|--------|
| [SOTA.md](SOTA.md) | top-level SOTA template | Absorbed verbatim |
| [charter.md](charter.md) | config role charter | Absorbed verbatim |
| [intent.md](intent.md) | intent problem statement | Absorbed verbatim |
| [review.md](review.md) | Kilo Code Stand review | Absorbed verbatim |

## Notes

- All 4 files were template placeholders with `{{TEMPLATE}}` markers in
  `phenotype-config`. Configra's actual governance is documented in
  the canonical Configra README.md + ADR-031 + L5-110 audit notes.
- The Kilo Code Stand in `review.md` is a fleet-wide review standard
  that all pheno-* repos follow; it is preserved here for reference.

## Reference

- ADR-031: `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- L5-110 migration plan: `findings/2026-06-18-L5-110-adr-035-impl.md`
- Configra canonical: https://github.com/KooshaPari/Configra
