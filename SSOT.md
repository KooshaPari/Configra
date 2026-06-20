# SSOT.md — Configra (Single Source of Truth)

**Date:** 2026-06-19
**Status:** ACTIVE

---

## File layout

```
Configra/
├── AGENTS.md                       # Project overview, scope, commands, conventions
├── llms.txt                        # LLM context index
├── SPEC.md                         # Top-level spec
├── SSOT.md                         # THIS FILE — single source of truth
├── WORKLOG.md                      # v2.1 schema worklog (11 cols incl. device:)
├── CHANGELOG.md                    # Keep a Changelog 1.1.0 format
├── README.md                       # Human-readable intro
├── LICENSE-MIT                     # MIT license text
├── LICENSE-APACHE                  # Apache 2.0 license text
├── Cargo.toml                      # Workspace root (resolver 2, MSRV 1.75)
├── Cargo.lock                      # Lockfile (committed)
├── deny.toml                       # cargo-deny config
├── FUNDING.yml                     # GitHub funding config
├── crates/                         # Workspace members
│   ├── pheno-config/               # Core config types + loader (645 LoC)
│   ├── config-schema/              # JSON-Schema validation
│   ├── phenotype-config-loader/    # Generic TOML/JSON loaders (absorbed 2026-06-18)
│   └── settly/                     # Legacy config crate (absorbed, back-compat)
├── typescript/                     # @phenotype/config-ts edge layer
│   └── packages/conft/             # (formerly Conft/typescript/)
├── docs/                           # Documentation
│   ├── SPEC.md                     # 1-page spec
│   ├── ARCHITECTURE.md             # Deeper architecture
│   ├── CONFIG.md                   # Meta-config reference
│   ├── slsa.md                     # SLSA provenance policy
│   ├── migrations/                 # Per-source migration notes
│   └── phenotype-config-absorbed/  # Absorbed docs from phenotype-config
├── ABSORBED-FROM/                  # Index of absorbed source repos
└── .github/
    └── workflows/
        ├── ci.yml                  # CI gate (fmt + clippy + test + deny)
        ├── release.yml             # crates.io publish
        ├── release-attestation.yml # cosign release attestation (stub)
        └── slsa-provenance.yml     # slsa-github-generator (stub)
```

## Conventions

### Branching

- `feat/<req-id>-<slug>-<date>` for features
- `chore/<req-id>-<slug>-<date>` for chore
- `fix/<req-id>-<slug>-<date>` for fixes
- `docs/<req-id>-<slug>-<date>` for docs

### Commits

- Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`,
  `test:`, `build:`, `ci:`)
- Optional scope in parens, e.g. `feat(configra): absorb phenotype-config`

### Worklog schema

- ADR-015 v2.1 (11 columns including `device:`)
- `device:` values: `macbook`, `heavy-runner`, `subagent`, `ci`

### Coverage gate (ADR-040)

- ≥80% for libs (this crate qualifies as lib)
- Enforced via `cargo-tarpaulin` in CI

### License

- MIT OR Apache-2.0 (dual)
- All source files use SPDX-License-Identifier in headers

## Source of authority

| Topic | Authority |
|---|---|
| Public API | `crates/pheno-config/src/lib.rs` |
| CLI surface | `crates/pheno-cli/src/main.rs` |
| Schema grammar | `crates/config-schema/src/` |
| Crypto defaults | `crates/pheno-config/src/crypto.rs` |
| Layered loading order | `SPEC.md` § "Layered loading" |
| Branching policy | This file § "Branching" |
| Commit policy | This file § "Commits" |
| Worklog schema | ADR-015 + ADR-025 |
| Coverage gate | ADR-040 |
| Migration history | `docs/migrations/` + `ABSORBED-FROM/` |

## Change process

1. Author an ADR if the change is governance-level (move to new repo,
   deprecate, archive, rename).
2. Create branch per convention.
3. Implement + tests + docs.
4. Update WORKLOG.md (1 row) + CHANGELOG.md (`[Unreleased]` section).
5. PR with conventional commit title.
6. CI must pass (fmt + clippy + test + deny + audit).
7. Coverage gate must hold (≥80%).
8. Squash merge; delete branch on merge.

## Anti-patterns

- ❌ Direct edits to `main` (always PR)
- ❌ Force-push to `main`
- ❌ Adding new top-level config files outside `crates/`
- ❌ Skipping WORKLOG.md update
- ❌ Skipping CHANGELOG.md entry
- ❌ Adding deps without updating `Cargo.lock` via `cargo update`
- ❌ Using `unwrap()` in non-test code (use `?` + thiserror)
- ❌ Secrets in plaintext at rest

## Related

- ADR-022 — Rust/TS split
- ADR-031 — Canonical name (supersedes `phenotype-config`)
- ADR-035 — Configra migration gates
- ADR-040 — Coverage gates per tier
- `AGENTS.md` — human/AI orientation
- `SPEC.md` — spec
- `docs/ARCHITECTURE.md` — architecture
- `docs/slsa.md` — SLSA policy