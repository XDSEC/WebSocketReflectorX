// Network Logs view - Display real-time logs
use std::collections::VecDeque;

use gpui::{Context, Render, SharedString, Window, div, prelude::*};

use crate::{
    models::{LogEntry, LogLevel},
    styles::colors,
};

pub struct NetworkLogsView {
    logs: VecDeque<LogEntry>,
}

impl NetworkLogsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            logs: VecDeque::new(),
        }
    }

    fn log_level_color(&self, level: LogLevel) -> gpui::Rgba {
        match level {
            LogLevel::Debug => gpui::rgba(0x888888FF),
            LogLevel::Info => colors::foreground(),
            LogLevel::Warn => colors::warning(),
            LogLevel::Error => colors::error(),
        }
    }

    fn log_level_text(&self, level: LogLevel) -> &'static str {
        match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    fn render_log_entry(&self, entry: &LogEntry, index: usize) -> impl IntoElement {
        let id = SharedString::from(format!("log-entry-{}", index));
        let level_color = self.log_level_color(entry.level);
        let level_text = self.log_level_text(entry.level);

        div()
            .id(id)
            .flex()
            .items_start()
            .gap_3()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(gpui::rgba(0x2A2A2AFF))
            .hover(|div| div.bg(gpui::rgba(0x00000020)))
            .child(
                div().flex().items_center().gap_2().min_w_32().child(
                    div()
                        .text_sm()
                        .text_color(gpui::rgba(0xAAAAAAFF))
                        .child(entry.timestamp.clone()),
                ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(level_color)
                    .min_w_16()
                    .child(level_text),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(gpui::rgba(0x888888FF))
                    .min_w_32()
                    .child(entry.target.clone()),
            )
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(colors::foreground())
                    .child(entry.message.clone()),
            )
    }

    fn render_empty_state(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .flex_1()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .text_color(gpui::rgba(0xAAAAAAFF))
                    .child("No logs yet"),
            )
            .child(
                div()
                    .text_color(gpui::rgba(0x888888FF))
                    .child("Logs will appear here when connections are active"),
            )
    }
}

impl Render for NetworkLogsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(gpui::rgba(0x2A2A2AFF))
                    .child(
                        div()
                            .text_xl()
                            .text_color(colors::foreground())
                            .child("Network Logs"),
                    )
                    .child(
                        div().flex().gap_2().child(
                            div()
                                .id("clear-logs-button")
                                .px_3()
                                .py_1()
                                .text_sm()
                                .bg(gpui::rgba(0x444444FF))
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|div| div.bg(gpui::rgba(0x555555FF)))
                                .on_click(cx.listener(|this, _event, _window, cx| {
                                    this.logs.clear();
                                    cx.notify();
                                }))
                                .child("Clear"),
                        ),
                    ),
            )
            .child(if self.logs.is_empty() {
                self.render_empty_state().into_any_element()
            } else {
                let elements: Vec<_> = self
                    .logs
                    .iter()
                    .enumerate()
                    .map(|(i, log)| self.render_log_entry(log, i))
                    .collect();
                div()
                    .flex()
                    .flex_col()
                    .children(elements)
                    .into_any_element()
            })
    }
}
