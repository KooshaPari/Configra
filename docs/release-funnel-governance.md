# Release Funnel Governance Specification

Status: proposed canonical policy for Phenotype repos

## Purpose

Define one funnel model for release lifecycle, graduation rules, maintenance rules,
demotion/dropdown rules, and minimum governance+traceability controls.

This spec aligns the existing stage codes:

`SP -> POC -> IP -> A -> FP -> B -> EP -> CN -> RC -> GA -> PROD -> LTS -> SS -> DEP -> AR -> EOL`

## Stage Definitions

| Stage | Name | Purpose | Expected branch family |
|---|---|---|---|
| `SP` | Specification/Planning | Problem framing, scope, design intent | `spike/*`, `spec/*` |
| `POC` | Proof of Concept | Technical feasibility proof | `poc/*` |
| `IP` | Initial Prototype | First integrated build path | `preview/*` |
| `A` | Alpha | Internal feature-complete iteration | `alpha/*` |
| `FP` | Feature Preview | Limited external preview readiness | `feature/*`, `preview/*` |
| `B` | Beta | Broad pre-release user validation | `beta/*`, `release/beta-*` |
| `EP` | Early Production | Production with guardrails and limits | `release/ep-*` |
| `CN` | Canary | Low-blast-radius production lane | `release/canary-*` |
| `RC` | Release Candidate | Freeze except release blockers | `release/rc-*` |
| `GA` | General Availability | Full public release baseline | `release/ga-*`, `main` |
| `PROD` | Production Channel | Operational production promotion lane | `prod/*`, `release/prod-*` |
| `LTS` | Long-Term Support | Stability maintenance and critical fixes | `release/lts-*`, `maintenance/*` |
| `SS` | Sunset/Stability-only | No new features, keep serviceable | `sunset/*` |
| `DEP` | Deprecated | Active migration off, clear removal timeline | `deprecate/*` |
| `AR` | Archived | Frozen except legal/security backport decisions | `archive/*` |
| `EOL` | End of Life | No support, no new releases | `eol/*` |

## Gate Tiers

Use cumulative tiers; each higher tier includes lower-tier requirements.

| Tier | Name | Required checks |
|---|---|---|
| `T0` | Hygiene | format, lint, basic policy-gate |
| `T1` | Core Quality | unit tests, secrets scan, dependency scan |
| `T2` | Integration | integration tests, SAST, coverage floor |
| `T3` | Release Readiness | e2e, perf baseline, SLA validation, SBOM, artifact checksum |
| `T4` | Production Assurance | manual security review, runbook validation, rollback drill evidence |

## Graduation Rules

Promotion is forward-only and must satisfy all checks for the stage's minimum tier,
plus semantic traceability controls.

| Stage | Minimum tier to enter | Graduation criteria to next stage |
|---|---|---|
| `SP` | `T0` | ADR+PRD+Plan approved; risk/assumption log exists |
| `POC` | `T0` | Feasibility evidence + explicit non-goals documented |
| `IP` | `T1` | First integrated path passes `T1`; owners assigned |
| `A` | `T2` | Feature contract stabilized; integration failures triaged |
| `FP` | `T2` | User-facing docs draft + eval scaffold present |
| `B` | `T2` | Coverage floor met; defect trend acceptable |
| `EP` | `T3` | Operational readiness partial (alerts, dashboards, rollback notes) |
| `CN` | `T3` | Canary metrics and error budgets within thresholds |
| `RC` | `T3` | Release freeze enforced; only blocker-class changes |
| `GA` | `T3` | Full release checklist complete and signed |
| `PROD` | `T4` | Promotion approval + rollback drill evidence + oncall ack |
| `LTS` | `T3` | LTS policy approved (support window + backport rules) |
| `SS` | `T2` | Sunset announcement + migration guide published |
| `DEP` | `T2` | Consumer migration > target threshold and no net-new feature work |
| `AR` | `T1` | Archive manifest complete and legal/security hold checks done |
| `EOL` | `T0` | EOL notice finalized and support path closed |

## Traceability Requirements

Each promotion PR must include traceability links that are machine-checkable.

Required links:

1. `ADR` (decision and constraints)
2. `PRD` (intent and acceptance)
3. `PLAN` (execution + dependency order)
4. `TEST` (verification mapping)
5. `EVAL` (evaluation code/artifacts and quality signals)
6. `DOC` (operator/user docs)
7. `GOV` (policy/ruleset references)

Minimum semantic traceability:

- Every acceptance criterion maps to at least one test or eval artifact.
- Every major codepath change maps to doc updates or explicit `N/A` rationale.
- Every stage transition is recorded with actor, timestamp, and evidence bundle.

## Maintenance and Dropdown Rules

Demotion is allowed when risk or quality regresses. Demotion must be explicit and auditable.

Mandatory demotion triggers:

- Sev1 or repeated Sev2 incidents tied to the release delta.
- Failed rollback or unverified recovery path.
- Policy-gate bypass without approved exception.
- Missing traceability bundle for promoted stage.

Dropdown policy:

1. `PROD -> RC/CN` for unstable fresh releases.
2. `GA -> RC` when release-critical regressions appear pre-prod rollout.
3. `LTS -> SS` when support obligations can no longer be met.

Demotion PR requirements:

- Incident or risk reference.
- Scope of rollback/demotion.
- Re-promotion exit criteria.
- Owner and deadline.

## Governance Controls

For `RC` and later stages:

- Stage-gates must be required checks in branch protection/rulesets.
- Merge-commit blocking policy enabled.
- Linear history enforced.
- Release branch protections and approvals enforced.

For `PROD` and later:

- Environment protection with required reviewers.
- Signed artifact provenance and checksum verification.
- Runbook link and rollback command verification.

## Notes for Existing Templates

Current workflow templates already encode early-to-GA gates.
This spec extends policy intent beyond GA to `PROD/LTS/SS/DEP/AR/EOL`,
and standardizes branch families and traceability artifacts for those stages.
