//! # configra-config
//!
//! Meta-configuration for the Configra configuration substrate.
//!
//! Extracts hardcoded defaults (ports, log levels, timeouts, paths) from
//! across the Configra workspace into a single, documented, env-and-file
//! driven configuration struct.
//!
//! ## Design
//!
//! - [`ConfigraConfig`] is the root struct, composed of sub-structs grouped
//!   by domain (`service`, `idempotency`, `watcher`).
//! - All fields have sensible defaults that mirror the hardcoded values
//!   previously embedded in `pheno-config`, `settly`, and `config-schema`.
//! - Values can be loaded from:
//!   1. **Environment variables** via [`ConfigraConfig::from_env`] (prefix: `CONFIGRA_`).
//!   2. **Config files** (TOML or JSON) via [`ConfigraConfig::from_file`].
//!   3. **The builder** via [`ConfigraConfigBuilder`].
//! - `#[serde(default)]` on every sub-struct ensures loading a partial file
//!   fills in the remaining fields with documented defaults.
//!
//! ## Example
//!
//! ```rust
//! use configra_config::ConfigraConfig;
//!
//! // All defaults — same as the old hardcoded values.
//! let cfg = ConfigraConfig::default();
//! assert_eq!(cfg.service.default_port, 8080);
//! assert_eq!(cfg.idempotency.default_ttl_secs, 86400);
//! ```
//!
//! ## Environment variables
//!
//! | Variable | Maps to | Default |
//! |---|---|---|
//! | `CONFIGRA_DEFAULT_PORT` | `service.default_port` | `8080` |
//! | `CONFIGRA_DEFAULT_LOG_LEVEL` | `service.default_log_level` | `"info"` |
//! | `CONFIGRA_DB_PATH_TEMPLATE` | `service.db_path_template` | `"/var/lib/{name}.db"` |
//! | `CONFIGRA_IDEMPOTENCY_TTL_SECS` | `idempotency.default_ttl_secs` | `86400` |
//! | `CONFIGRA_MAX_RETRIES` | `idempotency.default_max_retries` | `3` |
//! | `CONFIGRA_WATCH_POLL_MS` | `watcher.poll_interval_ms` | `1000` |
//! | `CONFIGRA_WATCH_ENABLED` | `watcher.enabled` | `true` |

use serde::{Deserialize, Serialize};
use std::path::Path;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors produced by config loading.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// An env-var had a bad value (e.g. non-numeric for a `u16` field).
    #[error("failed to parse env var `{var}`: {message}")]
    ParseError {
        /// The env var name.
        var: String,
        /// Human-readable parse failure detail.
        message: String,
    },

    /// A required env var was missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),

    /// An I/O error occurred while reading a config file.
    #[error("config file I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// A config file had invalid syntax (malformed TOML / JSON).
    #[error("config file parse error: {0}")]
    FileParseError(String),
}

/// Alias for `Result<T, ConfigError>`.
pub type Result<T> = std::result::Result<T, ConfigError>;

// ---------------------------------------------------------------------------
// Sub-config structs
// ---------------------------------------------------------------------------

/// Service-level configuration defaults.
///
/// These are the values that `pheno-config` and other service crates use as
/// fallback when no explicit override is provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServiceConfig {
    /// Default TCP listen port for services using Configra.
    ///
    /// Previously hardcoded as `8080` in `pheno-config/src/lib.rs` lines 137,
    /// 225, and 565.
    ///
    /// Env var: `CONFIGRA_DEFAULT_PORT`
    pub default_port: u16,

    /// Default tracing / log filter level.
    ///
    /// Previously hardcoded as `"info"` in `pheno-config/src/lib.rs` lines
    /// 139, 237–238, and 566.
    ///
    /// Env var: `CONFIGRA_DEFAULT_LOG_LEVEL`
    pub default_log_level: String,

    /// Template string for constructing database paths.
    ///
    /// The placeholder `{name}` is replaced with the service or application
    /// name. Previously implicit in test paths across the workspace.
    ///
    /// Env var: `CONFIGRA_DB_PATH_TEMPLATE`
    pub db_path_template: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            default_port: 8080,
            default_log_level: "info".to_owned(),
            db_path_template: "/var/lib/{name}.db".to_owned(),
        }
    }
}

/// Idempotency and retry configuration.
///
/// These values control the behaviour of `settly`'s idempotency store and
/// submission service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IdempotencyConfig {
    /// Default TTL (in seconds) for cached idempotency results.
    ///
    /// Previously hardcoded as `86_400` (24 hours) in
    /// `settly/src/adapters/idempotency.rs` line 34.
    ///
    /// Env var: `CONFIGRA_IDEMPOTENCY_TTL_SECS`
    pub default_ttl_secs: u64,

    /// Default maximum retry attempts for idempotent submissions.
    ///
    /// Passed as `max_retries` to `SubmissionService::new`. Previously
    /// implicit (callers hardcoded 2 or 3).
    ///
    /// Env var: `CONFIGRA_MAX_RETRIES`
    pub default_max_retries: u32,
}

impl Default for IdempotencyConfig {
    fn default() -> Self {
        Self {
            default_ttl_secs: 86_400, // 24 hours
            default_max_retries: 3,
        }
    }
}

/// File-watcher configuration.
///
/// Controls how `settly`'s `WatchableSource` implementations poll for changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WatcherConfig {
    /// Polling interval in milliseconds.
    ///
    /// Env var: `CONFIGRA_WATCH_POLL_MS`
    pub poll_interval_ms: u64,

    /// Whether file-watching sources are enabled by default.
    ///
    /// Env var: `CONFIGRA_WATCH_ENABLED`
    pub enabled: bool,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 1_000,
            enabled: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Root config struct
// ---------------------------------------------------------------------------

/// The canonical meta-configuration for Configra itself.
///
/// Composed of three sub-configs:
/// - [`ServiceConfig`]     — service defaults (port, log level, paths)
/// - [`IdempotencyConfig`] — idempotency cache TTL, retry counts
/// - [`WatcherConfig`]     — file-watcher polling interval
///
/// Load via:
/// - [`ConfigraConfig::default()`] — all documented defaults
/// - [`ConfigraConfig::from_env()`] — environment variables with `CONFIGRA_` prefix
/// - [`ConfigraConfig::from_file()`] — TOML or JSON file
/// - [`ConfigraConfigBuilder`] — programmatic construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigraConfig {
    /// Service-level configuration defaults.
    pub service: ServiceConfig,
    /// Idempotency and retry configuration.
    pub idempotency: IdempotencyConfig,
    /// File-watcher configuration.
    pub watcher: WatcherConfig,
}

impl Default for ConfigraConfig {
    /// Returns the fully documented default configuration.
    ///
    /// These match the previously hardcoded values throughout the workspace.
    fn default() -> Self {
        Self {
            service: ServiceConfig::default(),
            idempotency: IdempotencyConfig::default(),
            watcher: WatcherConfig::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Env-var name helpers
// ---------------------------------------------------------------------------

const ENV_PREFIX: &str = "CONFIGRA_";

fn env_name(suffix: &str) -> String {
    format!("{ENV_PREFIX}{suffix}")
}

fn parse_u64(var: &str, raw: &str) -> Result<u64> {
    raw.parse::<u64>().map_err(|e| ConfigError::ParseError {
        var: var.to_owned(),
        message: e.to_string(),
    })
}

fn parse_u16(var: &str, raw: &str) -> Result<u16> {
    raw.parse::<u16>().map_err(|e| ConfigError::ParseError {
        var: var.to_owned(),
        message: e.to_string(),
    })
}

fn parse_u32(var: &str, raw: &str) -> Result<u32> {
    raw.parse::<u32>().map_err(|e| ConfigError::ParseError {
        var: var.to_owned(),
        message: e.to_string(),
    })
}

fn parse_bool(var: &str, raw: &str) -> Result<bool> {
    match raw.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        other => Err(ConfigError::ParseError {
            var: var.to_owned(),
            message: format!("cannot parse `{other}` as boolean"),
        }),
    }
}

// ---------------------------------------------------------------------------
// Env-var loading
// ---------------------------------------------------------------------------

impl ConfigraConfig {
    /// Load configuration from environment variables with the `CONFIGRA_` prefix.
    ///
    /// Only the variables listed in the top-level module docs are read; any
    /// other `CONFIGRA_*` vars are ignored. Missing optional vars fall back
    /// to the documented defaults.
    pub fn from_env() -> Self {
        let default_port = std::env::var(env_name("DEFAULT_PORT"))
            .ok()
            .and_then(|v| parse_u16("CONFIGRA_DEFAULT_PORT", &v).ok())
            .unwrap_or(8080);

        let default_log_level =
            std::env::var(env_name("DEFAULT_LOG_LEVEL")).unwrap_or_else(|_| "info".to_owned());

        let db_path_template = std::env::var(env_name("DB_PATH_TEMPLATE"))
            .unwrap_or_else(|_| "/var/lib/{name}.db".to_owned());

        let idempotency_ttl_secs = std::env::var(env_name("IDEMPOTENCY_TTL_SECS"))
            .ok()
            .and_then(|v| parse_u64("CONFIGRA_IDEMPOTENCY_TTL_SECS", &v).ok())
            .unwrap_or(86_400);

        let max_retries = std::env::var(env_name("MAX_RETRIES"))
            .ok()
            .and_then(|v| parse_u32("CONFIGRA_MAX_RETRIES", &v).ok())
            .unwrap_or(3);

        let poll_interval_ms = std::env::var(env_name("WATCH_POLL_MS"))
            .ok()
            .and_then(|v| parse_u64("CONFIGRA_WATCH_POLL_MS", &v).ok())
            .unwrap_or(1_000);

        let watch_enabled = std::env::var(env_name("WATCH_ENABLED"))
            .ok()
            .and_then(|v| parse_bool("CONFIGRA_WATCH_ENABLED", &v).ok())
            .unwrap_or(true);

        Self {
            service: ServiceConfig {
                default_port,
                default_log_level,
                db_path_template,
            },
            idempotency: IdempotencyConfig {
                default_ttl_secs: idempotency_ttl_secs,
                default_max_retries: max_retries,
            },
            watcher: WatcherConfig {
                poll_interval_ms,
                enabled: watch_enabled,
            },
        }
    }

    /// Load configuration from a TOML or JSON file.
    ///
    /// The file extension (`.toml` or `.json`) determines the parser. Missing
    /// keys in the file fall back to `#[serde(default)]` so a partial file is
    /// valid.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "toml" => toml::from_str(&content)
                .map_err(|e| ConfigError::FileParseError(e.to_string())),
            "json" => serde_json::from_str(&content)
                .map_err(|e| ConfigError::FileParseError(e.to_string())),
            other => Err(ConfigError::FileParseError(format!(
                "unsupported config file extension: `.{other}` (expected `.toml` or `.json`)"
            ))),
        }
    }

    /// Shortcut: try a file, fall back to env, then to defaults.
    ///
    /// This is the recommended "one-call" constructor:
    /// 1. If `path` exists, load from that TOML/JSON file.
    /// 2. Otherwise load from environment variables.
    /// 3. If env loading is not desired, use [`ConfigraConfig::default()`] directly.
    pub fn load(path: Option<&Path>) -> Self {
        match path {
            Some(p) if p.exists() => Self::from_file(p).unwrap_or_default(),
            _ => Self::from_env(),
        }
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Programmatic builder for [`ConfigraConfig`].
///
/// All fields default to the same values as [`ConfigraConfig::default()`].
#[derive(Debug, Clone)]
pub struct ConfigraConfigBuilder {
    default_port: Option<u16>,
    default_log_level: Option<String>,
    db_path_template: Option<String>,
    idempotency_ttl_secs: Option<u64>,
    default_max_retries: Option<u32>,
    watch_poll_ms: Option<u64>,
    watch_enabled: Option<bool>,
}

impl Default for ConfigraConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigraConfigBuilder {
    /// Create a new builder with all fields unset (will use defaults on build).
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_port: None,
            default_log_level: None,
            db_path_template: None,
            idempotency_ttl_secs: None,
            default_max_retries: None,
            watch_poll_ms: None,
            watch_enabled: None,
        }
    }

    /// Set the default service port.
    #[must_use]
    pub fn default_port(mut self, port: u16) -> Self {
        self.default_port = Some(port);
        self
    }

    /// Set the default log level.
    #[must_use]
    pub fn default_log_level(mut self, level: impl Into<String>) -> Self {
        self.default_log_level = Some(level.into());
        self
    }

    /// Set the database path template.
    #[must_use]
    pub fn db_path_template(mut self, template: impl Into<String>) -> Self {
        self.db_path_template = Some(template.into());
        self
    }

    /// Set the idempotency TTL in seconds.
    #[must_use]
    pub fn idempotency_ttl_secs(mut self, ttl: u64) -> Self {
        self.idempotency_ttl_secs = Some(ttl);
        self
    }

    /// Set the default max retries.
    #[must_use]
    pub fn default_max_retries(mut self, retries: u32) -> Self {
        self.default_max_retries = Some(retries);
        self
    }

    /// Set the watcher polling interval in milliseconds.
    #[must_use]
    pub fn watch_poll_ms(mut self, ms: u64) -> Self {
        self.watch_poll_ms = Some(ms);
        self
    }

    /// Enable or disable the file watcher.
    #[must_use]
    pub fn watch_enabled(mut self, enabled: bool) -> Self {
        self.watch_enabled = Some(enabled);
        self
    }

    /// Build the final [`ConfigraConfig`].
    #[must_use]
    pub fn build(self) -> ConfigraConfig {
        let defaults = ConfigraConfig::default();
        ConfigraConfig {
            service: ServiceConfig {
                default_port: self.default_port.unwrap_or(defaults.service.default_port),
                default_log_level: self
                    .default_log_level
                    .unwrap_or(defaults.service.default_log_level),
                db_path_template: self
                    .db_path_template
                    .unwrap_or(defaults.service.db_path_template),
            },
            idempotency: IdempotencyConfig {
                default_ttl_secs: self
                    .idempotency_ttl_secs
                    .unwrap_or(defaults.idempotency.default_ttl_secs),
                default_max_retries: self
                    .default_max_retries
                    .unwrap_or(defaults.idempotency.default_max_retries),
            },
            watcher: WatcherConfig {
                poll_interval_ms: self
                    .watch_poll_ms
                    .unwrap_or(defaults.watcher.poll_interval_ms),
                enabled: self.watch_enabled.unwrap_or(defaults.watcher.enabled),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Test 1: Default values match the documented hardcoded originals
    // -----------------------------------------------------------------------
    #[test]
    fn defaults_match_hardcoded_originals() {
        let cfg = ConfigraConfig::default();

        // From pheno-config/src/lib.rs lines 137, 225, 565: port = 8080
        assert_eq!(cfg.service.default_port, 8080);

        // From pheno-config/src/lib.rs lines 139, 237, 566: log_level = "info"
        assert_eq!(cfg.service.default_log_level, "info");

        // From settly/src/adapters/idempotency.rs line 34: 86_400 (24h)
        assert_eq!(cfg.idempotency.default_ttl_secs, 86_400);

        // From submission_tests.rs: max_retries = 2 (builder default = 3)
        assert_eq!(cfg.idempotency.default_max_retries, 3);

        // Watcher defaults
        assert_eq!(cfg.watcher.poll_interval_ms, 1_000);
        assert!(cfg.watcher.enabled);
    }

    // -----------------------------------------------------------------------
    // Test 2: Builder overrides work correctly
    // -----------------------------------------------------------------------
    #[test]
    fn builder_overrides_fields() {
        let cfg = ConfigraConfigBuilder::new()
            .default_port(9090)
            .default_log_level("debug")
            .idempotency_ttl_secs(3600)
            .default_max_retries(5)
            .watch_poll_ms(500)
            .watch_enabled(false)
            .build();

        assert_eq!(cfg.service.default_port, 9090);
        assert_eq!(cfg.service.default_log_level, "debug");
        assert_eq!(cfg.idempotency.default_ttl_secs, 3600);
        assert_eq!(cfg.idempotency.default_max_retries, 5);
        assert_eq!(cfg.watcher.poll_interval_ms, 500);
        assert!(!cfg.watcher.enabled);
    }

    // -----------------------------------------------------------------------
    // Test 3: Partial builder leaves unset fields at defaults
    // -----------------------------------------------------------------------
    #[test]
    fn partial_builder_uses_defaults() {
        let cfg = ConfigraConfigBuilder::new().default_port(3000).build();

        // default_port was overridden
        assert_eq!(cfg.service.default_port, 3000);
        // Everything else falls back to documented defaults
        assert_eq!(cfg.service.default_log_level, "info");
        assert_eq!(cfg.idempotency.default_ttl_secs, 86_400);
        assert_eq!(cfg.idempotency.default_max_retries, 3);
        assert_eq!(cfg.watcher.poll_interval_ms, 1_000);
        assert!(cfg.watcher.enabled);
    }

    // -----------------------------------------------------------------------
    // Test 4: JSON round-trip
    // -----------------------------------------------------------------------
    #[test]
    fn json_round_trip() {
        let cfg = ConfigraConfigBuilder::new()
            .default_port(8080)
            .default_log_level("warn")
            .build();

        let json = serde_json::to_string_pretty(&cfg).expect("serialize");
        let deserialized: ConfigraConfig =
            serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.service.default_port, 8080);
        assert_eq!(deserialized.service.default_log_level, "warn");
        assert_eq!(deserialized.idempotency.default_ttl_secs, 86_400);
    }

    // -----------------------------------------------------------------------
    // Test 5: TOML file loading with partial content (uses #[serde(default)])
    // -----------------------------------------------------------------------
    #[test]
    fn toml_file_partial() {
        let dir = std::env::temp_dir();
        let path = dir.join("configra_test_partial.toml");
        let toml_content = r#"
[service]
default_port = 5000
"#;
        std::fs::write(&path, toml_content).expect("write");
        let cfg = ConfigraConfig::from_file(&path).expect("load");
        assert_eq!(cfg.service.default_port, 5000);
        // Unset fields fall back to defaults via #[serde(default)]
        assert_eq!(cfg.service.default_log_level, "info");
        assert_eq!(cfg.idempotency.default_ttl_secs, 86_400);
        let _ = std::fs::remove_file(&path);
    }

    // -----------------------------------------------------------------------
    // Test 6: Env-var loading (CONFIGRA_DEFAULT_PORT override)
    // -----------------------------------------------------------------------
    #[test]
    fn from_env_overrides_default_port() {
        // Save previous value to restore
        let prev = std::env::var("CONFIGRA_DEFAULT_PORT").ok();
        std::env::set_var("CONFIGRA_DEFAULT_PORT", "9999");

        let cfg = ConfigraConfig::from_env();
        assert_eq!(cfg.service.default_port, 9999);

        // Restore
        match prev {
            Some(v) => std::env::set_var("CONFIGRA_DEFAULT_PORT", v),
            None => std::env::remove_var("CONFIGRA_DEFAULT_PORT"),
        }
    }

    // -----------------------------------------------------------------------
    // Test 7: from_file rejects unsupported extension
    // -----------------------------------------------------------------------
    #[test]
    fn from_file_rejects_unsupported_extension() {
        let dir = std::env::temp_dir();
        let path = dir.join("configra_test_bad_ext.yaml");
        std::fs::write(&path, "key: value").expect("write");
        let err = ConfigraConfig::from_file(&path).expect_err("should fail");
        assert!(
            matches!(err, ConfigError::FileParseError(ref msg) if msg.contains("unsupported")),
            "expected FileParseError for .yaml, got {err:?}"
        );
        let _ = std::fs::remove_file(&path);
    }
}
