# Observability Remediation (Audit Area G)

**Status:** remediated (additive) · **Owner:** Configra maintainers · **Crate:** `configra-ops`

## Gap (pre-remediation)

| Requirement | Prior state | Risk |
|-------------|-------------|------|
| Structured logging with levels | `tracing` used ad hoc in `settly` / `pheno-config` feature gate only | No consistent bootstrap; hard to aggregate |
| Correlation IDs | None | Cannot trace config load across services |
| Metrics hooks | None | No counters for load errors / health |
| Health signal | Documented `settly health` CLI not implemented | Probes unavailable |

## Remediation (additive)

### 1. `configra-ops` library

New workspace crate at `crates/configra-ops` providing:

| Module | Capability |
|--------|------------|
| `logging` | `init_logging` + `LoggingConfig` — `tracing-subscriber` with `CONFIGRA_LOG_LEVEL` / `CONFIGRA_LOG_FORMAT` (`pretty` \| `json`) |
| `correlation` | `CorrelationId` UUID propagation via spans + `CONFIGRA_CORRELATION_ID_HEADER` |
| `metrics` | `MetricsHook` trait + `MetricsRegistry` in-process backend; standard names under `metrics::names` |
| `health` | `liveness` / `readiness` JSON reports + pluggable `HealthCheck` trait |
| `shutdown` | `GracefulShutdown` with SIGINT/SIGTERM + `CONFIGRA_SHUTDOWN_TIMEOUT_SECS` drain |

### 2. CLI health probe

```bash
configra-ops health          # liveness
configra-ops health --ready  # readiness (+ workspace check)
configra-ops health --json   # machine-readable
```

Exit code `0` = healthy, `1` = unhealthy (container/K8s compatible).

### 3. Settly re-export

`settly::infrastructure::observability` re-exports `configra-ops` primitives — existing call sites unchanged; new integrators opt in.

### 4. Environment contract

Documented in repo-root `.env.example` (no secrets):

- `CONFIGRA_LOG_LEVEL`, `CONFIGRA_LOG_FORMAT`
- `CONFIGRA_CORRELATION_ID`, `CONFIGRA_CORRELATION_ID_HEADER`
- `CONFIGRA_METRICS_ENABLED`
- `CONFIGRA_HEALTHCHECK_TIMEOUT_SECS`

## Verification

```bash
cargo test -p configra-ops
configra-ops health --json
CONFIGRA_LOG_FORMAT=json configra-ops health --ready
```

## Future (non-blocking)

- OTLP exporter feature flag on `configra-ops`
- Prometheus scrape endpoint (HTTP) behind optional `axum` feature
- Span emission inside `pheno-config` `load_*` paths (ADR-036 follow-up)

## Cross-references

- [deploy.md](../deploy.md)
- [OPS.md](OPS.md)
- ADR-036 (`pheno-config` tracing feature)
