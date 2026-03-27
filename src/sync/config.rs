use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub drive: DriveConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveConfig {
    pub enabled: bool,
    pub sync_interval_secs: u64,
    /// Cached Drive file ID, populated after first file discovery.
    pub file_id: String,
    /// Path to credentials.json, relative to ~/.todo/
    pub credentials_path: String,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            drive: DriveConfig {
                enabled: false,
                sync_interval_secs: 30,
                file_id: String::new(),
                credentials_path: "credentials.json".to_string(),
            },
        }
    }
}

impl SyncConfig {
    pub fn load(todo_dir: &Path) -> Self {
        let path = todo_dir.join("config.toml");
        let Ok(content) = fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self, todo_dir: &Path) -> Result<(), String> {
        let path = todo_dir.join("config.toml");
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn credentials_path(&self, todo_dir: &Path) -> PathBuf {
        let p = Path::new(&self.drive.credentials_path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            todo_dir.join(p)
        }
    }
}
