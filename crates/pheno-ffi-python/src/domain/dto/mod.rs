//! Data Transfer Objects for Python FFI.
//!
//! DTOs are simple, serializable structures for crossing the Python-Rust boundary.
//! These should be plain data types with no business logic.

/// Configuration entry as seen from Python.
#[derive(Debug, Clone)]
pub struct ConfigEntryPy {
    pub key: String,
    pub value: String,
    pub value_type: String,
}

impl ConfigEntryPy {
    pub fn new(key: String, value: String, value_type: String) -> Self {
        Self { key, value, value_type }
    }
}

/// Feature flag as seen from Python.
#[derive(Debug, Clone)]
pub struct FeatureFlagPy {
    pub name: String,
    pub enabled: bool,
    pub description: String,
}

impl FeatureFlagPy {
    pub fn new(name: String, enabled: bool, description: String) -> Self {
        Self { name, enabled, description }
    }
}

/// Audit record as seen from Python.
#[derive(Debug, Clone)]
pub struct AuditRecordPy {
    pub id: i64,
    pub old_value: Option<String>,
    pub new_value: String,
    pub changed_by: String,
    pub changed_at: String,
}

impl AuditRecordPy {
    pub fn new(
        id: i64,
        old_value: Option<String>,
        new_value: String,
        changed_by: String,
        changed_at: String,
    ) -> Self {
        Self {
            id,
            old_value,
            new_value,
            changed_by,
            changed_at,
        }
    }
}

/// Version info as seen from Python.
#[derive(Debug, Clone)]
pub struct VersionInfoPy {
    pub repo: String,
    pub our_version: String,
    pub upstream_version: String,
    pub synced_at: String,
}

impl VersionInfoPy {
    pub fn new(
        repo: String,
        our_version: String,
        upstream_version: String,
        synced_at: String,
    ) -> Self {
        Self {
            repo,
            our_version,
            upstream_version,
            synced_at,
        }
    }
}

/// Request to set a configuration value.
#[derive(Debug, Clone)]
pub struct SetConfigRequest {
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub updated_by: String,
}

impl SetConfigRequest {
    pub fn new(key: String, value: String, value_type: String, updated_by: String) -> Self {
        Self {
            key,
            value,
            value_type,
            updated_by,
        }
    }
}

/// Request to create a feature flag.
#[derive(Debug, Clone)]
pub struct CreateFlagRequest {
    pub name: String,
    pub description: String,
}

impl CreateFlagRequest {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

/// Request to set a secret.
#[derive(Debug, Clone)]
pub struct SetSecretRequest {
    pub key: String,
    pub plaintext: String,
}

impl SetSecretRequest {
    pub fn new(key: String, plaintext: String) -> Self {
        Self { key, plaintext }
    }
}

/// Request to bump version.
#[derive(Debug, Clone)]
pub struct BumpVersionRequest {
    pub repo: String,
    pub version: String,
}

impl BumpVersionRequest {
    pub fn new(repo: String, version: String) -> Self {
        Self { repo, version }
    }
}

/// Request to sync upstream version.
#[derive(Debug, Clone)]
pub struct SyncVersionRequest {
    pub repo: String,
    pub upstream: String,
}

impl SyncVersionRequest {
    pub fn new(repo: String, upstream: String) -> Self {
        Self { repo, upstream }
    }
}
