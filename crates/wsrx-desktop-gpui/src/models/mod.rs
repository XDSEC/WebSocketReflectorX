// Models - Data structures for the application
// This module contains all the data models used throughout the application

#![allow(dead_code)] // Models defined for future use

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub mod app_state;
pub mod events;

/// Represents a WebSocket tunnel configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tunnel {
    pub id: String,
    pub name: String,
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub enabled: bool,
}

/// Represents an active connection
#[derive(Clone, Debug)]
pub struct Connection {
    pub id: String,
    pub tunnel_id: String,
    pub status: ConnectionStatus,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Pending,
    Connected,
    Disconnected,
    Error,
}

/// Represents a log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Application settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub daemon_auto_start: bool,
    pub logging_level: String,
    pub show_network_logs: bool,
    pub theme: Theme,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            daemon_auto_start: true,
            logging_level: "info".to_string(),
            show_network_logs: true,
            theme: Theme::Auto,
        }
    }
}
