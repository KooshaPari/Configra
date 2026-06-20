# Configra — justfile
# SPDX-License-Identifier: MIT OR Apache-2.0
#
# Task runner mirroring CI. `just ci` is the canonical local gate.
#
# Install: `cargo install just` (https://github.com/casey/just)
# Run a recipe: `just <recipe>` (e.g. `just test`)
# List recipes: `just --list`

set shell := ["bash", "-uc"]
set dotenv-load

# ─── Variables ────────────────────────────────────────────────────────────────

crates := "pheno-config config-schema settly configra-config"
msrv   := "1.75"
rustflags := "-D warnings"

# ─── Build ────────────────────────────────────────────────────────────────────

# Build the entire workspace (debug)
build:
    cargo build --workspace

# Build with release profile
release:
    cargo build --workspace --release

# Build all workspace tests (no run)
build-tests:
    cargo build --workspace --tests

# Check the workspace (fast type-check)
check:
    cargo check --workspace --all-targets

# Check a single crate: just check-one pheno-config
check-one crate:
    cargo check -p {{crate}} --all-targets

# ─── Test ─────────────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test --workspace

# Run tests with stdout/stderr capture disabled
test-verbose:
    cargo test --workspace -- --nocapture

# Run tests for a specific crate: just test-crate pheno-config
test-crate crate:
    cargo test -p {{crate}}

# Run doc tests only
test-doc:
    cargo test --workspace --doc

# ─── Lint ─────────────────────────────────────────────────────────────────────

# Run clippy (warnings as errors) across the workspace
clippy:
    RUSTFLAGS="{{rustflags}}" cargo clippy --workspace --all-targets -- -D warnings

# Strict clippy with clippy::pedantic nursery disabled
clippy-strict:
    cargo clippy --workspace --all-targets -- -D warnings -W clippy::pedantic

# Check formatting (CI gate)
fmt-check:
    cargo fmt --check

# Format all code in place
fmt:
    cargo fmt

# Format-check all TOML files in the workspace
toml-fmt-check:
    cargo sort --workspace --check

# Sort all workspace dependencies
toml-fmt:
    cargo sort --workspace

# ─── Audit ────────────────────────────────────────────────────────────────────

# Run cargo-deny (advisories + licenses + sources + bans)
deny:
    cargo deny check

# Run cargo-deny with all features
deny-all:
    cargo deny check --all-features

# Run cargo-audit (RustSec advisory DB)
audit:
    cargo audit

# Run cargo-audit with fix-dry-run
audit-fix:
    cargo audit fix --dry-run

# ─── Grade ────────────────────────────────────────────────────────────────────

# Aggregate quality report: fmt + clippy + test + deny + audit
grade: fmt-check clippy test deny audit
    @echo "✅ grade: all green"

# Pre-commit hygiene: fmt + clippy + deny
hygiene: fmt-check clippy deny

# ─── CI (full local mirror) ───────────────────────────────────────────────────

# Full CI gate (matches .github/workflows/ci.yml)
ci: fmt-check clippy test deny
    @echo "✅ ci: all green"

# ─── Docs ─────────────────────────────────────────────────────────────────────

# Build docs (no deps)
docs:
    RUSTDOCFLAGS="{{rustflags}}" cargo doc --workspace --no-deps

# Open docs in browser
docs-open:
    RUSTDOCFLAGS="{{rustflags}}" cargo doc --workspace --no-deps --open

# ─── Utilities ────────────────────────────────────────────────────────────────

# Show outdated dependencies
outdated:
    cargo outdated --workspace

# Detect unused dependencies
udeps:
    cargo udeps --workspace

# Dependency tree for a crate: just tree pheno-config
tree crate:
    cargo tree -p {{crate}}

# Full dependency tree (workspace)
tree-all:
    cargo tree --workspace

# Show duplicated dependencies in the dep graph
tree-dups:
    cargo tree --workspace --duplicates

# ─── Cleanup ──────────────────────────────────────────────────────────────────

# Clean all build artifacts
clean:
    cargo clean

# Clean and rebuild
reset: clean build

# ─── Install ──────────────────────────────────────────────────────────────────

# Install the CLI from the local workspace
install-cli:
    cargo install --path crates/pheno-cli --locked

# ─── Bump / version ───────────────────────────────────────────────────────────

# Print the current workspace version
version:
    @grep '^version' Cargo.toml | head -1 | sed -E 's/.*"(.*)".*/\1/'

# ─── Meta ─────────────────────────────────────────────────────────────────────

# List all available recipes (default)
default:
    @just --list

# Show repo quick-info
info:
    @echo "Configra — Rust configuration substrate"
    @echo "  crates:  {{crates}}"
    @echo "  msrv:    {{msrv}}"
    @just version