use rusqlite::{params, Connection, Result};
use crate::fingerprint::FingerprintResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct ScanHistory {
    pub id: i32,
    pub target: String,
    pub timestamp: String,
    pub results: Vec<FingerprintResult>,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("secops_history.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scans (
            id INTEGER PRIMARY KEY,
            target TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            results_json TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn save_scan(target: &str, results: &[FingerprintResult]) -> Result<()> {
    let conn = init_db()?;
    let results_json = serde_json::to_string(results).unwrap_or_else(|_| "[]".to_string());
    
    conn.execute(
        "INSERT INTO scans (target, results_json) VALUES (?1, ?2)",
        params![target, results_json],
    )?;
    
    Ok(())
}

pub fn get_history() -> Result<Vec<ScanHistory>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT id, target, timestamp, results_json FROM scans ORDER BY timestamp DESC LIMIT 20")?;
    
    let history_iter = stmt.query_map([], |row| {
        let results_json: String = row.get(3)?;
        let results: Vec<FingerprintResult> = serde_json::from_str(&results_json).unwrap_or_default();
        
        Ok(ScanHistory {
            id: row.get(0)?,
            target: row.get(1)?,
            timestamp: row.get(2)?,
            results,
        })
    })?;
    
    let mut history = vec![];
    for h in history_iter {
        history.push(h?);
    }
    
    Ok(history)
}
