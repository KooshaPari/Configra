# SOTA dimensional research — Configra (absorbed)

> **Absorbed from `KooshaPari/phenotype-config` per ADR-031 (L5-110).**
> Original commit: `f86f8e9` on `main`, 2026-06-17.
> These files are **SOTA dimensional research templates** with `{{TEMPLATE}}`
> placeholders. They document the methodology and dimensions, not the
> actual research output. Configra will fill in these dimensions as
> part of L5-110 / T16 (substrate audit).
>
> Per ADR-031, all `phenotype-config` content migrates here. The templates
> are preserved verbatim so the methodology is not lost.

This directory contains the per-dimension SOTA research templates:

| File | Dimension |
|------|-----------|
| [technical.md](technical.md) | Architecture, algorithms, performance |
| [dx.md](dx.md) | Developer experience, CLI, local dev |
| [ux.md](ux.md) | End-user experience (if applicable) |
| [ax.md](ax.md) | Agent experience (Cursor, forge, Codex, Claude) |
| [security.md](security.md) | Threat model, compliance |
| [ops.md](ops.md) | Deploy, observe, maintain |
| [cost.md](cost.md) | Infra, API, maintenance cost |
| [alternatives.md](alternatives.md) | Master comparison index |
| [fork-rationale.md](fork-rationale.md) | Required if fork |

## Research standard

Each dimension file must include:

1. Weighted requirements
2. ≥3 alternatives (OSS + closed where relevant)
3. Verdict table with rejection reasons
4. Evolution triggers

PRs that introduce new dependencies must update the relevant dimension or add an ADR linked from [alternatives.md](alternatives.md).
