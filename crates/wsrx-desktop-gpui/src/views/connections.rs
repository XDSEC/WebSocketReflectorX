// Connections view - Manage tunnels and connections
use gpui::{Context, Render, SharedString, Window, div, prelude::*};

use crate::{models::Tunnel, styles::colors};

pub struct ConnectionsView {
    tunnels: Vec<Tunnel>,
}

impl ConnectionsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            tunnels: Vec::new(),
        }
    }

    fn render_tunnel_item(&self, tunnel: &Tunnel, index: usize) -> impl IntoElement {
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
                    .text_sm()
                    .text_color(gpui::rgba(0xAAAAAAFF))
                    .child(if tunnel.enabled {
                        "Enabled"
                    } else {
                        "Disabled"
                    }),
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("+ Add Tunnel"),
                    ),
            )
            .child(if self.tunnels.is_empty() {
                self.render_empty_state().into_any_element()
            } else {
                let elements: Vec<_> = self
                    .tunnels
                    .iter()
                    .enumerate()
                    .map(|(i, tunnel)| self.render_tunnel_item(tunnel, i))
                    .collect();
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(elements)
                    .into_any_element()
            })
    }
}
