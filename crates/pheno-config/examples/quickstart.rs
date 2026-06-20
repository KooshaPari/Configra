//! Quickstart example for pheno-config.
//!
//! Demonstrates programmatic config construction using `ConfigBuilder`.
//!
//! Run with:
//!   cargo run --example quickstart

use pheno_config::{Config, ConfigBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a Config with programmatic defaults
    let config: Config = ConfigBuilder::new()
        .url("https://example.com")
        .db_path("/var/lib/app.db")
        .port(8080)
        .log_level("info")
        .feature_flag("alpha")
        .build()?;

    println!("Loaded config: {:#?}", config);
    println!("Server port: {}", config.port);
    println!("DB URL: {}", config.url);

    Ok(())
}
