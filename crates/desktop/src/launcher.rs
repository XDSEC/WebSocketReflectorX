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

/// Prepares the log directory for a new session: prunes archived logs older
/// than 3 days and archives any `wsrx.log` left over from a previous (crashed)
/// session, so the current session always starts with a clean log.
///
/// Must be called before the logger is initialized and only after the
/// single-instance check (otherwise it would archive the running instance's
/// live log).
pub fn prepare_log_dir() {
    let dirs = project_dirs();
    let log_dir = dirs.data_local_dir().join("logs");
    if std::fs::create_dir_all(&log_dir).is_err() {
        return;
    }

    // Prune archived logs older than 3 days.
    let cutoff = std::time::SystemTime::now()
        .checked_sub(std::time::Duration::from_secs(3 * 24 * 60 * 60))
        .unwrap_or(std::time::UNIX_EPOCH);
    if let Ok(entries) = std::fs::read_dir(&log_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("log") {
                continue;
            }
            let stale = entry
                .metadata()
                .ok()
                .and_then(|meta| meta.modified().ok())
                .is_some_and(|modified| modified < cutoff);
            if stale {
                std::fs::remove_file(&path).unwrap_or_else(|err| {
                    eprintln!("Failed to remove stale log {}: {err}", path.display());
                });
            }
        }
    }

    // Archive any leftover live log from a crashed session.
    archive_current_log();
}

/// Archives the current session's `wsrx.log` to a timestamped file so the
/// live file stays scoped to a single run. Idempotent: a no-op when there is
/// no live log.
pub fn archive_current_log() {
    let dirs = project_dirs();
    let log_dir = dirs.data_local_dir().join("logs");
    let live = log_dir.join("wsrx.log");
    if !live.exists() {
        return;
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let archive = log_dir.join(format!("wsrx-{timestamp}.log"));
    if let Err(err) = std::fs::rename(&live, &archive) {
        eprintln!("Failed to archive log {}: {err}", live.display());
    }
}

/// Removes the single-instance lock file if present.
pub fn remove_lock() {
    remove_lock_file(lock_file_path().as_path());
}
