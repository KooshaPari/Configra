> **Work state:** ACTIVE · **Progress:** `█████░░░░░ 50%`
> Rust configuration framework (layered config/env/secrets); pre-1.0 · updated 2026-06-02

> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# Configra (phenotype-config)

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.83+-orange.svg?logo=rust&logoColor=white)](Cargo.toml)
[![Status](https://img.shields.io/badge/status-active-brightgreen.svg)](#)

Local-first configuration management, feature flags, secrets, and version tracking for Phenotype projects with auditable change history and CLI-first workflows.

## Overview

Configra is a comprehensive configuration SDK for Phenotype projects providing consistent management of settings, feature flags, secrets, and version information. It offers local-first persistence with team collaboration support, full audit trails, and point-in-time restore capabilities.

## Technology Stack

- **Language**: Rust (primary), with Go/Python bindings available
- **Frameworks**: Clap (CLI), Ratatui (TUI)
- **Persistence**: SQLite with auto-migration
- **Cryptography**: AES-256-GCM for secrets
- **Key Crates**: pheno-core, pheno-db, pheno-crypto, pheno-cli

## Key Features

- Local-first configuration with team sync support
- Feature flag lifecycle management (create, enable, disable, rollback)
- Secret value storage with encryption at rest
- Version inspection and rollout state tracking
- Comprehensive audit trail with change history
- Point-in-time restore capabilities
- Interactive terminal UI (`phenoctl tui`)
- CLI-first workflows with shell completion
- SQLite backend with auto-migration
- Zero external service dependencies

## Quick Start

```bash
# Clone repository
git clone https://github.com/KooshaPari/Configra.git
cd Configra

# Review governance
cat CLAUDE.md

# Build all crates
cargo build --workspace

# Install CLI tool
cargo install --path pheno-cli

# Configure application settings
phenoctl config set app.name "My Application"
phenoctl config set app.version "1.0.0"

# Manage feature flags
phenoctl flags create dark-mode --description "Enable dark mode"
phenoctl flags enable dark-mode
phenoctl flags status

# Manage secrets
phenoctl secrets set API_KEY
phenoctl secrets set DATABASE_URL

# Check versions
phenoctl version show

# Interactive TUI
phenoctl tui

# Run tests
cargo test --workspace
```

## Project Structure

```
Configra/
├── crates/
│   ├── pheno-core/                 # Core types and traits
│   │   ├── lib.rs
│   │   └── models.rs               # ConfigEntry, FeatureFlag, etc.
│   ├── pheno-db/                   # SQLite persistence
│   │   ├── lib.rs
│   │   ├── store.rs                # CRUD operations
│   │   ├── migrations.rs           # Schema management
│   │   └── audit.rs                # Audit trail
│   ├── pheno-crypto/               # AES-256-GCM encryption
│   │   ├── lib.rs
│   │   └── cipher.rs
│   └── pheno-cli/                  # CLI and TUI
│       ├── main.rs                 # Entry point
│       ├── commands/               # Command handlers
│       ├── tui/                    # Terminal UI
│       └── shell/                  # Shell completions
├── docs/
│   ├── ARCHITECTURE.md             # System design
│   ├── QUICKSTART.md               # Getting started
│   ├── GUIDE.md                    # Usage guide
│   └── API.md                      # API reference
├── tests/
│   ├── integration/                # Integration tests
│   └── e2e/                        # End-to-end tests
├── Cargo.toml                      # Workspace manifest
└── Cargo.lock                      # Dependency lock
```

## Related Phenotype Projects

- **[AgilePlus](../AgilePlus)** — Specification and work tracking
- **[phenotype-shared](../phenotype-shared)** — Shared infrastructure
- **[thegent](../thegent)** — Dotfiles management

## Governance & Documentation

- **CLAUDE.md** — Development standards and AgilePlus mandate
- **docs/ARCHITECTURE.md** — System design and crate relationships
- **docs/QUICKSTART.md** — Getting started guide
- **docs/GUIDE.md** — CLI and feature usage
- **License**: MIT

---

**Status**: Active development  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-04-24

## License

MIT — see [LICENSE](./LICENSE).
