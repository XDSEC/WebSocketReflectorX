#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::{
    App, AppContext, Application, AssetSource, AsyncApp, Bounds, SharedString, TitlebarOptions,
    WindowBounds, WindowDecorations, WindowKind, WindowOptions, point, px, size,
};

mod bridges;
mod components;
mod i18n;
mod icons;
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

/// Asset source that loads embedded SVG icons from binary
struct IconAssets;

impl AssetSource for IconAssets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Ok(icons::get_icon(path).map(|s| Cow::Borrowed(s.as_bytes())))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        // Return empty list - we don't need directory listing for embedded assets
        Ok(icons::list_icons())
    }
}

fn main() -> Result<()> {
    // Initialize logging with UI logger
    let (_console_guard, _file_guard, mut log_receiver) = logging::setup_with_ui()?;

    tracing::info!("Starting wsrx-desktop-gpui");

    // Initialize i18n with system locale
    i18n::init_locale();

    // Create and run the GPUI application with embedded assets
    Application::new()
        .with_assets(IconAssets)
        .run(|cx: &mut App| {
            // Create main window with centered bounds
            let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);

            // Platform-specific window configuration (following Zed's pattern)
            let titlebar_config = Some(TitlebarOptions {
                title: None, // Custom titlebar will show title
                appears_transparent: true,
                traffic_light_position: Some(point(px(9.0), px(9.0))),
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
                |window, cx| {
                    let root_view = cx.new(|cx| RootView::new(window, cx));
                    let root_view_ref = root_view.downgrade();
                    cx.spawn(move |async_app: &mut AsyncApp| {
                        let mut async_app = async_app.clone();
                        async move {
                            while let Some(log_entry) = log_receiver.recv().await {
                                let _ =
                                    root_view_ref.update(&mut async_app, |root, root_view_cx| {
                                        root.add_log(log_entry, root_view_cx);
                                    });
                            }
                        }
                    })
                    .detach();
                    root_view
                },
            )
            .expect("Failed to open window");

            tracing::info!("Application window created");

            cx.activate(true);
        });

    Ok(())
}
