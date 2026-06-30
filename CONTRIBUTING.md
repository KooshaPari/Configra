# Contributing to Configra

> **AI-Agent-Only Repository**  
> This repository is planned, maintained, and managed exclusively by AI agents
> (see README badge). Human contributions are welcome but the primary authorship
> model is agent-driven.

## Prerequisites

- Rust 1.75+ (see `rust-toolchain.toml` if present; otherwise `rustup update stable`)
- `cargo-deny` for supply-chain checks: `cargo install cargo-deny`
- `cargo-audit` for advisory checks: `cargo install cargo-audit`

## One-command verification

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo deny check
```

All five must pass before a PR is opened.

## Branch and commit conventions

- Branch prefix: `feat/`, `fix/`, `refactor/`, `docs/`, `chore/`, `ci/`
- Commit format: Conventional Commits — `feat(settly): add hot-reload watcher`
- No force-push to `main`; PRs require 1 reviewer

## Crate structure

Each crate under `crates/` has its own `README.md`, `CHANGELOG.md`, and
`AGENTS.md`. Changes to a crate must keep those files current.

| Crate | Concern |
|---|---|
| `pheno-config` | Typed `Config` + env/TOML/builder loading |
| `settly` | Settings lifecycle, encryption-at-rest, hot-reload |
| `config-schema` | JSON schema validation primitives |
| `phenotype-config-loader` | Generic file loaders |
| `configra-ops` | Observability primitives + health CLI |

## Security

See [SECURITY.md](./SECURITY.md) for the vulnerability disclosure policy.

## License

MIT OR Apache-2.0 — contributions are accepted under the same dual license.
