# Policy Gates and Review Bots

This document defines the canonical strictness behavior for stage-gates and review bots in
Phenotype templates.

## CodeRabbit Strictness Policy

- Canonical configuration lives in both:
  - `.coderabbit.yaml` (live repo config)
  - `templates/.coderabbit.yaml` (propagated template)
- Required baseline:
  - `pr_validation.block_on.severity: info`
- Rationale:
  - Keeps merge blocking at the minimum level to reduce false-positive hard blocks while still
    surfacing all review findings.

## Stage-Gates Strict/Skip Policy

- Workflow: `templates/stage-gates.yml`
- Strict mode signal:
  - Repository variable `STAGE_GATES_STRICT=true` enables strict mode.
  - Any other value (or unset) is treated as non-strict for rollout-safe defaults.

### Behavior in Non-Strict Mode (Default)

- Missing optional stage commands are reported and skipped without failing the workflow:
  - `e2e-smoke`, `e2e-full`, `bench`, `sla-check`, `build`, `regression`
- `targeted-regression` falls back to language-native unit test command when `regression` target is
  missing.

### Behavior in Strict Mode

- Missing stage-required commands fail the job immediately:
  - `e2e-smoke`, `e2e-full`, `bench`, `sla-check`, `build`, `regression`
- This supports policy-gate simulation and high-assurance rollout phases.

## Drift Control

- Any strictness change must update all three artifacts together:
  - `.coderabbit.yaml`
  - `templates/.coderabbit.yaml`
  - `templates/stage-gates.yml`
