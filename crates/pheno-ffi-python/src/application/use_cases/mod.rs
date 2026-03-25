//! Application use cases for Python FFI.
//!
//! These use cases encapsulate business rules and coordinate between
//! domain types and outbound ports.

use pheno_core::{ConfigEntry, FeatureFlag, SecretEntry, ValueType};
use pheno_db::Database;
use std::path::PathBuf;

/// Result type for use case operations.
pub type UseCaseResult<T> = Result<T, pheno_core::Error>;

/// Use case: Get configuration value.
pub struct GetConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> GetConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, key: &str) -> UseCaseResult<ConfigEntry> {
        self.db.get_config(self.namespace, key)
    }
}

/// Use case: Set configuration value.
pub struct SetConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> SetConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, entry: ConfigEntry) -> UseCaseResult<()> {
        self.db.set_config(&entry)
    }

    pub fn execute_with_params(
        &self,
        key: String,
        value: String,
        value_type: String,
        updated_by: String,
    ) -> UseCaseResult<()> {
        let vt: ValueType = value_type.parse()?;
        let entry = ConfigEntry {
            key,
            value,
            value_type: vt,
            namespace: self.namespace.to_string(),
            updated_at: chrono::Utc::now(),
            updated_by,
        };
        self.db.set_config(&entry)
    }
}

/// Use case: List all configuration values in namespace.
pub struct ListConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> ListConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self) -> UseCaseResult<Vec<ConfigEntry>> {
        self.db.list_config(self.namespace)
    }
}

/// Use case: Delete configuration value.
pub struct DeleteConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> DeleteConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, key: &str) -> UseCaseResult<()> {
        self.db.delete_config(self.namespace, key)
    }
}

/// Use case: Get audit log for a key.
pub struct AuditConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> AuditConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, key: &str) -> UseCaseResult<Vec<pheno_core::AuditRecord>> {
        self.db.audit_log(self.namespace, key)
    }
}

/// Use case: Restore config from audit record.
pub struct RestoreConfig<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> RestoreConfig<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, key: &str, audit_id: i64) -> UseCaseResult<ConfigEntry> {
        self.db.restore_config(self.namespace, key, audit_id)
    }
}

/// Use case: List feature flags.
pub struct ListFlags<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> ListFlags<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self) -> UseCaseResult<Vec<FeatureFlag>> {
        self.db.list_flags(self.namespace)
    }
}

/// Use case: Create feature flag.
pub struct CreateFlag<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> CreateFlag<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, flag: FeatureFlag) -> UseCaseResult<()> {
        self.db.set_flag(&flag)
    }

    pub fn execute_with_params(&self, name: String, description: String) -> UseCaseResult<()> {
        let flag = FeatureFlag {
            name,
            enabled: false,
            namespace: self.namespace.to_string(),
            description,
            updated_at: chrono::Utc::now(),
            stage: "SP".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        };
        self.db.set_flag(&flag)
    }
}

/// Use case: Toggle feature flag.
pub struct ToggleFlag<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> ToggleFlag<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn enable(&self, name: &str) -> UseCaseResult<()> {
        let mut flag = self.db.get_flag(self.namespace, name)?;
        flag.enabled = true;
        flag.updated_at = chrono::Utc::now();
        self.db.set_flag(&flag)
    }

    pub fn disable(&self, name: &str) -> UseCaseResult<()> {
        let mut flag = self.db.get_flag(self.namespace, name)?;
        flag.enabled = false;
        flag.updated_at = chrono::Utc::now();
        self.db.set_flag(&flag)
    }
}

/// Use case: Delete feature flag.
pub struct DeleteFlag<'a> {
    db: &'a Database,
    namespace: &'a str,
}

impl<'a> DeleteFlag<'a> {
    pub fn new(db: &'a Database, namespace: &'a str) -> Self {
        Self { db, namespace }
    }

    pub fn execute(&self, name: &str) -> UseCaseResult<()> {
        self.db.delete_flag(self.namespace, name)
    }
}

/// Use case: List secrets.
pub struct ListSecrets<'a> {
    db: &'a Database,
}

impl<'a> ListSecrets<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn execute(&self) -> UseCaseResult<Vec<String>> {
        self.db.list_secrets()
    }
}

/// Use case: Get secret.
pub struct GetSecret<'a> {
    db: &'a Database,
    encryption_key: &'a [u8],
}

impl<'a> GetSecret<'a> {
    pub fn new(db: &'a Database, encryption_key: &'a [u8]) -> Self {
        Self { db, encryption_key }
    }

    pub fn execute(&self, key: &str) -> UseCaseResult<String> {
        let entry = self.db.get_secret(key)?;
        let plaintext =
            pheno_crypto::decrypt(&entry.encrypted_value, &entry.nonce, self.encryption_key)?;
        Ok(String::from_utf8(plaintext)?)
    }
}

/// Use case: Set secret.
pub struct SetSecret<'a> {
    db: &'a Database,
    encryption_key: &'a [u8],
}

impl<'a> SetSecret<'a> {
    pub fn new(db: &'a Database, encryption_key: &'a [u8]) -> Self {
        Self { db, encryption_key }
    }

    pub fn execute(&self, key: &str, plaintext: &str) -> UseCaseResult<()> {
        let (ciphertext, nonce) =
            pheno_crypto::encrypt(plaintext.as_bytes(), self.encryption_key)?;
        let entry = SecretEntry {
            key: key.to_string(),
            encrypted_value: ciphertext,
            nonce,
            updated_at: chrono::Utc::now(),
        };
        self.db.set_secret(&entry)
    }
}

/// Use case: Delete secret.
pub struct DeleteSecret<'a> {
    db: &'a Database,
}

impl<'a> DeleteSecret<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn execute(&self, key: &str) -> UseCaseResult<()> {
        self.db.delete_secret(key)
    }
}

/// Use case: List versions.
pub struct ListVersions<'a> {
    db: &'a Database,
}

impl<'a> ListVersions<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn execute(&self) -> UseCaseResult<Vec<pheno_core::VersionInfo>> {
        self.db.list_versions()
    }
}

/// Use case: Bump version.
pub struct BumpVersion<'a> {
    db: &'a Database,
}

impl<'a> BumpVersion<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn execute(&self, repo: &str, version: &str) -> UseCaseResult<()> {
        let mut info = self.db.get_version(repo).unwrap_or(pheno_core::VersionInfo {
            repo: repo.to_string(),
            our_version: "0.0.0".to_string(),
            upstream_version: String::new(),
            synced_at: chrono::Utc::now(),
        });
        info.our_version = version.to_string();
        info.synced_at = chrono::Utc::now();
        self.db.set_version(&info)
    }
}

/// Use case: Sync upstream version.
pub struct SyncVersion<'a> {
    db: &'a Database,
}

impl<'a> SyncVersion<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn execute(&self, repo: &str, upstream: &str) -> UseCaseResult<()> {
        let mut info = self.db.get_version(repo).unwrap_or(pheno_core::VersionInfo {
            repo: repo.to_string(),
            our_version: "0.0.0".to_string(),
            upstream_version: String::new(),
            synced_at: chrono::Utc::now(),
        });
        info.upstream_version = upstream.to_string();
        info.synced_at = chrono::Utc::now();
        self.db.set_version(&info)
    }
}
