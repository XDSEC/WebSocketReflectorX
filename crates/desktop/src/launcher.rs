use slint::{ComponentHandle, PlatformError};
use tracing::info;

use crate::{bridges, server, ui::MainWindow};

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;

    info!("Initialization started...");

    info!("Setting up data bridges...");
    bridges::setup(&ui);

    info!("Launching API server...");
    server::setup(&ui);

    info!("Initialization is finished.");
    info!("高性能ですから! (∠・ω< )⌒☆");
    ui.run()?;
    Ok(ui)
}
