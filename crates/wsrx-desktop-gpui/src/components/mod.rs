// Components - Reusable UI elements built with GPUI
// These are lower-level components used across different views

#![allow(dead_code)] // Components defined for future use

pub mod button;
pub mod checkbox;
pub mod icon_button;
pub mod input;
pub mod modal;
pub mod prelude;
pub mod select;
pub mod status_indicator;
pub mod tab_navigation;
pub mod title_bar;
pub mod traits;
pub mod window_controls;

// pub use title_bar::TitleBar;
pub use window_controls::WindowControls;
