pub mod bridges;
pub mod logging;
pub mod main_window;
pub mod server;
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub mod ui {
    slint::include_modules!();
}
