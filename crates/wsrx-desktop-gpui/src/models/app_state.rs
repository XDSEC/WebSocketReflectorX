// Application State - Global application state management

#![allow(dead_code)] // State methods defined for future use

use std::collections::VecDeque;

use super::{Connection, LogEntry, Settings, Tunnel};

/// Current active page in the application
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    GetStarted,
    Connections,
    NetworkLogs,
    Settings,
}

/// Daemon connection status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DaemonStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

/// Global application state
/// This struct holds the main application data that is shared across views
pub struct AppState {
    /// Currently active page
    pub current_page: Page,

    /// List of configured tunnels
    pub tunnels: Vec<Tunnel>,

    /// Active connections
    pub connections: Vec<Connection>,

    /// Application settings
    pub settings: Settings,

    /// Recent log entries (circular buffer)
    pub recent_logs: VecDeque<LogEntry>,

    /// Maximum number of logs to keep in memory
    pub max_logs: usize,

    /// Current daemon status
    pub daemon_status: DaemonStatus,
}

impl AppState {
    /// Create a new AppState with default values
    pub fn new() -> Self {
        Self {
            current_page: Page::GetStarted,
            tunnels: Vec::new(),
            connections: Vec::new(),
            settings: Settings::default(),
            recent_logs: VecDeque::new(),
            max_logs: 10000,
            daemon_status: DaemonStatus::Stopped,
        }
    }

    /// Add a log entry, removing oldest if over capacity
    pub fn add_log(&mut self, entry: LogEntry) {
        if self.recent_logs.len() >= self.max_logs {
            self.recent_logs.pop_front();
        }
        self.recent_logs.push_back(entry);
    }

    /// Clear all logs
    pub fn clear_logs(&mut self) {
        self.recent_logs.clear();
    }

    /// Add or update a tunnel
    pub fn upsert_tunnel(&mut self, tunnel: Tunnel) {
        if let Some(pos) = self.tunnels.iter().position(|t| t.id == tunnel.id) {
            self.tunnels[pos] = tunnel;
        } else {
            self.tunnels.push(tunnel);
        }
    }

    /// Remove a tunnel by ID
    pub fn remove_tunnel(&mut self, tunnel_id: &str) {
        self.tunnels.retain(|t| t.id != tunnel_id);
    }

    /// Get a tunnel by ID
    pub fn get_tunnel(&self, tunnel_id: &str) -> Option<&Tunnel> {
        self.tunnels.iter().find(|t| t.id == tunnel_id)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
