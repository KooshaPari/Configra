# Deploying Configra Ops

Additive deployment guide for the `configra-ops` CLI and Docker image. Library crates (`settly`, `pheno-config`, etc.) embed observability via `configra-ops` re-exports.

## Prerequisites

- Rust 1.83+ (see `rust-toolchain.toml`)
- Docker 24+ (optional, for container probes)

## Local install

```bash
cargo install --path crates/configra-ops
configra-ops version
configra-ops health
configra-ops health --ready
```

Copy `.env.example` to `.env` and adjust levels/format for your environment.

## Docker

```bash
docker build -t configra-ops:local .
docker run --rm configra-ops:local health
docker run --rm -e CONFIGRA_LOG_FORMAT=json configra-ops:local health --ready
```

The image includes a `HEALTHCHECK` that runs `configra-ops health` every 30s.

## Kubernetes probes

```yaml
livenessProbe:
  exec:
    command: ["configra-ops", "health"]
  initialDelaySeconds: 5
  periodSeconds: 30
readinessProbe:
  exec:
    command: ["configra-ops", "health", "--ready"]
  initialDelaySeconds: 10
  periodSeconds: 15
```

## Graceful shutdown

Services using `configra_ops::GracefulShutdown` drain in-flight work for `CONFIGRA_SHUTDOWN_TIMEOUT_SECS` (default 30) after SIGINT/SIGTERM before running cleanup hooks.

```rust
use configra_ops::{GracefulShutdown, init_logging, LoggingConfig};

#[tokio::main]
async fn main() {
    let _ = init_logging(&LoggingConfig::default());
    GracefulShutdown::new().run(
        || async { /* drain HTTP / config reload workers */ },
        || async { /* close DB pools, flush metrics */ },
    ).await;
}
```

## Integrating in settly

```rust
use settly::infrastructure::observability::{init_logging, CorrelationId, LoggingConfig};

let _ = init_logging(&LoggingConfig::default());
let cid = CorrelationId::from_env_or_new();
let _span = cid.span("config_load").entered();
```

## Related docs

- [remediation/OBSERVABILITY.md](remediation/OBSERVABILITY.md) — audit area G
- [remediation/OPS.md](remediation/OPS.md) — audit area K
