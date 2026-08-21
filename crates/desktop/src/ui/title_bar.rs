use gpui::{App, ClickEvent, Context, Entity, IntoElement, Window};
use woocraft::{ActiveTheme, Icon, IconName, Theme, ThemeMode, TitleBar};

use crate::{daemon::ServerState, i18n, ui::RootView};

/// Renders the woocraft title bar with persisted theme / language handlers.
pub(crate) fn render_title_bar(
    _window: &mut Window,
    _cx: &mut Context<RootView>,
    this: &Entity<RootView>,
    state: &ServerState,
) -> impl IntoElement {
    let weak = this.downgrade();
    let settings_arc = state.settings.clone();

    TitleBar::new()
        .title("WebSocket Reflector X")
        .icon(Icon::new(IconName::GlobeStar))
        .theme_button(true)
        .language_button(true)
        .on_theme_button_click({
            let weak = weak.clone();
            let settings_arc = settings_arc.clone();
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let next = if cx.theme().mode.is_dark() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                Theme::set_mode(next, cx);
                let theme_str = if next.is_dark() { "dark" } else { "light" };
                settings_arc.blocking_write().theme = theme_str.to_string();
                let _ = weak.update(cx, |root, cx| {
                    root.settings.theme = theme_str.to_string();
                    cx.notify();
                });
            }
        })
        .on_language_button_click({
            let weak = weak.clone();
            let settings_arc = settings_arc.clone();
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let woocraft_locale = woocraft::locale();
                let locale = normalize_woocraft_locale(&woocraft_locale);
                i18n::set_locale(locale);
                settings_arc.blocking_write().language = locale.to_string();
                let _ = weak.update(cx, |root, cx| {
                    root.settings.language = locale.to_string();
                    cx.notify();
                });
            }
        })
}

/// Maps a woocraft locale (e.g. `"zh-hans"`, `"zh-hant"`, `"en-us"`) onto the
/// app's locale identifiers (`en_US` / `zh_CN` / `zh_TW`).
fn normalize_woocraft_locale(locale: &str) -> &'static str {
    match locale {
        "zh-hans" => i18n::LOCALE_ZH_CN,
        "zh-hant" => i18n::LOCALE_ZH_TW,
        _ => i18n::LOCALE_EN_US,
    }
}
