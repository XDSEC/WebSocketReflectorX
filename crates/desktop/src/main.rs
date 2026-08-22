// Prevent console window in addition to the GPUI window in Windows release
// builds when, e.g., starting the app via file manager. Ignored on other
// platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use gpui::{App, Bounds, Size as GpuiSize, WindowBounds, WindowOptions, px};
use wsrx_desktop::{daemon, launcher, logging, ui::RootView};

fn main() -> Result<(), Box<dyn Error>> {
    // If another instance is already running, ask it to pop up and exit.
    // This must run first so a second instance never touches the running
    // instance's live log.
    if launcher::try_notify_existing_instance() {
        std::process::exit(0);
    }

    // Prepare the log directory: prune >3-day archives and archive any stale
    // `wsrx.log` from a crashed session, so this session starts clean.
    launcher::prepare_log_dir();

    // Initialize the logger.
    let (console_guard, file_guard) = logging::setup()?;

    // Install the crypto backend for rustls.
    daemon::setup_crypto();

    // Spawn the background daemon (API server, latency worker, ...).
    let (state, events_rx) = daemon::spawn_background();

    // Load persisted settings and scopes into shared state before the UI
    // reads them.
    daemon::load_persisted_state(&state);

    // Launch the GPUI application with the woocraft component library.
    gpui_platform::application()
        .with_assets(wsrx_desktop::assets::asset_source())
        .run(move |cx: &mut App| {
            woocraft::init(cx);
            cx.activate(true);

            // Open the main window.
            let bounds = Bounds::centered(None, GpuiSize::new(px(1080.), px(600.)), cx);
            let window = cx
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        titlebar: Some(woocraft::TitleBar::title_bar_options()),
                        window_min_size: Some(GpuiSize::new(px(800.), px(540.))),
                        #[cfg(target_os = "linux")]
                        window_background: gpui::WindowBackgroundAppearance::Transparent,
                        #[cfg(target_os = "linux")]
                        window_decorations: Some(gpui::WindowDecorations::Client),
                        ..Default::default()
                    },
                    |window, cx| RootView::view(window, cx, state.clone()),
                )
                .expect("failed to open main window");

            window
                .update(cx, |_, window, cx| {
                    window.activate_window();
                    window.set_window_title("WebSocket Reflector X");
                    window.on_window_should_close(cx, {
                        let state = state.clone();
                        move |_, cx| {
                            daemon::shutdown(&state);
                            cx.quit();
                            true
                        }
                    });
                })
                .expect("failed to update main window");

            // Background -> UI event pump.
            let window_handle = window;
            cx.spawn(async move |cx| {
                while let Ok(event) = events_rx.recv().await {
                    let _ = window_handle.update(cx, |root, window, cx| {
                        root.handle_event(event, window, cx);
                    });
                }
            })
            .detach();

            // Safety net: archive the session log and drop the lock even if
            // the window-close handler was bypassed. Idempotent with
            // daemon::shutdown.
            let quit_state = state.clone();
            cx.on_app_quit(move |_| {
                daemon::shutdown(&quit_state);
                async {}
            })
            .detach();
        });

    drop(file_guard);
    drop(console_guard);
    Ok(())
}
