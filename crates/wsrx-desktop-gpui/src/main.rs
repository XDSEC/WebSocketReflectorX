#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::{
    App, AppContext, Application, AssetSource, Bounds, SharedString, TitlebarOptions,
    WindowBounds, WindowDecorations, WindowKind, WindowOptions, point, px, size,
};

mod bridges;
mod components;
mod i18n;
mod icons;
mod logging;
mod models;
mod styles;
mod ui_logger;
mod views;

// Initialize i18n at crate root with TOML locale files
// The path is relative to CARGO_MANIFEST_DIR (crate root)

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

// Include generated constants from build.rs
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use views::RootView;

/// Asset source that loads embedded SVG icons from binary
struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        // Handle icon paths like "icons/home.svg"
        if let Some(icon_name) = path.strip_prefix("icons/").and_then(|p| p.strip_suffix(".svg")) {
            if let Some(svg_content) = icons::get_icon(icon_name) {
                return Ok(Some(std::borrow::Cow::Borrowed(svg_content.as_bytes())));
            }
        }
        Ok(None)
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        // Return empty list - we don't need directory listing for embedded assets
        Ok(Vec::new())
    }
}

fn main() -> Result<()> {
    // Initialize logging
    let (_console_guard, _file_guard) = logging::setup()?;

    // Initialize i18n with system locale
    i18n::init_locale();

    // Create and run the GPUI application with embedded assets
    Application::new()
        .with_assets(EmbeddedAssets)
        .run(|cx: &mut App| {
        // Create main window with centered bounds
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);

        // Platform-specific window configuration (following Zed's pattern)
        let titlebar_config = Some(TitlebarOptions {
            title: None, // Custom titlebar will show title
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
            ..Default::default()
        });

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: titlebar_config,
                window_decorations: Some(WindowDecorations::Client), // Client-side decorations
                kind: WindowKind::Normal,
                is_movable: true,
                focus: true,
                show: true,
                window_min_size: Some(gpui::Size {
                    width: px(800.0),
                    height: px(600.0),
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| RootView::new(window, cx)),
        )
        .expect("Failed to open window");

        cx.activate(true);
    });

    Ok(())
}
