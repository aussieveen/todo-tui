mod app;
mod error;
mod events;
mod input;
mod persistence;
mod state;
mod sync;
mod ui;

use std::time::Duration;

use app::App;
use persistence::Persister;
use sync::{
    SyncParams, SyncStatus,
    auth::TokenStore,
    config::SyncConfig,
    drive::DriveClient,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let persister = Persister::new().map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
    let todo_dir = persister.dir().clone();

    let config = SyncConfig::load(&todo_dir);

    let sync_params = if config.drive.enabled {
        match init_sync(&config, &todo_dir).await {
            Ok(params) => Some(params),
            Err(e) => {
                eprintln!("Drive sync init failed: {e}");
                None
            }
        }
    } else {
        None
    };

    let terminal = ratatui::init();
    let mut app = App::new(persister, sync_params);

    if !config.drive.enabled {
        app.state.sync_status = SyncStatus::NotConfigured;
    }

    let result = app.run(terminal).await;
    ratatui::restore();

    result
}

async fn init_sync(config: &SyncConfig, todo_dir: &std::path::Path) -> Result<SyncParams, String> {
    let credentials_path = config.credentials_path(todo_dir);
    let token_path = todo_dir.join("token.json");

    if !credentials_path.exists() {
        eprintln!(
            "\n[Drive Sync] credentials.json not found at {}\n\
             To set up Google Drive sync:\n\
             1. Go to https://console.cloud.google.com\n\
             2. Create a project and enable the Google Drive API\n\
             3. Create OAuth 2.0 credentials (Desktop app type)\n\
             4. Download the JSON and save it as {}\n\
             5. Re-launch the app\n",
            credentials_path.display(),
            credentials_path.display(),
        );
        return Err("credentials.json not found".to_string());
    }

    let tokens = TokenStore::load(&credentials_path, &token_path)?;
    let mut drive = DriveClient::new(tokens);

    // Eagerly authenticate before ratatui starts so any browser-based OAuth
    // prompt can use the terminal normally.
    drive.ensure_authenticated().await?;

    let file_id = if !config.drive.file_id.is_empty() {
        config.drive.file_id.clone()
    } else {
        let id = drive.find_or_create_file().await?;
        // Persist the discovered file_id so we don't search on every launch.
        let mut updated = config.clone();
        updated.drive.file_id = id.clone();
        let _ = updated.save(todo_dir);
        id
    };

    Ok(SyncParams {
        drive,
        file_id,
        todo_dir: todo_dir.to_path_buf(),
        interval: Duration::from_secs(config.drive.sync_interval_secs),
    })
}
