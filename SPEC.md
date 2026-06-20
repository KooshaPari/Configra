# SPEC.md — Configra

**Status:** ACTIVE (canonical, ADR-031)
**Version:** 0.1.0 (pre-1.0)
**Date:** 2026-06-19
**Owner:** configra-circle

---

## Purpose

Configra is the canonical Rust configuration substrate for the Phenotype
organization. It provides 12-factor-style layered configuration loading,
schema validation, feature flags, encrypted secrets, point-in-time audit
trail, CLI, TUI, and TypeScript bindings.

## Non-goals

- Distributed runtime config sync → use `pheno-events` instead.
- Hot-reload across processes → planned post-1.0.
- Config-as-a-service → deferred.

## Layered loading (last-wins cascade)

1. Compile-time defaults (`Config::default()`)
2. TOML/JSON config file (`configra.toml`)
3. Environment variables (`CONFIGRA_*`)
4. CLI flags (`phenoctl ...`)
5. Builder override (`ConfigraConfigBuilder::build()`)

## Public API surface

```rust
pub struct Config { /* generic, type-safe */ }
pub trait ConfigSource { /* env, file, cli, builder */ }
pub enum ConfigError { Missing(String), InvalidType(String), Parse(String) }
```

- `pheno-config` — `Config` struct, `ConfigBuilder`, layered loader
- `config-schema` — JSON-Schema-style validation
- `settly` — legacy config crate (absorbed, kept for back-compat)
- `phenotype-config-loader` — generic TOML/JSON loaders (absorbed 2026-06-18)

## CLI commands

```
phenoctl config set <key> <value>
phenoctl config get <key>
phenoctl flags create <name> [--description <text>]
phenoctl flags enable|disable <name>
phenoctl secrets set <key>          # prompts for value
phenoctl audit log [--since <ts>]
phenoctl restore --to <ts>
```

## Persistence

- SQLite (`configra.db`)
- Auto-migration on startup (`migrations/` table)
- Encrypted at rest (AES-256-GCM for secrets table)

## Security

- Secrets encrypted with AES-256-GCM
- Master key from `CONFIGRA_MASTER_KEY` env (32 bytes, base64)
- Key rotation supported (re-encrypt on first read after rotation)

## Dependencies

- `tokio` (async runtime)
- `serde` + `serde_json` + `toml`
- `thiserror` + `anyhow`
- `aes-gcm` (encryption)
- `rusqlite` (SQLite persistence)
- `clap` (CLI)
- `ratatui` + `crossterm` (TUI)

## Coverage gate

≥80% per ADR-040 (lib tier).

## Related

- `docs/SPEC.md` — this document (1-page)
- `docs/ARCHITECTURE.md` — deeper architecture
- `docs/CONFIG.md` — meta-config reference
- `docs/slsa.md` — SLSA provenance policy
- ADR-022, ADR-031, ADR-035, ADR-040 (governing ADRs)

## License

MIT OR Apache-2.0 (dual). See `LICENSE-MIT` and `LICENSE-APACHE`.