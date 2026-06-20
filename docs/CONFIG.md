# Configra Configuration Reference

This document describes every configuration key that Configra itself accepts.
These are **meta-configuration** keys — they control how Configra's own crates
(`configra-config`, `pheno-config`, `settly`, `config-schema`) behave at
runtime.

All keys are optional; every field has a documented default (listed below).

---

## Loading order

Configra loads its meta-configuration in the following order (last wins):

1. **Compile-time defaults** (hardcoded in `configra-config::ConfigraConfig::default()`)
2. **Config file** (`configra.toml` or `configra.json`) — loaded via `ConfigraConfig::from_file()`
3. **Environment variables** (`CONFIGRA_*`) — loaded via `ConfigraConfig::from_env()`
4. **Builder** — loaded via `ConfigraConfigBuilder::build()`

---

## Configuration groups

### `[service]` — Service defaults

| Key | Type | Default | Env var | Description |
|---|---|---|---|---|
| `service.default_port` | `u16` | `8080` | `CONFIGRA_DEFAULT_PORT` | Default TCP port for services. Previously hardcoded in `pheno-config` src lines 137, 225, 565. |
| `service.default_log_level` | `string` | `"info"` | `CONFIGRA_DEFAULT_LOG_LEVEL` | Default tracing/log level filter. Previously hardcoded in `pheno-config` src lines 139, 237–238, 566. |
| `service.db_path_template` | `string` | `"/var/lib/{name}.db"` | `CONFIGRA_DB_PATH_TEMPLATE` | Template for constructing database paths. `{name}` is replaced at runtime. |

### `[idempotency]` — Idempotency and retry configuration

| Key | Type | Default | Env var | Description |
|---|---|---|---|---|
| `idempotency.default_ttl_secs` | `u64` | `86400` | `CONFIGRA_IDEMPOTENCY_TTL_SECS` | TTL (seconds) for cached idempotency results. 86400 s = 24 h. Previously hardcoded in `settly/src/adapters/idempotency.rs` line 34. |
| `idempotency.default_max_retries` | `u32` | `3` | `CONFIGRA_MAX_RETRIES` | Maximum retry attempts for idempotent submissions. |

### `[watcher]` — File-watcher configuration

| Key | Type | Default | Env var | Description |
|---|---|---|---|---|
| `watcher.poll_interval_ms` | `u64` | `1000` | `CONFIGRA_WATCH_POLL_MS` | Polling interval (ms) for file-change watchers. |
| `watcher.enabled` | `bool` | `true` | `CONFIGRA_WATCH_ENABLED` | Enable file-watching sources by default. |

---

## Environment variables

All environment variables use the `CONFIGRA_` prefix. Example:

```bash
export CONFIGRA_DEFAULT_PORT=9090
export CONFIGRA_DEFAULT_LOG_LEVEL=debug
export CONFIGRA_IDEMPOTENCY_TTL_SECS=3600
```

---

## Example config file (TOML)

```toml
# configra.toml
[service]
default_port = 9090
default_log_level = "debug"
db_path_template = "/data/{name}.db"

[idempotency]
default_ttl_secs = 3600
default_max_retries = 5

[watcher]
poll_interval_ms = 2000
enabled = true
```

---

## Example config file (JSON)

```json
{
  "service": {
    "default_port": 9090,
    "default_log_level": "debug",
    "db_path_template": "/data/{name}.db"
  },
  "idempotency": {
    "default_ttl_secs": 3600,
    "default_max_retries": 5
  },
  "watcher": {
    "poll_interval_ms": 2000,
    "enabled": true
  }
}
```

---

## Origin of defaults

Each default value in `configra-config` references the source file and line
where the value was previously hardcoded:

| Default | Previous location |
|---|---|
| `port = 8080` | `pheno-config/src/lib.rs` lines 137, 225, 565 |
| `log_level = "info"` | `pheno-config/src/lib.rs` lines 139, 237, 566 |
| `idempotency_ttl = 86400` | `settly/src/adapters/idempotency.rs` line 34 |
| `max_retries = 3` | Convention across submission test files |
| `poll_interval = 1000` | New — replaces implicit assumption |
| `watch_enabled = true` | New — replaces implicit assumption |
