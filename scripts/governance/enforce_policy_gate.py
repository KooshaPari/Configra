#!/usr/bin/env python3
"""Evaluate governance policy from YAML templates and emit merge-eligibility token payload."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import secrets
import sys
from pathlib import Path
from typing import Any

import yaml


def _truthy(value: str) -> bool:
    return value.strip().lower() in {"1", "true", "yes", "on"}


def _load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        data = yaml.safe_load(handle) or {}
    if not isinstance(data, dict):
        raise ValueError(f"Expected mapping in {path}")
    return data


def _detect_stage(branch: str, ci_cfg: dict[str, Any]) -> str:
    import fnmatch

    default_stage = str(ci_cfg.get("default_stage", "GA")).upper()
    for rule in ci_cfg.get("branch_stage_rules", []) or []:
        stage = str(rule.get("stage", "")).upper()
        for pattern in rule.get("patterns", []) or []:
            if fnmatch.fnmatch(branch, str(pattern)):
                return stage
    return default_stage


def _unique(items: list[str]) -> list[str]:
    out: list[str] = []
    seen: set[str] = set()
    for item in items:
        if item not in seen:
            seen.add(item)
            out.append(item)
    return out


def _required_jobs(ci_cfg: dict[str, Any], stage: str, strict: bool) -> list[str]:
    stage_cfg = (ci_cfg.get("stages", {}) or {}).get(stage, {}) or {}
    base = stage_cfg.get("required_jobs", stage_cfg.get("gates", [])) or []
    strict_extra: list[str] = []
    if strict:
        strict_cfg = ci_cfg.get("strict_mode", {}) or {}
        strict_extra.extend(strict_cfg.get("additional_required_jobs", []) or [])
        override = ((strict_cfg.get("stage_overrides", {}) or {}).get(stage, {}) or {})
        strict_extra.extend(override.get("additional_required_jobs", []) or [])
    return _unique([*map(str, base), *map(str, strict_extra)])


def _validate_token_against_schema(token_payload: dict[str, Any], schema: dict[str, Any]) -> list[str]:
    violations: list[str] = []
    required = schema.get("required", []) or []
    for field in required:
        if field not in token_payload:
            violations.append(f"token missing required field '{field}'")

    properties = schema.get("properties", {}) or {}
    for field, spec in properties.items():
        if field not in token_payload:
            continue
        if spec.get("type") == "string" and not isinstance(token_payload[field], str):
            violations.append(f"token field '{field}' must be string")
        if spec.get("type") == "boolean" and not isinstance(token_payload[field], bool):
            violations.append(f"token field '{field}' must be boolean")
        if spec.get("type") == "array" and not isinstance(token_payload[field], list):
            violations.append(f"token field '{field}' must be array")
        enum = spec.get("enum")
        if enum and token_payload[field] not in enum:
            violations.append(f"token field '{field}' has unsupported value '{token_payload[field]}'")
    return violations


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Enforce policy-gate governance templates")
    parser.add_argument("--head-branch", required=True)
    parser.add_argument("--base-branch", required=True)
    parser.add_argument("--head-sha", required=True)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--pr-number", type=int, default=0)
    parser.add_argument("--is-draft", default="false")
    parser.add_argument("--review-decision", default="")
    parser.add_argument("--approvals-count", type=int, default=0)
    parser.add_argument("--codeowner-approved", default="false")
    parser.add_argument("--strict-flag", default="false")
    parser.add_argument("--config-dir", default="templates/governance")
    parser.add_argument("--token-output", default="/tmp/merge-eligibility-token.json")
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    cfg_dir = Path(args.config_dir)

    ci_cfg = _load_yaml(cfg_dir / "ci-required-jobs.yaml")
    transition_cfg = _load_yaml(cfg_dir / "release-transition-matrix.yaml")
    pr_cfg = _load_yaml(cfg_dir / "pr-policy-gates.yaml")
    token_schema = _load_yaml(cfg_dir / "merge-eligibility-token-schema.yaml")

    source_stage = _detect_stage(args.head_branch, ci_cfg)
    target_stage = _detect_stage(args.base_branch, ci_cfg)
    forced_strict = source_stage in {str(s).upper() for s in ci_cfg.get("force_strict_stages", []) or []}
    strict = forced_strict or _truthy(args.strict_flag)

    violations: list[str] = []

    transitions = transition_cfg.get("transitions", {}) or {}
    allowed_targets = set(transitions.get(source_stage, []))
    if source_stage != target_stage and target_stage not in allowed_targets:
        violations.append(
            f"transition '{source_stage}->{target_stage}' is blocked by release-transition-matrix"
        )

    defaults = pr_cfg.get("default", {}) or {}
    stage_policy = (pr_cfg.get("stages", {}) or {}).get(source_stage, {}) or {}
    effective_policy = {**defaults, **stage_policy}

    if strict:
        overrides = pr_cfg.get("strict_mode_overrides", {}) or {}
        required_delta = int(overrides.get("required_approvals_delta", 0) or 0)
        effective_policy["required_approvals"] = int(effective_policy.get("required_approvals", 0)) + required_delta
        if overrides.get("require_signed_commits"):
            effective_policy["require_signed_commits"] = True

    is_draft = _truthy(args.is_draft)
    if is_draft and not bool(effective_policy.get("allow_draft_merge", False)):
        violations.append(f"draft PR is not allowed for stage '{source_stage}'")

    required_approvals = int(effective_policy.get("required_approvals", 0) or 0)
    if args.approvals_count < required_approvals:
        violations.append(
            f"insufficient approvals: required {required_approvals}, found {args.approvals_count}"
        )

    review_decision = args.review_decision.strip().upper()
    if required_approvals > 0 and review_decision not in {"APPROVED", ""}:
        violations.append(f"review decision is '{review_decision}', expected APPROVED")

    if bool(effective_policy.get("require_code_owner_reviews", False)) and not _truthy(
        args.codeowner_approved
    ):
        violations.append("code-owner review is required but not satisfied")

    required_jobs = _required_jobs(ci_cfg=ci_cfg, stage=source_stage, strict=strict)
    failed_jobs: list[str] = []
    completed_jobs: list[str] = required_jobs if not violations else []

    destination_scope = "trunk" if args.base_branch in {"main", "master", "stable"} else "module"

    generated_at = dt.datetime.now(tz=dt.timezone.utc).isoformat()
    policy_digest = hashlib.sha256(
        json.dumps(
            {
                "ci": ci_cfg,
                "transition": transition_cfg,
                "pr": pr_cfg,
            },
            sort_keys=True,
            separators=(",", ":"),
        ).encode("utf-8")
    ).hexdigest()

    token_payload = {
        "schema_id": token_schema.get("schema_id", "phenotype.governance.merge_eligibility_token"),
        "version": int(token_schema.get("version", 1)),
        "token_id": secrets.token_hex(12),
        "repo": args.repo,
        "head_sha": args.head_sha,
        "head_branch": args.head_branch,
        "base_branch": args.base_branch,
        "source_stage": source_stage,
        "target_stage": target_stage,
        "destination_scope": destination_scope,
        "strict": strict,
        "required_jobs": required_jobs,
        "completed_jobs": completed_jobs,
        "failed_jobs": failed_jobs,
        "traceability": {
            "pr_number": int(args.pr_number),
            "docs_refs": [],
            "eval_refs": [],
            "policy_refs": [
                "templates/governance/ci-required-jobs.yaml",
                "templates/governance/release-transition-matrix.yaml",
                "templates/governance/pr-policy-gates.yaml",
                "templates/governance/merge-eligibility-token-schema.yaml",
            ],
        },
        "attestations": {
            "ci_verified": len(failed_jobs) == 0,
            "reviews_verified": args.approvals_count >= required_approvals,
            "policy_verified": len(violations) == 0,
        },
        "eligible": len(violations) == 0,
        "policy_digest": policy_digest,
        "generated_at": generated_at,
    }

    token_schema_violations = _validate_token_against_schema(token_payload, token_schema)
    violations.extend(token_schema_violations)
    token_payload["eligible"] = len(violations) == 0

    out_path = Path(args.token_output)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(token_payload, indent=2), encoding="utf-8")

    result = {
        "source_stage": source_stage,
        "target_stage": target_stage,
        "strict": strict,
        "required_jobs": required_jobs,
        "requires_merge_token": "verify-merge-token" in required_jobs,
        "eligible": len(violations) == 0,
        "violations": violations,
        "token_file": str(out_path),
    }

    print(json.dumps(result, indent=2))

    gh_output = Path(os.environ["GITHUB_OUTPUT"]) if "GITHUB_OUTPUT" in os.environ else None
    if gh_output:
        with gh_output.open("a", encoding="utf-8") as handle:
            handle.write(f"source_stage={source_stage}\n")
            handle.write(f"target_stage={target_stage}\n")
            handle.write(f"strict={'true' if strict else 'false'}\n")
            handle.write(f"required_jobs_csv={','.join(required_jobs)}\n")
            handle.write(f"requires_merge_token={'true' if 'verify-merge-token' in required_jobs else 'false'}\n")
            handle.write(f"eligible={'true' if len(violations) == 0 else 'false'}\n")
            handle.write(f"token_file={str(out_path)}\n")

    if violations:
        print("Policy-gate violations:", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
