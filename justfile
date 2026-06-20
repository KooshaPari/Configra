# Configra — justfile
# SPDX-License-Identifier: MIT OR Apache-2.0

# ─── Build ────────────────────────────────────────────────────────────────────

# Build the entire workspace
build:
    cargo build --workspace

# Build with release profile
release:
    cargo build --workspace --release

# ─── Test ─────────────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Run tests for a specific crate
test-crate crate:
    cargo test -p {{crate}}

# ─── Lint ─────────────────────────────────────────────────────────────────────

# Run clippy (warnings as errors)
clippy:
    cargo clippy --workspace -- -D warnings

# Run rustfmt check
fmt-check:
    cargo fmt --check

# Format all code
fmt:
    cargo fmt

# ─── Audit ────────────────────────────────────────────────────────────────────

# Run cargo-deny (advisory + license + sources)
deny:
    cargo deny check

# Run cargo-audit
audit:
    cargo audit

# ─── CI (full gate) ───────────────────────────────────────────────────────────

# Full CI gate: format check → clippy → test → deny
ci: fmt-check clippy test deny

# ─── Cleanup ──────────────────────────────────────────────────────────────────

# Clean all build artifacts
clean:
    cargo clean

# ─── Docs ─────────────────────────────────────────────────────────────────────

# Build documentation
docs:
    cargo doc --workspace --no-deps

# Open documentation in browser
docs-open:
    cargo doc --workspace --no-deps --open

# ─── Utilities ────────────────────────────────────────────────────────────────

# Show outdated dependencies
outdated:
    cargo outdated

# Check for unused dependencies
udeps:
    cargo udeps

# Generate dependency tree for a crate
tree crate:
    cargo tree -p {{crate}}
