use std::{path::Path, time::Duration};

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

    if lock_file.exists() && try_focus_existing_instance(&lock_file) {
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

fn try_focus_existing_instance(lock_file: &Path) -> bool {
    eprintln!("Detected existing instance lock file. Trying to notify running app...");

    let Some(api_port) = read_lock_file_port(lock_file) else {
        return false;
    };

    match notify_existing_instance(api_port) {
        Ok(()) => {
            eprintln!("Notification sent.");
            true
        }
        Err(err) => {
            eprintln!("Failed to notify existing app: {err}. Removing stale lock file.");
            remove_lock_file(lock_file);
            false
        }
    }
}

fn remove_lock_file(lock_file: &Path) {
    std::fs::remove_file(lock_file).unwrap_or_else(|err| {
        if err.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove lock file: {err}");
        }
    });
}

fn read_lock_file_port(lock_file: &Path) -> Option<u16> {
    match std::fs::read_to_string(lock_file) {
        Ok(port) => match port.trim().parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                eprintln!("Invalid lock file content: {err}. Removing stale lock file.");
                remove_lock_file(lock_file);
                None
            }
        },
        Err(err) => {
            eprintln!("Failed to read lock file: {err}. Removing stale lock file.");
            remove_lock_file(lock_file);
            None
        }
    }
}

fn notify_existing_instance(api_port: u16) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_else(|err| {
            eprintln!(
                "Failed to create loopback HTTP client: {err}. Falling back to default client."
            );
            reqwest::blocking::Client::new()
        });

    let response = client
        .post(format!("http://127.0.0.1:{api_port}/popup"))
        .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
        .send()
        .map_err(|err| err.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("unexpected response status {}", response.status()))
    }
}

pub fn cleanup_runtime_state(ui: &slint::Weak<MainWindow>) {
    if let Some(window) = ui.upgrade() {
        bridges::settings::save_config(&window);
        daemon::save_scopes(ui);
    }

    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            eprintln!("Unable to find project config directories");
            return;
        }
    };

    cleanup_runtime_files(proj_dirs.data_local_dir());
}

pub fn shutdown(ui: &slint::Weak<MainWindow>) {
    cleanup_runtime_state(ui);
    std::process::exit(0);
}

fn cleanup_runtime_files(data_local_dir: &Path) {
    let log_dir = data_local_dir.join("logs");
    std::fs::remove_dir_all(log_dir).unwrap_or_else(|_| {
        eprintln!("Failed to remove log directory");
    });

    let lock_file = data_local_dir.join(".rx.is.alive");
    remove_lock_file(&lock_file);
}
