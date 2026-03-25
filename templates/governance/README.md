# Governance Templates

This directory defines the machine-readable release governance contract for the full funnel:

`SP -> POC -> IP -> A -> FP -> B -> EP -> CN -> RC -> GA -> PROD -> LTS -> SS -> DEP -> AR -> EOL`

`HF` is a parallel expedited maintenance lane that can re-enter `RC/GA/PROD/LTS`.

Files:
- `release-transition-matrix.yaml`: forward-only transition matrix and blocked transition policy.
- `pr-policy-gates.yaml`: PR governance requirements by stage (approvals, code owners, strict deltas).
- `ci-required-jobs.yaml`: CI required jobs by stage, branch-to-stage rules, strict-stage overrides.
- `merge-eligibility-token-schema.yaml`: schema for merge-eligibility attestations/tokens.

## Resolver Scripts

- `scripts/governance/resolve_stage_requirements.py`
  - Resolves stage + strict mode + required jobs.
  - Example: `python scripts/governance/resolve_stage_requirements.py --branch release/1.2.0-rc.1`

- `scripts/governance/enforce_policy_gate.py`
  - Loads and enforces all governance templates.
  - Emits a merge-eligibility token payload JSON.

## Reusable Workflows

- `templates/reusable-policy-gate.yml`
- `templates/workflows/issue-merge-token.yml`
- `templates/workflows/verify-merge-token.yml`

These are intended to be rolled out into `.github/workflows/` for governed repositories.
