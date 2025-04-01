pub mod bridges;
pub mod main_window;
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub mod ui {
    slint::include_modules!();
}
