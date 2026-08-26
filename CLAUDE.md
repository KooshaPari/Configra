# CLAUDE.md

## Project Overview

Configra (formerly phenotype-config) is a Rust configuration framework for Phenotype projects. It provides typed config loading, settings lifecycle management, JSON schema validation, and encryption-at-rest. It is a tier-2 library substrate with a five-crate Cargo workspace.

## Tech Stack

- **Language:** Rust 1.75+ (MSRV), TypeScript edge layer
- **Cryptography:** AES-256-GCM + Argon2id KDF (in `settly`, behind `encryption` feature flag)
- **Hot-reload:** `notify` v6 + tokio broadcast channel (behind `hot-reload` feature flag)
- **Supply chain:** `cargo-deny`, `cargo-audit`, CycloneDX SBOM, TruffleHog

## Workspace Crates

| Crate | Purpose |
|---|---|
| `pheno-config` | Typed `Config` struct with env-var + TOML + builder loading |
| `settly` | Settings lifecycle, validation, migration, encryption-at-rest |
| `config-schema` | JSON schema field/object validation primitives |
| `phenotype-config-loader` | Generic `load_json<T>` / `load_toml<T>` file loaders |
| `configra-ops` | Observability + ops primitives, health/version CLI |

## Build & Test

```bash
cargo build --workspace
cargo test --workspace
cargo run -p configra-ops -- health
cargo run -p configra-ops -- version
```

## Development Notes

- Pre-1.0, active development.
- Part of the Phenotype polyrepo portfolio.
- Supply chain: `cargo-deny`, `cargo-audit` weekly, CycloneDX SBOM, TruffleHog scan.
- No external services required for library use.
