# Contributing to Configra
# SPDX-License-Identifier: MIT OR Apache-2.0

Thanks for your interest in contributing to **Configra** — the canonical Rust
configuration substrate for the Phenotype organization. This guide explains how
to set up your environment, propose changes, and submit a pull request.

> **AI-DD notice**: Configra is planned, maintained, and managed by AI Agents
> under the Phenotype AI-DD metaproject. See `AGENTS.md` for the agent
> playbook. Human contributions are still welcome; bug reports and small
> PRs will be triaged asynchronously.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Project structure](#project-structure)
3. [Getting started](#getting-started)
4. [Development workflow](#development-workflow)
5. [Style guide](#style-guide)
6. [Commit messages](#commit-messages)
7. [Pull requests](#pull-requests)
8. [Reporting bugs](#reporting-bugs)
9. [Suggesting features](#suggesting-features)
10. [Security](#security)
11. [License](#license)

## Code of Conduct

This project and everyone participating in it is governed by the
[Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected
to uphold this code. Report unacceptable behavior to `koosha@phenotype.org`.

## Project structure

```
Configra/
├── crates/
│   ├── pheno-config/         # canonical config types + traits
│   ├── config-schema/        # schema validation layer
│   ├── settly/               # legacy config crate (absorbed)
│   └── configra-config/      # extracted hardcoded config
├── typescript/               # TypeScript edge bindings (Conft)
├── docs/                     # architecture, migrations, config docs
├── ABSORBED-FROM/            # absorbed-content index (historical)
├── .github/                  # workflows, templates, CODEOWNERS
├── justfile                  # task runner
├── deny.toml                 # cargo-deny configuration
└── Cargo.toml                # workspace manifest
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the full hexagonal
port-adapter design.

## Getting started

### Prerequisites

- **Rust** ≥ 1.75 (MSRV). Install via [rustup](https://rustup.rs/).
- **`just`** task runner. `cargo install just`
- **`cargo-deny`** for license/advisory checks. `cargo install cargo-deny --locked`
- **`cargo-audit`** for RustSec advisories. `cargo install cargo-audit --locked`

### Clone

```bash
git clone https://github.com/KooshaPari/Configra.git
cd Configra
```

### Build & test

```bash
just build      # cargo build --workspace
just test       # cargo test --workspace
just ci         # fmt + clippy + test + deny  (matches GitHub CI)
```

## Development workflow

1. **Branch off `main`** using one of:
   - `feat/<slug>-<YYYY-MM-DD>` for new features
   - `fix/<slug>-<YYYY-MM-DD>` for bug fixes
   - `chore/<slug>-<YYYY-MM-DD>` for housekeeping

2. **Make your changes** in small, focused commits.

3. **Run the local CI gate** before pushing:
   ```bash
   just ci
   ```

4. **Push and open a PR** against `main`. Fill out the PR template.

5. **Address review feedback** — reviews may come from automated agents
   under the AI-DD metaproject and from humans on the `@kooshapari` review
   queue.

6. **Squash-merge** once approved. The PR title becomes the commit message
   on `main`.

## Style guide

- **Rust**: standard `rustfmt` (default style) + `clippy::all` clean.
  Run `just fmt` and `just clippy` before committing.
- **TOML**: keep workspace manifests sorted — see `cargo-sort` comments in
  `justfile`.
- **Markdown**: GitHub-flavored Markdown, ATX-style headings, 100-col
  soft-wrap is fine.
- **YAML/JSON**: 2-space indent (enforced via `.editorconfig`).
- **Line endings**: LF everywhere except Windows batch files (none today).

## Commit messages

We follow **[Conventional Commits](https://www.conventionalcommits.org/)**:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Common types:

| Type       | Purpose                                              |
| ---------- | ---------------------------------------------------- |
| `feat`     | A new user-facing feature                            |
| `fix`      | A bug fix                                            |
| `chore`    | Tooling, governance, or non-functional changes       |
| `docs`     | Documentation only changes                           |
| `refactor` | Code change that neither fixes a bug nor adds a feat |
| `test`     | Adding or correcting tests                           |
| `build`    | Changes to build system or external dependencies     |
| `ci`       | Changes to CI configuration files/scripts            |
| `perf`     | A code change that improves performance              |

Examples:

```
feat(config): add layered override resolution
fix(secret): handle 32-byte nonce mismatch on decrypt
chore(ci): pin GitHub Actions to commit SHAs
```

## Pull requests

- Fill out the PR template completely.
- Link the relevant issue (e.g. `Closes #42`).
- Keep PRs small and focused. Aim for <800 lines of diff where possible.
- Ensure CI is green before requesting review.
- PR labels: `governance` for cleanup work, `breaking` for SemVer-major
  changes.

## Reporting bugs

Use the **Bug report** issue template. Please include:

- Rust version (`rustc --version`)
- OS and architecture
- Minimal reproduction (cargo snippet or shell commands)
- Expected vs. actual behavior
- Stack trace (if applicable)

## Suggesting features

Use the **Feature request** template. Explain the **why**, not just the
**what**. We prefer proposals that come with:

- A concrete use case
- A short design sketch (interfaces, trait changes, or example usage)
- A note on migration impact

## Security

Please **do not** file public issues for suspected vulnerabilities. See
[`SECURITY.md`](SECURITY.md) for coordinated disclosure instructions.

## License

By contributing to Configra, you agree that your contributions will be
licensed under the project's dual **MIT OR Apache-2.0** license. See
[`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE) (or
the equivalent header in each source file) for details.

---

Questions? Open a discussion or reach out via `koosha@phenotype.org`.