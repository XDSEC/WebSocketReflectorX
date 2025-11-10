// UI Logger - Custom tracing subscriber that sends logs to the UI
//
// This layer captures tracing events and forwards them to a channel
// that the UI can consume to display logs in real-time.

use std::sync::Arc;

use chrono::Local;
use tokio::sync::mpsc;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

use crate::models::LogEntry;

/// A tracing layer that sends log entries to the UI via a channel
pub struct UiLogLayer {
    sender: Arc<mpsc::UnboundedSender<LogEntry>>,
}

impl UiLogLayer {
    /// Create a new UI log layer
    /// Returns the layer and a receiver for consuming log entries
    pub fn new() -> (Self, mpsc::UnboundedReceiver<LogEntry>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let layer = Self {
            sender: Arc::new(sender),
        };
        (layer, receiver)
    }
}

impl<S> Layer<S> for UiLogLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Extract log level
        let level = match *event.metadata().level() {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        };

        // Extract target (module path)
        let target = event.metadata().target();

        // Format timestamp
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Extract message from event
        // Note: This is a simplified approach. For production, you'd want to
        // use a visitor pattern to properly extract all fields.
        let mut message = String::new();
        event.record(&mut MessageVisitor {
            message: &mut message,
        });

        // Create log entry
        let log_entry = LogEntry {
            timestamp,
            level: level.to_string(),
            target: target.to_string(),
            message,
        };

        // Send to UI (ignore errors if receiver is dropped)
        let _ = self.sender.send(log_entry);
    }
}

/// Visitor for extracting the message from a tracing event
struct MessageVisitor<'a> {
    message: &'a mut String,
}

impl<'a> tracing::field::Visit for MessageVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            *self.message = format!("{:?}", value);
            // Remove quotes added by Debug formatting
            if self.message.starts_with('"') && self.message.ends_with('"') {
                *self.message = self.message[1..self.message.len() - 1].to_string();
            }
        } else {
            // Append other fields to the message
            if !self.message.is_empty() {
                self.message.push_str(", ");
            }
            self.message.push_str(&format!("{}={:?}", field.name(), value));
        }
    }
}

/// Set up logging with UI layer
/// Returns guards for file and console loggers, plus a receiver for UI logs
pub fn setup_with_ui() -> anyhow::Result<(
    tracing_appender::non_blocking::WorkerGuard,
    tracing_appender::non_blocking::WorkerGuard,
    mpsc::UnboundedReceiver<LogEntry>,
)> {
    use std::fs;

    use directories::ProjectDirs;
    use tracing_appender::non_blocking;
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

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

    // UI logger
    let (ui_layer, ui_receiver) = UiLogLayer::new();

    // Set up the subscriber with console, file, and UI output
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
        .with(ui_layer)
        .init();

    tracing::info!("Logging initialized for wsrx-desktop-gpui with UI layer");

    Ok((console_guard, file_guard, ui_receiver))
}
