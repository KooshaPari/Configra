//! Python bindings (PyO3) for Phenotype configuration.
//!
//! These adapters implement the inbound ports for the Python ecosystem.
//! They delegate to application use cases for business logic.

use crate::application::use_cases::*;
use pheno_db::Database;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;

fn to_pyerr(e: pheno_core::Error) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

// ============================================================================
// Shared Database Wrapper
// ============================================================================

struct Db(Mutex<Database>);

impl Db {
    fn lock(&self) -> std::sync::MutexGuard<'_, Database> {
        self.0.lock().unwrap()
    }
}

fn open_db(path: &str) -> PyResult<Database> {
    let p = PathBuf::from(path);
    Database::open(&p).map_err(to_pyerr)
}

// ============================================================================
// PhenoConfig Python Adapter
// ============================================================================

#[pyclass]
struct PhenoConfig {
    db: Db,
    namespace: String,
}

#[pymethods]
impl PhenoConfig {
    #[new]
    #[pyo3(signature = (db_path, namespace = "default".to_string()))]
    fn new(db_path: String, namespace: String) -> PyResult<Self> {
        let db = open_db(&db_path)?;
        Ok(Self {
            db: Db(Mutex::new(db)),
            namespace,
        })
    }

    fn get(&self, key: String) -> PyResult<(String, String, String)> {
        let db = self.db.lock();
        let use_case = GetConfig::new(&db, &self.namespace);
        let entry = use_case.execute(&key).map_err(to_pyerr)?;
        Ok((entry.key, entry.value, entry.value_type.to_string()))
    }

    #[pyo3(signature = (key, value, value_type = "string".to_string()))]
    fn set(&self, key: String, value: String, value_type: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = SetConfig::new(&db, &self.namespace);
        use_case.execute_with_params(
            key,
            value,
            value_type,
            std::env::var("USER").unwrap_or_else(|_| "python".to_string()),
        )
        .map_err(to_pyerr)
    }

    fn list(&self) -> PyResult<Vec<(String, String, String)>> {
        let db = self.db.lock();
        let use_case = ListConfig::new(&db, &self.namespace);
        let entries = use_case.execute().map_err(to_pyerr)?;
        Ok(entries
            .iter()
            .map(|e| (e.key.clone(), e.value.clone(), e.value_type.to_string()))
            .collect())
    }

    fn delete(&self, key: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = DeleteConfig::new(&db, &self.namespace);
        use_case.execute(&key).map_err(to_pyerr)
    }

    fn audit(&self, key: String) -> PyResult<Vec<(i64, Option<String>, String, String, String)>> {
        let db = self.db.lock();
        let use_case = AuditConfig::new(&db, &self.namespace);
        let records = use_case.execute(&key).map_err(to_pyerr)?;
        Ok(records
            .iter()
            .map(|r| {
                (
                    r.id,
                    r.old_value.clone(),
                    r.new_value.clone(),
                    r.changed_by.clone(),
                    r.changed_at.to_rfc3339(),
                )
            })
            .collect())
    }

    fn restore(&self, key: String, audit_id: i64) -> PyResult<(String, String)> {
        let db = self.db.lock();
        let use_case = RestoreConfig::new(&db, &self.namespace);
        let entry = use_case.execute(&key, audit_id).map_err(to_pyerr)?;
        Ok((entry.key, entry.value))
    }
}

// ============================================================================
// FeatureFlags Python Adapter
// ============================================================================

#[pyclass]
struct FeatureFlags {
    db: Db,
    namespace: String,
}

#[pymethods]
impl FeatureFlags {
    #[new]
    #[pyo3(signature = (db_path, namespace = "default".to_string()))]
    fn new(db_path: String, namespace: String) -> PyResult<Self> {
        let db = open_db(&db_path)?;
        Ok(Self {
            db: Db(Mutex::new(db)),
            namespace,
        })
    }

    fn list(&self) -> PyResult<Vec<(String, bool, String)>> {
        let db = self.db.lock();
        let use_case = ListFlags::new(&db, &self.namespace);
        let flags = use_case.execute().map_err(to_pyerr)?;
        Ok(flags
            .iter()
            .map(|f| (f.name.clone(), f.enabled, f.description.clone()))
            .collect())
    }

    #[pyo3(signature = (name, description = "".to_string()))]
    fn create(&self, name: String, description: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = CreateFlag::new(&db, &self.namespace);
        use_case.execute_with_params(name, description).map_err(to_pyerr)
    }

    fn enable(&self, name: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = ToggleFlag::new(&db, &self.namespace);
        use_case.enable(&name).map_err(to_pyerr)
    }

    fn disable(&self, name: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = ToggleFlag::new(&db, &self.namespace);
        use_case.disable(&name).map_err(to_pyerr)
    }

    fn delete(&self, name: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = DeleteFlag::new(&db, &self.namespace);
        use_case.execute(&name).map_err(to_pyerr)
    }
}

// ============================================================================
// Secrets Python Adapter
// ============================================================================

#[pyclass]
struct Secrets {
    db: Db,
    encryption_key: Vec<u8>,
}

#[pymethods]
impl Secrets {
    #[new]
    fn new(db_path: String, hex_key: String) -> PyResult<Self> {
        let db = open_db(&db_path)?;
        let encryption_key = hex::decode(&hex_key)
            .map_err(|e| PyRuntimeError::new_err(format!("invalid hex key: {e}")))?;
        if encryption_key.len() != 32 {
            return Err(PyRuntimeError::new_err(
                "key must be 32 bytes (64 hex chars)",
            ));
        }
        Ok(Self {
            db: Db(Mutex::new(db)),
            encryption_key,
        })
    }

    fn set(&self, key: String, plaintext: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = SetSecret::new(&db, &self.encryption_key);
        use_case.execute(&key, &plaintext).map_err(to_pyerr)
    }

    fn get(&self, key: String) -> PyResult<String> {
        let db = self.db.lock();
        let use_case = GetSecret::new(&db, &self.encryption_key);
        use_case.execute(&key).map_err(to_pyerr)
    }

    fn list(&self) -> PyResult<Vec<String>> {
        let db = self.db.lock();
        let use_case = ListSecrets::new(&db);
        use_case.execute().map_err(to_pyerr)
    }

    fn delete(&self, key: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = DeleteSecret::new(&db);
        use_case.execute(&key).map_err(to_pyerr)
    }
}

// ============================================================================
// VersionInfo Python Adapter
// ============================================================================

#[pyclass]
struct VersionInfoPy_ {
    db: Db,
}

#[pymethods]
impl VersionInfoPy_ {
    #[new]
    fn new(db_path: String) -> PyResult<Self> {
        let db = open_db(&db_path)?;
        Ok(Self {
            db: Db(Mutex::new(db)),
        })
    }

    fn show(&self) -> PyResult<Vec<(String, String, String, String)>> {
        let db = self.db.lock();
        let use_case = ListVersions::new(&db);
        let versions = use_case.execute().map_err(to_pyerr)?;
        Ok(versions
            .iter()
            .map(|v| {
                (
                    v.repo.clone(),
                    v.our_version.clone(),
                    v.upstream_version.clone(),
                    v.synced_at.to_rfc3339(),
                )
            })
            .collect())
    }

    fn bump(&self, repo: String, version: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = BumpVersion::new(&db);
        use_case.execute(&repo, &version).map_err(to_pyerr)
    }

    fn sync(&self, repo: String, upstream: String) -> PyResult<()> {
        let db = self.db.lock();
        let use_case = SyncVersion::new(&db);
        use_case.execute(&repo, &upstream).map_err(to_pyerr)
    }
}

// ============================================================================
// Module Entry Point
// ============================================================================

#[pymodule]
fn phenotype_config(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PhenoConfig>()?;
    m.add_class::<FeatureFlags>()?;
    m.add_class::<Secrets>()?;
    m.add_class::<VersionInfoPy_>()?;
    Ok(())
}
