// Logging setup for wsrx-desktop-gpui
use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use tracing_appender::non_blocking;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn setup() -> Result<(
    tracing_appender::non_blocking::WorkerGuard,
    tracing_appender::non_blocking::WorkerGuard,
)> {
    // Get platform-specific directories
    let proj_dirs = ProjectDirs::from("org", "xdsec", "wsrx-desktop-gpui")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    let log_dir = proj_dirs.cache_dir();
    fs::create_dir_all(log_dir)?;

    // Console logger
    let (console_non_blocking, console_guard) = non_blocking(std::io::stderr());

    // File logger
    let file_appender = tracing_appender::rolling::daily(log_dir, "wsrx-desktop-gpui.log");
    let (file_non_blocking, file_guard) = non_blocking(file_appender);

    // Set up the subscriber with both console and file output
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(console_non_blocking)
                .with_filter(EnvFilter::from_default_env()),
        )
        .with(
            fmt::layer()
                .json()
                .with_writer(file_non_blocking)
                .with_filter(EnvFilter::from_default_env()),
        )
        .init();

    tracing::info!("Logging initialized for wsrx-desktop-gpui");

    Ok((console_guard, file_guard))
}
