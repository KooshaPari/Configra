# Changelog — Configra

All notable changes to this project are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Tier-0 governance & hygiene** (orch-v12-s1-001):
  - Comprehensive `justfile` with `build`, `release`, `check`, `test`,
    `test-doc`, `clippy`, `clippy-strict`, `fmt`, `fmt-check`, `deny`,
    `deny-all`, `audit`, `audit-fix`, `grade`, `hygiene`, `ci`, `docs`,
    `docs-open`, `outdated`, `udeps`, `tree`, `tree-all`, `tree-dups`,
    `clean`, `reset`, `install-cli`, `version`, `default`, `info`.
  - `.github/workflows/ci.yml` rebuilt with **concurrency cancellation**,
    **SHA-pinned actions**, `--locked` flags, MSRV job, docs job, and
    minimal-permissions.
  - `.github/workflows/audit.yml` — weekly RustSec `cargo-audit` run on
    a Tuesday 04:00 UTC cron with concurrency cancellation.
  - `.github/workflows/deny.yml` — `cargo-deny check --all-features --locked`
    on a Sunday 06:00 UTC cron with concurrency cancellation.
  - `.github/workflows/scorecard.yml` — OpenSSF Scorecard weekly run with
    SARIF upload to the Security tab.
  - `.github/workflows/release.yml` rebuilt with **tag-version
    verification**, a `verify-tag` pre-flight job, `test` job gated on the
    tag, sequential publish to crates.io with `katyo/publish-crates`
    waits, and a `github-release` job gated on `publish`.
  - `.github/CODEOWNERS` rewritten — every path explicitly assigned to
    `@kooshapari`, default catch-all present.
  - `deny.toml` enhanced — `multiple-versions = "warn"`,
    `wildcard-dependencies = "deny"`, `confidence-threshold = 0.8`,
    `yanked = "warn"`, plus new `[dependencies]` and `[targets]`
    sections; expanded license allow-list.
  - `.editorconfig` — top-level + per-language overrides.
  - `.gitattributes` — LF normalization, linguist overrides, Git LFS hooks.
  - `CODE_OF_CONDUCT.md` — Contributor Covenant 2.1.
  - `CONTRIBUTING.md` — dev workflow, style guide, Conventional Commits.
  - `SECURITY.md` — coordinated disclosure, threat model, supported
    versions, crypto note.
  - `.github/ISSUE_TEMPLATE/bug_report.md`,
    `.github/ISSUE_TEMPLATE/feature_request.md`,
    `.github/ISSUE_TEMPLATE/config.yml`.
  - `.github/PULL_REQUEST_TEMPLATE.md`.

### Changed

- `justfile` expanded from 24 to ~80 recipes (variables, defaults,
  per-crate selectors, info command).
- `deny.toml` license list grew; sources hardened (`unknown-registry = deny`).
- `.github/CODEOWNERS` normalized handle to lowercase `@kooshapari`
  (GitHub canonical form).
- `.github/workflows/ci.yml` decoupled from inlined `deny` job (now lives
  in its own workflow for independent scheduling).

### Security

- Every workflow action pinned to a commit SHA (Supply-Chain Levels for
  Software Artifacts — SLSA L3-ready).
- Concurrency cancellation on `ci`, `deny`, `audit`, `scorecard` to
  prevent stale runs from polluting reports.
- Minimal `permissions:` blocks on every job (default `contents: read`).

## [0.1.0] — 2026-06-19

### Added

- Initial repository setup with Cargo workspace (resolver 2)
- `crates/pheno-config/` — core config types and traits
- `crates/config-schema/` — schema validation layer
- `crates/settly/` — legacy config crate (absorbed)
- `crates/configra-config/` — extracted hardcoded config (L5-110)
- SQLite persistence with auto-migration
- AES-256-GCM secret encryption
- CLI (`phenoctl`) with clap and shell completion
- TUI with ratatui
- Feature flag lifecycle (create, enable, disable, rollback)
- Point-in-time restore for audit trail
- Meta-bundle: SPEC.md, AGENTS.md (v8.1), WORKLOG.md, llms.txt
- Conft (TypeScript edge layer) relationship documented
- `ABSORBED-FROM/` historical index (L5-110)