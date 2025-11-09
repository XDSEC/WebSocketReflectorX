// Connections view - Manage tunnels and connections
use gpui::{Context, Render, Window, div, prelude::*};
use crate::styles::colors;

pub struct ConnectionsView {
}

impl ConnectionsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
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
                    .text_xl()
                    .text_color(colors::foreground())
                    .mb_4()
                    .child("Connections")
            )
            .child(
                div()
                    .text_color(colors::foreground())
                    .child("No tunnels configured yet")
            )
    }
}

