use async_compat::Compat;
use directories::ProjectDirs;
use slint::PlatformError;
use tracing::info;

use crate::{bridges, daemon, ui::MainWindow};

pub fn setup() -> Result<MainWindow, PlatformError> {
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            eprintln!("Unable to find project config directories");
            return Err(PlatformError::Other(
                "Unable to find project config directories".to_string(),
            ));
        }
    };
    let lock_file = proj_dirs.data_local_dir().join(".rx.is.alive");
    if lock_file.exists() {
        eprintln!("Another instance of the application is already running.");
        let api_port = std::fs::read_to_string(&lock_file).unwrap_or_else(|_| {
            eprintln!("Failed to read lock file");
            std::fs::remove_file(&lock_file).unwrap_or_else(|_| {
                eprintln!("Failed to remove lock file");
            });
            std::process::exit(1);
        });
        eprintln!("Notify the other instance to raise...");
        slint::spawn_local(Compat::new(async move {})).expect("Failed to spawn thread");
        let client = reqwest::blocking::Client::new();
        match client
            .post(format!("http://127.0.0.1:{api_port}/popup"))
            .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
            .send()
        {
            Ok(_) => {
                eprintln!("Notification sent.");
            }
            Err(e) => {
                eprintln!("Failed to send notification: {e}, removing lock file.");
                std::fs::remove_file(&lock_file).unwrap_or_else(|_| {
                    eprintln!("Failed to remove lock file");
                });
            }
        }
        std::process::exit(0);
    }

    let ui = MainWindow::new()?;

    info!("WSRX initialization started...");

    info!("Setting up data bridges...");
    bridges::setup(&ui);
    bridges::settings::load_config(&ui);

    info!("Launching API server...");
    daemon::setup(&ui);

    info!("Initialization is finished.");
    info!("高性能ですから! (∠・ω< )⌒☆");

    Ok(ui)
}

pub fn shutdown(ui: &slint::Weak<MainWindow>) {
    let window = ui.upgrade().unwrap();
    bridges::settings::save_config(&window);
    daemon::save_scopes(ui);
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
    let lock_file = proj_dirs.data_local_dir().join(".rx.is.alive");
    std::fs::remove_file(lock_file).unwrap_or_else(|_| {
        eprintln!("Failed to remove lock file");
    });
    std::process::exit(0);
}
