// Connections view - Manage tunnels and connections
use std::net::SocketAddr;

use gpui::{Context, Render, SharedString, Window, div, prelude::*, px};

use crate::{models::Tunnel, styles::colors};

pub struct ConnectionsView {
    tunnels: Vec<Tunnel>,
    show_add_modal: bool,
    new_tunnel_name: String,
    new_tunnel_local: String,
    new_tunnel_remote: String,
}

impl ConnectionsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            tunnels: Vec::new(),
            show_add_modal: false,
            new_tunnel_name: String::new(),
            new_tunnel_local: String::from("127.0.0.1:8080"),
            new_tunnel_remote: String::from("ws://example.com"),
        }
    }

    fn add_tunnel(&mut self, cx: &mut Context<Self>) {
        // Parse addresses
        let local_addr: Result<SocketAddr, _> = self.new_tunnel_local.parse();
        let remote_addr: Result<SocketAddr, _> = self.new_tunnel_remote.parse();

        if let (Ok(local), Ok(remote)) = (local_addr, remote_addr) {
            let tunnel = Tunnel {
                id: format!("tunnel-{}", self.tunnels.len()),
                name: if self.new_tunnel_name.is_empty() {
                    format!("Tunnel {}", self.tunnels.len() + 1)
                } else {
                    self.new_tunnel_name.clone()
                },
                local_addr: local,
                remote_addr: remote,
                enabled: true,
            };

            self.tunnels.push(tunnel);
            self.show_add_modal = false;
            self.new_tunnel_name.clear();
            self.new_tunnel_local = String::from("127.0.0.1:8080");
            self.new_tunnel_remote = String::from("ws://example.com");
            cx.notify();
        }
    }

    fn remove_tunnel(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.tunnels.len() {
            self.tunnels.remove(index);
            cx.notify();
        }
    }

    fn toggle_tunnel(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(tunnel) = self.tunnels.get_mut(index) {
            tunnel.enabled = !tunnel.enabled;
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
                    .child("No tunnels configured"),
            )
            .child(
                div()
                    .text_color(gpui::rgba(0x888888FF))
                    .child("Click the + button to create your first tunnel"),
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
                            .child("+ Add Tunnel"),
                    ),
            )
            .child(if self.tunnels.is_empty() {
                self.render_empty_state().into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(
                        self.tunnels
                            .iter()
                            .enumerate()
                            .map(|(index, tunnel)| {
                                let id = SharedString::from(format!("tunnel-{}", index));
                                let status_color = if tunnel.enabled {
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
                                            .child(div().w_3().h_3().rounded_full().bg(status_color))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_color(colors::foreground())
                                                            .child(tunnel.name.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(gpui::rgba(0xAAAAAAFF))
                                                            .child(format!(
                                                                "{} → {}",
                                                                tunnel.local_addr, tunnel.remote_addr
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
                                                    .id(SharedString::from(format!("toggle-{}", index)))
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .text_sm()
                                                    .cursor_pointer()
                                                    .bg(if tunnel.enabled {
                                                        gpui::rgba(0x28A745FF)
                                                    } else {
                                                        gpui::rgba(0x555555FF)
                                                    })
                                                    .hover(|div| {
                                                        div.bg(if tunnel.enabled {
                                                            gpui::rgba(0x218838FF)
                                                        } else {
                                                            gpui::rgba(0x666666FF)
                                                        })
                                                    })
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        this.toggle_tunnel(index, cx);
                                                    }))
                                                    .child(if tunnel.enabled { "Enabled" } else { "Disabled" }),
                                            )
                                            .child(
                                                div()
                                                    .id(SharedString::from(format!("delete-{}", index)))
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .text_sm()
                                                    .cursor_pointer()
                                                    .bg(colors::error())
                                                    .hover(|div| div.bg(gpui::rgba(0xFF6655FF)))
                                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                                        this.remove_tunnel(index, cx);
                                                    }))
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
                            .child("Add New Tunnel"),
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
                                    .child("Tunnel 1"),
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
                                    .child(self.new_tunnel_local.clone()),
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
                                    .child(self.new_tunnel_remote.clone()),
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
                                        this.add_tunnel(cx);
                                    }))
                                    .child("Add"),
                            ),
                    ),
            )
    }
}
