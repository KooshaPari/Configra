#!/usr/bin/env bash
set -euo pipefail

required_paths=(
  "contracts/stage-gates-v1.contract.json"
  "templates/stage-gates/v1/stage-gates.yml"
  "templates/stage-gates.yml"
  "templates/.coderabbit.yaml"
  "templates/template-sync/dependency-fallbacks.csv"
  "docs/guides/stage-gates-contract.md"
  "scripts/validate_stage_gates_contract.sh"
)

missing=0
for path in "${required_paths[@]}"; do
  if [[ -e "${path}" ]]; then
    echo "OK: ${path}"
  else
    echo "MISSING: ${path}"
    missing=1
  fi
done

if [[ "${missing}" -ne 0 ]]; then
  echo "Stage-gates contract validation failed."
  exit 1
fi

echo "Stage-gates contract validation passed."
