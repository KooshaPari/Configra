# ABSORBED-FROM: `pheno-config` (local-only)

**Original location:** `repos/pheno-config/` (local-only directory in the monorepo)
**Status:** N/A (not a GitHub repo; `gh api repos/KooshaPari/pheno-config` returns 404)
**Absorbed into Configra:** [Configra PR #45](https://github.com/KooshaPari/Configra/pull/45) (byte-identical 645-LoC copy)

---

## What was absorbed

| Concern | Original location | New canonical Configra location |
|---|---|---|
| Type-gated `Config` struct (URL, PORT, LOG_LEVEL, DB_PATH, FEATURE_FLAGS) + `load_from_env` / `load_from_file` / `load_from_toml_file` / `combine` / `ConfigBuilder` / `Config::merge` | `repos/pheno-config/src/lib.rs` (645 LoC) | [`Configra:crates/pheno-config/src/lib.rs`](../../crates/pheno-config/src/lib.rs) (byte-identical) |
| Tests (`load_from_env_with_prefix_filters_unrelated_vars`, `load_from_env_defaults_port_8080`, `load_from_file_valid_json`, etc.) | `repos/pheno-config/tests/config_test.rs` | [`Configra:crates/pheno-config/tests/`](../../crates/pheno-config/tests/) |
| Examples (cascade.rs, quickstart.rs, validation.rs) | `repos/pheno-config/examples/` | [`Configra:crates/pheno-config/examples/`](../../crates/pheno-config/examples/) (added via [Configra PR #48](https://github.com/KooshaPari/Configra/pull/48)) |
| Tracing test (L5-112) | (none) | [`Configra:crates/pheno-config/tests/tracing_test.rs`](../../crates/pheno-config/tests/tracing_test.rs) (added via [Configra PR #48](https://github.com/KooshaPari/Configra/pull/48)) |
| Documentation (AGENTS.md, README.md, CHANGELOG.md, WORKLOG.md, llms.txt) | `repos/pheno-config/{AGENTS.md,README.md,CHANGELOG.md,WORKLOG.md,llms.txt}` | preserved in `repos/pheno-config/` (local-only); canonical versions in `Configra:crates/pheno-config/` |

## Migration status

- [x] `repos/pheno-config/src/lib.rs` (645 LoC) byte-identical copy absorbed into `Configra:crates/pheno-config/src/lib.rs` via [Configra PR #45](https://github.com/KooshaPari/Configra/pull/45) (MERGED 2026-06-18 09:04)
- [x] Examples + tracing test added via [Configra PR #48](https://github.com/KooshaPari/Configra/pull/48) (MERGED 2026-06-19 01:10)
- [x] Migration log at `Configra:docs/migrations/2026-06-18-from-pheno-config.md` (MERGED)
- [ ] Local `repos/pheno-config/` directory deletion (future cleanup, NOT in L5-110 scope)

## Notes

- `repos/pheno-config/` is a local-only workspace member with multiple stale remotes (pointing to Dmouse92/AgilePlus, KooshaPari/FocalPoint, KooshaPari/phenoShared, etc.). It is NOT a separate GitHub repo.
- The content of `repos/pheno-config/src/lib.rs` and `Configra:crates/pheno-config/src/lib.rs` are **byte-identical** (645 LoC, same line count, same content).
- Future cleanup: the local `repos/pheno-config/` directory should be physically deleted after a fleet-wide announcement. NOT in L5-110 scope.

## L5-110 implementation log

See [`findings/2026-06-18-L5-110-adr-035-impl.md`](https://github.com/KooshaPari/repos/blob/main/findings/2026-06-18-L5-110-adr-035-impl.md) for the full implementation log.
