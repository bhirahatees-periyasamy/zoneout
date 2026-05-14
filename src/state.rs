use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct FocusState {
    pub enabled: bool,
    pub started_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub daemon_pid: u32,
}

pub fn state_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".focus").join("state.json")
}

pub fn load() -> Option<FocusState> {
    let path = state_path();
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save(state: &FocusState) -> std::io::Result<()> {
    let path = state_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(path, json)
}

pub fn clear() -> std::io::Result<()> {
    let path = state_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
