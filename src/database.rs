use once_cell::sync::Lazy;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::fingerprint;

/// Global thread-safe SQLite connectionpool/mutex.
static DB_CONN: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

/// Final result structure stored in the database history.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScanHistory {
    /// Database unique ID.
    pub id: i64,
    /// Timestamp of the scan.
    pub timestamp: String,
    /// Targeted IP or Hostname.
    pub target: String,
    /// Summary or serialized result array.
    pub results: String,
}

/// Initializes the SQLite database if it hasn't been already.
///
/// Creates the `history` table for storing persistent scan results.
pub fn init_db() -> anyhow::Result<()> {
    let conn = Connection::open("aetheris_history.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            target TEXT NOT NULL,
            results TEXT NOT NULL
        )",
        [],
    )?;

    let mut db = DB_CONN.lock().unwrap();
    *db = Some(conn);
    Ok(())
}

/// Persists a scan result list to the history table.
pub fn save_scan(target: &str, results: &[fingerprint::FingerprintResult]) -> anyhow::Result<()> {
    let db = DB_CONN.lock().unwrap();
    if let Some(conn) = db.as_ref() {
        let results_json = serde_json::to_string(results)?;
        conn.execute(
            "INSERT INTO history (target, results) VALUES (?, ?)",
            params![target, results_json],
        )?;
    }
    Ok(())
}

/// Retrieves all previous scan sessions from the database.
pub fn get_history() -> anyhow::Result<Vec<ScanHistory>> {
    let db = DB_CONN.lock().unwrap();
    if let Some(conn) = db.as_ref() {
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, target, results FROM history ORDER BY id DESC LIMIT 50",
        )?;
        let history_iter = stmt.query_map([], |row| {
            Ok(ScanHistory {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                target: row.get(2)?,
                results: row.get(3)?,
            })
        })?;

        let mut results = vec![];
        for h in history_iter {
            results.push(h?);
        }
        Ok(results)
    } else {
        Ok(vec![])
    }
}
