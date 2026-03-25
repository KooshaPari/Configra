# Governance Templates

Machine-readable governance templates for polyrepo enforcement.

## Files

1. `release-transition-matrix.yaml`
2. `pr-policy-gates.yaml`
3. `ci-required-jobs.yaml`
4. `merge-eligibility-token-schema.yaml`

## Intended Usage

1. Policy-gate workflows read transition and PR policy templates.
2. CI stage-gates map stages to required jobs using CI matrix template.
3. Merge gateway verifies CI-issued signed eligibility tokens against token schema.
4. Module/trunk promotions are allowed only when transition matrix permits.
