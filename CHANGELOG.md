# Changelog — Configra

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-06-19

### Added

- Initial repository setup with Cargo workspace (resolver 2)
- `crates/pheno-config/` — core config types and traits
- `crates/config-schema/` — schema validation layer
- `crates/settly/` — legacy config crate (absorbed)
- SQLite persistence with auto-migration
- AES-256-GCM secret encryption
- CLI (`phenoctl`) with clap and shell completion
- TUI with ratatui
- Feature flag lifecycle (create, enable, disable, rollback)
- Point-in-time restore for audit trail
- Meta-bundle: SPEC.md, AGENTS.md (v8.1), WORKLOG.md, llms.txt
- Conft (TypeScript edge layer) relationship documented
