//! System tray integration for the desktop app.
//!
//! When the "running in tray" setting is enabled the window minimizes to the
//! tray on close; the tray menu offers Show / Quit and tray clicks pop the
//! window back up.

static TRAY_THREAD: OnceLock<()> = OnceLock::new();

use std::sync::OnceLock;

use gpui::{App, WindowHandle};
use woocraft::{
    Tray, TrayAppContext, TrayEvent, TrayMenuItem, TrayMouseButton, Error as TrayError,
};

use crate::{
    daemon::{ServerState, UiEvent},
    i18n,
    ui::RootView,
};

/// Enables the tray icon. Idempotent: the event-forwarding thread is started
/// once for the app lifetime.
pub fn enable(cx: &mut App, state: &ServerState) -> Result<(), TrayError> {
    let logo = include_bytes!("../assets/logo.png").to_vec();
    let tray = Tray::new()
        .tooltip("WebSocket Reflector X")
        .icon_bytes(logo)
        .menu(vec![
            TrayMenuItem::action("show", i18n::t("Show")),
            TrayMenuItem::separator(),
            TrayMenuItem::action("quit", i18n::t("Quit")),
        ]);

    cx.set_tray(tray)?;

    if let Some(events_rx) = woocraft::tray_events(cx) {
        let events = state.events.clone();
        TRAY_THREAD.get_or_init(|| {
            std::thread::Builder::new()
                .name("wsrx-tray-events".to_string())
                .spawn(move || {
                    while let Ok(event) = events_rx.recv() {
                        let ui_event = match event {
                            TrayEvent::MenuClicked { id } => match id.as_str() {
                                "show" => UiEvent::Popup,
                                "quit" => UiEvent::Quit,
                                _ => continue,
                            },
                            TrayEvent::Click {
                                button: TrayMouseButton::Left,
                                ..
                            } => UiEvent::Popup,
                            _ => continue,
                        };
                        if events.send_blocking(ui_event).is_err() {
                            break;
                        }
                    }
                })
                .expect("failed to spawn tray event thread");
        });
    }

    Ok(())
}

/// Disables the tray icon.
pub fn disable(cx: &mut App) {
    let _ = cx.remove_tray();
}

/// Restores and focuses the main window from the tray.
pub fn popup(window: &WindowHandle<RootView>, cx: &mut App) {
    let _ = window.update(cx, |_, window, _| {
        window.activate_window();
    });
}
