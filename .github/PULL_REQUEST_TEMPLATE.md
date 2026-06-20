# Configra — Pull Request template
# SPDX-License-Identifier: MIT OR Apache-2.0

<!-- Thank you for contributing to Configra! Please fill out the sections below. -->

## Summary

<!-- A 1–3 sentence summary of what this PR changes and why. -->

## Type of change

<!-- Check one or more. -->

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation only
- [ ] Refactor (no functional change)
- [ ] Test addition / improvement
- [ ] Build / CI / tooling

## Related issues

<!-- Link any related issues: `Closes #42`, `Refs #17`, etc. -->

## Crates affected

- [ ] `pheno-config`
- [ ] `config-schema`
- [ ] `settly`
- [ ] `configra-config`
- [ ] CLI (`phenoctl`)
- [ ] TUI
- [ ] Docs only
- [ ] Repo-wide (justfile, workflows, governance)

## Quality gate

<!-- All boxes below must be checked. CI mirrors the same checks. -->

- [ ] `just fmt-check` passes (or `just fmt` applied)
- [ ] `just clippy` passes (`-D warnings`)
- [ ] `just test` passes (workspace)
- [ ] `just deny` passes (or justification in PR body)
- [ ] MSRV (1.75) still compiles (`cargo check --workspace`)
- [ ] `cargo doc --workspace --no-deps` is warning-free
- [ ] New code is covered by tests (unit and/or integration)
- [ ] Public API surface documented (rustdoc + CHANGELOG if user-visible)

## Breaking change details

<!-- Required if you checked "Breaking change" above. -->

- What breaks:
- Migration path:
- Deprecation shim added? [ ] Yes / [ ] No

## Screenshots / logs

<!-- Optional: paste CLI/TUI screenshots or CI logs here. -->

## Checklist

- [ ] My branch is up to date with `main`
- [ ] My commits follow [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] I have read [`CONTRIBUTING.md`](../CONTRIBUTING.md) and [`AGENTS.md`](../AGENTS.md)
- [ ] I have not introduced new clippy warnings
- [ ] I have not introduced new dependency versions without justification