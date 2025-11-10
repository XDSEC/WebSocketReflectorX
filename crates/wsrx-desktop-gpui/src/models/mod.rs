// Models - Data structures for the application
// This module contains all the data models used throughout the application
// Architecture: Scope-centric (not page-centric)

#![allow(dead_code)] // Models defined for future use

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod app_state;
pub mod events;

/// Represents a tunnel instance (connection between local and remote)
/// Called "Instance" in original Slint code
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// Display label for the tunnel
    pub label: String,
    /// Remote WebSocket address (ws:// or wss://)
    pub remote: String,
    /// Local address (IP:port)
    pub local: String,
    /// Latency in milliseconds (-1 if not connected)
    pub latency: i32,
    /// Which scope owns this tunnel
    pub scope_host: String,
}

/// Represents a scope (domain that can control tunnels)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scope {
    /// Unique identifier (domain name)
    pub host: String,
    /// Display name for the scope
    pub name: String,
    /// Current state: "pending", "allowed", "syncing"
    pub state: String,
    /// Comma-separated feature flags: "basic", "pingfall"
    pub features: String,
    /// Feature-specific settings (JSON)
    #[serde(default)]
    pub settings: HashMap<String, Value>,
}

/// Represents a log entry from tracing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp string (e.g., "2025-11-10 15:30:45")
    pub timestamp: String,
    /// Log level: "DEBUG", "INFO", "WARN", "ERROR"
    pub level: String,
    /// Module/target name (e.g., "wsrx::tunnel")
    pub target: String,
    /// Log message
    pub message: String,
}

impl LogEntry {
    pub fn level_color(&self) -> gpui::Rgba {
        match self.level.as_str() {
            "DEBUG" => gpui::rgba(0x888888FF),
            "INFO" => gpui::rgba(0x5DADE2FF),
            "WARN" => gpui::rgba(0xF39C12FF),
            "ERROR" => gpui::rgba(0xE74C3CFF),
            _ => gpui::rgba(0xAAAAAAFF),
        }
    }

    pub fn opacity(&self) -> f32 {
        match self.level.as_str() {
            "DEBUG" => 0.5,
            "INFO" => 0.8,
            _ => 1.0,
        }
    }
}

/// Application settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub language: String,
    pub running_in_tray: bool,
    pub api_port: u16,
    #[serde(default)]
    pub online: bool,
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
            theme: Theme::Auto,
            language: "en".to_string(),
            running_in_tray: false,
            api_port: 7609,
            online: false,
        }
    }
}
