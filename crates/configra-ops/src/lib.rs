//! Observability and operations primitives for Configra services.
//!
//! Additive substrate for audit areas G (Observability) and K (Ops):
//! structured logging, correlation IDs, metrics hooks, health/readiness
//! probes, and graceful shutdown helpers.

pub mod correlation;
pub mod health;
pub mod logging;
pub mod metrics;
pub mod shutdown;

pub use correlation::{CorrelationId, CorrelationLayer};
pub use health::{CheckResult, HealthCheck, HealthReport, HealthStatus, liveness, readiness};
pub use logging::{LogFormat, LoggingConfig, init_logging};
pub use metrics::{MetricsHook, MetricsRegistry, NoopMetricsHook};
pub use shutdown::{GracefulShutdown, ShutdownConfig, shutdown_signal};

/// Crate version (matches workspace package version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
