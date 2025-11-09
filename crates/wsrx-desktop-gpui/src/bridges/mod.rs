// Bridges - Integration layer between UI and core functionality
// This module contains the bridges that connect the UI to the wsrx daemon and other services

pub mod daemon;
pub mod settings;
pub mod system_info;

pub use daemon::DaemonBridge;
pub use settings::SettingsBridge;
pub use system_info::SystemInfoBridge;
