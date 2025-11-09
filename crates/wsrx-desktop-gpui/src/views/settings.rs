// Settings view - Application configuration
use gpui::{Context, Render, Window, div, prelude::*};
use crate::styles::colors;
use crate::models::{Settings, Theme};

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
            .bg(gpui::rgba(0x2a2a2aff))
            .rounded_md()
            .child(
                div()
                    .text_color(colors::foreground())
                    .child(label)
            )
            .child(
                div()
                    .text_color(gpui::rgba(0xaaaaaaff))
                    .child(value)
            )
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
                    .child("Settings")
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
                                "Auto-start Daemon",
                                if self.settings.daemon_auto_start { "Enabled" } else { "Disabled" }
                            ))
                            .child(self.render_setting_row(
                                "Show Network Logs",
                                if self.settings.show_network_logs { "Enabled" } else { "Disabled" }
                            ))
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
                                }
                            ))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(self.render_section_title("Logging"))
                            .child(self.render_setting_row(
                                "Log Level",
                                &self.settings.logging_level
                            ))
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
                                    .bg(gpui::rgba(0x2a2a2aff))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_color(colors::foreground())
                                                    .child("WebSocket Reflector X")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(gpui::rgba(0xaaaaaaff))
                                                    .child("Version 0.5.14")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(gpui::rgba(0x888888ff))
                                                    .child("Controlled TCP-over-WebSocket tunneling tool")
                                            )
                                    )
                            )
                    )
            )
    }
}

