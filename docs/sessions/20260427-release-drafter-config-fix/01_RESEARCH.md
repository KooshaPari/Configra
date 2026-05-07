# Research

## Live evidence

- Issue `#3` points to the `Release Drafter` workflow.
- The repository currently has `release-drafter.yml` under `.github/workflows/`, but no
  repo-local `.github/release-drafter.yml` or `.github/release-drafter.yaml`.
- The reusable workflow delegates to `release-drafter/release-drafter@v6`, whose docs require a
  config file in `.github/` by default.

## Decision

Add the default config file in `.github/` rather than changing the reusable workflow wrapper.
