// Events - Application event definitions for inter-component communication

use super::{Instance, LogEntry, Scope};

/// Events that can occur in the application
#[derive(Clone, Debug)]
pub enum AppEvent {
    /// Page navigation event
    NavigateToPage(super::app_state::Page),

    /// Instance (tunnel) related events
    InstanceCreated(Instance),
    InstanceUpdated(Instance),
    InstanceDeleted(String), // local address

    /// Scope-related events
    ScopeAdded(Scope),
    ScopeUpdated(Scope),
    ScopeRemoved(String),  // host
    ScopeAllowed(String),  // host
    ScopeDeclined(String), // host

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
