//! Entity definitions for Python FFI domain.
//!
//! These are thin wrappers around the domain types from pheno-core/pheno-db.
//! They exist to provide a stable interface for the Python bindings.

use chrono::Utc;
use pheno_core::{ConfigEntry, FeatureFlag, SecretEntry, ValueType};
use crate::domain::dto::{
    AuditRecordPy, ConfigEntryPy, CreateFlagRequest, FeatureFlagPy, SetConfigRequest,
    SetSecretRequest, VersionInfoPy,
};

/// Convert from ConfigEntry to Python DTO.
impl From<&ConfigEntry> for ConfigEntryPy {
    fn from(entry: &ConfigEntry) -> Self {
        ConfigEntryPy::new(
            entry.key.clone(),
            entry.value.clone(),
            entry.value_type.to_string(),
        )
    }
}

/// Convert from ConfigEntry PyTuple.
impl ConfigEntryPy {
    pub fn to_tuple(&self) -> (String, String, String) {
        (self.key.clone(), self.value.clone(), self.value_type.clone())
    }
}

/// Convert from FeatureFlag to Python DTO.
impl From<&FeatureFlag> for FeatureFlagPy {
    fn from(flag: &FeatureFlag) -> Self {
        FeatureFlagPy::new(
            flag.name.clone(),
            flag.enabled,
            flag.description.clone(),
        )
    }
}

/// Convert from FeatureFlag PyTuple.
impl FeatureFlagPy {
    pub fn to_tuple(&self) -> (String, bool, String) {
        (self.name.clone(), self.enabled, self.description.clone())
    }
}

/// Convert from audit record to Python DTO.
impl AuditRecordPy {
    pub fn from_audit_record(
        id: i64,
        old_value: Option<String>,
        new_value: Option<String>,
        changed_by: String,
        changed_at: chrono::DateTime<Utc>,
    ) -> Self {
        AuditRecordPy::new(
            id,
            old_value,
            new_value,
            changed_by,
            changed_at.to_rfc3339(),
        )
    }

    pub fn to_tuple(&self) -> (i64, Option<String>, String, String, String) {
        (
            self.id,
            self.old_value.clone(),
            self.new_value.clone(),
            self.changed_by.clone(),
            self.changed_at.clone(),
        )
    }
}

/// Convert from VersionInfo to Python DTO.
impl From<&pheno_core::VersionInfo> for VersionInfoPy {
    fn from(info: &pheno_core::VersionInfo) -> Self {
        VersionInfoPy::new(
            info.repo.clone(),
            info.our_version.clone(),
            info.upstream_version.clone(),
            info.synced_at.to_rfc3339(),
        )
    }
}

/// Convert from VersionInfo PyTuple.
impl VersionInfoPy {
    pub fn to_tuple(&self) -> (String, String, String, String) {
        (
            self.repo.clone(),
            self.our_version.clone(),
            self.upstream_version.clone(),
            self.synced_at.clone(),
        )
    }
}

/// Convert SetConfigRequest to ConfigEntry.
impl SetConfigRequest {
    pub fn into_config_entry(self, namespace: String) -> Result<ConfigEntry, pheno_core::Error> {
        let value_type: ValueType = self.value_type.parse()?;
        Ok(ConfigEntry {
            key: self.key,
            value: self.value,
            value_type,
            namespace,
            updated_at: Utc::now(),
            updated_by: self.updated_by,
        })
    }
}

/// Convert CreateFlagRequest to FeatureFlag.
impl CreateFlagRequest {
    pub fn into_feature_flag(self, namespace: String) -> FeatureFlag {
        FeatureFlag {
            name: self.name,
            enabled: false,
            namespace,
            description: self.description,
            updated_at: Utc::now(),
            stage: "SP".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        }
    }
}
