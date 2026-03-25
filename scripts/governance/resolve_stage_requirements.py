#!/usr/bin/env python3
"""Resolve stage, strict-mode, and required CI jobs from governance config."""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import sys
from pathlib import Path
from typing import Any

import yaml


def _default_config_path() -> Path:
    return Path(__file__).resolve().parents[2] / "templates" / "governance" / "ci-required-jobs.yaml"


def _truthy(value: str) -> bool:
    return value.strip().lower() in {"1", "true", "yes", "on"}


def _load_yaml(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise FileNotFoundError(f"Config file not found: {path}")
    with path.open("r", encoding="utf-8") as handle:
        data = yaml.safe_load(handle) or {}
    if not isinstance(data, dict):
        raise ValueError("Top-level YAML value must be a mapping")
    return data


def _detect_stage(branch: str, config: dict[str, Any]) -> str:
    default_stage = str(config.get("default_stage", "GA")).upper()
    rules = config.get("branch_stage_rules", []) or []
    for rule in rules:
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


def _resolve_required_jobs(config: dict[str, Any], stage: str, strict: bool) -> list[str]:
    stages = config.get("stages", {}) or {}
    stage_cfg = stages.get(stage, {}) or {}
    base_jobs = stage_cfg.get("required_jobs", stage_cfg.get("gates", [])) or []
    if not isinstance(base_jobs, list):
        raise ValueError(f"stages.{stage}.required_jobs must be a list")

    strict_jobs: list[str] = []
    if strict:
        strict_cfg = config.get("strict_mode", {}) or {}
        strict_jobs.extend(strict_cfg.get("additional_required_jobs", []) or [])
        overrides = strict_cfg.get("stage_overrides", {}) or {}
        strict_jobs.extend((overrides.get(stage, {}) or {}).get("additional_required_jobs", []) or [])

    return _unique([*map(str, base_jobs), *map(str, strict_jobs)])


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Resolve stage requirements")
    parser.add_argument("--stage", help="Explicit stage code")
    parser.add_argument("--branch", help="Branch name for stage detection")
    parser.add_argument("--strict", action=argparse.BooleanOptionalAction, default=None)
    parser.add_argument("--strict-flag", default="false", help="Strict flag value (true/false)")
    parser.add_argument("--config", type=Path, default=_default_config_path())
    return parser.parse_args()


def main() -> int:
    args = _parse_args()
    if not args.stage and not args.branch:
        print("error: either --stage or --branch is required", file=sys.stderr)
        return 2

    try:
        config = _load_yaml(args.config)
        stage = (args.stage or _detect_stage(args.branch or "", config)).upper()
        available_stages = set((config.get("stages", {}) or {}).keys())
        if stage not in available_stages:
            raise KeyError(f"Unknown stage '{stage}'")

        forced_stages = {str(s).upper() for s in (config.get("force_strict_stages", []) or [])}
        strict_from_flag = _truthy(args.strict_flag)
        strict = bool(args.strict) if args.strict is not None else (strict_from_flag or stage in forced_stages)

        required_jobs = _resolve_required_jobs(config=config, stage=stage, strict=strict)
    except (FileNotFoundError, ValueError, KeyError, yaml.YAMLError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2

    result = {
        "stage": stage,
        "strict": strict,
        "required_jobs": required_jobs,
        "required_jobs_csv": ",".join(required_jobs),
    }
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
