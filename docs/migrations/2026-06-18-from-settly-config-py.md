# Migration: `settly-config-py` (deprecated federated Python service) → `Configra`

**Date:** 2026-06-18
**ADRs:**
- [ADR-031 — `KooshaPari/Configra` is the canonical config substrate](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md)
- [ADR-017 — `settly-*` full deprecation (V6 Track 5 closure)](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-017-settly-archive.md)

---

> **Status note:** `settly-config-py` was a **conceptual** per-language split of
> the federated `settly-config` service (per ADR-031's table: "`KooshaPari/settly-config*`
> DEPRECATED, per-lang, ADR-017"). No separate `KooshaPari/settly-config-py`
> repository exists on GitHub as of 2026-06-18. This document is the canonical
> migration record for any consumer that *believed* they were depending on
> `settly-config-py` (typically via a vendored copy or a path dependency on
> a local `settly-py/` directory inside `KooshaPari/Settly` or one of the
> federated `settly-*` repos).

## What this doc covers

If you were using Python code that:

- imported `from settly_py import ...` (or any module path beginning with `settly_py`,
  `settly_py_config`, `settly_config_py`, or similar), or
- depended on a `settly-config-py` git URL in `pyproject.toml` /
  `requirements.txt`, or
- ran a `settly-py` service / CLI as a federated config backend,

then this document tells you where that code now lives and how to migrate.

## What moved

| Was (per ADR-031 conceptual split)                          | Is now                                                                                |
|------------------------------------------------------------|---------------------------------------------------------------------------------------|
| `settly-config-py` (Python port of the `settly` Rust crate) | **No replacement crate.** Python consumers should call `Configra`'s Rust crate via `PyO3`/`maturin`, OR adopt [`pydantic-settings`](https://docs.pydantic.dev/latest/concepts/pydantic_settings/) directly. |
| Per-language federated service concept                     | **Deprecated** per ADR-017 (V6 Track 5 closure). Per ADR-023, no random `phenoShared` placement. |
| In-repo `settly-py/` or `python/` subdir in `Settly`       | **Removed** (or archived) with the `Settly` repo. No code was promoted to a canonical Python crate. |

## Why no Python crate was created

The decision (per ADR-023 substrate placement) is that the canonical config
substrate is **language-specific single-crate**: one Rust crate in `Configra`.
Python consumers are best served by:

1. **PyO3 bindings** — wrap `Configra`'s Rust crate as a Python module via
   `maturin`. This is the path of least surprise: same schema, same validation,
   same source-of-truth. (Future work; not part of this absorb.)
2. **Adopt `pydantic-settings` directly** — idiomatic Python; no federation
   needed. `pydantic-settings` is the de-facto standard for 12-factor config
   in Python. No `pheno-` flavor is required.

There is **no** `pheno-config-py` or `configra-py` crate at this time. ADR-031
explicitly chose to consolidate to a single Rust canonical crate rather than
spin out per-language federated services.

## How to migrate consumers

### Option A: PyO3 bindings (future, when available)

```toml
# pyproject.toml
[project]
dependencies = [
  "configra-py",  # NOT YET PUBLISHED
]
```

```py
# config is then validated against the same Rust schema
from configra_py import Settings
settings = Settings.load("feature-flags")
```

This option is **not yet available**; it is the planned future work. Watch
[`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) issues for
progress.

### Option B: adopt `pydantic-settings` directly (recommended today)

```toml
# pyproject.toml
[project]
dependencies = [
  "pydantic>=2",
  "pydantic-settings>=2",
]
```

```py
# config.py
from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="APP_", toml_file="config.toml")
    api_url: str
    log_level: str = "info"
    feature_flags: list[str] = []

settings = Settings()  # env + TOML overlay (12-factor)
```

This is the recommended migration for any Python consumer that was depending
on a federated `settly-py` service.

### Option C: HTTP/RPC consumer of the old federated service

```diff
- # Old: HTTP call to settly-config-py service
- import httpx
- r = httpx.get("https://settly-py.example.com/api/settings/feature-flags")
- settings = r.json()
+ # New: in-process pydantic-settings
+ from config import Settings
+ settings = Settings()
```

The federated-service pattern is **discouraged** per ADR-023 (substrate
placement: `pheno-*-lib` for libraries, federated services for
**stateful**, long-running backends only — and config is not stateful).

## Timeline

| Date          | Event                                                                 |
|---------------|-----------------------------------------------------------------------|
| 2026-06-15    | ADR-017 accepted — `settly-*` full deprecation (V6 Track 5 closure). |
| 2026-06-17    | ADR-031 accepted — `settly-config-py` named as deprecated in the per-lang table. |
| 2026-06-18    | This migration doc authored (T10.6 of v8 DAG).                       |
| TBD           | Possible future PyO3 bindings (`configra-py`) — tracked separately.  |

## Cross-references

- **ADR-031** — `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- **ADR-017** — `settly-*` full deprecation (V6 Track 5 closure)
- **ADR-023** — substrate placement (Rule 3 — no random `phenoShared`)
- **ADR-018** — PRCP pattern (Polyglot Reuse via Canonical Ports)
- [`pydantic-settings` docs](https://docs.pydantic.dev/latest/concepts/pydantic_settings/) — recommended Python migration target
- `findings/2026-06-17-L5-104-7-configra-absorb-plan.md`
- `plans/2026-06-18-v8-dag-stable.md` § 3.2 (T10)

L5-104.7 — T10.6 / 2026-06-18-from-settly-config-py
