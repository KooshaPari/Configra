//! Validation example for pheno-config.
//!
//! Shows how to attach validators that produce structured errors
//! using pheno-config's own `ConfigError` type.
//!
//! Run with:
//!   cargo run --example validation

use pheno_config::ConfigError;

#[derive(Debug)]
struct ServerConfig {
    port: u16,
    host: String,
    max_connections: u32,
}

impl ServerConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.port < 1024 {
            return Err(ConfigError::MissingField(
                format!("server.port: port must be >= 1024 (non-privileged), got {}", self.port),
            ));
        }
        if self.host.is_empty() {
            return Err(ConfigError::MissingField(
                "server.host: host cannot be empty".into(),
            ));
        }
        if self.max_connections == 0 || self.max_connections > 10_000 {
            return Err(ConfigError::MissingField(
                format!("server.max_connections: must be 1..=10000, got {}", self.max_connections),
            ));
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Valid config
    let good = ServerConfig {
        port: 8080,
        host: "0.0.0.0".into(),
        max_connections: 100,
    };
    good.validate()?;
    println!("✓ Good config: {:#?}", good);

    // Invalid config (port < 1024)
    let bad = ServerConfig {
        port: 80,
        host: "0.0.0.0".into(),
        max_connections: 100,
    };
    match bad.validate() {
        Err(ConfigError::MissingField(msg)) => {
            eprintln!("✗ Validation failed: {}", msg);
        }
        other => panic!("expected ConfigError::MissingField, got {:?}", other),
    }

    Ok(())
}
