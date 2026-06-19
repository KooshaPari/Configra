# Charter — phenotype-config

> **Boundary class:** sdk-domain  
> **Role:** config  
> **Lifecycle:** active  
> **Genesis template:** HexaKit `templates/genesis/` v1.0.0  
> **RFC:** [phenotype-registry docs/rfc/002-settly-config-role.md](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/rfc/002-settly-config-role.md)

## Mission

Canonical `config` domain role owner — layered configuration, validation, and env for Phenotype-org. Rust core (`settly`); TS and Python edges documented, not merged here.

## Scope

### In scope

- `crates/settly/` — Rust config core (migrated from HexaKit transitional `crates/settly`)
- Genesis governance: intent, charter, review, SOTA, OKF
- RFC linkage to Conft (TS) and `phenotype-python-sdk` `phenotype-config` (Py)

### Out of scope

| Boundary | Owner |
|----------|-------|
| TS npm config surface | **Conft** |
| Python config package | **phenotype-python-sdk** `packages/phenotype-config` |
| Genesis templates | **HexaKit** |
| Application config consumers | product repos |

## Governance artifacts

| Artifact | Path |
|----------|------|
| Intent | [intent.md](intent.md) |
| Review | [review.md](review.md) |
| SOTA | [SOTA.md](SOTA.md) |
| OKF | [okf/manifest.okf.yaml](okf/manifest.okf.yaml) |

## Changelog

| Date | Change |
|------|--------|
| 2026-06-17 | Initial repo bootstrap per RFC 002 |
