#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

usage() {
  echo "usage: $0 verify" >&2
  exit 1
}

[[ "${1:-}" == verify ]] || usage

echo "==> cargo fmt --check (workspace)"
cargo fmt --all -- --check

echo "==> cargo clippy (workspace, -D warnings)"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> cargo test (workspace)"
cargo test --workspace

echo "quality-gate: OK"
