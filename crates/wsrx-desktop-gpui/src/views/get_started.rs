// Get Started view - Onboarding page
use gpui::{Context, Render, Window, div, prelude::*};

use crate::styles::colors;

pub struct GetStartedView {}

impl GetStartedView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for GetStartedView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .text_color(colors::foreground())
                    .child("Welcome to WebSocket Reflector X"),
            )
            .child(
                div()
                    .text_base()
                    .text_color(colors::foreground())
                    .child("Get started by creating your first tunnel"),
            )
    }
}
