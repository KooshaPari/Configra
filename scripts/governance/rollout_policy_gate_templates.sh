#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-/Users/kooshapari/CodeProjects/Phenotype/repos}"
MODE="${MODE:-dry-run}" # dry-run | apply

SOURCE_REPO="$(cd "$(dirname "$0")/../.." && pwd)"
WORKFLOW_FILES=(
  "reusable-policy-gate.yml"
  "issue-merge-token.yml"
  "verify-merge-token.yml"
  "policy-gate.yml"
)

TARGETS=()
while IFS= read -r line; do
  TARGETS+=("$line")
done < <(ls -d "$ROOT"/*-governance 2>/dev/null || true)

if [[ "${#TARGETS[@]}" -eq 0 ]]; then
  echo "No governed repos found under $ROOT"
  exit 0
fi

echo -e "repo\tmode\tstatus\tnote"
for repo in "${TARGETS[@]}"; do
  name="$(basename "$repo")"
  workflow_dir="$repo/.github/workflows"

  if [[ ! -d "$workflow_dir" ]]; then
    echo -e "$name\t$MODE\tskipped\tmissing .github/workflows"
    continue
  fi

  if [[ "$MODE" == "apply" ]]; then
    cp "$SOURCE_REPO/templates/reusable-policy-gate.yml" "$workflow_dir/reusable-policy-gate.yml"
    cp "$SOURCE_REPO/templates/workflows/issue-merge-token.yml" "$workflow_dir/issue-merge-token.yml"
    cp "$SOURCE_REPO/templates/workflows/verify-merge-token.yml" "$workflow_dir/verify-merge-token.yml"
    cp "$SOURCE_REPO/.github/workflows/policy-gate.yml" "$workflow_dir/policy-gate.yml"
    echo -e "$name\t$MODE\tapplied\t4 workflows synced"
  else
    echo -e "$name\t$MODE\tready\twould sync 4 workflows"
  fi
done
