#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::{
    App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};

mod bridges;
mod components;
mod i18n;
mod logging;
mod models;
mod styles;
mod views;

// Initialize i18n at crate root with TOML locale files
// The path is relative to CARGO_MANIFEST_DIR (crate root)

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

// Include generated constants from build.rs
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use views::RootView;

fn main() -> Result<()> {
    // Initialize logging
    let (_console_guard, _file_guard) = logging::setup()?;

    // Initialize i18n with system locale
    i18n::init_locale();

    // Create and run the GPUI application
    Application::new().run(|cx: &mut App| {
        // Create main window with centered bounds
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);

        // Platform-specific titlebar configuration
        #[cfg(target_os = "macos")]
        let titlebar_config = Some(TitlebarOptions {
            title: Some("WebSocket Reflector X".into()),
            appears_transparent: true,
            traffic_light_position: Some(gpui::point(px(16.0), px(18.0))),
            ..Default::default()
        });

        #[cfg(not(target_os = "macos"))]
        let titlebar_config = None;

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: titlebar_config,
                focus: true,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| RootView::new(window, cx)),
        )
        .expect("Failed to open window");

        cx.activate(true);
    });

    Ok(())
}
