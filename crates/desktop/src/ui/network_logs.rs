use gpui::{Context, IntoElement, ParentElement, Styled, Window, div};
use woocraft::{ActiveTheme, ScrollableElement as _, v_flex};

use crate::{models::LogEntry, ui::RootView};

/// Renders the network logs page, streaming JSON log entries from the log file.
pub(crate) fn render_network_logs(
    _window: &mut Window,
    cx: &mut Context<RootView>,
    root: &mut RootView,
) -> impl IntoElement {
    let theme = cx.theme();
    let logs: Vec<LogEntry> = root.logs().to_vec();

    let foreground = theme.foreground;
    let muted = theme.muted_foreground;
    let blue = theme.blue;
    let warning = theme.warning;
    let danger = theme.danger;
    let border = theme.border;

    v_flex()
        .size_full()
        .overflow_y_scrollbar()
        .py_6()
        .children(logs.iter().map(|log| {
            render_log_row(
                log,
                foreground,
                muted,
                blue,
                warning,
                danger,
                border,
            )
        }))
        .child(div().flex_1())
}

#[allow(clippy::too_many_arguments)]
fn render_log_row(
    log: &LogEntry,
    foreground: gpui::Hsla,
    muted: gpui::Hsla,
    blue: gpui::Hsla,
    warning: gpui::Hsla,
    danger: gpui::Hsla,
    border: gpui::Hsla,
) -> impl IntoElement {
    let level_color = match log.level.as_str() {
        "DEBUG" => muted,
        "INFO" => blue,
        "WARN" => warning,
        _ => danger,
    };
    let timestamp = log
        .timestamp
        .with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    v_flex()
        .px_6()
        .py_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()

                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(level_color)
                        .child(log.level.clone()),
                )
                .child(
                    div()
                        .flex_1()

                        .text_color(muted)
                        .child(log.target.clone()),
                )
                .child(
                    div()

                        .text_color(muted)
                        .child(timestamp),
                ),
        )
        .child(
            div()

                .mt_1()
                .text_color(foreground)
                .child(log.fields.message.clone()),
        )
        .child(div().h_px().mt_3().bg(border))
}
