//! Multi-source cascade example for pheno-config.
//!
//! Demonstrates the 12-factor config cascade using `combine()`:
//!   TOML file (lowest priority) ← Environment vars (highest priority)
//!
//! Run with:
//!   cargo run --example cascade
//!
//! Override with env:
//!   PHENO_CONFIG_PORT=9090 cargo run --example cascade

use pheno_config::combine;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write a sample config.toml for demonstration
    fs::write(
        "pheno_config_example.toml",
        r#"
url = "https://example.com"
db_path = "/var/lib/app.db"
port = 8080
log_level = "info"
feature_flags = ["alpha", "beta"]
"#,
    )?;

    // combine() loads the TOML file first, then overlays env vars
    // matching the given prefix (PHENO_CONFIG_*). Env vars override
    // file values when present.
    let config = combine(Path::new("pheno_config_example.toml"), "PHENO_CONFIG")?;

    println!("Resolved config: {:#?}", config);
    println!(
        "Field provenance: server.port = {} (env overrides > file > default)",
        config.port
    );

    // Cleanup
    fs::remove_file("pheno_config_example.toml")?;

    Ok(())
}
