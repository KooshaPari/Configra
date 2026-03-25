# Polyrepo Governance Blueprint

Status: canonical policy blueprint for agent-operated Phenotype polyrepo

## Scope

This blueprint defines end-to-end governance across release/version policy,
PR/review policy, GitOps policy, CI/CD/DevOps policy, and agent execution policy.
It is designed for multi-repo operation with shared templates and reusable workflows.

## Control Planes

1. Policy plane: rulesets, branch protection, required checks, CODEOWNERS, policy-gate.
2. Workflow plane: reusable workflows, template inheritance, pin/version discipline.
3. Delivery plane: release stages, versioning, attestation, promotion control.
4. Runtime plane: staged deploy approvals, rollback governance, incident hooks.
5. Agent plane: autonomy bounds, evidence requirements, destructive-action constraints.
6. Audit plane: drift detection, conformance scores, exception expiry and ownership.

## Canonical Release Funnel

`SP -> POC -> IP -> A -> FP -> B -> EP -> CN -> RC -> GA -> PROD -> LTS -> SS -> DEP -> AR -> EOL`

See [release-funnel-governance.md](./release-funnel-governance.md) for stage definitions,
graduation tiers, and dropdown/demotion policy.

## Governance Artifacts (Machine-Readable)

The following templates are the policy source for automation:

1. `templates/governance/release-transition-matrix.yaml`
2. `templates/governance/pr-policy-gates.yaml`
3. `templates/governance/ci-required-jobs.yaml`
4. `templates/governance/merge-eligibility-token-schema.yaml`

## Merge Credential Model

Promotion is credentialed by CI-issued attestations:

1. Branch classified into stage.
2. Required checks pass for that stage/tier.
3. CI issues signed merge-eligibility token (short TTL).
4. Merge gate verifies signature, stage transition legality, and destination scope.
5. Only then is merge into target stage lane allowed.

Destination scopes:

- `module`: module-specific stage branch, e.g. `module/auth/beta`.
- `trunk`: central stage branch, e.g. `trunk/beta`, `trunk/prod`.

## PR/Review Governance

Minimum global controls:

1. One concern per PR; use stacked PRs for multi-part delivery.
2. Required approvals and resolved threads before merge.
3. Required checks are blocking; failing checks forbid merge.
4. Merge commits blocked unless explicit policy exception.
5. PR must include risk class, rollback plan, and traceability bundle links.

## GitOps Governance

1. Git is source of truth for policy/workflow/environment config.
2. Environment mutation flows through PR + reconciler, not ad-hoc control plane edits.
3. Break-glass path requires incident link, bounded exception window, and reconciliation PR.
4. Desired-vs-actual drift audits run on schedule with SLA-backed remediation.

## CI/CD/DevOps Governance

1. Reusable workflows are centrally managed and ref-pinned.
2. Stage-appropriate checks are mandatory (see CI matrix template).
3. Promotion from `RC+` requires stronger gates and environment approvals.
4. `PROD+` requires provenance and rollback evidence.
5. Deployment without matching stage token is denied.

## Agent Operating Contract

1. Agents must follow repository AGENTS contract and governance policy templates.
2. Agents must provide auditable command/check evidence for governance-impacting changes.
3. Agents must avoid destructive actions without explicit approval.
4. Policy bypasses require explicit exception records with expiry and owner.

## Rollout Plan (Polyrepo)

1. Baseline:
   - Install template policies in governance repos.
   - Normalize required checks and branch protections.
2. Enforcement:
   - Enable token-verification gate for stage promotions.
   - Enforce transition matrix in policy-gate workflows.
3. Hardening:
   - Strict mode for `RC+`.
   - Introduce provenance and release attestation requirements.
4. Continuous Ops:
   - Drift scan + conformance scorecards.
   - Exception ledger and expiry enforcement.

## SLO Targets

1. Policy conformance: >= 98% repos in compliance.
2. Workflow drift MTTR: < 3 business days.
3. PR governance compliance: >= 95%.
4. Production release attestation coverage: 100%.
5. CI reliability on default branches: >= 99%.
