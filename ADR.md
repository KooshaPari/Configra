# Architecture Decision Records -- phenotype-config

## ADR-001: Rust Workspace with Fine-Grained Crates

**Status:** Accepted
**Context:** The config SDK needs separation of concerns: core types, storage, crypto, CLI.
**Decision:** Rust workspace with four crates: pheno-core (types + traits), pheno-db (SQLite), pheno-crypto (AES-256-GCM), pheno-cli (clap + ratatui).
**Alternatives:** Single crate (less modular), separate repos (too fragmented for a coherent SDK).
**Consequences:** Clean dependency graph; each crate testable in isolation; pheno-cli depends on all three.

## ADR-002: SQLite for Local Storage

**Status:** Accepted
**Context:** Config, flags, and secrets need persistent local storage with query support and audit trail.
**Decision:** Use SQLite via pheno-db with auto-migration, CRUD operations, and audit log table.
**Alternatives:** JSON files (no query/audit), embedded key-value (no relational queries), PostgreSQL (not local-first).
**Consequences:** Single-file database at `.phenotype/config.db`; no external service dependency.

## ADR-003: AES-256-GCM for Secret Encryption

**Status:** Accepted
**Context:** Secrets stored locally must be encrypted at rest.
**Decision:** pheno-crypto uses AES-256-GCM authenticated encryption with key derivation.
**Alternatives:** ChaCha20-Poly1305 (equally valid, less ubiquitous), no encryption (unacceptable).
**Consequences:** Key management becomes a user responsibility; pheno-crypto is a pure crypto module.

## ADR-004: clap + ratatui for CLI/TUI

**Status:** Accepted
**Context:** The user interface needs both scriptable CLI commands and an interactive terminal UI.
**Decision:** Use clap for CLI argument parsing and ratatui for the interactive TUI mode.
**Alternatives:** tui-rs (predecessor, unmaintained), crossterm only (lower-level).
**Consequences:** Two UI modes from a single binary; clap handles dispatch, ratatui handles interactive sessions.
