use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, img, px};
use woocraft::{
    ActiveTheme, Button, ButtonVariants as _, CodeEditor, Disableable as _, DropdownMenu as _,
    Icon, IconName, PopupMenuItem, ScrollableElement as _, Selectable, h_flex, v_flex,
};

use crate::{daemon, i18n, ui::RootView};

/// Renders the settings page.
pub(crate) fn render_settings(
    _window: &mut Window, cx: &mut Context<RootView>, root: &mut RootView,
) -> impl IntoElement {
    let weak = cx.entity().downgrade();
    let state = root.state().clone();

    let muted_foreground = cx.theme().muted_foreground;
    let border = cx.theme().border;

    let has_updates = root.has_updates();
    let version = root.version().to_string();
    let language = root.settings().language.clone();
    let running_in_tray = root.settings().running_in_tray;
    let insecure_tls = root.settings().insecure_tls;
    let cursor = if root.cursor_visible() { "_" } else { " " };
    let info = root.info().to_string();

    v_flex()
        .gap_1()
        .px_8()
        .py_6()
        .overflow_y_scrollbar()
        .size_full()
        .child(
            // Header
            h_flex()
                .items_center()
                .gap_3()
                .pl_3()
                .child(img("logo.svg").size(px(48.)))
                .child(
                    v_flex()
                        .child(
                            div()

                                .font_weight(gpui::FontWeight::BOLD)
                                .child("WebSocket Reflector X"),
                        )
                        .child(
                            div()
                                .opacity(0.6)
                                .child(format!(
                                    "{}{cursor}",
                                    "Idealism is that you will never receive something back,\nbut nonetheless still decide to give."
                                )),
                        ),
                ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Version and updates
            settings_row(
                cx,
                i18n::t("Version and Updates"),
                Button::new("version")
                    .flat()
                    .icon(Icon::new(if has_updates {
                        IconName::CloudArrowUp
                    } else {
                        IconName::CheckmarkCircle
                    }))
                    .label(if has_updates {
                        format!("{version} {}", i18n::t("Update available"))
                    } else {
                        version
                    })
                    .selected(has_updates)
                    .disabled(!has_updates)
                    .on_click(|_, _, _| {
                        RootView::open_link(
                            "https://github.com/XDSEC/WebSocketReflectorX/releases",
                        );
                    }),
            ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Running in system tray when closed
            settings_row(
                cx,
                i18n::t("Running in system tray when closed"),
                Button::new("tray-toggle")
                    .flat()
                    .icon(Icon::new(if running_in_tray {
                        IconName::ToggleRight
                    } else {
                        IconName::ToggleLeft
                    }))
                    .label(if running_in_tray {
                        i18n::t("Enabled")
                    } else {
                        i18n::t("Disabled")
                    })
                    .on_click({
                        let weak = weak.clone();
                        let state = state.clone();
                        move |_, _, cx| {
                            let running = {
                                let mut settings = state.settings.blocking_write();
                                settings.running_in_tray = !settings.running_in_tray;
                                settings.running_in_tray
                            };
                            daemon::persist_settings_sync(&state);
                            if running {
                                if let Err(err) = crate::tray::enable(cx, &state) {
                                    tracing::error!("failed to enable system tray: {err}");
                                }
                            } else {
                                crate::tray::disable(cx);
                            }
                            let _ = weak.update(cx, |root, cx| {
                                root.settings.running_in_tray = running;
                                cx.notify();
                            });
                        }
                    }),
            ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Language / Locale
            settings_row(
                cx,
                i18n::t("Language / Locale"),
                Button::new("language-selector")
                    .flat()
                    .icon(Icon::new(IconName::LocalLanguage))
                    .label(language_display_name(&language))
                    .dropdown_menu({
                        let weak = weak.clone();
                        let state = state.clone();
                        move |menu, _, _| {
                            let mut menu = menu;
                            for (code, display) in [
                                ("en_US", "English"),
                                ("zh_CN", "简体中文"),
                                ("zh_TW", "繁體中文"),
                            ] {
                                let checked = code == language;
                                let weak = weak.clone();
                                let state = state.clone();
                                menu = menu.item(
                                    PopupMenuItem::new(display)
                                        .checked(checked)
                                        .on_click(move |_, _, cx| {
                                            i18n::set_locale(code);
                                            state
                                                .settings
                                                .blocking_write()
                                                .language = code.to_string();
                                            daemon::persist_settings_sync(&state);
                                            let _ = weak.update(cx, |root, cx| {
                                                root.settings.language = code.to_string();
                                                cx.notify();
                                            });
                                        }),
                                );
                            }
                            menu
                        }
                    }),
            ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Insecure TLS: unconditionally trust server certificates
            v_flex()
                .gap_1()
                .child(settings_row(
                    cx,
                    i18n::t("Allow insecure TLS connections"),
                    Button::new("insecure-tls-toggle")
                        .flat()
                        .icon(Icon::new(if insecure_tls {
                            IconName::ToggleRight
                        } else {
                            IconName::ToggleLeft
                        }))
                        .label(if insecure_tls {
                            i18n::t("Enabled")
                        } else {
                            i18n::t("Disabled")
                        })
                        .on_click({
                            let weak = weak.clone();
                            let state = state.clone();
                            move |_, _, cx| {
                                let enabled = {
                                    let mut settings = state.settings.blocking_write();
                                    settings.insecure_tls = !settings.insecure_tls;
                                    settings.insecure_tls
                                };
                                if enabled {
                                    tracing::warn!(
                                        "Insecure TLS is enabled by the user, \
                                         certificate verification is now disabled."
                                    );
                                }
                                daemon::persist_settings_sync(&state);
                                let _ = weak.update(cx, |root, cx| {
                                    root.settings.insecure_tls = enabled;
                                    cx.notify();
                                });
                            }
                        }),
                ))
                .child(
                    div()
                        .pl_3()
                        .pr_3()
                        .text_color(cx.theme().danger)
                        .opacity(0.85)
                        .child(i18n::t(
                            "WARNING: when enabled, wsrx will skip certificate verification and unconditionally trust any certificate for wss:// connections, including self-signed or forged ones. This makes you vulnerable to man-in-the-middle attacks. Only enable it when you know what you are doing.",
                        )),
                ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Export network logs
            settings_row(
                cx,
                i18n::t("Export network logs"),
                Button::new("export-logs")
                    .flat()
                    .icon(Icon::new(IconName::ArrowExport))
                    .label(i18n::t("Export"))
                    .on_click(|_, _, _| RootView::open_logs_dir()),
            ),
        )
        .child(div().h_px().bg(border))
        .child(
            // Support
            settings_row(
                cx,
                i18n::t("Have problems? Find support here."),
                Button::new("support")
                    .flat()
                    .icon(Icon::new(IconName::Question))
                    .label(i18n::t("Support"))
                    .on_click(|_, _, _| {
                        RootView::open_link(
                            "https://github.com/XDSEC/WebSocketReflectorX/issues",
                        );
                    }),
            ),
        )
        .child(div().h_px().bg(border))
        .child(
            // System information
            settings_row(
                cx,
                i18n::t("System information for bug reporting and debugging"),
                Button::new("copy-info")
                    .flat()
                    .icon(Icon::new(IconName::Copy))
                    .label(i18n::t("Copy"))
                    .on_click({
                        let info = info.clone();
                        move |_, _, cx| {
                            RootView::copy_to_clipboard(cx, &info);
                        }
                    }),
            ),
        )
        .child(
            CodeEditor::new(&root.info_editor)
                .h(px(160.))
                .w_full()
                .appearance(true)
                .bordered(true)
        )
        .child(div().h_6())
        .child(
            v_flex()
                .gap_1()
                .child(
                    div()

                        .text_color(muted_foreground)
                        .child(
                            "Powered by Reverier-Xu, with caffeine, a cat named 'dog', and love.",
                        ),
                )
                .child(
                    div()

                        .text_color(muted_foreground)
                        .child("(c) 2022 - 2025 XDSEC, distributed with MIT license."),
                ),
        )
}

/// A labeled settings row with an action control on the right.
fn settings_row(
    cx: &mut Context<RootView>, label: impl IntoElement, control: impl IntoElement,
) -> impl IntoElement {
    let theme = cx.theme();
    h_flex()
        .items_center()
        .gap_1()
        .child(div().flex_1().text_color(theme.foreground).child(label))
        .child(control)
}

fn language_display_name(language: &str) -> String {
    match language {
        "zh_CN" => "简体中文",
        "zh_TW" => "繁體中文",
        _ => "English",
    }
    .to_string()
}
