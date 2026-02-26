use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub value_type: ValueType,
    pub namespace: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    String,
    Int,
    Float,
    Bool,
    Json,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::Json => write!(f, "json"),
        }
    }
}

impl std::str::FromStr for ValueType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "string" => Ok(Self::String),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "json" => Ok(Self::Json),
            _ => Err(Error::Other(format!("unknown value type: {s}"))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    pub enabled: bool,
    pub namespace: String,
    pub description: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretEntry {
    pub key: String,
    pub encrypted_value: Vec<u8>,
    pub nonce: Vec<u8>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub repo: String,
    pub our_version: String,
    pub upstream_version: String,
    pub synced_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: i64,
    pub key: String,
    pub namespace: String,
    pub old_value: Option<String>,
    pub new_value: String,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
}

pub trait ConfigStore {
    fn get_config(&self, namespace: &str, key: &str) -> Result<ConfigEntry>;
    fn set_config(&self, entry: &ConfigEntry) -> Result<()>;
    fn list_config(&self, namespace: &str) -> Result<Vec<ConfigEntry>>;
    fn delete_config(&self, namespace: &str, key: &str) -> Result<()>;
    fn audit_log(&self, namespace: &str, key: &str) -> Result<Vec<AuditRecord>>;
    fn restore_config(&self, namespace: &str, key: &str, audit_id: i64) -> Result<ConfigEntry>;
}

pub trait FlagStore {
    fn get_flag(&self, namespace: &str, name: &str) -> Result<FeatureFlag>;
    fn list_flags(&self, namespace: &str) -> Result<Vec<FeatureFlag>>;
    fn set_flag(&self, flag: &FeatureFlag) -> Result<()>;
    fn delete_flag(&self, namespace: &str, name: &str) -> Result<()>;
}

pub trait SecretStore {
    fn get_secret(&self, key: &str) -> Result<SecretEntry>;
    fn set_secret(&self, entry: &SecretEntry) -> Result<()>;
    fn list_secrets(&self) -> Result<Vec<String>>;
    fn delete_secret(&self, key: &str) -> Result<()>;
}

pub trait VersionStore {
    fn get_version(&self, repo: &str) -> Result<VersionInfo>;
    fn set_version(&self, info: &VersionInfo) -> Result<()>;
    fn list_versions(&self) -> Result<Vec<VersionInfo>>;
}
