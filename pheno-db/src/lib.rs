use chrono::{DateTime, Utc};
use pheno_core::*;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::Database(format!("create dir: {e}")))?;
        }
        let conn =
            Connection::open(path).map_err(|e| Error::Database(format!("open: {e}")))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| Error::Database(e.to_string()))?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS config_entries (
                namespace TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                value_type TEXT NOT NULL DEFAULT 'string',
                updated_at TEXT NOT NULL,
                updated_by TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (namespace, key)
            );
            CREATE TABLE IF NOT EXISTS config_audit (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                namespace TEXT NOT NULL,
                key TEXT NOT NULL,
                old_value TEXT,
                new_value TEXT NOT NULL,
                changed_by TEXT NOT NULL DEFAULT '',
                changed_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS feature_flags (
                namespace TEXT NOT NULL,
                name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 0,
                description TEXT NOT NULL DEFAULT '',
                updated_at TEXT NOT NULL,
                PRIMARY KEY (namespace, name)
            );
            CREATE TABLE IF NOT EXISTS secrets (
                key TEXT PRIMARY KEY,
                encrypted_value BLOB NOT NULL,
                nonce BLOB NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS version_info (
                repo TEXT PRIMARY KEY,
                our_version TEXT NOT NULL,
                upstream_version TEXT NOT NULL DEFAULT '',
                synced_at TEXT NOT NULL
            );
            ",
            )
            .map_err(|e| Error::Database(e.to_string()))
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_default()
}

impl ConfigStore for Database {
    fn get_config(&self, namespace: &str, key: &str) -> Result<ConfigEntry> {
        self.conn
            .query_row(
                "SELECT key, value, value_type, namespace, updated_at, updated_by FROM config_entries WHERE namespace=?1 AND key=?2",
                params![namespace, key],
                |row| {
                    Ok(ConfigEntry {
                        key: row.get(0)?,
                        value: row.get(1)?,
                        value_type: row.get::<_, String>(2)?.parse().unwrap_or(ValueType::String),
                        namespace: row.get(3)?,
                        updated_at: parse_dt(&row.get::<_, String>(4)?),
                        updated_by: row.get(5)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound(format!("{namespace}/{key}")),
                _ => Error::Database(e.to_string()),
            })
    }

    fn set_config(&self, entry: &ConfigEntry) -> Result<()> {
        let old = self.get_config(&entry.namespace, &entry.key).ok();
        self.conn
            .execute(
                "INSERT INTO config_entries (namespace, key, value, value_type, updated_at, updated_by) VALUES (?1,?2,?3,?4,?5,?6)
                 ON CONFLICT(namespace, key) DO UPDATE SET value=excluded.value, value_type=excluded.value_type, updated_at=excluded.updated_at, updated_by=excluded.updated_by",
                params![
                    entry.namespace,
                    entry.key,
                    entry.value,
                    entry.value_type.to_string(),
                    entry.updated_at.to_rfc3339(),
                    entry.updated_by,
                ],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        self.conn
            .execute(
                "INSERT INTO config_audit (namespace, key, old_value, new_value, changed_by, changed_at) VALUES (?1,?2,?3,?4,?5,?6)",
                params![
                    entry.namespace,
                    entry.key,
                    old.map(|o| o.value),
                    entry.value,
                    entry.updated_by,
                    entry.updated_at.to_rfc3339(),
                ],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    fn list_config(&self, namespace: &str) -> Result<Vec<ConfigEntry>> {
        let mut stmt = self.conn
            .prepare("SELECT key, value, value_type, namespace, updated_at, updated_by FROM config_entries WHERE namespace=?1 ORDER BY key")
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map(params![namespace], |row| {
                Ok(ConfigEntry {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    value_type: row.get::<_, String>(2)?.parse().unwrap_or(ValueType::String),
                    namespace: row.get(3)?,
                    updated_at: parse_dt(&row.get::<_, String>(4)?),
                    updated_by: row.get(5)?,
                })
            })
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }

    fn delete_config(&self, namespace: &str, key: &str) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM config_entries WHERE namespace=?1 AND key=?2",
                params![namespace, key],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    fn audit_log(&self, namespace: &str, key: &str) -> Result<Vec<AuditRecord>> {
        let mut stmt = self.conn
            .prepare("SELECT id, key, namespace, old_value, new_value, changed_by, changed_at FROM config_audit WHERE namespace=?1 AND key=?2 ORDER BY id")
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map(params![namespace, key], |row| {
                Ok(AuditRecord {
                    id: row.get(0)?,
                    key: row.get(1)?,
                    namespace: row.get(2)?,
                    old_value: row.get(3)?,
                    new_value: row.get(4)?,
                    changed_by: row.get(5)?,
                    changed_at: parse_dt(&row.get::<_, String>(6)?),
                })
            })
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }

    fn restore_config(&self, namespace: &str, key: &str, audit_id: i64) -> Result<ConfigEntry> {
        let record: AuditRecord = self.conn
            .query_row(
                "SELECT id, key, namespace, old_value, new_value, changed_by, changed_at FROM config_audit WHERE id=?1 AND namespace=?2 AND key=?3",
                params![audit_id, namespace, key],
                |row| {
                    Ok(AuditRecord {
                        id: row.get(0)?,
                        key: row.get(1)?,
                        namespace: row.get(2)?,
                        old_value: row.get(3)?,
                        new_value: row.get(4)?,
                        changed_by: row.get(5)?,
                        changed_at: parse_dt(&row.get::<_, String>(6)?),
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound(format!("audit record {audit_id}")),
                _ => Error::Database(e.to_string()),
            })?;
        let restored = ConfigEntry {
            key: key.to_string(),
            value: record.new_value.clone(),
            value_type: ValueType::String,
            namespace: namespace.to_string(),
            updated_at: Utc::now(),
            updated_by: "restore".to_string(),
        };
        self.set_config(&restored)?;
        Ok(restored)
    }
}

impl FlagStore for Database {
    fn get_flag(&self, namespace: &str, name: &str) -> Result<FeatureFlag> {
        self.conn
            .query_row(
                "SELECT name, enabled, namespace, description, updated_at FROM feature_flags WHERE namespace=?1 AND name=?2",
                params![namespace, name],
                |row| {
                    Ok(FeatureFlag {
                        name: row.get(0)?,
                        enabled: row.get::<_, i32>(1)? != 0,
                        namespace: row.get(2)?,
                        description: row.get(3)?,
                        updated_at: parse_dt(&row.get::<_, String>(4)?),
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound(format!("{namespace}/{name}")),
                _ => Error::Database(e.to_string()),
            })
    }

    fn list_flags(&self, namespace: &str) -> Result<Vec<FeatureFlag>> {
        let mut stmt = self.conn
            .prepare("SELECT name, enabled, namespace, description, updated_at FROM feature_flags WHERE namespace=?1 ORDER BY name")
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map(params![namespace], |row| {
                Ok(FeatureFlag {
                    name: row.get(0)?,
                    enabled: row.get::<_, i32>(1)? != 0,
                    namespace: row.get(2)?,
                    description: row.get(3)?,
                    updated_at: parse_dt(&row.get::<_, String>(4)?),
                })
            })
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }

    fn set_flag(&self, flag: &FeatureFlag) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO feature_flags (namespace, name, enabled, description, updated_at) VALUES (?1,?2,?3,?4,?5)
                 ON CONFLICT(namespace, name) DO UPDATE SET enabled=excluded.enabled, description=excluded.description, updated_at=excluded.updated_at",
                params![
                    flag.namespace,
                    flag.name,
                    flag.enabled as i32,
                    flag.description,
                    flag.updated_at.to_rfc3339(),
                ],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    fn delete_flag(&self, namespace: &str, name: &str) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM feature_flags WHERE namespace=?1 AND name=?2",
                params![namespace, name],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }
}

impl SecretStore for Database {
    fn get_secret(&self, key: &str) -> Result<SecretEntry> {
        self.conn
            .query_row(
                "SELECT key, encrypted_value, nonce, updated_at FROM secrets WHERE key=?1",
                params![key],
                |row| {
                    Ok(SecretEntry {
                        key: row.get(0)?,
                        encrypted_value: row.get(1)?,
                        nonce: row.get(2)?,
                        updated_at: parse_dt(&row.get::<_, String>(3)?),
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound(key.to_string()),
                _ => Error::Database(e.to_string()),
            })
    }

    fn set_secret(&self, entry: &SecretEntry) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO secrets (key, encrypted_value, nonce, updated_at) VALUES (?1,?2,?3,?4)
                 ON CONFLICT(key) DO UPDATE SET encrypted_value=excluded.encrypted_value, nonce=excluded.nonce, updated_at=excluded.updated_at",
                params![
                    entry.key,
                    entry.encrypted_value,
                    entry.nonce,
                    entry.updated_at.to_rfc3339(),
                ],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    fn list_secrets(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn
            .prepare("SELECT key FROM secrets ORDER BY key")
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }

    fn delete_secret(&self, key: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM secrets WHERE key=?1", params![key])
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }
}

impl VersionStore for Database {
    fn get_version(&self, repo: &str) -> Result<VersionInfo> {
        self.conn
            .query_row(
                "SELECT repo, our_version, upstream_version, synced_at FROM version_info WHERE repo=?1",
                params![repo],
                |row| {
                    Ok(VersionInfo {
                        repo: row.get(0)?,
                        our_version: row.get(1)?,
                        upstream_version: row.get(2)?,
                        synced_at: parse_dt(&row.get::<_, String>(3)?),
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Error::NotFound(repo.to_string()),
                _ => Error::Database(e.to_string()),
            })
    }

    fn set_version(&self, info: &VersionInfo) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO version_info (repo, our_version, upstream_version, synced_at) VALUES (?1,?2,?3,?4)
                 ON CONFLICT(repo) DO UPDATE SET our_version=excluded.our_version, upstream_version=excluded.upstream_version, synced_at=excluded.synced_at",
                params![
                    info.repo,
                    info.our_version,
                    info.upstream_version,
                    info.synced_at.to_rfc3339(),
                ],
            )
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    fn list_versions(&self) -> Result<Vec<VersionInfo>> {
        let mut stmt = self.conn
            .prepare("SELECT repo, our_version, upstream_version, synced_at FROM version_info ORDER BY repo")
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(VersionInfo {
                    repo: row.get(0)?,
                    our_version: row.get(1)?,
                    upstream_version: row.get(2)?,
                    synced_at: parse_dt(&row.get::<_, String>(3)?),
                })
            })
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }
}
