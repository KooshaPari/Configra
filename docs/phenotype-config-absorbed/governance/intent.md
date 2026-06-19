# Intent — phenotype-config

## Problem statement

Settly lived in HexaKit as a transitional `crates/settly` while the fleet rationalized. HexaKit is now genesis-only; config domain code belongs under the `config` role per DOMAIN_ROLES and RFC 002.

## Success criteria

- [x] New repo with `settly` crate migrated from HexaKit
- [ ] Pyron repointed to `phenotype-config` / `settly` path dep
- [ ] Settly archive deleted at 100% boundary coverage
- [ ] Genesis doc set complete

## Originating prompts

| Date | Tool | Session | Summary |
|------|------|---------|---------|
| 2026-06-17 | cursor | b561a593 | [fleet audit + RFC 002](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/intent/prompts/cursor/20260617-b561a593-1729-44da-b90d-0cfbdf9d72ef-t1.md) |

## Synthesized goals

1. Own `config` role Rust core (Tier 1)
2. Document Conft + Py edges without merging into this repo
