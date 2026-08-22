use std::{path::Path, time::Duration};

use directories::ProjectDirs;
use tracing::error;

/// Returns the `org.xdsec.wsrx` project directories, exiting with an error
/// message when the platform cannot provide them.
pub fn project_dirs() -> ProjectDirs {
    match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            eprintln!("Unable to find project config directories");
            std::process::exit(1);
        }
    }
}

/// The lock file used for single-instance detection. Contains the API port.
pub fn lock_file_path() -> std::path::PathBuf {
    project_dirs().data_local_dir().join(".rx.is.alive")
}

/// Writes the lock file with the API server port.
pub fn write_lock_file(port: u16) {
    let lock_file = lock_file_path();
    std::fs::write(&lock_file, port.to_string()).unwrap_or_else(|err| {
        error!("Failed to write lock file {}: {err}", lock_file.display());
        std::process::exit(1);
    });
}

/// Removes the lock file, ignoring "not found" errors.
pub fn remove_lock_file(lock_file: &Path) {
    std::fs::remove_file(lock_file).unwrap_or_else(|err| {
        if err.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove lock file: {err}");
        }
    });
}

/// Checks whether another wsrx instance is already running. When one is found,
/// it is notified to pop up its window and this instance should exit.
///
/// Returns `true` when the running instance was notified successfully.
pub fn try_notify_existing_instance() -> bool {
    let lock_file = lock_file_path();
    if !lock_file.exists() {
        return false;
    }

    eprintln!("Detected existing instance lock file. Trying to notify running app...");

    let Some(api_port) = read_lock_file_port(&lock_file) else {
        return false;
    };

    match notify_existing_instance(api_port) {
        Ok(()) => {
            eprintln!("Notification sent.");
            true
        }
        Err(err) => {
            eprintln!("Failed to notify existing app: {err}. Removing stale lock file.");
            remove_lock_file(&lock_file);
            false
        }
    }
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

/// Removes runtime files (logs, lock file) that should not outlive the app.
pub fn cleanup_runtime_files() {
    let dirs = project_dirs();
    let data_local_dir = dirs.data_local_dir().to_path_buf();

    let log_dir = data_local_dir.join("logs");
    std::fs::remove_dir_all(&log_dir).unwrap_or_else(|_| {
        eprintln!("Failed to remove log directory");
    });

    remove_lock_file(lock_file_path().as_path());
}
