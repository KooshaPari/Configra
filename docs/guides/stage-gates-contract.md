# Stage-Gates Reusable Contract (v1)

`stage-gates` is promoted as a versioned reusable contract surface for canary-first rollout.

## Canonical Contract

- Contract metadata: `contracts/stage-gates-v1.contract.json`
- Canonical template: `templates/stage-gates/v1/stage-gates.yml`
- Compatibility alias: `templates/stage-gates.yml`

## Strictness Signals

The v1 contract requires strictness wiring via:

- `STAGE_GATES_STRICT` support in the stage-gates workflow template.
- CodeRabbit severity policy in `templates/.coderabbit.yaml`.

## Dependency Fallback Mapping

The canary template-sync lane consumes owner-scoped fallback mappings from:

- `templates/template-sync/dependency-fallbacks.csv`

Columns:

- `owner`
- `dependency_key` (`template_repo` or `stage_gates_repo`)
- `fallback_slug`
- `fallback_workspace_dir`
- `remediation_hint`

This enables deterministic fallback resolution and actionable repo-level remediation hints when dependency checkout fails.

## Canary Output Artifacts

Canary runs must emit machine-readable outputs:

- Readiness matrix CSV: `stage-gates-canary-readiness-matrix-<run_id>-<attempt>.csv`
- Rollout decision CSV: `stage-gates-canary-decision-<run_id>-<attempt>.csv`
- Rollout decision JSON: `stage-gates-canary-decision-<run_id>-<attempt>.json`
- Ranked next-repo rollout CSV: `stage-gates-next-repo-rollout-<run_id>-<attempt>.csv`
- Ranked next-repo rollout JSON: `stage-gates-next-repo-rollout-<run_id>-<attempt>.json`

Remediation output should include:

- `remediation_category` (deterministic enum)
- `remediation_hint` (human-readable short hint)
- `next_command` (concrete command to execute next for that category)

## Canary Phase-Gate Criteria

Use deterministic pass/fail criteria for canary rollout decisions:

- `hold` fail conditions: analyzed coverage `< 100%` or any `repo_unreachable` run outcome.
- `phase-1` pass conditions: analyzed coverage `= 100%` and `canary_ready=true` for at least `50%` of canary repos.
- `phase-2` pass conditions: analyzed coverage `= 100%` and `canary_ready=true` for at least `80%` of canary repos.
- `broad` pass conditions: analyzed coverage `= 100%` and `canary_ready=true` for `100%` of canary repos for `2` consecutive runs.

Recommended next candidate repos after `phase-1` pass:

- `parpour`
- `phenodocs`
- `tokenledger`

## SemVer Policy

- Major: breaking gate/stage/output contract changes.
- Minor: backward-compatible stage/gate additions.
- Patch: non-breaking fixes and docs-only updates.

## Validation

Run:

```bash
bash scripts/validate_stage_gates_contract.sh
```

This checks contract metadata and required artifacts for the versioned stage-gates surface.
