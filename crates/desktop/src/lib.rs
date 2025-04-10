pub mod bridges;
pub mod daemon;
pub mod launcher;
pub mod logging;
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub mod ui {
    slint::include_modules!();
}
