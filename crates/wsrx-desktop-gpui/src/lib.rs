// wsrx-desktop-gpui lib root

#[macro_use]
extern crate rust_i18n;

// Initialize i18n at crate root with TOML locale files
// The path is relative to CARGO_MANIFEST_DIR (crate root)
i18n!("locales", fallback = "en");

pub mod bridges;
pub mod components;
pub mod i18n;
pub mod logging;
pub mod models;
pub mod styles;
pub mod views;

// Include generated constants from build.rs
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

// Re-export commonly used types
pub use components::*;
pub use models::*;
pub use styles::*;
pub use views::*;
