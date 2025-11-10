// Events - Application event definitions for inter-component communication

use super::{Connection, LogEntry, Tunnel};

/// Events that can occur in the application
#[derive(Clone, Debug)]
pub enum AppEvent {
    /// Page navigation event
    NavigateToPage(super::app_state::Page),

    /// Tunnel-related events
    TunnelCreated(Tunnel),
    TunnelUpdated(Tunnel),
    TunnelDeleted(String), // tunnel_id
    TunnelEnabled(String),
    TunnelDisabled(String),

    /// Connection-related events
    ConnectionEstablished(Connection),
    ConnectionClosed(String), // connection_id
    ConnectionError {
        connection_id: String,
        error: String,
    },

    /// Daemon-related events
    DaemonStarted,
    DaemonStopped,
    DaemonError(String),

    /// Log events
    LogReceived(LogEntry),
    ClearLogs,

    /// Settings events
    SettingsUpdated,
    ThemeChanged,

    /// UI events
    ShowNotification {
        title: String,
        message: String,
    },
    ShowError(String),
}
