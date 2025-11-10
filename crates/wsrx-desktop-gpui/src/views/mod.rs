// Views - High-level UI components in GPUI
// Each view corresponds to a page/screen in the application

pub mod connections;
pub mod get_started;
pub mod network_logs;
pub mod root;
pub mod settings;
pub mod sidebar;

pub use connections::ConnectionsView;
pub use get_started::GetStartedView;
pub use network_logs::NetworkLogsView;
pub use root::RootView;
pub use settings::SettingsView;
pub use sidebar::SidebarView;
