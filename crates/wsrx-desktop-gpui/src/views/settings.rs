// Settings view - Application configuration
use gpui::{Context, Render, Window, div, prelude::*};

use crate::{
    models::{Settings, Theme},
    styles::colors,
};

pub struct SettingsView {
    settings: Settings,
}

impl SettingsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            settings: Settings::default(),
        }
    }

    fn render_section_title(&self, title: &str) -> impl IntoElement {
        let title = title.to_string();
        div()
            .text_lg()
            .text_color(colors::foreground())
            .mb_3()
            .child(title)
    }

    fn render_setting_row(&self, label: &str, value: &str) -> impl IntoElement {
        let label = label.to_string();
        let value = value.to_string();
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .mb_2()
            .bg(gpui::rgba(0x2A2A2AFF))
            .rounded_md()
            .child(div().text_color(colors::foreground()).child(label))
            .child(div().text_color(gpui::rgba(0xAAAAAAFF)).child(value))
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .child(
                div()
                    .text_xl()
                    .text_color(colors::foreground())
                    .mb_6()
                    .child("Settings"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(self.render_section_title("Application"))
                            .child(self.render_setting_row(
                                "Running in Tray",
                                if self.settings.running_in_tray {
                                    "Enabled"
                                } else {
                                    "Disabled"
                                },
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(self.render_section_title("Appearance"))
                            .child(self.render_setting_row(
                                "Theme",
                                match self.settings.theme {
                                    Theme::Light => "Light",
                                    Theme::Dark => "Dark",
                                    Theme::Auto => "Auto",
                                },
                            ))
                            .child(self.render_setting_row("Language", &self.settings.language)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(self.render_section_title("Daemon"))
                            .child(self.render_setting_row(
                                "API Port",
                                &self.settings.api_port.to_string(),
                            ))
                            .child(self.render_setting_row(
                                "Status",
                                if self.settings.online {
                                    "Online"
                                } else {
                                    "Offline"
                                },
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(self.render_section_title("About"))
                            .child(
                                div()
                                    .px_4()
                                    .py_3()
                                    .bg(gpui::rgba(0x2A2A2AFF))
                                    .rounded_md()
                                    .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_color(colors::foreground())
                                                .child("WebSocket Reflector X"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(gpui::rgba(0xAAAAAAFF))
                                                .child("Version 0.5.14"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(gpui::rgba(0x888888FF))
                                                .child(
                                                    "Controlled TCP-over-WebSocket tunneling tool",
                                                ),
                                        ),
                                ),
                            ),
                    ),
            )
    }
}
