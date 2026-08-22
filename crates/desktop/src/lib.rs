pub mod assets;
pub mod daemon;
pub mod i18n;
pub mod launcher;
pub mod logging;
pub mod models;
pub mod ui;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));
