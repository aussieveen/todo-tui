pub mod auth;
pub mod config;
pub mod drive;
pub mod task;

use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Utc};

use self::drive::DriveClient;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SyncStatus {
    NotConfigured,
    Idle,
    Syncing,
    Synced(DateTime<Utc>),
    Offline,
    Error(String),
}

/// All parameters needed to spawn the background sync task, gathered during startup.
pub struct SyncParams {
    pub drive: DriveClient,
    pub file_id: String,
    pub todo_dir: PathBuf,
    pub interval: Duration,
}
