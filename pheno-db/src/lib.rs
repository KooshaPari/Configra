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
        let conn = Connection::open(path).map_err(|e| Error::Database(format!("open: {e}")))?;
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
                stage TEXT NOT NULL DEFAULT 'SP',
                transience_class TEXT NOT NULL DEFAULT 'F',
                channel TEXT NOT NULL DEFAULT '[\"dev\"]',
                retire_at_stage TEXT,
                PRIMARY KEY (namespace, name)
            );
            CREATE TABLE IF NOT EXISTS stage_transitions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flag_name TEXT NOT NULL,
                from_stage TEXT NOT NULL,
                to_stage TEXT NOT NULL,
                transitioned_at TEXT NOT NULL,
                transitioned_by TEXT NOT NULL
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
            .map_err(|e| Error::Database(e.to_string()))?;

        // Add columns if migrating from older schema
        let _ = self.conn.execute_batch(
            "ALTER TABLE feature_flags ADD COLUMN stage TEXT NOT NULL DEFAULT 'SP';
             ALTER TABLE feature_flags ADD COLUMN transience_class TEXT NOT NULL DEFAULT 'F';
             ALTER TABLE feature_flags ADD COLUMN channel TEXT NOT NULL DEFAULT '[\"dev\"]';
             ALTER TABLE feature_flags ADD COLUMN retire_at_stage TEXT;",
        );

        Ok(())
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_default()
}

fn parse_channel(s: &str) -> Vec<String> {
    serde_json::from_str(s).unwrap_or_else(|_| vec!["dev".to_string()])
}

fn encode_channel(ch: &[String]) -> String {
    serde_json::to_string(ch).unwrap_or_else(|_| "[\"dev\"]".to_string())
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
                    value_type: row
                        .get::<_, String>(2)?
                        .parse()
                        .unwrap_or(ValueType::String),
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

fn read_flag_row(row: &rusqlite::Row) -> rusqlite::Result<FeatureFlag> {
    Ok(FeatureFlag {
        name: row.get(0)?,
        enabled: row.get::<_, i32>(1)? != 0,
        namespace: row.get(2)?,
        description: row.get(3)?,
        updated_at: parse_dt(&row.get::<_, String>(4)?),
        stage: row.get(5)?,
        transience_class: row.get(6)?,
        channel: parse_channel(&row.get::<_, String>(7)?),
        retire_at_stage: row.get(8)?,
    })
}

const FLAG_COLS: &str = "name, enabled, namespace, description, updated_at, stage, transience_class, channel, retire_at_stage";

impl FlagStore for Database {
    fn get_flag(&self, namespace: &str, name: &str) -> Result<FeatureFlag> {
        self.conn
            .query_row(
                &format!("SELECT {FLAG_COLS} FROM feature_flags WHERE namespace=?1 AND name=?2"),
                params![namespace, name],
                read_flag_row,
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    Error::NotFound(format!("{namespace}/{name}"))
                }
                _ => Error::Database(e.to_string()),
            })
    }

    fn list_flags(&self, namespace: &str) -> Result<Vec<FeatureFlag>> {
        let mut stmt = self
            .conn
            .prepare(&format!(
                "SELECT {FLAG_COLS} FROM feature_flags WHERE namespace=?1 ORDER BY name"
            ))
            .map_err(|e| Error::Database(e.to_string()))?;
        let rows = stmt
            .query_map(params![namespace], read_flag_row)
            .map_err(|e| Error::Database(e.to_string()))?;
        rows.into_iter()
            .map(|r| r.map_err(|e| Error::Database(e.to_string())))
            .collect()
    }

    fn set_flag(&self, flag: &FeatureFlag) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO feature_flags (namespace, name, enabled, description, updated_at, stage, transience_class, channel, retire_at_stage) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
                 ON CONFLICT(namespace, name) DO UPDATE SET enabled=excluded.enabled, description=excluded.description, updated_at=excluded.updated_at, stage=excluded.stage, transience_class=excluded.transience_class, channel=excluded.channel, retire_at_stage=excluded.retire_at_stage",
                params![
                    flag.namespace,
                    flag.name,
                    flag.enabled as i32,
                    flag.description,
                    flag.updated_at.to_rfc3339(),
                    flag.stage,
                    flag.transience_class,
                    encode_channel(&flag.channel),
                    flag.retire_at_stage,
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

    fn promote_flag(&self, namespace: &str, name: &str, new_stage: &str, by: &str) -> Result<()> {
        let flag = self.get_flag(namespace, name)?;
        let current: Stage = flag.stage.parse()?;
        let target: Stage = new_stage.parse()?;

        if target.ordinal() <= current.ordinal() {
            return Err(Error::InvalidTransition(format!(
                "cannot move {} from {} to {} (stages must advance forward)",
                name, flag.stage, new_stage
            )));
        }

        let now = Utc::now();
        self.conn
            .execute(
                "UPDATE feature_flags SET stage=?1, updated_at=?2 WHERE namespace=?3 AND name=?4",
                params![new_stage, now.to_rfc3339(), namespace, name],
            )
            .map_err(|e| Error::Database(e.to_string()))?;

        self.conn
            .execute(
                "INSERT INTO stage_transitions (flag_name, from_stage, to_stage, transitioned_at, transitioned_by) VALUES (?1,?2,?3,?4,?5)",
                params![name, flag.stage, new_stage, now.to_rfc3339(), by],
            )
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    fn audit_flags(&self, namespace: &str) -> Result<Vec<FeatureFlag>> {
        // Return flags whose current stage ordinal >= their retire_at_stage ordinal
        let all = self.list_flags(namespace)?;
        Ok(all
            .into_iter()
            .filter(|f| {
                if let (Ok(current), Some(ref retire_str)) =
                    (f.stage.parse::<Stage>(), &f.retire_at_stage)
                {
                    if let Ok(retire) = retire_str.parse::<Stage>() {
                        return current.ordinal() >= retire.ordinal();
                    }
                }
                false
            })
            .collect())
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
        let mut stmt = self
            .conn
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Result<Database> {
        let path = std::path::PathBuf::from(":memory:");
        Database::open(&path)
    }

    #[test]
    fn test_database_open() {
        let db = test_db();
        assert!(db.is_ok());
    }

    #[test]
    fn test_set_and_get_config() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let entry = ConfigEntry {
            key: "api_url".to_string(),
            value: "https://api.example.com".to_string(),
            value_type: ValueType::String,
            namespace: "app".to_string(),
            updated_at: now,
            updated_by: "admin".to_string(),
        };

        db.set_config(&entry)?;
        let retrieved = db.get_config("app", "api_url")?;

        assert_eq!(retrieved.key, entry.key);
        assert_eq!(retrieved.value, entry.value);
        assert_eq!(retrieved.namespace, entry.namespace);
        Ok(())
    }

    #[test]
    fn test_get_nonexistent_config() -> Result<()> {
        let db = test_db()?;
        let result = db.get_config("app", "nonexistent");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_list_config() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();

        let entry1 = ConfigEntry {
            key: "key1".to_string(),
            value: "value1".to_string(),
            value_type: ValueType::String,
            namespace: "app".to_string(),
            updated_at: now,
            updated_by: "admin".to_string(),
        };

        let entry2 = ConfigEntry {
            key: "key2".to_string(),
            value: "value2".to_string(),
            value_type: ValueType::Int,
            namespace: "app".to_string(),
            updated_at: now,
            updated_by: "admin".to_string(),
        };

        db.set_config(&entry1)?;
        db.set_config(&entry2)?;

        let list = db.list_config("app")?;
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].key, "key1");
        assert_eq!(list[1].key, "key2");
        Ok(())
    }

    #[test]
    fn test_delete_config() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let entry = ConfigEntry {
            key: "to_delete".to_string(),
            value: "value".to_string(),
            value_type: ValueType::String,
            namespace: "app".to_string(),
            updated_at: now,
            updated_by: "admin".to_string(),
        };

        db.set_config(&entry)?;
        db.delete_config("app", "to_delete")?;

        let result = db.get_config("app", "to_delete");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_audit_log() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let entry = ConfigEntry {
            key: "config_key".to_string(),
            value: "value1".to_string(),
            value_type: ValueType::String,
            namespace: "app".to_string(),
            updated_at: now,
            updated_by: "admin".to_string(),
        };

        db.set_config(&entry)?;

        let log = db.audit_log("app", "config_key")?;
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].new_value, "value1");
        assert_eq!(log[0].old_value, None);
        Ok(())
    }

    #[test]
    fn test_set_and_get_flag() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let flag = FeatureFlag {
            name: "new_feature".to_string(),
            enabled: true,
            namespace: "app".to_string(),
            description: "A new feature".to_string(),
            updated_at: now,
            stage: "B".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: Some("GA".to_string()),
        };

        db.set_flag(&flag)?;
        let retrieved = db.get_flag("app", "new_feature")?;

        assert_eq!(retrieved.name, flag.name);
        assert_eq!(retrieved.enabled, flag.enabled);
        assert_eq!(retrieved.stage, flag.stage);
        Ok(())
    }

    #[test]
    fn test_list_flags() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();

        let flag1 = FeatureFlag {
            name: "feature1".to_string(),
            enabled: true,
            namespace: "app".to_string(),
            description: "Feature 1".to_string(),
            updated_at: now,
            stage: "A".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        };

        let flag2 = FeatureFlag {
            name: "feature2".to_string(),
            enabled: false,
            namespace: "app".to_string(),
            description: "Feature 2".to_string(),
            updated_at: now,
            stage: "SP".to_string(),
            transience_class: "C".to_string(),
            channel: vec!["staging".to_string()],
            retire_at_stage: None,
        };

        db.set_flag(&flag1)?;
        db.set_flag(&flag2)?;

        let list = db.list_flags("app")?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    #[test]
    fn test_delete_flag() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let flag = FeatureFlag {
            name: "to_delete".to_string(),
            enabled: true,
            namespace: "app".to_string(),
            description: "To delete".to_string(),
            updated_at: now,
            stage: "A".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        };

        db.set_flag(&flag)?;
        db.delete_flag("app", "to_delete")?;

        let result = db.get_flag("app", "to_delete");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_promote_flag() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let flag = FeatureFlag {
            name: "feature".to_string(),
            enabled: true,
            namespace: "app".to_string(),
            description: "A feature".to_string(),
            updated_at: now,
            stage: "A".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        };

        db.set_flag(&flag)?;
        db.promote_flag("app", "feature", "B", "admin")?;

        let promoted = db.get_flag("app", "feature")?;
        assert_eq!(promoted.stage, "B");
        Ok(())
    }

    #[test]
    fn test_promote_flag_backward_fails() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let flag = FeatureFlag {
            name: "feature".to_string(),
            enabled: true,
            namespace: "app".to_string(),
            description: "A feature".to_string(),
            updated_at: now,
            stage: "GA".to_string(),
            transience_class: "F".to_string(),
            channel: vec!["dev".to_string()],
            retire_at_stage: None,
        };

        db.set_flag(&flag)?;
        let result = db.promote_flag("app", "feature", "B", "admin");

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_set_and_get_secret() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let secret = SecretEntry {
            key: "db_password".to_string(),
            encrypted_value: vec![1, 2, 3, 4],
            nonce: vec![5, 6, 7, 8],
            updated_at: now,
        };

        db.set_secret(&secret)?;
        let retrieved = db.get_secret("db_password")?;

        assert_eq!(retrieved.key, secret.key);
        assert_eq!(retrieved.encrypted_value, secret.encrypted_value);
        Ok(())
    }

    #[test]
    fn test_list_secrets() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();

        let secret1 = SecretEntry {
            key: "secret1".to_string(),
            encrypted_value: vec![1, 2, 3],
            nonce: vec![4, 5, 6],
            updated_at: now,
        };

        let secret2 = SecretEntry {
            key: "secret2".to_string(),
            encrypted_value: vec![7, 8, 9],
            nonce: vec![10, 11, 12],
            updated_at: now,
        };

        db.set_secret(&secret1)?;
        db.set_secret(&secret2)?;

        let list = db.list_secrets()?;
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"secret1".to_string()));
        assert!(list.contains(&"secret2".to_string()));
        Ok(())
    }

    #[test]
    fn test_delete_secret() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let secret = SecretEntry {
            key: "to_delete".to_string(),
            encrypted_value: vec![1, 2, 3],
            nonce: vec![4, 5, 6],
            updated_at: now,
        };

        db.set_secret(&secret)?;
        db.delete_secret("to_delete")?;

        let result = db.get_secret("to_delete");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_set_and_get_version() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();
        let version = VersionInfo {
            repo: "my-repo".to_string(),
            our_version: "1.2.3".to_string(),
            upstream_version: "1.2.4".to_string(),
            synced_at: now,
        };

        db.set_version(&version)?;
        let retrieved = db.get_version("my-repo")?;

        assert_eq!(retrieved.repo, version.repo);
        assert_eq!(retrieved.our_version, version.our_version);
        Ok(())
    }

    #[test]
    fn test_list_versions() -> Result<()> {
        let db = test_db()?;
        let now = Utc::now();

        let version1 = VersionInfo {
            repo: "repo1".to_string(),
            our_version: "1.0.0".to_string(),
            upstream_version: "1.0.1".to_string(),
            synced_at: now,
        };

        let version2 = VersionInfo {
            repo: "repo2".to_string(),
            our_version: "2.0.0".to_string(),
            upstream_version: "2.0.1".to_string(),
            synced_at: now,
        };

        db.set_version(&version1)?;
        db.set_version(&version2)?;

        let list = db.list_versions()?;
        assert_eq!(list.len(), 2);
        Ok(())
    }
}
