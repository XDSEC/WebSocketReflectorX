use directories::ProjectDirs;
use slint::PlatformError;
use tracing::info;

use crate::{bridges, daemon, ui::MainWindow};

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;

    info!("Initialization started...");

    info!("Setting up data bridges...");
    bridges::setup(&ui);

    info!("Launching API server...");
    daemon::setup(&ui);

    info!("Initialization is finished.");
    info!("高性能ですから! (∠・ω< )⌒☆");

    Ok(ui)
}

pub fn shutdown() {
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            eprintln!("Unable to find project config directories");
            return;
        }
    };
    let log_dir = proj_dirs.data_local_dir().join("logs");
    std::fs::remove_dir_all(log_dir).unwrap_or_else(|_| {
        eprintln!("Failed to remove log directory");
    });
    std::process::exit(0);
}
