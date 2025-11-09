// wsrx-desktop-gpui lib root
pub mod bridges;
pub mod components;
pub mod logging;
pub mod models;
pub mod styles;
pub mod views;

// Include generated constants from build.rs
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

// Re-export commonly used types
pub use models::*;
pub use views::*;
pub use components::*;
pub use styles::*;
