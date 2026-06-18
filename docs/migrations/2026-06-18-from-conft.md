# Migration: `Conft` (TS edge) — relationship to `Configra`

**Date:** 2026-06-18
**ADR:** [ADR-031 — `KooshaPari/Configra` is the canonical config substrate](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md)
**Companion ADR:** [ADR-022 — config consolidation (Rust/TS edge split)](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-022-config-consolidation.md)

---

> **TL;DR:** `Conft` is **kept**, not absorbed. It is the TypeScript edge layer
> of the config substrate, paired with `Configra` (Rust). This document
> describes how the two cooperate; it is **not** a deprecation notice for
> `Conft`.

## What this doc is (and is not)

`Configra` is the canonical Rust config crate (`crates/settly/` + `crates/pheno-config/`).
`Conft` is the canonical TypeScript edge layer (Zod-validated bindings).
ADR-022 (2026-06-15) established the **Rust/TS split**. ADR-031 (2026-06-17)
supersedes ADR-022 only for **naming** (rename `phenotype-config` → `Configra`).
The split itself remains in force:

| Language | Canonical repo                        | Role                              |
|----------|---------------------------------------|-----------------------------------|
| Rust     | [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) | Core config substrate             |
| TypeScript | [`KooshaPari/Conft`](https://github.com/KooshaPari/Conft)    | TS edge layer (Zod-validated bindings) |

So: `Conft` does **not** migrate *into* `Configra`. It sits **alongside**
`Configra` as the polyglot counterpart.

## What `Conft` provides (not duplicated by `Configra`)

- **TypeScript-first DX** — `import { config } from '@phenotype/config-ts'`
- **Zod-based runtime validation** — schema-as-code at the TS edge
- **Edge / SSR / VitePress integrations** — `Taskfile.yml`, `docs:dev`, `docs:build`
- **Library-consumer-journey E2E suite** — `tests/e2e/` (vitest, run in CI per `e2e-2026-06-16`)
- **SLSA Build attestation + CycloneDX SBOM** — Conft owns these for the TS edge

## What `Configra` provides (not duplicated by `Conft`)

- **Rust-native config substrate** — `crates/settly/`, `crates/pheno-config/`
- **Hexagonal architecture** — domain/application/adapters/infrastructure split
- **Validator derive** — `#[derive(validator::Validate)]` on `Settings`
- **Postgres + Redis adapters** — `tokio-postgres` + `sqlx` + `redis`
- **Workspace-wide enforcement** — all `pheno-*-rust` consumers use `Configra`

## How they cooperate

A polyglot service (e.g. a TS frontend talking to a Rust backend) reads
config from **both** layers:

```ts
// Frontend (TypeScript) — uses Conft
import { config } from "@phenotype/config-ts";
const apiUrl = config.api.baseUrl; // validated against Zod schema at load
```

```rust
// Backend (Rust) — uses Configra
use configra::settly::{Settings, SettingsRepository, PostgresAdapter};
let settings = Settings::load("feature-flags")?;
```

Both layers share the **schema intent** but use **different validation
primitives** (Rust `validator` derive vs. TS Zod). Drift between them is
caught by the `phenotype-journeys` repo's library-consumer-journey suite.

## TS migration impact: zero

If you previously consumed a config crate from a TypeScript project, you
were using `Conft` (`@phenotype/config-ts`) — and you **continue** to use it.
ADR-031 changes the **name of the Rust canonical repo**; it does not touch
the TS layer.

```diff
  // package.json — unchanged
  {
    "dependencies": {
      "@phenotype/config-ts": "^0.1.0"
    }
  }
```

## What is *not* in `Conft`

`Conft` does **not** contain:

- The `settly` Rust crate (that lives in `Configra/crates/settly/`)
- The `pheno-config` Rust crate (that lives in `Configra/crates/pheno-config/`)
- Any Python or Go bindings (those would be separate edge-layer repos if
  created in the future; per ADR-023, no random `phenoShared` placement)

## Cross-references

- **ADR-031** — `docs/adr/2026-06-17/ADR-031-configra-absorb.md`
- **ADR-022** — config consolidation (Rust/TS split; **still in force** for the split)
- **ADR-023** — substrate placement (Rule 3)
- Configra PR [#44](https://github.com/KooshaPari/Configra/pull/44) — Rust absorb (does NOT touch Conft)
- Configra PR [#45](https://github.com/KooshaPari/Configra/pull/45) — pheno-config absorb (does NOT touch Conft)
- `phenotype-journeys` — library-consumer-journey E2E (drift detection)
- `plans/2026-06-18-v8-dag-stable.md` § 3.2 (T10.5 — "Conft KEEP")

L5-104.7 — T10.6 / 2026-06-18-from-conft
