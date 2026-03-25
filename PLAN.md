# phenotype-config Implementation Plan

**Status:** Active
**Stack:** Rust workspace (pheno-core, pheno-db, pheno-crypto, pheno-cli)

## Phase 1: Core Types and Storage

| Task | Description | Depends On |
|------|-------------|------------|
| P1.1 | pheno-core: ConfigEntry, FeatureFlag, SecretEntry, VersionInfo types | -- |
| P1.2 | pheno-core: Store traits (ConfigStore, FlagStore, SecretStore) | P1.1 |
| P1.3 | pheno-db: SQLite backend with auto-migration | P1.2 |
| P1.4 | pheno-db: CRUD operations for config, flags, secrets | P1.3 |
| P1.5 | pheno-db: Audit trail table and point-in-time restore | P1.4 |

## Phase 2: Crypto and CLI

| Task | Description | Depends On |
|------|-------------|------------|
| P2.1 | pheno-crypto: AES-256-GCM encrypt/decrypt | P1.1 |
| P2.2 | pheno-cli: clap CLI with config/flags/secrets/version subcommands | P1.4, P2.1 |
| P2.3 | pheno-cli: Secret set with no-echo prompt | P2.1, P2.2 |

## Phase 3: TUI

| Task | Description | Depends On |
|------|-------------|------------|
| P3.1 | ratatui app scaffold with panel navigation | P2.2 |
| P3.2 | Config panel: list, edit, restore | P3.1 |
| P3.3 | Flags panel: create, toggle, list | P3.1 |
| P3.4 | Secrets panel: set, get, list | P3.1 |

## Phase 4: Quality

| Task | Description | Depends On |
|------|-------------|------------|
| P4.1 | Unit tests for pheno-core types and traits | P1.2 |
| P4.2 | Integration tests for pheno-db CRUD and audit | P1.5 |
| P4.3 | CI pipeline: cargo clippy, fmt, test | P4.1 |
