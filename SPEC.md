# Configra — Specification

**Version:** 0.1.0
**Status:** ACTIVE
**Last Updated:** 2026-06-19

## Purpose

Configra is the canonical Rust configuration substrate for the Phenotype
organization. It provides local-first configuration management, feature flags,
secrets, and version tracking with auditable change history and CLI-first
workflows.

## Scope

- Layered configuration (env → TOML → CLI args → defaults)
- Feature flag lifecycle management (create, enable, disable, rollback)
- Secret value storage with AES-256-GCM encryption at rest
- Version inspection and rollout state tracking
- Comprehensive audit trail with point-in-time restore

## Non-Goals

- Distributed coordination (use a federated service for that)
- Real-time config pushing (poll-based refresh only)
- Configuration for non-Phenotype projects

## Architecture

See `docs/ARCHITECTURE.md` for detailed system design.

Crate layout:
- `crates/settly/` — legacy config crate (deprecated, absorbed)
- `crates/pheno-config/` — canonical Rust config types + traits
- `crates/config-schema/` — schema validation layer

## Related ADRs

- ADR-022 — config consolidation (Rust/TS edge split)
- ADR-031 — Configra is the canonical config substrate
- ADR-035 — Configra migration gates

## Governance

This repository is planned, maintained, and managed by AI Agents under the
Phenotype AI-DD metaproject. See AGENTS.md for agent instructions.
