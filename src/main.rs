mod app;
mod error;
mod events;
mod input;
mod persistence;
mod state;
mod ui;

use app::App;
use persistence::Persister;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let persister = Persister::new().map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

    let terminal = ratatui::init();
    let result = App::new(persister).run(terminal).await;
    ratatui::restore();

    result
}
