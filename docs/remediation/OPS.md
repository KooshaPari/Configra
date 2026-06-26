# Operations Remediation (Audit Area K)

**Status:** remediated (additive) · **Owner:** Configra maintainers

## Gap (pre-remediation)

| Requirement | Prior state | Risk |
|-------------|-------------|------|
| Container packaging | No root `Dockerfile`; security workflows gated on missing file | No reproducible deploy artifact |
| Health checks in CI/CD | None at repo level | Silent runtime regressions |
| Deploy runbook | README aspirational (`pheno-cli` paths) | On-call friction |
| Graceful shutdown | `ConfigKitError::Shutdown` only | Abrupt termination under orchestrators |
| Env template | `settly/.env.example` only, minimal | Missing ops vars at workspace root |

## Remediation (additive)

### 1. Multi-stage Dockerfile

Root `Dockerfile`:

- **builder:** `rust:1.83-bookworm` → `cargo build --release -p configra-ops`
- **runtime:** `debian:bookworm-slim` + `ca-certificates`
- **HEALTHCHECK:** `configra-ops health` (30s interval, 5s timeout)

No existing files removed or replaced.

### 2. Deploy documentation

`docs/deploy.md` covers:

- Local `cargo install`
- Docker build/run
- Kubernetes liveness/readiness probe snippets
- `GracefulShutdown` integration pattern
- Link to settly observability re-exports

### 3. Environment template

Repo-root `.env.example` — observability + shutdown knobs, pheno-config consumer examples, **no secrets**.

### 4. Graceful shutdown substrate

`configra_ops::GracefulShutdown`:

1. Await SIGINT / SIGTERM
2. Drain in-flight work (timeout from `CONFIGRA_SHUTDOWN_TIMEOUT_SECS`)
3. Run async cleanup hook
4. Emit `configra_shutdown_total` metric when enabled

### 5. CI alignment

Existing workflows in `crates/settly/.github/workflows/security*.yml` already conditionally scan when `Dockerfile` exists — root Dockerfile now satisfies that gate without modifying workflows.

## Operational checklist

| Step | Command |
|------|---------|
| Build image | `docker build -t configra-ops:local .` |
| Smoke health | `docker run --rm configra-ops:local health` |
| Readiness | `docker run --rm configra-ops:local health --ready` |
| Logs JSON | `CONFIGRA_LOG_FORMAT=json configra-ops health` |
| Unit tests | `cargo test -p configra-ops` |

## Rollback

Remediation is purely additive. Remove `crates/configra-ops`, root `Dockerfile`, `.env.example`, and `docs/deploy.md` / `docs/remediation/*` to revert — no existing crate APIs changed.

## Cross-references

- [OBSERVABILITY.md](OBSERVABILITY.md)
- [deploy.md](../deploy.md)
- `crates/settly/docs/journeys/quick-start.md` (`settly health` → use `configra-ops health` until dedicated settly binary ships)
