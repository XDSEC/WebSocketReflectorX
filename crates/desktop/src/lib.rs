pub mod bridges;
pub mod launcher;
pub mod logging;
pub mod server;
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub mod ui {
    slint::include_modules!();
}
