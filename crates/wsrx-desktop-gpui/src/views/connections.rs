// Connections view - Manage instances (tunnels) and connections
use gpui::{Context, Render, SharedString, Window, div, prelude::*, px};

use crate::{models::Instance, styles::colors};

pub struct ConnectionsView {
    instances: Vec<Instance>,
    show_add_modal: bool,
    new_instance_label: String,
    new_instance_local: String,
    new_instance_remote: String,
}

impl ConnectionsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            instances: Vec::new(),
            show_add_modal: false,
            new_instance_label: String::new(),
            new_instance_local: String::from("127.0.0.1:8080"),
            new_instance_remote: String::from("ws://example.com"),
        }
    }

    fn add_instance(&mut self, cx: &mut Context<Self>) {
        let instance = Instance {
            label: if self.new_instance_label.is_empty() {
                format!("Instance {}", self.instances.len() + 1)
            } else {
                self.new_instance_label.clone()
            },
            local: self.new_instance_local.clone(),
            remote: self.new_instance_remote.clone(),
            latency: -1, // Not connected yet
            scope_host: "default-scope".to_string(),
        };

        self.instances.push(instance);
        self.show_add_modal = false;
        self.new_instance_label.clear();
        self.new_instance_local = String::from("127.0.0.1:8080");
        self.new_instance_remote = String::from("ws://example.com");
        cx.notify();
    }

    fn remove_instance(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.instances.len() {
            self.instances.remove(index);
            cx.notify();
        }
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
                    .child("No instances configured"),
            )
            .child(
                div()
                    .text_color(gpui::rgba(0x888888FF))
                    .child("Click the + button to create your first instance"),
            )
    }
}

impl Render for ConnectionsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(colors::foreground())
                            .child("Connections"),
                    )
                    .child(
                        div()
                            .id("add-tunnel-button")
                            .px_4()
                            .py_2()
                            .bg(colors::accent())
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|div| div.bg(gpui::rgba(0x0088DDFF)))
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.show_add_modal = true;
                                cx.notify();
                            }))
                            .child("+ Add Instance"),
                    ),
            )
            .child(if self.instances.is_empty() {
                self.render_empty_state().into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(
                        self.instances
                            .iter()
                            .enumerate()
                            .map(|(index, instance)| {
                                let id = SharedString::from(format!("instance-{}", index));
                                let latency_text = if instance.latency >= 0 {
                                    format!("{} ms", instance.latency)
                                } else {
                                    "--".to_string()
                                };
                                let status_color = if instance.latency >= 0 {
                                    colors::success()
                                } else {
                                    gpui::rgba(0x888888FF)
                                };

                                div()
                                    .id(id)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px_4()
                                    .py_3()
                                    .mb_2()
                                    .bg(gpui::rgba(0x2A2A2AFF))
                                    .rounded_md()
                                    .hover(|div| div.bg(gpui::rgba(0x333333FF)))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .child(
                                                div().w_3().h_3().rounded_full().bg(status_color),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_color(colors::foreground())
                                                            .child(instance.label.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(gpui::rgba(0xAAAAAAFF))
                                                            .child(format!(
                                                                "{} → {}",
                                                                instance.local, instance.remote
                                                            )),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .text_sm()
                                                    .text_color(status_color)
                                                    .child(latency_text),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!(
                                                        "delete-{}",
                                                        index
                                                    )))
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .text_sm()
                                                    .cursor_pointer()
                                                    .bg(colors::error())
                                                    .hover(|div| div.bg(gpui::rgba(0xFF6655FF)))
                                                    .on_click(cx.listener(
                                                        move |this, _event, _window, cx| {
                                                            this.remove_instance(index, cx);
                                                        },
                                                    ))
                                                    .child("Delete"),
                                            ),
                                    )
                            })
                            .collect::<Vec<_>>(),
                    )
                    .into_any_element()
            })
            .when(self.show_add_modal, |div| {
                div.child(self.render_add_modal(cx))
            })
    }
}

impl ConnectionsView {
    fn render_add_modal(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000080))
            .child(
                div()
                    .bg(colors::background())
                    .rounded_lg()
                    .p_6()
                    .w(px(400.0))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(colors::foreground())
                            .child("Add New Instance"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors::foreground())
                                    .child("Tunnel Name"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(gpui::rgba(0x2A2A2AFF))
                                    .text_color(colors::foreground())
                                    .child(self.new_instance_label.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors::foreground())
                                    .child("Local Address"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(gpui::rgba(0x2A2A2AFF))
                                    .text_color(colors::foreground())
                                    .child(self.new_instance_local.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors::foreground())
                                    .child("Remote Address"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(gpui::rgba(0x2A2A2AFF))
                                    .text_color(colors::foreground())
                                    .child(self.new_instance_remote.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .justify_end()
                            .child(
                                div()
                                    .id("cancel-button")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .bg(gpui::rgba(0x444444FF))
                                    .cursor_pointer()
                                    .hover(|div| div.bg(gpui::rgba(0x555555FF)))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.show_add_modal = false;
                                        cx.notify();
                                    }))
                                    .child("Cancel"),
                            )
                            .child(
                                div()
                                    .id("add-button")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .bg(colors::accent())
                                    .cursor_pointer()
                                    .hover(|div| div.bg(gpui::rgba(0x0088DDFF)))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.add_instance(cx);
                                    }))
                                    .child("Add"),
                            ),
                    ),
            )
    }
}
