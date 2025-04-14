use directories::ProjectDirs;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize the logger.
pub fn setup()
-> Result<(non_blocking::WorkerGuard, non_blocking::WorkerGuard), Box<dyn std::error::Error>> {
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            eprintln!("Unable to find project config directories");
            return Err("Unable to find project config directories".into());
        }
    };
    let log_dir = proj_dirs.data_local_dir().join("logs");
    std::fs::create_dir_all(&log_dir)?;
    let file_appender = rolling::RollingFileAppender::builder()
        .rotation(rolling::Rotation::NEVER)
        .filename_prefix("wsrx")
        .filename_suffix("log");
    let file_appender = file_appender.build(std::path::Path::new(&log_dir).canonicalize()?)?;

    let (non_blocking_file, file_guard) = non_blocking(file_appender);
    let (non_blocking_console, console_guard) = non_blocking(std::io::stdout());
    let file_log_layer = Layer::new()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .json();

    let console_log_layer = Layer::new()
        .with_writer(non_blocking_console)
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_log_layer)
        .with(console_log_layer)
        .init();

    Ok((console_guard, file_guard))
}
